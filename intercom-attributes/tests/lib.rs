
extern crate difference;
use difference::Changeset;

use std::fs;
use std::io::Read;

#[test]
fn check_expansions() {

    let paths = fs::read_dir( "tests/data" ).unwrap();

    let source_paths = paths
            .into_iter()
            .map( |e| e.expect("Failed to read entry").path() )
            .map( |p| p.to_str().unwrap().to_owned() )
            .filter( |p| p.ends_with( ".source.rs" ) )
            .collect::<Vec<_>>();

    let mut failed = 0;
    for source_path in source_paths {
        println!( "{}", source_path );
        let mut target_file = fs::File::open( source_path.replace(
                ".source.",
                ".target." ) ).unwrap();
        // let mut source_file = fs::File::open( source_path ).unwrap();

        let mut source = build( source_path );
        let mut target = String::new();
        target_file.read_to_string( &mut target ).expect( "Failed to read target" );

        // Ensure the linebreaks are the same for both.
        target = target.replace( "\r", "" );
        source = source.replace( "\r", "" );

        let changeset = Changeset::new(
                source.trim(), target.trim(), "\n" );
        if changeset.diffs.len() > 1 {

            eprintln!( "{}", changeset );
            failed += 1;
        }
    }

    assert_eq!( failed, 0, "{} tests failed", failed );
}

fn build( path : String ) -> String {

    let output = std::process::Command::new( "rustc" )
            .args( &[
                "--crate-name", "source",
                "--crate-type", "lib",
                &path,
                "--out-dir", "tests/out",
                "-L", "dependency=../target/debug/deps",
                "--extern", "intercom=../target/debug/libintercom.rlib",
                "--pretty=expanded",
                "-Z", "unstable-options",
            ] )
            .output()
            .expect( "Failed to execute" );

    if ! output.status.success() {
        eprintln!( "FAILED TO COMPILE SOURCE {}", path );
        panic!( format!( "{}", String::from_utf8( output.stderr ).unwrap() ) );
    }

    String::from_utf8( output.stdout ).expect( "Bad output" )
}
