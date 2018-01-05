
use std::path::Path;
use std::io;

use clap::ArgMatches;
use intercom_common::utils;
use intercom_common::guid::*;
use intercom_common::model;
use intercom_common::methodinfo;
use intercom_common::foreign_ty::*;
use error::*;

/// Runs the 'cpp' subcommand.
pub fn run( cpp_params : &ArgMatches ) -> AppResult {

    // Parse the sources and convert the result into an IDL.
    let path = Path::new( cpp_params.value_of( "path" ).unwrap() );
    let krate = if path.is_file() {
            model::ComCrate::parse_cargo_toml( path )
        } else {
            model::ComCrate::parse_cargo_toml( &path.join( "Cargo.toml" ) )
        }.unwrap();
    result_to_cpp( &krate, &mut io::stdout() );

    Ok(())
}

/// Converts the parse result into an header  that gets written to stdout.
fn result_to_cpp(
    c : &model::ComCrate,
    out : &mut io::Write,
) {
    // Unwrap the library. We require one to be specified here.
    let lib = c.lib().as_ref().unwrap();

    // Introduce all interfaces so we don't get errors on undeclared items.
    let foreign = CTyHandler;
    let itf_forward_declarations = c.interfaces().iter().map( |( _, itf )| {
        format!( "{}struct {};",
                 get_indentation( 1 ), foreign.get_name( c, itf.name() ) )
    } ).collect::<Vec<_>>().join( "\n" );

    // Raw interfaces that give direct access to the components implemented in Rust.
    let itfs = c.interfaces().iter().map( |( _, itf )| {
        generate_raw_interface( c, itf, 1 )
    } ).collect::<Vec<_>>().join( "\n" );

    // Generate class descriptors.
    let class_descriptors = lib.coclasses().iter().map(| class_name | {

        // Get the class details by matching the name.
        let class = &c.structs()[ class_name.as_ref() ];

        generate_class_descriptor( c, lib.name(), class, 1 )
    } ).collect::<Vec<_>>().join( "\n" );

    // Generate using statements that flatten the raw interfaces
    // to mimic the results of a Microsoft MIDL compiler.
    let flat_interface_declarations = c.interfaces().iter().map(|(_, itf)| {
        flatten_interface( c, lib.name(), itf, 1 )
    } ).collect::<Vec<_>>().join( "\n" );

    // Generate using statements that flatten the class descriptors
    // to mimic the results of a Microsoft MIDL compiler.
    let flat_class_descriptor_declarations = lib.coclasses()
            .iter()
            .map(|class_name| {

        // Get the class details by matching the name.
        let class  = &c.structs()[ class_name.as_ref() ];

        flatten_class_descriptor( class, 1 )
    } ).collect::<Vec<_>>().join( "\n" );

    // We got the interfaces and classes. We can format and output the raw interfaces.
    let raw_namespace: String = format!( r###"
>       #include <array>
>       #include <intercom.h>
>       namespace {}
>       {{
>       namespace raw
>       {{
>       {}
>       {}
>       {}
>       }}
>       }}
>       #ifdef INTERCOM_FLATTEN_DECLARATIONS
>       {}
>       {}
>       #endif
>       "###,
        lib.name(), itf_forward_declarations, itfs, class_descriptors,
        flat_interface_declarations, flat_class_descriptor_declarations );
    let raw_namespace = raw_namespace.replace( ">       ", "" );
    writeln!( out, "{}", raw_namespace ).unwrap();
}

fn generate_raw_interface(
    c : &model::ComCrate,
    itf: &model::ComInterface,
    base_indentation: usize
) -> String {
    let foreign = CTyHandler;

    // Now that we have methods sorted out, we can construct the final
    // interface definition.
    let indentation = get_indentation( base_indentation );
    let itf_text = format!( "{}struct {} : IUnknown\n{}{{\n{}static constexpr intercom::IID ID = {};\n\n{}\n{}}};\n",
            indentation, foreign.get_name( c, itf.name() ), indentation,
            get_indentation( base_indentation + 1 ), guid_to_binary( itf.iid() ),
            generate_raw_methods( c, itf, base_indentation + 1 ), indentation );
    itf_text
}

