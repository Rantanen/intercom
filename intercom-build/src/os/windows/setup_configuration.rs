
extern crate glob;
extern crate winapi;
use self::winapi::shared::minwindef::{DWORD, HKEY};
use self::winapi::um::winnt::{KEY_READ};
use self::winapi::um::winreg;

extern crate intercom;
use intercom::*;

use std::path::PathBuf;
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use ::host;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
const CLSID_SetupConfiguration : GUID = GUID {
    data1: 0x177f_0c4a,
    data2: 0x1cd3,
    data3: 0x4de7,
    data4: [ 0xa3, 0x2c, 0x71, 0xdb, 0xbb, 0x9f, 0xa3, 0x6d ]
};

#[repr(C)]
#[derive(Default)]
pub struct FILETIME {
    low_part : u32,
    high_part : u32,
}

#[com_interface( com_iid = "B41463C3-8866-43B5-BC33-2B0676F7F42E" )]
pub trait ISetupInstance
{
    fn get_instance_id( &self ) -> ComResult< String >;

    fn get_install_date( &self ) -> ComResult< FILETIME >;

    fn get_installation_name( &self ) -> ComResult< String >;

    fn get_installation_path( &self ) -> ComResult< String >;

    fn get_installation_version( &self ) -> ComResult< String >;

    fn get_display_name( &self, lcid: u32 ) -> ComResult< String >;

    fn get_description( &self, lcid: u32 ) -> ComResult< String >;

    fn resolve_path( &self, path : String ) -> ComResult< String >;
}

#[com_interface( com_iid = "6380BCFF-41D3-4B2E-8B2E-BF8A6810C848" )]
pub trait IEnumSetupInstances
{
    fn next(
        &self,
        celt : u32
    ) -> ComResult< ( ComItf< dyn ISetupInstance >, u32 ) >;

    fn skip( &self, celt : u32 ) -> ComResult<()>;

    fn reset( &self ) -> ComResult<()>;

    fn clone( &self ) -> ComResult< ComItf< dyn IEnumSetupInstances > >;
}

#[com_interface( com_iid = "26AAB78C-4A60-49D6-AF3B-3C35BC93365D" )]
pub trait ISetupConfiguration2
{
    fn enum_instances( &self )
        -> ComResult< ComItf< dyn IEnumSetupInstances > >;

    fn get_instance_for_current_process( &self )
        -> ComResult< ComItf< dyn ISetupInstance > >;

    fn get_instance_for_path( &self, path : String )
        -> ComResult< ComItf< dyn ISetupInstance > >;

    fn enum_all_instances( &self )
        -> ComResult< ComItf< dyn IEnumSetupInstances > >;
}

pub struct ToolPaths {
    pub mt : PathBuf,
    pub rc : PathBuf,
    pub midl : PathBuf,

    pub vs_bin : PathBuf,

    pub libs : Vec<PathBuf>,
    pub incs : Vec<PathBuf>,
}

/// Finds a path for the target within the roots.
///
/// * `roots` - Roots to search the target under.
/// * `target` - Path to send. May contain directories, such as `x86/foo.lib`.
///
/// Returns the full path to the target or None if the target was not found.
fn find_path( roots : &[&PathBuf], path : &str ) -> Option< PathBuf > {

    // Glob options. We'll want case insensitive searching thanks to Windows.
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: true,
        require_literal_leading_dot: true,
    };

    // Go through all the roots one by one.
    for root in roots {

        // Create the glob pattern. Concatenate the root with the target and
        // throw ** in between.
        let root_str = root.to_string_lossy();
        let pattern = format!( "{}/**/{}", root_str, path );

        // Go through all entries.
        for entry in glob::glob_with( &pattern, &options ).unwrap() {
            if let Ok( entry_path ) = entry {

                // The first entry we find, we'll return.
                return Some( entry_path );
            }
        }
    }

    // No paths found. Return none as the default.
    None
}

/// Get compiler paths from absolute file paths.
///
/// The compiler takes paths to the directories containing the files we want
/// as LIB and INCLUDE paths. This function strips the file name from the path
/// and deduplicates the resulting directories.
fn get_compiler_paths( paths : &[ PathBuf ] ) -> Vec<PathBuf>
{
    // Get the directory paths from the file paths.
    let mut dir_paths = paths.iter()
        .filter_map(
            |p| p.parent().and_then(
                |p| Some( p.to_owned() ) ) )
        .collect::<Vec<_>>();

    // De-duplicate the directories.
    dir_paths.sort();
    dir_paths.dedup();

    dir_paths
}

