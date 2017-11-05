
use std::collections::HashMap;
use super::*;
use parse::*;

pub fn run( idl_params : &ArgMatches ) -> AppResult {

    let path_str = format!(
            "{}/src/**/*.rs",
            idl_params.value_of( "path" ).unwrap() );

    let ( renames, result ) = parse_crate( path_str )?;
    result_to_idl( &result, &renames );

    Ok(())
}

fn result_to_idl(
    r : &ParseResult,
    rn : &HashMap<String, String>,
) {
    let itf_introductions = r.interfaces.iter().map(|itf| {
        format!( r###"
            interface {};
        "###, try_rename( rn, &itf.name ) )
    } ).collect::<Vec<_>>().join( "\n" );

    let itfs = r.interfaces.iter().map(|itf| {

        let methods = itf.methods.iter().enumerate().map(|(i,m)| {

            let args = m.arguments.iter().map(|a| {
                let ( attrs, out_ptr ) = match a.dir {
                    ArgDirection::In => ( "in", "" ),
                    ArgDirection::Out => ( "out", "*" ),
                    ArgDirection::Return => ( "out, retval", "*" ),
                };
                format!( "[{}] {}{} {}", attrs, a.ty, out_ptr, a.name )
            } ).collect::<Vec<_>>().join( ", " );

            format!( r###"
                [id({:X})]
                {} {}( {} );
            "###, i, m.rvalue, camel_case( &m.name ), args )
        } ).collect::<Vec<_>>().join( "\n" );
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

    let classes = r.class_names.iter().map(|class_name| {

        let coclass = r.classes.iter().find(|cls| &cls.name == class_name )
                .unwrap();

        let interfaces = coclass.interfaces.iter().map(|itf| {

            format!( r###"
                interface {};"###, try_rename( rn, &itf ) )
        } ).collect::<Vec<_>>().join( "\n" );
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
