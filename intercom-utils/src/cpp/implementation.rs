use std::collections::HashMap;
use cpp::*;

/// Generates implementation for the specified library.
pub fn generate(
    r : &ParseResult,
    _rn : &HashMap<String, String>
) -> String {

    // Implementation of the library descriptor.
    let library_descriptor = generate_library_descriptor( &r.libname );

    // We got the required compontents, format the final implementation.
    let raw_implementation: String = format!( r###"
>       #include "{}.h"

>       {}
"###,
        &r.libname, library_descriptor );
    raw_implementation.replace( ">       ", "" )
}

/// Generates implementation for the library descriptor.
fn generate_library_descriptor(
    libname: &str
) -> String {

    // The name of the current platform.
    let name = format!(
            "#ifdef _MSC_VER\n{}const char {}::Descriptor::NAME[] = \"{}.dll\";\n#else\n{}const char {}::Descriptor::NAME[]= \"lib{}.so\";\n#endif",
            get_indentation( 1 ), libname, libname, get_indentation( 1 ), libname, libname );

    // Platform specific names.
    let windows_name = format!( "const char {}::Descriptor::WINDOWS_NAME[] = \"{}.dll\";", libname, libname );
    let posix_name = format!( "const char {}::Descriptor::POSIX_NAME[] = \"lib{}.so\";", libname, libname );

    // Construct the final output.
    format!("{}\n{}\n{}", name, windows_name, posix_name )
}
