//! Generators for file formats that can be derived from the intercom
//! libraries.

use std::collections::HashMap;

use intercom::type_system::TypeSystemName;
use intercom::typelib::{Interface, TypeInfo, TypeLib};

/// A common error type for all the generators.
#[derive(Fail, Debug)]
pub enum GeneratorError
{
    #[fail(display = "IoError: {}", _0)]
    IoError(#[cause] ::std::io::Error),

    #[fail(display = "Invalid type library: {}", _0)]
    LibraryError(String),
}

impl From<::std::io::Error> for GeneratorError
{
    fn from(e: ::std::io::Error) -> GeneratorError
    {
        GeneratorError::IoError(e)
    }
}

impl From<String> for GeneratorError
{
    fn from(s: String) -> GeneratorError
    {
        GeneratorError::LibraryError(s)
    }
}

pub struct ModelOptions
{
    pub type_systems: Vec<TypeSystemOptions>,
}

pub struct TypeSystemOptions
{
    pub ts: TypeSystemName,
    pub use_full_name: bool,
}

pub struct LibraryContext<'a>
{
    pub itfs_by_ref: HashMap<String, &'a Interface>,
    pub itfs_by_name: HashMap<String, &'a Interface>,
}

impl<'a> LibraryContext<'a>
{
    fn try_from(lib: &'a TypeLib) -> Result<LibraryContext<'a>, GeneratorError>
    {
        let itfs_by_name: HashMap<String, &Interface> = lib
            .types
            .iter()
            .filter_map(|t| match t {
                TypeInfo::Interface(itf) => Some(itf),
                _ => None,
            })
            .map(|itf| (itf.as_ref().name.to_string(), &**(itf.as_ref())))
            .collect();
        let itfs_by_ref: HashMap<String, &Interface> = lib
            .types
            .iter()
            .filter_map(|t| match t {
                TypeInfo::Class(cls) => Some(cls),
                _ => None,
            })
            .flat_map(|cls| &cls.as_ref().interfaces)
            .map(|itf_ref| {
                (
                    itf_ref.name.to_string(),
                    itfs_by_name[itf_ref.name.as_ref()],
                )
            })
            .collect();
        Ok(LibraryContext {
            itfs_by_name,
            itfs_by_ref,
        })
    }
}

/// Convert the Rust identifier from `snake_case` to `PascalCase`
pub fn pascal_case<T: AsRef<str>>(input: T) -> String
{
    let input = input.as_ref();

    // Allocate the output string. We'll never increase the amount of
    // characters so we can reserve string buffer using the input string length.
    let mut output = String::new();
    output.reserve(input.len());

    // Process each character from the input.
    let mut capitalize = true;
    for c in input.chars() {
        // Check the capitalization requirement.
        if c == '_' {
            // Skip '_' but capitalize the following character.
            capitalize = true;
        } else if capitalize {
            // Capitalize. Add the uppercase characters.
            for c_up in c.to_uppercase() {
                output.push(c_up)
            }

            // No need to capitalize any more.
            capitalize = false;
        } else {
            // No need to capitalize. Just add the character as is.
            output.push(c);
        }
    }
    output
}

pub mod cpp;
pub mod idl;
