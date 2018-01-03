use std::collections::HashMap;
use parse::*;
use intercom_common::utils;
use cpp::*;

/// Converts the parse result into an header  that gets written to stdout.
pub fn generate(
    r : &ParseResult,
    rn : &HashMap<String, String>
) -> String {

    // Generate descriptor for the library.
    let library_descriptor = generate_library_descriptor( 1 );

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
#ifndef INTERCOM_LIBRARY_{}_H
#define INTERCOM_LIBRARY_{}_H
>       #include <array>
>       #include <intercom.h>
>       namespace {}
>       {{
>       {}

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
#endif
>       "###,
        &r.libname, &r.libname, &r.libname,
        library_descriptor, itf_forward_declarations, itfs, class_descriptors,
        flat_interface_declarations, flat_class_descriptor_declarations );
    raw_namespace.replace( ">       ", "" )
}

fn generate_library_descriptor(
    base_indentation: usize
) -> String {

    let member_indentation = get_indentation( base_indentation + 1 );
    let windows_name = format!("{}static const char WINDOWS_NAME[];", member_indentation );
    let posix_name = format!("{}static const char POSIX_NAME[];", member_indentation );
    let name = format!("{}static const char NAME[];", member_indentation );

    let descriptor_indentation = get_indentation( base_indentation );
    format!( "{}class Descriptor\n{}{{\n{}public:\n{}\n{}\n{}\n{}}};",
        descriptor_indentation, descriptor_indentation, descriptor_indentation,
        name, windows_name, posix_name, descriptor_indentation )
}

fn generate_raw_interface(
    rn : &HashMap<String, String>,
    interface: &Interface,
    base_indentation: usize
) -> String {

    // Now that we have methods sorted out, we can construct the final
    // interface definition.
    let indentation = get_indentation( base_indentation );
    let interface_text = format!( "{}struct {} : IUnknown\n{}{{\n{}static constexpr intercom::IID ID = {};\n{}\n{}}};\n",
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
>       public:

>           static constexpr intercom::CLSID ID = {};

>           static constexpr std::array<intercom::IID, {}> INTERFACES = {{
{}
>           }};

>           using Library = {}::Descriptor;

>           {}Descriptor() = delete;
>           ~{}Descriptor() = delete;

>       }};"###,
    class_name, clsid, coclass.interfaces.len(), interfaces, libname, class_name, class_name, );
    class_descriptor.replace( ">       ", &get_indentation( indentation ) )
}

