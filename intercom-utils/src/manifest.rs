
use clap::ArgMatches;
use error::*;
use std::path::Path;

use intercom_common::model;
use intercom_common::utils;

/// Run the manifest sub-command.
#[allow(dead_code)]
pub fn run( manifest_params : &ArgMatches ) -> AppResult {

    // Parse the source files and emit the manifest.
    let path = Path::new( manifest_params.value_of( "path" ).unwrap() );
    let krate = if path.is_file() {
            model::ComCrate::parse_cargo_toml( path )
        } else {
            model::ComCrate::parse_cargo_toml( &path.join( "Cargo.toml" ) )
        }.unwrap();
    result_to_manifest( &krate );

    Ok(())
}

/// Prints the manifest based on the parse result.
fn result_to_manifest(
    c : &model::ComCrate
) {
    let lib = c.lib().as_ref().unwrap();

    // Gather all the com classes. These need to be declared in the manifest.
    let classes = lib.coclasses().iter().map(|class_name| {

        let coclass = &c.structs()[ class_name.as_ref() ];
        format!( r###"
                <comClass progid="{}.{}"
                    clsid="{{{}}}" />
        "###, utils::pascal_case(
            lib.name() ),
            coclass.name(),
            coclass.clsid().as_ref().unwrap() )
    } ).collect::<Vec<_>>().join( "\n" );


    // Print the manifest.
    println!( r###"
        <?xml version="1.0" encoding="utf-8" standalone="yes"?>
        <assembly manifestVersion="1.0" xmlns="urn:schemas-microsoft-com:asm.v1">
            <assemblyIdentity type="win32" name="{0}.Assembly" version="1.0.0.0" />
            <file name="{0}.dll">
                <typelib tlbid="{{{1}}}"
                    version="1.0"
                    helpdir="" />
                {2}
            </file>
        </assembly>
        "###,
        utils::pascal_case( lib.name() ),
        lib.libid(),
        classes );
}
