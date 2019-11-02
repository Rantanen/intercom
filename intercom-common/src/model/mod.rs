//!
//! COM library parse model.
//!
//! Defines the items constructed from the various COM attributes.
//!
//! Should unify COM attribute expansion and crate parsing for IDL/Manifest/etc.
//! purposes in the future.
//!

#[derive(Fail, Debug)]
#[non_exhaustive]
pub enum ParseError
{
    #[fail(display = "Parsing [com_library] failed: {}", _0)]
    ComLibrary(String),

    #[fail(display = "Parsing [com_class] item {} failed: {}", _0, _1)]
    ComClass(String, String),

    #[fail(display = "Parsing [com_struct] item {} failed: {}", _0, _1)]
    ComStruct(String, String),

    #[fail(display = "Parsing [com_interface] item {} failed: {}", _0, _1)]
    ComInterface(String, String),

    #[fail(display = "Parsing [com_impl] {} for {} failed: {}", _0, _1, _2)]
    ComImpl(String, String, String),

    #[fail(display = "Processing crate failed: {}", _0)]
    ComCrate(String),

    #[fail(display = "Reading TOML failed: {}", _0)]
    CargoToml(String),
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
mod comimpl;
pub use self::comimpl::*;
mod comstruct;
pub use self::comstruct::*;
