
use std;
use toml;
use glob;
use intercom_common;
use std::error::Error;

/// Intercom utils error type.
///
/// In a application-level error handling we don't really care about responding
/// to errors currently. What we need is a way to report them though.
///
/// The `AppError` carries the error message and we can use `From<T>` impls to
/// convert various other error types into it.
#[derive(Debug)]
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

impl From<toml::de::Error> for AppError {
    fn from( e : toml::de::Error ) -> AppError {
        AppError( String::from( e.description() ) )
    }
}

impl std::fmt::Display for AppError {
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( f, "{}", self.0 )
    }
}

