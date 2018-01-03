
use std::collections::HashMap;
use parse::*;
use clap::ArgMatches;
use intercom_common::utils;
use intercom_common::guid::*;
use error::*;
use std::io;

/// Runs the 'cpp' subcommand.
pub fn run( cpp_params : &ArgMatches ) -> AppResult {

    // Parse the sources and convert the result into an IDL.
    let ( renames, result ) = parse_crate(
            cpp_params.value_of( "path" ).unwrap() )?;
    result_to_cpp( &result, &renames, &mut io::stdout() );

    Ok(())
}

/// Converts the parse result into an header  that gets written to stdout.
fn result_to_cpp(
    r : &ParseResult,
    rn : &HashMap<String, String>,
    out : &mut io::Write,
) {
    // Introduce all interfaces so we don't get errors on undeclared items.
    let itf_forward_declarations = r.interfaces.iter().map(|itf| {
        format!( "{}struct {};", get_indentation( 1 ), try_rename( rn, &itf.name ) )
    } ).collect::<Vec<_>>().join( "\n" );

    // Raw interfaces that give direct access to the components implemented in Rust.
    let itfs = r.interfaces.iter().map(|itf| {
        generate_raw_interface( rn, itf, 1 )
    } ).collect::<Vec<_>>().join( "\n" );

    // Generate class descriptors.
    let class_descriptors = r.class_names.iter().map(|class_name| {

        // Get the class details by matching the name.
        let class  = r.classes.iter().find(|cls| &cls.name == class_name )
            .unwrap();

        generate_class_descriptor( rn, &r.libname, class_name, class, 1 )
    } ).collect::<Vec<_>>().join( "\n" );

    // Generate using statements that flatten the raw interfaces
    // to mimic the results of a Microsoft MIDL compiler.
    let flat_interface_declarations = r.interfaces.iter().map(|itf| {
        flatten_interface( rn, &r.libname, itf, 1 )
    } ).collect::<Vec<_>>().join( "\n" );

    // Generate using statements that flatten the class descriptors
    // to mimic the results of a Microsoft MIDL compiler.
    let flat_class_descriptor_declarations = r.class_names.iter().map(|class_name| {

                // Get the class details by matching the name.
        let class  = r.classes.iter().find(|cls| &cls.name == class_name )
            .unwrap();

        flatten_class_descriptor( class_name, class, 1 )
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
        &r.libname, itf_forward_declarations, itfs, class_descriptors,
        flat_interface_declarations, flat_class_descriptor_declarations );
    let raw_namespace = raw_namespace.replace( ">       ", "" );
    writeln!( out, "{}", raw_namespace ).unwrap();
}

fn generate_raw_interface(
    rn : &HashMap<String, String>,
    interface: &Interface,
    base_indentation: usize
) -> String {

    // Now that we have methods sorted out, we can construct the final
    // interface definition.
    let indentation = get_indentation( base_indentation );
    let interface_text = format!( "{}struct {} : IUnknown\n{}{{\n{}static constexpr intercom::IID ID = {};\n\n{}\n{}}};\n",
            indentation, try_rename( rn, &interface.name ), indentation,
            get_indentation( base_indentation + 1 ), guid_to_binary( &interface.iid ),
            generate_raw_methods( interface, base_indentation + 1 ), indentation );
    interface_text
}

fn generate_raw_methods(
    itf: &Interface,
    indentation: usize
) -> String {

    // Get the method definitions for the current interface.
    itf.methods.iter().enumerate().map(|(_i,m)| {

        // Construct the argument list.
        let args = m.arguments.iter().map(|a| {

            // Argument direction affects both the argument attribute and
            // whether the argument is passed by pointer or value.
            let out_ptr = match a.dir {
                ArgDirection::In => "",
                ArgDirection::Out | ArgDirection::Return => "*",
            };
            format!( "{}{} {}", to_cpp_type( &a.ty ), out_ptr, a.name )

        } ).collect::<Vec<_>>().join( ", " );

        // Format the method.
        // To maintain backwards compatibility all new methods should
        // be added to the end of the traits.
        format!( r###"{}virtual {} INTERCOM_CC {}( {} ) = 0;"###, get_indentation( indentation ), to_cpp_type( &m.rvalue ), utils::pascal_case( &m.name ), args )

    } ).collect::<Vec<_>>().join( "\n" )
}

fn flatten_interface(
    rn : &HashMap<String, String>,
    libname: &str,
    interface: &Interface,
    indentation: usize
) -> String {
    let interface_name = try_rename( rn, &interface.name );
    let using_iid = format!( "static constexpr intercom::IID IID_{} = {};",
            utils::pascal_case( &interface_name ), guid_to_binary( &interface.iid ) );
    let using_interface = format!( "using {} = {}::raw::{};", utils::pascal_case( &interface_name ), libname,  &interface_name );
    let indentation = get_indentation( indentation );
    format!("{}{}\n{}{}", indentation, using_iid, indentation, using_interface )
}

fn flatten_class_descriptor(
    class_name: &str,
    coclass: &CoClass,
    indentation: usize
) -> String {
    let using_clsid = format!( "static constexpr intercom::CLSID CLSID_{} = {};",
            utils::pascal_case( class_name ), guid_to_binary( &coclass.clsid ) );
    let indentation = get_indentation( indentation );
    format!("{}{}", indentation, using_clsid )
}

fn generate_class_descriptor(
    rn : &HashMap<String, String>,
    libname: &str,
    class_name: &str,
    coclass: &CoClass,
    indentation: usize
) -> String {

    // Create a list of interfaces to be declared in the class descriptor.
    let interfaces =  coclass.interfaces.iter().map(|itf| {
            format!( r###"{}{}::raw::{}::ID"###,
                &get_indentation( indentation + 2 ), libname, try_rename( rn, itf ) )
        } ).collect::<Vec<_>>().join( ",\n" );

    // Class descriptors hold information about COM classes implemented by the library.
    let clsid = guid_to_binary( &coclass.clsid );
    let class_descriptor: String = format!( r###"
>       class {}Descriptor
>       {{
>           static constexpr intercom::CLSID ID = {};

>           static constexpr std::array<intercom::IID, {}> INTERFACES = {{
{}
>           }};

>           {}Descriptor() = delete;
>           ~{}Descriptor() = delete;

>       }};
>       "###,
    class_name, clsid, coclass.interfaces.len(), interfaces, class_name, class_name );
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
    ty: &str
) -> &str {

    match ty {
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
        _ => ty,
    }
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
