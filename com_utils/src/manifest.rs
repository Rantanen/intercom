
use std::collections::HashMap;
use super::*;
use parse::*;

pub fn run( idl_params : &ArgMatches ) -> AppResult {

    let path_str = format!(
            "{}/src/**/*.rs",
            idl_params.value_of( "path" ).unwrap() );

    let ( renames, result ) = parse_crate( path_str )?;
    result_to_manifest( &result, &renames );

    Ok(())
}

fn result_to_manifest(
    r : &ParseResult,
    rn : &HashMap<String, String>,
) {
    let classes = r.class_names.iter().map(|class_name| {

        let coclass = r.classes.iter().find(|cls| &cls.name == class_name )
                .unwrap();

        format!( r###"
                <comClass progid="{}.{}"
                    clsid="{{{}}}" />
        "###, r.libname.as_ref().unwrap(), coclass.name, coclass.clsid )
    } ).collect::<Vec<_>>().join( "\n" );


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
        r.libname.as_ref().unwrap(),
        r.libname.as_ref().unwrap(),
        r.libid.as_ref().unwrap(),
        classes );
}
