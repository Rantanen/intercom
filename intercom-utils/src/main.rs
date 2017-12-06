#![feature(try_trait)]

#[macro_use]
extern crate clap;
extern crate syn;
extern crate glob;
extern crate intercom_common;

mod idl;
mod manifest;
mod parse;

use clap::{App, AppSettings, SubCommand, Arg, ArgMatches};
use std::error::Error;

/// Intercom utils error type.
///
/// In a application-level error handling we don't really care about responding
/// to errors currently. What we need is a way to report them though.
///
/// The `AppError` carries the error message and we can use `From<T>` impls to
/// convert various other error types into it.
pub struct AppError( String );

/// Intercom utils result type.
pub type AppResult = Result< (), AppError >;

impl From<String> for AppError {
    fn from( e : String ) -> AppError {
        AppError( e )
    }
}

impl<'a> From<&'a str> for AppError {
    fn from( e : &'a str ) -> AppError {
        AppError( e.to_owned() )
    }
}

impl From<intercom_common::error::MacroError> for AppError {
    fn from( e : intercom_common::error::MacroError ) -> AppError {
        AppError( e.msg )
    }
}

impl From<glob::PatternError> for AppError {
    fn from( e : glob::PatternError ) -> AppError {
        AppError( String::from( e.description() ) )
    }
}

impl From<std::io::Error> for AppError {
    fn from( e : std::io::Error ) -> AppError {
        AppError( String::from( e.description() ) )
    }
}

impl std::fmt::Display for AppError {
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( f, "{}", self.0 )
    }
}

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
