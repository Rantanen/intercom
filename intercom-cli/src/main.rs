#![feature(inner_deref)]
#![allow(clippy::match_bool)]

use std::io;
use std::path::Path;
use std::fs::File;

#[macro_use] extern crate failure;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, SubCommand, Arg, ArgMatches};

#[cfg(windows)]
mod embed;

mod typelib;
mod generators;


/// Main entry point.
fn main() {

    // Define the command line arguments using clap.
    let app = App::new( "Rust COM utility" )
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
                .arg( Arg::with_name( "source" )
                   .long( "source" )
                   .value_name( "source_file" )
                   .help( "File path for the generated source file. '-' for stdout." )
                )
                .arg( Arg::with_name( "header" )
                   .long( "header" )
                   .value_name( "header_file" )
                   .help( "File path for the generated header file. '-' for stdout." )
                )
                .arg( Arg::with_name( "all" )
                    .long( "all" )
                    .help( "Include both Automation and Raw type systems in the C++ implementation.{n}\
                           Normally the implementation only includes the Raw type system interfaces." )
                )
            );

    #[cfg(windows)]
    let app = app.subcommand( SubCommand::with_name( "embed-typelib" )
            .about( "Builds and embeds the typelib into a DLL" )
            .arg( Arg::with_name( "path" )
               .help( "Path to the DLL." )
               .index( 1 )
            )
        );

    // Run the command and report possible errors.
    if let Err( e ) = run_cmd( &app.get_matches() ) {
        eprintln!( "{}", e );
        std::process::exit(1);
    }
}

fn run_cmd( matches : &ArgMatches ) -> Result<(), failure::Error>
{
    let opts = generators::ModelOptions {
        type_systems: vec![
            generators::TypeSystemOptions {
                ts: intercom::type_system::TypeSystemName::Automation,
                use_full_name: true,
            },
            generators::TypeSystemOptions {
                ts: intercom::type_system::TypeSystemName::Raw,
                use_full_name: true,
            },
        ]
    };

    match matches.subcommand() {
        ( "read-typelib", Some( args ) ) => {
            let path = Path::new( args.value_of( "path" ).unwrap() );
            println!( "{:#?}", typelib::read_typelib( path )? );
        },
        #[cfg(windows)]
        ( "embed-typelib", Some( args ) ) => {
            embed::embed_typelib( Path::new( args.value_of("path").unwrap() ), opts )?;
        },
        ( "idl", Some( args ) ) => {
            let path = Path::new( args.value_of( "path" ).unwrap() );
            let lib = typelib::read_typelib( path )?;
            generators::idl::write( lib, opts, &mut io::stdout() )?;
        },
        ( "cpp", Some( args ) ) => {
            let path = Path::new( args.value_of( "path" ).unwrap() );
            let lib = typelib::read_typelib( path )?;

            let header_writer : Result<_, failure::Error>
                = args.value_of("header").map(|path|
                    if path == "-" {
                        Ok(Box::new(io::stdout()) as Box<dyn io::Write>)
                    } else {
                        Ok(Box::new(File::create(path)?) as Box<dyn io::Write>)
                    })
                    .map_or(Ok(None), |v| v.map(Some));
            let source_writer : Result<_, failure::Error>
                = args.value_of("source").map(|path|
                    if path == "-" {
                        Ok(Box::new(io::stdout()) as Box<dyn io::Write>)
                    } else {
                        Ok(Box::new(File::create(path)?) as Box<dyn io::Write>)
                    })
                    .map_or(Ok(None), |v| v.map(Some));

            {
                let mut header_writer = header_writer?;
                let mut source_writer = source_writer?;
                return Ok( generators::cpp::write(
                    lib, opts,
                    header_writer.as_mut().map(|b| b as &mut dyn io::Write),
                    source_writer.as_mut().map(|b| b as &mut dyn io::Write),
                )? );
            }
        },
        _ => unreachable!(),
    }

    Ok(())
}
