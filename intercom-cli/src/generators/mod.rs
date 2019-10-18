
//! Generators for file formats that can be derived from the intercom
//! libraries.

use intercom::type_system::TypeSystemName;

/// A common error type for all the generators.
#[derive(Fail, Debug)]
pub enum GeneratorError {

    #[fail( display = "{}", _0 )]
    IoError( #[cause] ::std::io::Error ),
}

impl From<::std::io::Error> for GeneratorError {
    fn from( e : ::std::io::Error ) -> GeneratorError {
        GeneratorError::IoError( e )
    }
}

pub struct ModelOptions {
    pub type_systems: Vec<TypeSystemOptions>
}

pub struct TypeSystemOptions {
    pub ts: TypeSystemName,
    pub use_full_name: bool,
}

/// Convert the Rust identifier from `snake_case` to `PascalCase`
pub fn pascal_case<T: AsRef<str>>( input : T ) -> String {
    let input = input.as_ref();

    // Allocate the output string. We'll never increase the amount of
    // characters so we can reserve string buffer using the input string length.
    let mut output = String::new();
    output.reserve( input.len() );

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
                output.push( c_up )
            }

            // No need to capitalize any more.
            capitalize = false;

        } else {

            // No need to capitalize. Just add the character as is.
            output.push( c );
        }

    }
    output
}


pub mod idl;
