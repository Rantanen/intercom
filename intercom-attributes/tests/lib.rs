
extern crate rustfmt_nightly;
extern crate difference;
extern crate regex;

use difference::Changeset;
use rustfmt_nightly as rustfmt;

use std::fs;
use std::io::Read;

// Given the default Rust test runner doesn't expose programmatic test cases
// we are using single "check_expansions" test to process all the data files.
//
// This is similar approach to what rustfmt does.

#[test]
fn check_expansions() {

    // Get the source test data files.
    let test_data = fs::read_dir( "tests/data" ).unwrap();
    let source_paths = test_data
            .into_iter()
            .map( |e| e.expect("Failed to read entry").path() )
            .map( |p| p.to_str().unwrap().to_owned() )
            .filter( |p| p.ends_with( ".source.rs" ) )
            .collect::<Vec<_>>();

    // Process each source file. Track the failures.
    let mut failed = 0;
    for source_path in source_paths {

        // Construct the target file path by replacing the ".source.rs" with a
        // ".target.rs". There's a small discrepancy here as the .source.rs had
        // to be at the end for the file to count as source file, but here
        // we are replacing the .target. everywhere in the file name.
        //
        // This shouldn't matter in practice as these are test files and we can
        // decide on their naming as we write them.
        let mut target_file = fs::File::open( source_path.replace(
                ".source.rs",
                ".target.rs" ) ).unwrap();

        // Get the source and target code.
        //
        // The source is compiled using rustc and the target is just read
        // from the disk.
        let mut source_code = build( &source_path );
        let mut target_code = String::new();
        target_file.read_to_string( &mut target_code )
                    .expect( "Failed to read target" );

        // Generate diffs for both sources
        // Ensure the linebreaks are the same for both. This seems to be
        // somewhat of an issue on AppVeyor.
        target_code = target_code.replace( "\r", "" );
        source_code = source_code.replace( "\r", "" );

        // 
        // Use rustfmt to format both pieces of code so that we have a
        // canonical format for them. Without rustfmt we'd need to match the
        // compiler pretty print format in the reference target files - which,
        // despite its name, isn't very pretty.
        let changeset = Changeset::new(
                &format( source_code.trim() ),
                &format( target_code.trim() ),
                "\n" );

        // If these were equal, there's only one "Same" diff segment.
        // If there is more than one, they differed.
        if changeset.diffs.len() > 1 {

            // Print the changeset for debugging purposes and increment the
            // amount of failed items. By default this prints nice colored diff.
            println!( "{}", changeset );
            failed += 1;
        }
    }

    // Ensure there were no failures.
    //
    // If we fail here, cargo will display our printlns to the user.
    assert_eq!( failed, 0, "{} tests failed", failed );
}

/// Compiles a single file using rustc using similar options than what
/// cargo would have used.
fn build( path : &str ) -> String {

    // Launch rustc.
    let output = std::process::Command::new( "rustc" )
            .args( &[
                "--crate-name", "source",
                "--crate-type", "lib",
                path,
                "--out-dir", "tests/out",
                "-L", "dependency=../target/debug/deps",
                "--extern", "intercom=../target/debug/libintercom.rlib",
                "--pretty=expanded",
                "-Z", "unstable-options",
            ] )
            .output()
            .expect( "Failed to execute" );

    // Ensure the compilation was successful.
    if ! output.status.success() {
        println!( "FAILED TO COMPILE SOURCE {}", path );
        panic!( format!(
                "{}\n\n{}",
                String::from_utf8( output.stdout ).unwrap(),
                String::from_utf8( output.stderr ).unwrap()
            ) );
    }

    // stdout is utf8 byte stream. Parse it into a string.
    String::from_utf8( output.stdout ).expect( "Bad output" )
}

/// Removes comments from the code.
fn strip_comments( code : &str ) -> String {
    let re = regex::Regex::new( r"//.*" ).expect( "Bad regex" );
    re.replace_all( code, "" ).into_owned()
}

fn strip_empty_lines( code : &str ) -> String {
    let re = regex::Regex::new( r"^\s+$" ).expect( "Bad regex" );
    re.replace_all( code, "" ).into_owned().replace( "\n\n", "" )
}

fn format( code : &str ) -> String {

    // Strip comments. This allows us to embed comments in the target files
    // without requiring the attributes to expand these comments.
    let code = strip_comments( code );
    let code = strip_empty_lines( &code );

    // Use default config but instead of altering the source/target files just
    // "display" the formatted code. This results in the output being available
    // in the 'out' parameter.
    let mut config = rustfmt::config::Config::default();
    config.override_value( "write_mode", "Display" );

    let mut out : Vec<u8> = vec![];
    rustfmt::format_input(
            rustfmt::Input::Text( code ),
            &config,
            Some( &mut out ) ).expect( "Failed to format" );

    // Convert the UTF-8 output into a string.
    String::from_utf8( out ).expect( "Bad output" )
}
