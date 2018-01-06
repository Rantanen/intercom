
#[derive(Fail, Debug)]
#[non_exhaustive]
pub enum GeneratorError {

    #[fail( display = "Could not parse crate")]
    CrateParseError,

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
