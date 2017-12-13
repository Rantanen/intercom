#![feature(try_trait)]

#[macro_use]
extern crate clap;
extern crate syn;
extern crate glob;
extern crate intercom_common;
extern crate toml;

mod idl;
mod manifest;
mod parse;
mod error;

use clap::{App, AppSettings, SubCommand, Arg};

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
        .get_matches();

    // Match the sub-command and invoke the correct command handler.
    if let Err( e ) = match matches.subcommand() {
        ( "idl", Some( args ) ) => { idl::run( args ) },
        ( "manifest", Some( args ) ) => { manifest::run( args ) },
        _ => unreachable!(),
    } {
        // Error occurred in the sub-command. Report it before stopping.
        eprintln!( "{}", e );
    }
}
