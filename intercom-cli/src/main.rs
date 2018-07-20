use std::io;
use std::path::Path;
use std::fs::File;

extern crate intercom_common;
extern crate failure;

use intercom_common::generators;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, SubCommand, Arg, ArgMatches};


/// Main entry point.
fn main() {

    // Define the command line arguments using clap.
    let matches = App::new( "Rust COM utility" )
            .version( crate_version!() )
            .author( "Mikko Rantanen <rantanen@jubjubnest.net>" )
            .setting( AppSettings::SubcommandRequiredElseHelp )
            .subcommand( SubCommand::with_name( "idl" )
                .about( "Generates IDL file from the Rust crate" )
                .arg( Arg::with_name( "path" )
                   .help( "Path to the crate to process" )
                   .default_value( "." )
                   .index( 1 )
                )
            )
            .subcommand( SubCommand::with_name( "manifest" )
                .about( "Generates a manifest file for the Rust crate for \
                            registration free COM" )
                .arg( Arg::with_name( "path" )
                   .help( "Path to the crate to process" )
                   .default_value( "." )
                   .index( 1 )
                )
            )
            .subcommand( SubCommand::with_name( "cpp" )
                .about( "Generates C++ header files from the Rust crate" )
                .arg( Arg::with_name( "path" )
                   .help( "Path to the crate to process" )
                   .default_value( "." )
                   .index( 1 )
                )
                .arg( Arg::with_name( "output" )
                   .help( "Target where the C++ header file and associated library implementation are generated." )
                   .default_value( "." )
                   .index( 2 )
                )
            )
        .get_matches();

    // Run the command and report possible errors.
    if let Err( e ) = run_cmd( &matches ) {
        eprintln!( "{}", e );
    }
}

fn run_cmd( matches : &ArgMatches ) -> Result<(), failure::Error>
{
    match matches.subcommand() {
        ( "idl", Some( args ) ) => {
            let path = Path::new( args.value_of( "path" ).unwrap() );
            let model = generators::idl::IdlModel::from_path( path )?;
            model.write( &mut io::stdout() )?;
        },
        ( "manifest", Some( args ) ) => {
            let path = Path::new( args.value_of( "path" ).unwrap() );
            let model = generators::manifest::ManifestModel::from_path( path )?;
            model.write( &mut io::stdout() )?;
        },
        ( "cpp", Some( args ) ) => {
            let path = Path::new( args.value_of( "path" ).unwrap() );
            let model = generators::cpp::CppModel::from_path( path )?;

            let output = Path::new( args.value_of( "output" ).unwrap() );
            std::fs::create_dir_all( output ).expect( "Preparing output failed." );
            model.write_header( &mut File::create(
                    output.join( format!( "{}.hpp", model.lib_name ) ) )? )?;
            model.write_source( &mut File::create(
                    output.join( format!( "{}.cpp", model.lib_name ) ) )? )?;
        },
        _ => unreachable!(),
    }

    Ok(())
}