fn generate_raw_methods(
    c: &model::ComCrate,
    itf: &model::ComInterface,
    indentation: usize
) -> String {

    // Get the method definitions for the current interface.
    itf.methods().iter().map( |m| {

        // Construct the argument list.
        let args = m.raw_com_args().iter().map(|a| {

            // Argument direction affects both the argument attribute and
            // whether the argument is passed by pointer or value.
            let out_ptr = match a.dir {
                methodinfo::Direction::In => "",
                methodinfo::Direction::Out
                    | methodinfo::Direction::Retval => "*",
            };
            format!( "{}{} {}", to_cpp_type( c, &a.arg.ty ), out_ptr, a.arg.name )

        } ).collect::<Vec<_>>().join( ", " );

        // Format the method.
        // To maintain backwards compatibility all new methods should
        // be added to the end of the traits.
        format!( r###"{}virtual {} INTERCOM_CC {}( {} ) = 0;"###,
            get_indentation( indentation ),
            to_cpp_type( c, &m.returnhandler.com_ty() ),
            utils::pascal_case( m.name.as_ref() ), args )

    } ).collect::<Vec<_>>().join( "\n" )
}

fn flatten_interface(
    c : &model::ComCrate,
    libname: &str,
    interface: &model::ComInterface,
    indentation: usize
) -> String {
    let foreign = CTyHandler;
    let interface_name = foreign.get_name( c, interface.name() );
    let using_iid = format!( "static constexpr intercom::IID IID_{} = {};",
            utils::pascal_case( &interface_name ),
            guid_to_binary( interface.iid() ) );
    let using_interface = format!( "using {} = {}::raw::{};",
            utils::pascal_case( &interface_name ), libname,  &interface_name );
    let indentation = get_indentation( indentation );
    format!("{}{}\n{}{}", indentation, using_iid, indentation, using_interface )
}

fn flatten_class_descriptor(
    coclass: &model::ComStruct,
    indentation: usize
) -> String {
    let using_clsid = format!( "static constexpr intercom::CLSID CLSID_{} = {};",
            utils::pascal_case( coclass.name().as_ref() ),
            guid_to_binary( coclass.clsid().as_ref().unwrap() ) );
    let indentation = get_indentation( indentation );
    format!("{}{}", indentation, using_clsid )
}

fn generate_class_descriptor(
    c : &model::ComCrate,
    libname: &str,
    coclass: &model::ComStruct,
    indentation: usize
) -> String {

    // Create a list of interfaces to be declared in the class descriptor.
    let foreign = CTyHandler;
    let interfaces =  coclass.interfaces().iter().map(|itf| {
            format!( r###"{}{}::raw::{}::ID"###,
                &get_indentation( indentation + 2 ), libname, foreign.get_name( c, itf ) )
        } ).collect::<Vec<_>>().join( ",\n" );

    // Class descriptors hold information about COM classes implemented by the library.
    let clsid = guid_to_binary( coclass.clsid().as_ref().unwrap() );
    let class_descriptor: String = format!( r###"
>       class {name}Descriptor
>       {{
>           static constexpr intercom::CLSID ID = {};

>           static constexpr std::array<intercom::IID, {}> INTERFACES = {{
{}
>           }};

>           {name}Descriptor() = delete;
>           ~{name}Descriptor() = delete;

>       }};
>       "###,
    clsid, coclass.interfaces().len(), interfaces, name = coclass.name() );
    class_descriptor.replace( ">       ", &get_indentation( indentation ) )
}

/// Gets an empty string for ind One level of indentation is four spaces.
fn get_indentation(
    level: usize
) -> String {
    let spaces = level * 4;
    let indentation = format!( r###"{: <1$}"###, "", spaces );
    indentation
}

/// Converts a Rust type into applicable C++ type.
fn to_cpp_type(
    c: &model::ComCrate,
    ty: &::syn::Ty
) -> String {

    let foreign = CTyHandler;
    let name = foreign.get_ty( c, ty ).unwrap();
    match name.as_str() {
        "int8" => "int8_t",
        "uint8" => "uint8_t",
        "int16" => "int16_t",
        "uint16" => "uint16_t",
        "int32" => "int32_t",
        "uint32" => "uint32_t",
        "int64" => "int64_t",
        "uint64" => "uint64_t",
        "BStr" => "intercom::BSTR",
        "HRESULT" => "intercom::HRESULT",
        _ => return name,
    }.to_owned()
}

/// Converts a guid to binarys representation.
fn guid_to_binary(
    g: &GUID
) -> String {

    format!( "{{0x{:08x},0x{:04x},0x{:04x},{{0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x}}}}}",
            g.data1, g.data2, g.data3,
            g.data4[0], g.data4[1], g.data4[2], g.data4[3],
            g.data4[4], g.data4[5], g.data4[6], g.data4[7] )
}
