
//! Generators for file formats that can be derived from the intercom
//! libraries.
//!
//! **Requires the optional `generators` feature**

use model;

/// A common error type for all the generators.
#[derive(Fail, Debug)]
#[non_exhaustive]
pub enum GeneratorError {

    #[fail( display = "{}", _0 )]
    CrateParseError( #[cause] model::ParseError ),

    #[fail( display = "Missing [com_library] attribute" )]
    MissingLibrary,

    #[fail( display = "Type {} requires CLSID", _0 )]
    MissingClsid( String ),

    #[fail( display = "Unsupported type: {}", _0 )]
    UnsupportedType( String ),

    #[fail( display = "{}", _0 )]
    IoError( #[cause] ::std::io::Error ),
}

impl From<::std::io::Error> for GeneratorError {
    fn from( e : ::std::io::Error ) -> GeneratorError {
        GeneratorError::IoError( e )
    }
}

pub mod idl;
pub mod manifest;
pub mod cpp;
