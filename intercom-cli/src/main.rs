use std::io;
use std::path::Path;
use std::fs::File;

#[macro_use] extern crate failure;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, SubCommand, Arg, ArgMatches};

mod typelib;
mod generators;


/// Main entry point.
fn main() {

    // Define the command line arguments using clap.
    let matches = App::new( "Rust COM utility" )
            .version( crate_version!() )
            .author( "Mikko Rantanen <rantanen@jubjubnest.net>" )
            .setting( AppSettings::SubcommandRequiredElseHelp )
            .subcommand( SubCommand::with_name( "read-typelib" )
                .about( "Reads the type library." )
                .arg( Arg::with_name( "path" )
                   .help( "Path to the type library." )
                   .index( 1 )
                )
            )
            .subcommand( SubCommand::with_name( "idl" )
                .about( "Generates IDL file from the Rust crate" )
                .arg( Arg::with_name( "path" )
                   .help( "Path to the crate to process" )
                   .default_value( "." )
                   .index( 1 )
                )
                .arg( Arg::with_name( "all" )
                    .long( "all" )
                    .help( "Include both Automation and Raw type systems in the IDL.{n}\
                           Normally the IDL only includes the Automation type system interfaces." )
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
                .arg( Arg::with_name( "all" )
                    .long( "all" )
                    .help( "Include both Automation and Raw type systems in the C++ implementation.{n}\
                           Normally the implementation only includes the Raw type system interfaces." )
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
        ( "read-typelib", Some( args ) ) => {
            let path = Path::new( args.value_of( "path" ).unwrap() );
            println!( "{:?}", typelib::read_typelib( path )? );
        },
        ( "idl", Some( args ) ) => {
            let path = Path::new( args.value_of( "path" ).unwrap() );
            let lib = typelib::read_typelib( path )?;
            let opts = generators::ModelOptions {
                type_systems: vec![
                    generators::TypeSystemOptions {
                        ts: intercom::type_system::TypeSystemName::Automation,
                        use_full_name: false,
                    },
                    generators::TypeSystemOptions {
                        ts: intercom::type_system::TypeSystemName::Raw,
                        use_full_name: true,
                    },
                ]
            };
            generators::idl::write( lib, opts, &mut io::stdout() )?;
        },
        _ => unreachable!(),
    }

    Ok(())
}
