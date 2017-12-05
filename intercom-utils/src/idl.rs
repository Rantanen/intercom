
use std::collections::HashMap;
use super::*;
use parse::*;

/// Runs the 'idl' subcommand.
pub fn run( idl_params : &ArgMatches ) -> AppResult {

    // The 'path' command line parameter points to the crate root.
    // We get the rust sources from the /src/ subdirectory.
    let path_str = format!(
            "{}/src/**/*.rs",
            idl_params.value_of( "path" ).unwrap() );

    // Parse the sources and convert the result into an IDL.
    let ( renames, result ) = parse_crate( path_str )?;
    result_to_idl( &result, &renames );

    Ok(())
}

/// Converts the parse result into an IDL that gets written to stdout.
fn result_to_idl(
    r : &ParseResult,
    rn : &HashMap<String, String>,
) {
    // Introduce all interfaces so we don't get errors on undeclared items.
    let itf_introductions = r.interfaces.iter().map(|itf| {
        format!( r###"
            interface {};
        "###, try_rename( rn, &itf.name ) )
    } ).collect::<Vec<_>>().join( "\n" );

    // Define all interfaces.
    let itfs = r.interfaces.iter().map(|itf| {

        // Get the method definitions for the current interface.
        let methods = itf.methods.iter().enumerate().map(|(i,m)| {

            // Construct the argument list.
            let args = m.arguments.iter().map(|a| {

                // Argument direction affects both the argument attribute and
                // whether the argument is passed by pointer or value.
                let ( attrs, out_ptr ) = match a.dir {
                    ArgDirection::In => ( "in", "" ),
                    ArgDirection::Return => ( "out, retval", "*" ),
                };
                format!( "[{}] {}{} {}", attrs, a.ty, out_ptr, a.name )

            } ).collect::<Vec<_>>().join( ", " );

            // Format the method. We use the method index as the [id(..)] value.
            // This means backwards compatibility is maintained as long as all
            // new methods are added to the end of the traits.
            format!( r###"
                [id({:X})]
                {} {}( {} );
            "###, i, m.rvalue, pascal_case( &m.name ), args )

        } ).collect::<Vec<_>>().join( "\n" );

        // Now that we have methods sorted out, we can construct the final
        // interface definition.
        format!( r###"
            [
                object,
                uuid( {:X} ),
                nonextensible,
                pointer_default(unique)
            ]
            interface {} : IUnknown
            {{
                {}
            }}
        "###, itf.iid, try_rename( rn, &itf.name ), methods )

    } ).collect::<Vec<_>>().join( "\n" );

    // Create coclass definitions.
    //
    // Here r.class_names contains the class names that were defined in the
    // [com_library] attribute. This is our source for the classes to include
    // in the IDL. r.classes has the actual class details, but might include
    // classes that are not exposed by the library.
    let classes = r.class_names.iter().map(|class_name| {

        // Get the class details by matching the name.
        let coclass = r.classes.iter().find(|cls| &cls.name == class_name )
                .unwrap();

        // Get the interfaces the class implements.
        let interfaces = coclass.interfaces.iter().map(|itf| {
            format!( r###"
                interface {};"###, try_rename( rn, &itf ) )
        } ).collect::<Vec<_>>().join( "\n" );

        // Format the final coclass definition now that we have the class
        // details.
        format!( r###"
            [
                uuid( {:X} )
            ]
            coclass {}
            {{
                {}
            }}
        "###, coclass.clsid, coclass.name, interfaces )
    } ).collect::<Vec<_>>().join( "\n" );

    // We got the interfaces and classes. We can format and output the IDL.
    println!( r###"
        [
            uuid( {:X} )
        ]
        library {}
        {{
            importlib("stdole2.tlb");
            {}
            {}
            {}
        }}
        "###,
    r.libid.as_ref().unwrap(),
    r.libname.as_ref().unwrap(),
    itf_introductions,
    itfs,
    classes );
}
