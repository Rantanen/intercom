
extern crate cc;

use ::std::env;
use ::std::path::Path;
use ::std::fs::File;
use ::std::process::Command;
use ::std::io::Write;

use ::host;

mod setup_configuration;

pub fn build() {

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
                .expect( &format!( "Could not create file: {:?}", idl_path ) );
        let model = ::intercom::generators::idl::IdlModel::from_path(
                    Path::new( &toml_dir ) )
                .expect( "Failed to form IDL from the sources" );
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
                .unwrap().success() {

                    panic!( "midl.exe did not execute successfully" );
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
                .unwrap().success() {

                    panic!( "mt.exe did not execute successfully" );
                }
    }

    // Create a resource script that references the tlb and the manifest.
    {
        let mut rc = File::create( &rc_path ).unwrap();
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
                    .unwrap().success() {

                        panic!( "rc.exe did not execute successfully" );
                    }

            // 'rc.exe' will result in 'foo.res'. We'll need 'foo.res.lib' as
            // rustc will insist on '.lib' extension.
            ::std::fs::rename( &res_path, &lib_path )
                    .expect( &format!(
                            "Failed to rename {:?} to {:?}",
                            res_path, lib_path ) );

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
                    .unwrap().success() {

                        panic!( "windres.exe did not execute successfully" );
                    }
            cc::Build::new()
                    .object( &res_path )
                    .compile( &format!( "lib{}.res.a", pkg_name ) );
        }
    }
}
