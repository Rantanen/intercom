
use intercom::*;
use std::path::PathBuf;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
const CLSID_SetupConfiguration : GUID = GUID {
    data1: 0x177f0c4a,
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

#[com_interface( "B41463C3-8866-43B5-BC33-2B0676F7F42E" )]
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

#[com_interface( "6380BCFF-41D3-4B2E-8B2E-BF8A6810C848" )]
pub trait IEnumSetupInstances
{
    fn next(
        &self,
        celt : u32
    ) -> ComResult< ( ComItf< ISetupInstance >, u32 ) >;

    fn skip( &self, celt : u32 ) -> ComResult<()>;

    fn reset( &self ) -> ComResult<()>;

    fn clone( &self ) -> ComResult< ComItf< IEnumSetupInstances > >;
}

#[com_interface( "26AAB78C-4A60-49D6-AF3B-3C35BC93365D" )]
pub trait ISetupConfiguration2
{
    fn enum_instances( &self )
        -> ComResult< ComItf< IEnumSetupInstances > >;

    fn get_instance_for_current_process( &self )
        -> ComResult< ComItf< ISetupInstance > >;

    fn get_instance_for_path( &self, path : String )
        -> ComResult< ComItf< ISetupInstance > >;

    fn enum_all_instances( &self )
        -> ComResult< ComItf< IEnumSetupInstances > >;
}

pub struct ToolPaths {
    pub mt : PathBuf,
    pub rc : PathBuf,
    pub midl : PathBuf,

    pub vs_bin : PathBuf,

    pub libs : Vec<PathBuf>,
    pub incs : Vec<PathBuf>,
}

fn find_path( roots : &[&PathBuf], path : &str ) -> Option< PathBuf > {

    for root in roots {
        let root_str = root.to_string_lossy();
        let pattern = format!( "{}/**/{}", root_str, path );
        let options = ::glob::MatchOptions {
            case_sensitive: false,
            require_literal_separator: true,
            require_literal_leading_dot: true,
        };
        for entry in ::glob::glob_with( &pattern, &options ).unwrap() {
            if let Ok( path ) = entry {
                return Some( PathBuf::from( path ) );
            }
        }
    }

    None
}

fn get_compiler_paths( paths : &[ PathBuf ] ) -> Vec<PathBuf>
{
    let mut dir_paths = paths.into_iter()
        .filter_map(
            |p| p.parent().and_then(
                |p| Some( p.to_owned() ) ) )
        .collect::<Vec<_>>();
    dir_paths.sort();
    dir_paths.dedup();

    dir_paths
}

pub fn get_tool_paths() -> Result<ToolPaths, String> {
    ::intercom::runtime::initialize()
            .map_err( |hr| format!( "Failed to initialize COM: {:?}", hr ) )?;

    let setup_conf = ComItf::<ISetupConfiguration2>
            ::create( CLSID_SetupConfiguration ).unwrap();
    let instances = setup_conf.enum_instances().unwrap();

    let ( next, _ ) = instances.next( 1 ).unwrap();
    let installation_path = next.get_installation_path().unwrap();

    let hklm = ::winreg::RegKey::predef( ::winreg::enums::HKEY_LOCAL_MACHINE );
    let installed_roots = hklm.open_subkey_with_flags(
            r"SOFTWARE\Microsoft\Windows Kits\Installed Roots",
            ::winreg::enums::KEY_READ ).unwrap();
    let kitroot : String = installed_roots.get_value( "KitsRoot10" ).unwrap();
    let kitversion = installed_roots.enum_keys().nth( 0 ).unwrap().unwrap();

    ::intercom::runtime::uninitialize();

    let sdkroot = PathBuf::from( &kitroot );
    let sdk_lib_root = sdkroot.join( "Lib" ).join( &kitversion );
    let sdk_include_root = sdkroot.join( "Include" ).join( &kitversion );

    let mut vsroot = PathBuf::from( &installation_path );
    vsroot.push( r"VC\Tools\MSVC" );

    let libs = get_compiler_paths( &vec![
            find_path( &[ &sdk_lib_root ], r"x64\ucrt.lib" ).unwrap(),
            find_path( &[ &sdk_lib_root ], r"x64\ole32.lib" ).unwrap(),
            find_path( &[ &vsroot ], r"lib\x64\vcruntime.lib" ).unwrap(),
        ] );

    let incs = get_compiler_paths( &vec![
            find_path( &[ &sdk_include_root ], r"oaidl.idl" ).unwrap(),
            find_path( &[ &sdk_include_root ], r"wtypes.idl" ).unwrap(),
        ] );

    let mut vs_bin = find_path( &[ &vsroot ], r"Hostx64\x64\cl.exe" ).unwrap();
    vs_bin.pop();

    Ok( ToolPaths {
        mt: sdkroot.join( format!( r"bin\{}\x64\mt.exe", kitversion ) ),
        rc: sdkroot.join( format!( r"bin\{}\x64\rc.exe", kitversion ) ),
        midl: sdkroot.join( format!( r"bin\{}\x64\midl.exe", kitversion ) ),

        vs_bin: vs_bin,

        libs: libs,
        incs: incs,
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
        ::intercom::runtime::initialize().unwrap();

        let setup_conf = ComItf ::<ISetupConfiguration2>
                ::create( CLSID_SetupConfiguration ).unwrap();
        let instances = setup_conf.enum_instances().unwrap();

        let ( next, _ ) = instances.next( 1 ).unwrap();
        let installation_name = next.get_installation_name().unwrap();
        let installation_path = next.get_installation_path().unwrap();
        let installation_version = next.get_installation_version().unwrap();

        let vswhere_path = get_intercom_root().join( "scripts/vswhere.exe" );
        let installation_name_output = Command::new( &vswhere_path )
                .arg( "/nologo" )
                .arg( "-property" ).arg( "installationName" )
                .output()
                .unwrap();
        let installation_path_output = Command::new( &vswhere_path )
                .arg( "/nologo" )
                .arg( "-property" ).arg( "installationPath" )
                .output()
                .unwrap();
        let installation_version_output = Command::new( &vswhere_path )
                .arg( "/nologo" )
                .arg( "-property" ).arg( "installationVersion" )
                .output()
                .unwrap();

        let installation_name_actual = String::from_utf8_lossy(
                &installation_name_output.stdout );
        let installation_path_actual = String::from_utf8_lossy(
                &installation_path_output.stdout );
        let installation_version_actual = String::from_utf8_lossy(
                &installation_version_output.stdout );

        assert_eq!( installation_name_actual.trim(), installation_name );
        assert_eq!( installation_path_actual.trim(), installation_path );
        assert_eq!( installation_version_actual.trim(), installation_version );

        ::intercom::runtime::uninitialize();
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

    #[test]
    fn get_tool_paths_returns_valid_paths() {
        ::intercom::runtime::initialize().unwrap();

        let paths = get_tool_paths().unwrap();
        assert!( paths.mt.exists() );
        assert!( paths.rc.exists() );
        assert!( paths.midl.exists() );

        ::intercom::runtime::uninitialize();
    }
}
