
use std::io::Write;
use std::path::Path;
use clap::ArgMatches;
use intercom_common::utils;
use intercom_common::model;
use intercom_common::methodinfo;
use intercom_common::foreign_ty::*;
use error::*;
use std::io;

/// Runs the 'idl' subcommand.
#[allow(dead_code)]
pub fn run( idl_params : &ArgMatches ) -> AppResult {

    // Parse the sources and convert the result into an IDL.
    let path = Path::new( idl_params.value_of( "path" ).unwrap() );
    let krate = if path.is_file() {
            model::ComCrate::parse_cargo_toml( path )
        } else {
            model::ComCrate::parse_cargo_toml( &path.join( "Cargo.toml" ) )
        }.unwrap();
    result_to_idl( &krate, &mut io::stdout() );

    Ok(())
}

#[allow(dead_code)]
pub fn create_idl( path : &Path, out : &mut Write ) -> Result<(), ()> {

    // Parse the sources and convert the result into an IDL.
    let krate = if path.is_file() {
            model::ComCrate::parse_cargo_toml( path )
        } else {
            model::ComCrate::parse_cargo_toml( &path.join( "Cargo.toml" ) )
        }.unwrap();
    result_to_idl( &krate, out );

    Ok(())
}

/// Converts the parse result into an IDL that gets written to stdout.
fn result_to_idl(
    c : &model::ComCrate,
    out : &mut io::Write,
) {
    let foreign = CTyHandler;

    // Introduce all interfaces so we don't get errors on undeclared items.
    let itf_introductions = c.interfaces().iter().map(|(_, itf)| {
        format!( r###"
            interface {};
        "###, foreign.get_name( c, itf.name() ) )
    } ).collect::<Vec<_>>().join( "\n" );

    // Define all interfaces.
    let itfs = c.interfaces().iter().map(|(_, itf)| {

        // Get the method definitions for the current interface.
        let methods = itf.methods().iter().enumerate().map(|(i,m)| {

            // Construct the argument list.
            let args = m.raw_com_args().iter().map(|a| {

                // Argument direction affects both the argument attribute and
                // whether the argument is passed by pointer or value.
                let ( attrs, out_ptr ) = match a.dir {
                    methodinfo::Direction::In => ( "in", "" ),
                    methodinfo::Direction::Out => ( "out", "*" ),
                    methodinfo::Direction::Retval => ( "out, retval", "*" ),
                };
                format!( "[{}] {}{} {}",
                    attrs,
                    foreign.get_ty( c, &a.arg.ty ).unwrap(), out_ptr,
                    a.arg.name )

            } ).collect::<Vec<_>>().join( ", " );

            // Format the method. We use the method index as the [id(..)] value.
            // This means backwards compatibility is maintained as long as all
            // new methods are added to the end of the traits.
            format!( r###"
                [id({:-X})]
                {} {}( {} );
            "###,
            i,
            foreign.get_ty( c, &m.returnhandler.com_ty() ).unwrap(),
            utils::pascal_case( m.name.as_ref() ),
            args )

        } ).collect::<Vec<_>>().join( "\n" );

        // Now that we have methods sorted out, we can construct the final
        // interface definition.
        format!( r###"
            [
                object,
                uuid( {:-X} ),
                nonextensible,
                pointer_default(unique)
            ]
            interface {} : IUnknown
            {{
                {}
            }}
        "###, itf.iid(), foreign.get_name( c, itf.name() ), methods )

    } ).collect::<Vec<_>>().join( "\n" );

    // Create coclass definitions.
    //
    // Here r.class_names contains the class names that were defined in the
    // [com_library] attribute. This is our source for the classes to include
    // in the IDL. r.classes has the actual class details, but might include
    // classes that are not exposed by the library.
    let classes = c.lib().as_ref().unwrap().coclasses().iter().map(|class_name| {

        // Get the class details by matching the name.
        let coclass = &c.structs()[ class_name.as_ref() ];

        // Get the interfaces the class implements.
        let interfaces = coclass.interfaces().iter().map(|itf_name| {
            let itf = &c.interfaces()[ itf_name.as_ref() ];
            format!( r###"
                interface {};"###, foreign.get_name( c, itf.name() ) )
        } ).collect::<Vec<_>>().join( "\n" );

        // Format the final coclass definition now that we have the class
        // details.
        format!( r###"
            [
                uuid( {:-X} )
            ]
            coclass {}
            {{
                {}
            }}
        "###, coclass.clsid().as_ref().unwrap(), coclass.name(), interfaces )
    } ).collect::<Vec<_>>().join( "\n" );

    // We got the interfaces and classes. We can format and output the IDL.
    writeln!( out, r###"
        [
            uuid( {:-X} )
        ]
        library {}
        {{
            importlib("stdole2.tlb");
            {}
            {}
            {}
        }}
        "###,
    c.lib().as_ref().unwrap().libid(),
    utils::pascal_case( c.lib().as_ref().unwrap().name() ),
    itf_introductions,
    itfs,
    classes ).unwrap();
}