/// Gets the Windows kit path and version.
fn get_kit_path() -> Result<( String, String ), String> {

    unsafe {

        // Turn the various string constants into WCHAR* strings for winapi.
        let key = OsString::from( "SOFTWARE\\Microsoft\\Windows Kits\\Installed Roots\0" )
                    .encode_wide()
                    .collect::<Vec<_>>();
        let value = OsString::from( "KitsRoot10\0" )
                    .encode_wide()
                    .collect::<Vec<_>>();

        // Open the Installed Roots registry key.
        let mut hkey_roots : HKEY = ::std::ptr::null_mut();
        let mut disp : DWORD = 0;
        winreg::RegCreateKeyExW(
            winreg::HKEY_LOCAL_MACHINE,
            key.as_slice().as_ptr(),
            0, ::std::ptr::null_mut(), 0,
            KEY_READ,
            ::std::ptr::null_mut(),
            &mut hkey_roots,
            &mut disp );
        if hkey_roots.is_null() {
            return Err( "Could not open Windows Kits registry key".to_owned() );
        }

        // Get the 'KitsRoot10' value from the above key.
        let mut buf = Vec::with_capacity( 250 );
        let mut buf_len : DWORD = 250;
        let hr = winreg::RegQueryValueExW(
                hkey_roots, value.as_slice().as_ptr(), ::std::ptr::null_mut(),
                ::std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut _,
                &mut buf_len );
        if hr != 0 {
            return Err( "Could not read KitsRoot10 registry value".to_owned() );
        }

        // Turn the value into a rust string.
        buf.set_len( ( buf_len / 2 ) as usize - 1 );
        let kitsroot_osstring = OsString::from_wide( &buf );
        let kitsroot = kitsroot_osstring.to_string_lossy().to_owned();

        // Get the first sub-key under the Installed Roots. We don't really
        // care which kit version this is as long as it's one that is present
        // on the system.
        let mut buf = Vec::with_capacity( 250 );
        let mut buf_len : DWORD = 250;
        let hr = winreg::RegEnumKeyExW(
            hkey_roots, 0,
            buf.as_mut_ptr(), &mut buf_len,
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut() );
        if hr != 0 {
            return Err( "Could not read Windows Kits versions".to_owned() );
        }

        // Turn the installed version to a rust string.
        buf.set_len( buf_len as usize );
        let kitsversion_osstring = OsString::from_wide( &buf );
        let kitsversion = kitsversion_osstring.to_string_lossy().to_owned();

        Ok( ( kitsroot.into_owned(), kitsversion.into_owned() ) )
    }
}

/// Gets the VS 2017+ installation path.
fn get_vs_path() -> Result<String, String> {

    intercom::runtime::initialize()
            .map_err( |hr| format!( "Failed to initialize COM: {:?}", hr ) )?;

    let installation_path = {

        // Get the COM API entry point for the new VS configuration API.
        let setup_conf = ComRc::<dyn ISetupConfiguration2>
                ::create( CLSID_SetupConfiguration ).unwrap();

        // Get the first instance.
        let instances = setup_conf.enum_instances().unwrap();
        let ( next, _ ) = instances.next( 1 ).unwrap();

        // Read the installation path. We don't care about anything else for now.
        next.get_installation_path().unwrap()
    };

    intercom::runtime::uninitialize();

    Ok( installation_path )
}

pub fn get_tool_paths() -> Result<ToolPaths, String> {

    // Get the Windows kit base path and version.
    let ( kitroot, kitversion ) = get_kit_path()?;
    let kitroot = PathBuf::from( &kitroot );

    // Form the kit lib and include paths.
    // let kit_lib_root = kitroot.join( "Lib" ).join( &kitversion );
    let kit_include_root = kitroot.join( "Include" ).join( &kitversion );

    // Get the Visual C++ tools path.
    let vsroot = PathBuf::from( &get_vs_path()? ).join( r"VC\Tools\MSVC" );

    // Resolve the required libs.
    let libs = get_compiler_paths( &[

            // These paths cover most of the lib dirs. However no idea if we
            // need them so they are commented out until something complains.

            // find_path( &[ &kit_lib_root ], r"x64\ucrt.lib" ).unwrap(),
            // find_path( &[ &kit_lib_root ], r"x64\ole32.lib" ).unwrap(),
            // find_path( &[ &vsroot ], r"lib\x64\vcruntime.lib" ).unwrap(),
        ] );

    // Resolve the required include directories.
    let incs = get_compiler_paths( &[
            find_path( &[ &kit_include_root ], r"oaidl.idl" ).unwrap(),
            find_path( &[ &kit_include_root ], r"wtypes.idl" ).unwrap(),
        ] );

    // We'll need the C compiler for the preprocessing of the IDL.
    let mut vs_bin = find_path( &[ &vsroot ], r"Hostx64\x64\cl.exe" ).unwrap();
    vs_bin.pop();

    // Resource compiler depends on the toolchain.
    let host = host::get_host();
    let rc = match host.compiler {
        host::Compiler::Msvc =>
            kitroot.join( format!( r"bin\{}\x64\rc.exe", kitversion ) ),
        host::Compiler::Gnu =>
            PathBuf::from( "windres.exe" ),  // Needs to be on PATH.
    };

    Ok( ToolPaths {
        mt: kitroot.join( format!( r"bin\{}\x64\mt.exe", kitversion ) ),
        midl: kitroot.join( format!( r"bin\{}\x64\midl.exe", kitversion ) ),

        rc, vs_bin, libs, incs
    } )
}

