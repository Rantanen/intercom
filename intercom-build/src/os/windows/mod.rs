
extern crate intercom_utils;

use ::std::env;
use ::std::path::Path;
use ::std::fs::File;
use ::std::process::Command;
use ::std::io::Write;

mod setup_configuration;

pub fn build() {

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
    let res_path = Path::new( &out_dir )
            .join( format!( "{}.res", pkg_name ) );
    let lib_path = Path::new( &out_dir )
            .join( format!( "{}.res.lib", pkg_name ) );

    // Generate IDL using intercom_utils.
    {
        let mut idl = File::create( &idl_path )
                .expect( &format!( "Could not create file: {:?}", idl_path ) );
        intercom_utils::create_idl(
                    &Path::new( &toml_dir ),
                    &mut idl )
                .expect( "Failed to form IDL" );
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
                        env::var( "PATH" ).unwrap_or( "".to_owned() ) ) )
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

    // Compile the resource script into a resource dll.
    {
        if ! Command::new( paths.rc )
                .current_dir( &out_dir )
                .arg( &rc_path )
                .status()
                .unwrap().success() {

                    panic!( "rc.exe did not execute successfully" );
                }
    }

    // Rename the resource dll in such a way that rustc will find it.
    // On MSVC this means appending '.lib' in the end.
    ::std::fs::rename( &res_path, &lib_path )
            .expect( &format!(
                    "Failed to rename {:?} to {:?}",
                    res_path, lib_path ) );

    // Instruct rustc to link the resource dll.
    println!( "cargo:rustc-link-lib=dylib={}", &res_path.to_string_lossy()[2..] );
}
