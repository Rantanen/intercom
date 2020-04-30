//!
//! COM library parse model.
//!
//! Defines the items constructed from the various COM attributes.
//!
//! Should unify COM attribute expansion and crate parsing for IDL/Manifest/etc.
//! purposes in the future.
//!

#[derive(Fail, Debug)]
pub enum ParseError
{
    #[fail(display = "Parsing [com_library] failed: {}", _0)]
    ComLibrary(String),

    #[fail(display = "Parsing [com_class] item {} failed: {}", _0, _1)]
    ComClass(String, String),

    #[fail(display = "Parsing [com_interface] item {} failed: {}", _0, _1)]
    ComInterface(String, String),

    #[fail(display = "Parsing [com_signature] item {} failed: {}", _0, _1)]
    ComSignature(String, String),

    #[fail(display = "Processing crate failed: {}", _0)]
    ComCrate(String),

    #[fail(display = "Reading TOML failed: {}", _0)]
    CargoToml(String),

    #[doc(hidden)]
    #[fail(display = "<Internal>")]
    __NonExhaustive,
}

pub type ParseResult<T> = Result<T, ParseError>;

#[macro_use]
mod macros;

mod comlibrary;
pub use self::comlibrary::*;
mod comclass;
pub use self::comclass::*;
mod cominterface;
pub use self::cominterface::*;
mod comsignature;
pub use self::comsignature::*;
