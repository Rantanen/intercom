#![feature(try_trait)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate if_chain;
extern crate syn;
extern crate glob;
extern crate com_common;

mod idl;
mod manifest;
mod parse;

use clap::{App, AppSettings, SubCommand, Arg, ArgMatches};
use std::error::Error;

pub struct AppError( String );
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

impl From<com_common::error::MacroError> for AppError {
    fn from( e : com_common::error::MacroError ) -> AppError {
        AppError( String::from( e.msg ) )
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

fn main() {
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

    if let Err( e ) = match matches.subcommand() {
        ( "idl", Some( args ) ) => { idl::run( args ) },
        ( "manifest", Some( args ) ) => { manifest::run( args ) },
        _ => unreachable!(),
    } {
        eprintln!( "{}", e );
    }
}
