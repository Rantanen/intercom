
use std::collections::HashMap;
use super::*;
use parse::*;

use intercom_common::utils;

/// Run the manifest sub-command.
pub fn run( idl_params : &ArgMatches ) -> AppResult {

    // Parse the source files and emit the manifest.
    let ( renames, result ) = parse_crate(
            idl_params.value_of( "path" ).unwrap() )?;
    result_to_manifest( &result, &renames );

    Ok(())
}

/// Prints the manifest based on the parse result.
fn result_to_manifest(
    r : &ParseResult,
    _rn : &HashMap<String, String>,
) {
    // Gather all the com classes. These need to be declared in the manifest.
    let classes = r.class_names.iter().map(|class_name| {

        let coclass = r.classes.iter().find(|cls| &cls.name == class_name )
                .unwrap();

        format!( r###"
                <comClass progid="{}.{}"
                    clsid="{{{}}}" />
        "###, utils::pascal_case( &r.libname ), coclass.name, coclass.clsid )
    } ).collect::<Vec<_>>().join( "\n" );


    // Print the manifest.
    println!( r###"
        <?xml version="1.0" encoding="utf-8" standalone="yes"?>
        <assembly manifestVersion="1.0" xmlns="urn:schemas-microsoft-com:asm.v1">
            <assemblyIdentity type="win32" name="{}.Assembly" version="1.0.0.0" />
            <file name="{}.dll">
                <typelib tlbid="{{{}}}"
                    version="1.0"
                    helpdir="" />
                {}
            </file>
        </assembly>
        "###,
        utils::pascal_case( &r.libname ),
        utils::pascal_case( &r.libname ),
        r.libid.as_ref().unwrap(),
        classes );
}
