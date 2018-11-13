
extern crate cc;

use ::std::env;
use ::std::path::Path;
use ::std::fs::File;
use ::std::process::Command;
use ::std::io::Write;

use ::host;
use ::BuildError;

mod setup_configuration;

/// Executes the Windows-specific build steps.
///
/// # Arguments
///
/// * `all_type_systems` -
///     True to include both Automation and Raw type systems in the embedded IDLs. Normally the
///     build only includes Automation type system in the embedded IDL.
pub fn build( all_type_systems : bool ) -> Result<(), BuildError> {

    // Get the host.
    let host = host::get_host();

    // Read the Cargo environment variables.
    let out_dir = env::var( "OUT_DIR" )
            .expect( "OUT_DIR not set" );
    let pkg_name = env::var( "CARGO_PKG_NAME" )
            .expect( "CARGO_PKG_NAME not set" );
    let toml_dir = env::var( "CARGO_MANIFEST_DIR" )
            .expect( "CARGO_MANIFEST_DIR not set" );

    // Derive the various intermediate file names.
    let dll_name = format!( "{}.dll", pkg_name );
    let manifest_path = Path::new( &out_dir )
            .join( format!( "{}.manifest", pkg_name ) );
    let idl_path = Path::new( &out_dir )
            .join( format!( "{}.idl", pkg_name ) );
    let tlb_path = Path::new( &out_dir )
            .join( format!( "{}.tlb", pkg_name ) );
    let rc_path = Path::new( &out_dir )
            .join( format!( "{}.rc", pkg_name ) );

    let res_name = format!( "{}.res", pkg_name );
    let res_path = Path::new( &out_dir ).join( &res_name );

    // The lib name will depend on the compiler as the different linkers,
    // dictated by the compiler toolchain, will have different requirements for
    // the file names.
    let lib_name = match host.compiler {
        host::Compiler::Msvc => format!( "{}.lib", res_name ),
        host::Compiler::Gnu => format!( "lib{}.a", res_name ),
    };

    let lib_path = Path::new( &out_dir ).join( &lib_name );

    // Generate IDL using intercom_utils.
    {
        let mut idl = File::create( &idl_path )
                .map_err( |e| BuildError::IoError(
                        format!( "Failed to create file {}",
                                 &idl_path.to_string_lossy() ),
                        e ) )?;
        let model = ::intercom_common::generators::idl::IdlModel::from_path(
                    Path::new( &toml_dir ), all_type_systems )
                .map_err( |e| BuildError::ParseError( e.to_string() ) )?;
        model.write( &mut idl )
                .expect( "Failed to write IDL to file" );
        idl.sync_all()
                .expect( "Writing IDL failed" );
    }

    // Get the various VS/WindowsKits paths.
    let paths = setup_configuration::get_tool_paths()
                .expect( "Could not resolve Windows toolchain" );

    // Turn the lib paths to LIB env variable format.
    let lib_paths = paths.libs
            .iter()
            .map( |l| l.to_string_lossy() )
            .collect::<Vec<_>>();
    let libs = lib_paths.join( ";" );

    // Turn the include paths to INCLUDE env variable format.
    let inc_paths = paths.incs
            .iter()
            .map( |l| l.to_string_lossy() )
            .collect::<Vec<_>>();
    let incs = inc_paths.join( ";" );

    // Invoke midl.exe to turn the idl into tlb.
    {
        if ! Command::new( paths.midl )
                .env( "PATH",
                    format!( "{};{}",
                        &paths.vs_bin.to_string_lossy(),
                        env::var( "PATH" )
                            .unwrap_or_else( |_| "".to_owned() ) ) )
                .env( "LIB", libs )
                .env( "INCLUDE", incs )
                .current_dir( &out_dir )
                .arg( &idl_path ).arg( "/tlb" ).arg( &tlb_path )
                .status()
                .map_err( |e| BuildError::CommandError(
                        format!( "Failed to execute MIDL: {:?}", e ) ) )?
                .success() {

                    // Command failed.
                    return Err( BuildError::CommandError(
                            "MIDL did not execute successfully".to_string() ) );
                }
    }

    // Invoke mt.exe to create a manifest from the tlb.
    {
        if ! Command::new( paths.mt )
                .current_dir( &out_dir )
                .arg( format!( "-tlb:{}", tlb_path.to_string_lossy() ) )
                .arg( format!( "-dll:{}", dll_name ) )
                .arg( format!( "-out:{}", manifest_path.to_string_lossy() ) )
                .status()
                .map_err( |e| BuildError::CommandError(
                        format!( "Failed to execute Manifest Tool: {:?}", e ) ) )?
                .success() {

                    return Err( BuildError::CommandError(
                        "Manifest Tool did not execute successfully".to_string() ) );
                }
    }

    // Create a resource script that references the tlb and the manifest.
    {
        let mut rc = File::create( &rc_path )
                .map_err( |e| BuildError::IoError(
                        format!( "Failed to create file {}",
                                 &rc_path.to_string_lossy() ),
                        e ) )?;
        writeln!(
            rc, "1 24 \"{}\"",
            &manifest_path.to_string_lossy().replace( r"\", r"\\" )
        ).expect( "Could not write resource script." );
        writeln!(
            rc, "1 typelib \"{}\"",
            &tlb_path.to_string_lossy().replace( r"\", r"\\" )
        ).expect( "Could not write resource script." );
    }

    // Compile the resource script into a resource object file.
    // The final command depends on the comiler toolchain we use as MSVC will
    // use rc.exe while MinGW will use windres.exe.
    match host.compiler {
        host::Compiler::Msvc => {
            if ! Command::new( paths.rc )
                    .current_dir( &out_dir )
                    .arg( &rc_path )
                    .status()
                    .map_err( |e| BuildError::CommandError(
                            format!( "Failed to execute Microsoft Resource Compiler: {:?}", e ) ) )?
                    .success() {

                        return Err( BuildError::CommandError(
                            "Microsoft Resource Compiler did not execute successfully".to_string() ) );
                    }

            // 'rc.exe' will result in 'foo.res'. We'll need 'foo.res.lib' as
            // rustc will insist on '.lib' extension.
            ::std::fs::rename( &res_path, &lib_path )
                    .map_err( |e| BuildError::IoError(
                        format!( "Failed to rename {} to {}",
                                 &res_path.to_string_lossy(),
                                 &lib_path.to_string_lossy() ), e ) )?;

            // Instruct rustc to link the resource dll.
            println!( "cargo:rustc-link-lib=dylib={}", res_name );
            println!( "cargo:rustc-link-search=native={}", out_dir );
        },
        host::Compiler::Gnu => {
            if ! Command::new( paths.rc )
                    .current_dir( &out_dir )
                    .arg( "-J" ).arg( "rc" )
                    .arg( "-i" ).arg( &rc_path )
                    .arg( "-O" ).arg( "coff" )
                    .arg( "-o" ).arg( &res_path )
                    .status()
                    .map_err( |e| BuildError::CommandError(
                            format!( "Failed to execute GNU windres: {:?}", e ) ) )?
                    .success() {

                        return Err( BuildError::CommandError(
                            "GNU windres did not execute successfully".to_string() ) );
                    }
            cc::Build::new()
                    .object( &res_path )
                    .compile( &format!( "lib{}.res.a", pkg_name ) );
        }
    }

    Ok(())
}