#[cfg(test)]
mod test
{
    use super::*;
    use ::std::process::Command;
    use ::std::path::PathBuf;

    #[test]
    fn get_vs_2017_details() {
        intercom::runtime::initialize().unwrap();
        {

            let setup_conf = ComRc::<dyn ISetupConfiguration2>
                    ::create( CLSID_SetupConfiguration ).unwrap();
            let instances = setup_conf.enum_instances().unwrap();

            let ( next, _ ) = instances.next( 1 ).unwrap();
            let installation_name = next.get_installation_name().unwrap();
            let installation_path = next.get_installation_path().unwrap();
            let installation_version = next.get_installation_version().unwrap();

            let installation_name_actual = get_vswhere_property( "installationName" );
            let installation_path_actual = get_vswhere_property( "installationPath" );
            let installation_version_actual = get_vswhere_property( "installationVersion" );

            assert_eq!( installation_name_actual.trim(), installation_name );
            assert_eq!( installation_path_actual.trim(), installation_path );
            assert_eq!( installation_version_actual.trim(), installation_version );

        }
        intercom::runtime::uninitialize();
    }

    fn get_intercom_root() -> PathBuf {
        let mut root_path = ::std::env::current_exe().unwrap();
        loop {
            if root_path.join( "intercom-build" ).exists() {
                break;
            }
            assert!( root_path.pop() );
        }

        root_path
    }

    /// Gets the specified vswhere property.
    fn get_vswhere_property(
        property: &str,
    ) -> String {

        // First attempt to get the property without the "BuildTools" product.
        // If we don't get a result then try "BuildTools" explicitly.
        // We do this in two steps to avoid issues when the developer has both the Visual Studio and the BuildTools installed.
        let v = Some("[15, 16)");
        match get_vswhere_property_for_products( property, &[], v )
        {
            Ok( value ) => value,
            Err( _ ) => get_vswhere_property_for_products(
                    property, &[ "BuildTools" ], v ).unwrap(),
        }
    }

    /// Gets the specified vswhere property for the specified products.
    fn get_vswhere_property_for_products(
        property: &str,
        products: &[&str],
        version: Option<&str>,
    ) -> Result<String, String>
    {
        let vswhere_path = get_intercom_root().join( "scripts/vswhere.exe" );
        let mut cmd = Command::new( &vswhere_path );
        cmd.arg( "/nologo" )
            .arg( "-property" ).arg( property );
        if products.len() > 0 {
            cmd.arg( "-products" ).args( products );
        }
        if let Some(v) = version {
            cmd.arg( "-version" ).arg( v );
        }
        let property_from_output = cmd.output().unwrap().stdout;

        // Ensure we got exactly one result.
        let property_from_output: String = String::from_utf8_lossy( &property_from_output ).to_owned().to_string();
        let mut values: Vec<&str> = property_from_output
                .split( "\r\n" )
                .filter_map( |s| { if s.is_empty() { None } else { Some( s ) } } )
                .collect();

        values.sort();
        values.first()
                .map( |l| l.to_string() )
                .ok_or_else( || format!( "No VS build tools installed" ) )
    }

    #[test]
    fn get_tool_paths_returns_valid_paths() {
        intercom::runtime::initialize().unwrap();

        // The HOST env variable is set for build.rs, not for tests.
        // We need to fake one here.
        ::std::env::set_var( "HOST", "x86_64-pc-windows-msvc" );

        let paths = get_tool_paths().unwrap();
        assert!( paths.mt.exists() );
        assert!( paths.rc.exists() );
        assert!( paths.midl.exists() );

        intercom::runtime::uninitialize();
    }
}
