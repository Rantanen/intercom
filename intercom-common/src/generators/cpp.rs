
//! Enables the generation of header and source files for using intercom
//! libraries from C++ projects.

extern crate std;

use std::io::Write;
use std::path::Path;

use super::GeneratorError;

use crate::tyhandlers::{Direction, ModelTypeSystem, ModelTypeSystemConfig};
use crate::foreign_ty::*;
use crate::type_parser::*;
use crate::guid::*;
use crate::model;
use crate::model::{ComCrate, ComInterfaceVariant};
use crate::ast_converters::GetIdent;
use crate::utils;

use handlebars::Handlebars;

#[derive(PartialEq, Serialize, Debug)]
pub struct CppModel {
    pub lib_name : String,
    pub interfaces: Vec<CppInterface>,
    pub coclass_count : usize,
    pub coclasses: Vec<CppCoClass>,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct CppInterface {
    pub name : String,
    pub iid_struct : String,
    pub base : Option<String>,
    pub methods : Vec<CppMethod>,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct CppMethod {
    pub name : String,
    pub ret_type : String,
    pub args : Vec<CppArg>,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct CppArg {
    pub name : String,
    pub arg_type : String,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct CppCoClass {
    pub name : String,
    pub clsid_struct : String,
    pub interface_count : usize,
    pub interfaces : Vec<String>,
}

/// Types that can have C++ representaion can implement this to allow code generation.
trait CppTypeInfo<'s> {

    /// Gets full type for C++.
    fn to_cpp(
        &self,
        direction: Direction,
        krate : &ComCrate,
        ts_config : &ModelTypeSystemConfig,
    ) -> String;

    /// Gets the C++ compatile type name for this type.
    fn get_cpp_type_name(
        &self,
        krate : &ComCrate,
        ts_config : &ModelTypeSystemConfig,
    ) -> String;

    /// Determines whether this type should be passed as a pointer.
    fn is_pointer(
        &self
    ) -> bool;
}

impl<'s> dyn CppTypeInfo<'s> {

    /// Gets the name of a custom type for C++.
    fn get_cpp_name_for_custom_type(
        krate : &ComCrate,
        ty_name : &str,
        ts_config : &ModelTypeSystemConfig,
    ) -> String {

        let itf = if let Some( itf ) = krate.interfaces().get( ty_name ) {
            itf
        } else {
            return ty_name.to_owned()
        };

        let base_name = if itf.item_type() == crate::utils::InterfaceType::Struct {
            format!( "I{}", itf.name() )
        } else {
            ty_name.to_owned()
        };

        ts_config.get_unique_name( &base_name )
    }
}

impl<'s> CppTypeInfo<'s> for TypeInfo<'s> {

    fn to_cpp(
        &self,
        direction: Direction,
        krate : &ComCrate,
        ts_config : &ModelTypeSystemConfig,
    ) -> String
    {
        // Argument direction affects both the argument attribute and
        // whether the argument is passed by pointer or value.
        let out_ptr = match direction {
            Direction::In
            | Direction::Retval => "",
            Direction::Out => "*",
        };

        // TODO: Enable once verified that the "const" works.
        // We want to enable if for interface methods and parameters.
        // let const_specifier = if self.is_mutable || self.pass_by != PassBy::Reference { "" } else { "const " };
        let const_specifier = "";

        let type_name = self.get_cpp_type_name( krate, ts_config );
        let ptr = if self.is_pointer() { "*" } else { "" };
        let ptr = format!( "{}{}", ptr, out_ptr );
        format!("{}{}{}", const_specifier, type_name, ptr )
    }

    fn get_cpp_type_name(
        &self,
        krate : &ComCrate,
        ts_config : &ModelTypeSystemConfig,
    ) -> String {

        let type_name = self.get_leaf().get_name();
        match type_name.as_str() {
            "RawComPtr" => "*void".to_owned(),
            "Variant" => "intercom::VARIANT".to_owned(),
            "VariantBool" => "intercom::VARIANT_BOOL".to_owned(),
            "InBSTR" | "OutBSTR" => "intercom::BSTR".to_owned(),
            "InCStr" | "OutCStr" => "char*".to_owned(),
            "usize" => "size_t".to_owned(),
            "i8" => "int8_t".to_owned(),
            "u8" => "uint8_t".to_owned(),
            "i16" => "int16_t".to_owned(),
            "u16" => "uint16_t".to_owned(),
            "i32" => "int32_t".to_owned(),
            "u32" => "uint32_t".to_owned(),
            "i64" => "int64_t".to_owned(),
            "u64" => "uint64_t".to_owned(),
            "HRESULT" => "intercom::HRESULT".to_owned(),
            "f64" => "double".to_owned(),
            "f32" => "float".to_owned(),
            "c_void" => "void".to_owned(),
            "c_char" => "char".to_owned(),
            t => CppTypeInfo::get_cpp_name_for_custom_type( krate, t, ts_config ),
        }
    }

    fn is_pointer(
        &self
    ) -> bool
    {
        // Rust wrappers represent reference counted objects
        // so they are always passed as a pointer.
        if let RustType::Wrapper( _, _ ) = self.rust_type {
            return true;
        }

        match self.pass_by {
            PassBy::Value
            | PassBy::Reference => false,
            PassBy::Ptr => true,
        }
    }
}

impl CppModel {

    /// Generates the model from files in the path.
    ///
    /// - `path` - The path must point to a crate root containing Cargo.toml or
    ///            to the Cargo.toml itself.
    pub fn from_path(
        path : &Path,
        all_type_systems : bool,
    ) -> Result<CppModel, GeneratorError>
    {
        let krate = model::ComCrate::parse_package( path )
                .map_err( GeneratorError::CrateParseError )?;
        CppModel::from_crate( &krate, all_type_systems )
    }


    /// Converts the parse result into an header  that gets written to stdout.
    pub fn from_crate(
        c : &model::ComCrate,
        all_type_systems : bool,
    ) -> Result<CppModel, GeneratorError> {

        let itf_variant_filter : Box<dyn Fn( &( &ModelTypeSystem, &ComInterfaceVariant ) ) -> bool> =
                match all_type_systems {
                    true => Box::new( | _ | true ),
                    false => Box::new( | ( ts, _ ) | match ts {
                        ModelTypeSystem::Raw => true,
                        _ => false
                    } ),
                };

        let foreign = CTypeHandler;
        let lib = c.lib().as_ref().ok_or( GeneratorError::MissingLibrary )?;

        // Introduce all interfaces so we don't get errors on undeclared items.
        let interfaces = c.interfaces().iter()
            .flat_map(|(_, itf)| itf.variants().iter()
            .filter( itf_variant_filter.as_ref() )
            .map(|(&ts, itf_variant)| {

                // Define the config to use when constructing the type names.
                let ts_config = ModelTypeSystemConfig {
                    effective_system: ts,
                    is_default: ! all_type_systems,
                };

                // Get the method definitions for the current interface.
                let methods = itf_variant.methods().iter().map( |m| {

                    // Construct the argument list.
                    let args = m.raw_com_args().iter().map( |a| {

                        // Redirect return values converted out arguments.
                        let dir = match a.dir {
                            Direction::Retval => Direction::Out,
                            d => d,
                        };

                        // Get the foreign type for the arg type in C++ format.
                        let com_ty = a.handler.com_ty( dir );
                        let type_info = foreign.get_ty( &com_ty )
                                .ok_or_else( || GeneratorError::UnsupportedType(
                                                utils::ty_to_string( &a.ty ) ) )?;
                        Ok( CppArg {
                            name : a.name.to_string(),
                            arg_type : type_info.to_cpp( dir, c, &ts_config ),
                        } )

                    } ).collect::<Result<Vec<_>, GeneratorError>>()?;

                    let ret_ty = m.returnhandler.com_ty();
                    let ret_ty = foreign.get_ty( &ret_ty )
                            .ok_or_else( || GeneratorError::UnsupportedType(
                                            utils::ty_to_string( &ret_ty ) ) )?;
                    Ok( CppMethod {
                    name: utils::pascal_case( m.display_name.to_string() ),
                        ret_type: ret_ty.to_cpp( Direction::Retval, c, &ts_config ),
                        args
                    } )

                } ).collect::<Result<Vec<_>, GeneratorError>>()?;

                let ( itf_name, iid ) = match all_type_systems {
                    false => (
                        itf.name(),
                        itf.variants()[ &ModelTypeSystem::Raw ].iid() ),
                    true => (
                        itf_variant.unique_name(),
                        itf_variant.iid() ),
                };

                Ok( CppInterface {
                    name: foreign.get_name( c, itf_name ),
                    iid_struct: guid_as_struct( iid ),
                    base: itf.base_interface().as_ref()
                            .map( |i| foreign.get_name( c, i ) ),
                    methods,
                } )

            } )
            .collect::<Vec<_>>() )
            .collect::<Result<Vec<_>, GeneratorError>>()?;

        // Generate class descriptors.
        let classes = lib.coclasses().iter().map( |class_path| {

            // Get the class details by matching the name.
            let class_name = class_path
                    .get_ident()
                    .expect( "coclass had no name" )
                    .to_string();
            let coclass = &c.structs().get( &class_name )
                    .ok_or_else( || GeneratorError::TypeNotFound( class_name.clone() ) )?;

            // Create a list of interfaces to be declared in the class descriptor.
            let interfaces = coclass.interfaces().iter()
                .flat_map(|itf_name| {
                    let result = c.interfaces().get( &itf_name.to_string() )
                            .ok_or_else( || GeneratorError::TypeNotFound( itf_name.to_string() ) );

                    match result {
                        Ok( itf ) => itf.variants().iter()
                            .filter( itf_variant_filter.as_ref() )
                            .map( |(_, itf_variant)| {

                                Ok( foreign.get_name( c, match all_type_systems {
                                    false => itf.name(),
                                    true => itf_variant.unique_name(),
                                } ) )
                            } ).collect::<Vec<_>>(),
                        Err( e ) => vec![ Err( e ) ],
                    }
                } )
                .collect::<Result<Vec<_>, GeneratorError>>()?;

            let clsid = coclass.clsid().as_ref()
                    .ok_or_else( || GeneratorError::MissingClsid(
                                        coclass.name().to_string() ) )?;

            Ok( CppCoClass {
                name : class_name.to_string(),
                clsid_struct : guid_as_struct( clsid ),
                interface_count : interfaces.len(),
                interfaces,
            } )

        } ).collect::<Result<Vec<_>, GeneratorError>>()?;

        Ok( CppModel {
            lib_name : lib.name().to_owned(),
            interfaces,
            coclass_count: classes.len(),
            coclasses : classes,
        } )
    }

    /// Generates the C++ header file.
    ///
    /// - `out` - The writer to use for output.
    pub fn write_header(
        &self,
        out : &mut dyn Write,
    ) -> Result<(), GeneratorError>
    {
        let mut reg = Handlebars::new();
        reg.register_template_string(
                "cpp_header",
                include_str!( "cpp_header.hbs" ) )
            .expect( "Error in the built-in C++ header template." );

        let rendered = reg
                .render( "cpp_header", self )
                .expect( "Rendering a valid ComCrate to C++ header failed" );
        write!( out, "{}", rendered )?;

        Ok(())
    }

    /// Generates the C++ source file.
    ///
    /// - `out` - The writer to use for output.
    pub fn write_source(
        &self,
        out : &mut dyn Write,
    ) -> Result<(), GeneratorError>
    {
        let mut reg = Handlebars::new();
        reg.register_template_string(
                "cpp_source",
                include_str!( "cpp_source.hbs" ) )
            .expect( "Error in the built-in C++ source template." );

        let rendered = reg
                .render( "cpp_source", self )
                .expect( "Rendering a valid ComCrate to C++ source failed" );
        write!( out, "{}", rendered )?;

        Ok(())
    }
}

/// Converts a guid to binarys representation.
pub fn guid_as_struct(
    g: &GUID
) -> String {

    format!( "{{0x{:08x},0x{:04x},0x{:04x},{{0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x}}}}}",
            g.data1, g.data2, g.data3,
            g.data4[0], g.data4[1], g.data4[2], g.data4[3],
            g.data4[4], g.data4[5], g.data4[6], g.data4[7] )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn crate_to_cpp() {

        let krate = model::ComCrate::parse( "com_library", &[ r#"
            com_library!( libid = "11112222-3333-4444-5555-666677778888", CoClass );

            #[com_interface( raw_iid = "22223333-4444-5555-6666-777788889999", base = NO_BASE )]
            trait IInterface {
                fn method( &self, a : u32 ) -> ComResult<bool>;
            }

            #[com_class( clsid = "33334444-5555-6666-7777-888899990000", CoClass, IInterface )]
            struct CoClass;

            #[com_interface( raw_iid = "44445555-6666-7777-8888-99990000AAAA" )]
            #[com_impl]
            impl CoClass {
                pub fn new() -> CoClass { CoClass }
                pub fn com_method( &self, b : u32 ) {}
            }

            #[com_impl]
            impl IInterface for CoClass {
                fn method( &self, a : u32 ) -> ComResult<bool> { unreachable!() }
            }
        "# ] ).expect( "Could not parse test crate" );

        let expected_cpp = CppModel {
            lib_name : "com_library".to_owned(),
            interfaces : vec![
                CppInterface {
                    name : "IInterface".to_owned(),
                    base : None,
                    iid_struct : "{0x22223333,0x4444,0x5555,{0x66,0x66,0x77,0x77,0x88,0x88,0x99,0x99}}".to_owned(),
                    methods : vec![
                        CppMethod {
                            name : "Method".to_owned(),
                            ret_type : "intercom::HRESULT".to_owned(),
                            args : vec![
                                CppArg {
                                    name : "a".to_owned(),
                                    arg_type : "uint32_t".to_owned(),
                                },
                                CppArg {
                                    name : "__out".to_owned(),
                                    arg_type : "bool*".to_owned(),
                                },
                            ]
                        }
                    ]
                },
                CppInterface {
                    name : "ICoClass".to_owned(),
                    base : Some( "IUnknown".to_owned() ),
                    iid_struct : "{0x44445555,0x6666,0x7777,{0x88,0x88,0x99,0x99,0x00,0x00,0xaa,0xaa}}".to_owned(),
                    methods : vec![
                        CppMethod {
                            name : "ComMethod".to_owned(),
                            ret_type : "void".to_owned(),
                            args : vec![
                                CppArg {
                                    name : "b".to_owned(),
                                    arg_type : "uint32_t".to_owned(),
                                },
                            ]
                        }
                    ]
                },
                CppInterface {
                    name : "IAllocator".to_owned(),
                    base : Some( "IUnknown".to_owned() ),
                    iid_struct : "{0x7a6f6564,0x04b5,0x4455,{0xa2,0x23,0xea,0x05,0x12,0xb8,0xcc,0x63}}".to_owned(),
                    methods : vec![
                        CppMethod {
                            name : "AllocBstr".to_owned(),
                            ret_type : "char*".to_owned(),
                            args : vec![
                                CppArg {
                                    name : "text".to_owned(),
                                    arg_type : "uint16_t*".to_owned(),
                                },
                                CppArg {
                                    name : "len".to_owned(),
                                    arg_type : "uint32_t".to_owned(),
                                },
                            ]
                        },
                        CppMethod {
                            name : "FreeBstr".to_owned(),
                            ret_type : "void".to_owned(),
                            args : vec![
                                CppArg {
                                    name : "bstr".to_owned(),
                                    arg_type : "char*".to_owned(),
                                },
                            ]
                        },
                        CppMethod {
                            name : "Alloc".to_owned(),
                            ret_type : "void*".to_owned(),
                            args : vec![
                                CppArg {
                                    name : "len".to_owned(),
                                    arg_type : "size_t".to_owned(),
                                },
                            ]
                        },
                        CppMethod {
                            name : "Free".to_owned(),
                            ret_type : "void".to_owned(),
                            args : vec![
                                CppArg {
                                    name : "ptr".to_owned(),
                                    arg_type : "void*".to_owned(),
                                },
                            ]
                        },
                    ]
                },
                CppInterface {
                    name : "IErrorStore".to_owned(),
                    base : Some( "IUnknown".to_owned() ),
                    iid_struct : "{0x7586c49a,0xabbd,0x4a06,{0xb5,0x88,0xe3,0xd0,0x2b,0x43,0x1f,0x01}}".to_owned(),
                    methods : vec![
                        CppMethod {
                            name : "GetErrorInfo".to_owned(),
                            ret_type : "intercom::HRESULT".to_owned(),
                            args : vec![
                                CppArg {
                                    name : "__out".to_owned(),
                                    arg_type : "IErrorInfo**".to_owned(),
                                },
                            ]
                        },
                        CppMethod {
                            name : "SetErrorInfo".to_owned(),
                            ret_type : "intercom::HRESULT".to_owned(),
                            args : vec![
                                CppArg {
                                    name : "info".to_owned(),
                                    arg_type : "IErrorInfo*".to_owned(),
                                },
                            ]
                        },
                        CppMethod {
                            name : "SetErrorMessage".to_owned(),
                            ret_type : "intercom::HRESULT".to_owned(),
                            args : vec![
                                CppArg {
                                    name : "msg".to_owned(),
                                    arg_type : "char*".to_owned(),
                                },
                            ]
                        },
                    ]
                },
            ],
            coclass_count: 3,
            coclasses : vec![
                CppCoClass {
                    name : "CoClass".to_owned(),
                    clsid_struct : "{0x33334444,0x5555,0x6666,{0x77,0x77,0x88,0x88,0x99,0x99,0x00,0x00}}".to_owned(),
                    interface_count: 2,
                    interfaces : vec![
                        "ICoClass".to_owned(),
                        "IInterface".to_owned(),
                    ],
                },
                CppCoClass {
                    name : "Allocator".to_owned(),
                    clsid_struct : "{0xec444090,0x9cdc,0x31a4,{0x40,0x23,0xd0,0x45,0x8c,0x5c,0xd4,0x5c}}".to_owned(),
                    interface_count: 1,
                    interfaces : vec![
                        "IAllocator".to_owned(),
                    ],
                },
                CppCoClass {
                    name : "ErrorStore".to_owned(),
                    clsid_struct : "{0x1467b819,0x62df,0x3720,{0x4e,0xe6,0x6e,0x76,0xfd,0x4e,0x11,0x20}}".to_owned(),
                    interface_count: 1,
                    interfaces : vec![
                        "IErrorStore".to_owned(),
                    ]
                },
            ],
        };

        let actual_cpp = CppModel::from_crate( &krate, false ).unwrap();

        assert_eq!( expected_cpp, actual_cpp );
    }

    #[test]
    fn bstr_method() {

        let krate = model::ComCrate::parse( "com_library", &[ r#"
            com_library!( libid = "11112222-3333-4444-5555-666677778888", CoClass );

            #[com_class( clsid = "33334444-5555-6666-7777-888899990000", CoClass )]
            struct CoClass;

            #[com_interface( raw_iid = "44445555-6666-7777-8888-99990000AAAA" )]
            #[com_impl]
            impl CoClass {
                pub fn new() -> CoClass { CoClass }
                pub fn cstr_method( &self, b : String ) -> String {}
            }
        "# ] ).expect( "Could not parse test crate" );

        let expected_method =
            CppMethod {
                name : "CstrMethod".to_owned(),
                ret_type : "char*".to_owned(),
                args : vec![
                    CppArg {
                        name : "b".to_owned(),
                        arg_type : "char*".to_owned(),
                    },
                ]
            };

        let actual_model = CppModel::from_crate( &krate, false ).unwrap();
        let actual_method =
                actual_model
                    .interfaces
                        .iter()
                        .find( |c| c.name == "ICoClass" )
                        .unwrap()
                    .methods
                        .iter()
                        .find( |m| m.name == "CstrMethod" )
                        .unwrap();

        assert_eq!( &expected_method, actual_method );
    }
}
