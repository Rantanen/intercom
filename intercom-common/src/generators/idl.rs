
//! Enables the generation of IDL file that describes intercom library.

use std::io::Write;
use std::path::Path;

use super::GeneratorError;

use utils;
use model;
use model::{ComCrate, ComInterfaceVariant};
use tyhandlers::{Direction, TypeSystem, TypeSystemConfig};
use foreign_ty::*;
use type_parser::*;

use handlebars::Handlebars;

#[derive(PartialEq, Serialize, Debug)]
pub struct IdlModel {
    pub lib_id : String,
    pub lib_name : String,
    pub interfaces : Vec<IdlInterface>,
    pub coclasses : Vec<IdlCoClass>,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct IdlInterface {
    pub name : String,
    pub base : Option<String>,
    pub iid : String,
    pub methods : Vec<IdlMethod>,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct IdlMethod {
    pub name : String,
    pub idx : usize,
    pub ret_type : String,
    pub args : Vec<IdlArg>,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct IdlArg {
    pub name : String,
    pub arg_type : String,
    pub attributes : String,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct IdlCoClass {
    pub name : String,
    pub clsid : String,
    pub interfaces : Vec<String>,
}

/// Types that can have IDL representaion can implement this to allow code generation.
trait IdlTypeInfo<'s> {

    /// Gets full type for IDL.
    fn to_idl(
        &self,
        krate : &ComCrate,
        ts_config : &TypeSystemConfig,
    ) -> String;

    /// Gets the IDL compatile type name for this type.
    fn get_idl_type_name(
        &self,
        krate: &model::ComCrate,
        ts_config : &TypeSystemConfig,
    ) -> String;

    /// Determines whether this type should be passed as a pointer.
    fn is_pointer(
        &self
    ) -> bool;
}

impl IdlModel {

    /// Generates the model from files in the path.
    ///
    /// - `path` - The path must point to a crate root containing Cargo.toml or
    ///            to the Cargo.toml itself.
    pub fn from_path(
        path : &Path,
        all_type_systems : bool,
    ) -> Result<IdlModel, GeneratorError>
    {
        let krate = model::ComCrate::parse_package( path )
                .map_err( GeneratorError::CrateParseError )?;
        IdlModel::from_crate( &krate, all_type_systems )
    }

    /// Converts the parse result into an IDL that gets written to stdout.
    pub fn from_crate(
        c : &model::ComCrate,
        all_type_systems : bool,
    ) -> Result<IdlModel, GeneratorError> {

        let itf_variant_filter : Box<Fn( &( &TypeSystem, &ComInterfaceVariant ) ) -> bool> =
                match all_type_systems {
                    true => Box::new( | _ | true ),
                    false => Box::new( | ( ts, _ ) | match ts {
                        TypeSystem::Automation => true,
                        _ => false
                    } ),
                };

        let foreign = CTypeHandler;
        let lib = c.lib().as_ref().ok_or( GeneratorError::MissingLibrary )?;

        // Define all interfaces.
        let itfs = c.interfaces().iter()
            .flat_map(|(_, itf)| itf.variants().iter()
            .filter( itf_variant_filter.as_ref() )
            .map(|(&ts, itf_variant)| {

                // Get the method definitions for the current interface.
                let methods = itf_variant.methods().iter().enumerate().map(|(i,m)| {

                    // Define the config to use when constructing the type names.
                    let ts_config = TypeSystemConfig {
                        effective_system: ts,
                        is_default: ! all_type_systems,
                    };

                    // Construct the argument list.
                    let args = m.raw_com_args().iter().map(|a| {

                        // Argument direction affects both the argument attribute and
                        // whether the argument is passed by pointer or value.
                        let ( attrs, out_ptr ) = match a.dir {
                            Direction::In => ( "in", "" ),
                            Direction::Out => ( "out", "*" ),
                            Direction::Retval => ( "out, retval", "*" ),
                        };

                        // Get the foreign type for the arg type.
                        let com_ty = a.handler.com_ty();
                        let idl_type = foreign
                            .get_ty( &com_ty )
                            .ok_or_else( || GeneratorError::UnsupportedType(
                                            utils::ty_to_string( &a.ty ) ) )?;

                        Ok( IdlArg {
                            name : a.name.to_string(),
                            arg_type : format!( "{}{}", idl_type.to_idl( c, &ts_config ), out_ptr ),
                            attributes : attrs.to_owned(),
                        } )

                    } ).collect::<Result<Vec<_>, GeneratorError>>()?;

                    let ret_ty = m.returnhandler.com_ty();
                    let ret_ty = foreign
                            .get_ty( &ret_ty )
                            .ok_or_else( || GeneratorError::UnsupportedType(
                                            utils::ty_to_string( &ret_ty ) ) )?;
                    Ok( IdlMethod {
                    name: utils::pascal_case( m.display_name.to_string() ),
                        idx: i,
                        ret_type: ret_ty.to_idl( c, &ts_config ),
                        args
                    } )

                } ).collect::<Result<Vec<_>, GeneratorError>>()?;

                let ( itf_name, base_name ) = match all_type_systems {
                    false => (
                        itf.name(),
                        itf.base_interface() ),
                    true => (
                        itf_variant.unique_name(),
                        itf_variant.unique_base_interface() ),
                };

                // Now that we have methods sorted out, we can construct the final
                // interface definition.
                Ok( IdlInterface {
                    name: foreign.get_name( c, itf_name ),
                    base: base_name.as_ref().map( |i| foreign.get_name( c, i ) ),
                    iid: format!( "{:-X}", itf_variant.iid() ),
                    methods,
                } )

            } )
            .collect::<Vec<_>>() )
            .collect::<Result<Vec<_>, GeneratorError>>()?;

        // Create coclass definitions.
        //
        // Here r.class_names contains the class names that were defined in the
        // [com_library] attribute. This is our source for the classes to include
        // in the IDL. r.classes has the actual class details, but might include
        // classes that are not exposed by the library.
        let classes = lib.coclasses().iter().map(|class_name| {

            // Get the class details by matching the name.
            let coclass = &c.structs()[ &class_name.to_string() ];

            // Get the interfaces the class implements.
            let interfaces = coclass.interfaces().iter()
                .flat_map(|itf_name| {
                    let itf = &c.interfaces()[ &itf_name.to_string() ];
                    itf.variants().iter()
                        .filter( itf_variant_filter.as_ref() )
                        .map( |(_, itf_variant)| {

                            foreign.get_name( c, match all_type_systems {
                                false => itf.name(),
                                true => itf_variant.unique_name(),
                            } )
                        } ).collect::<Vec<_>>()
                } ).collect();

            let clsid = coclass.clsid().as_ref()
                    .ok_or_else( || GeneratorError::MissingClsid(
                                        coclass.name().to_string() ) )?;
            Ok( IdlCoClass {
                name : coclass.name().to_string(),
                clsid: format!( "{:-X}", clsid ),
                interfaces
            } )
        } ).collect::<Result<_,GeneratorError>>()?;

        Ok( IdlModel {
            lib_id : format!( "{:-X}", lib.libid() ),
            lib_name : utils::pascal_case( lib.name() ),
            interfaces : itfs,
            coclasses : classes,
        } )
    }

    /// Generates the manifest content.
    ///
    /// - `out` - The writer to use for output.
    pub fn write(
        &self,
        out : &mut Write
    ) -> Result<(), GeneratorError> {

        let mut reg = Handlebars::new();
        reg.register_template_string( "idl", include_str!( "idl.hbs" ) )
                .expect( "Error in the built-in IDL template." );

        let rendered = reg
                .render( "idl", self )
                .expect( "Rendering a valid ComCrate to IDL failed" );
        write!( out, "{}", rendered )?;

        Ok(())
    }
}

impl<'s> IdlTypeInfo<'s> {

    /// Gets the name of a custom type for IDL.
    fn get_idl_name_for_custom_type(
        krate : &ComCrate,
        ty_name : &str,
        ts_config : &TypeSystemConfig,
    ) -> String {

        let itf = if let Some( itf ) = krate.interfaces().get( ty_name ) {
            itf
        } else {
            return ty_name.to_owned()
        };

        let base_name = if itf.item_type() == ::utils::InterfaceType::Struct {
            format!( "I{}", itf.name() )
        } else {
            ty_name.to_string()
        };

        ts_config.get_unique_name( &base_name )
    }
}

impl<'s> IdlTypeInfo<'s> for TypeInfo<'s> {

    fn to_idl(
        &self,
        krate : &ComCrate,
        ts_config : &TypeSystemConfig,
    ) -> String {

        // We want to enable if for interface methods and parameters.
        let const_specifier = if self.is_mutable || self.pass_by != PassBy::Reference { "" } else { "const " };

        let type_name = self.get_leaf().get_idl_type_name( krate, ts_config );
        let ptr = if self.is_pointer() { "*" } else { "" };
        format!("{}{}{}", const_specifier, type_name, ptr )
    }

    /// Gets the name of the IDL type for this type.
    fn get_idl_type_name(
        &self,
        krate : &ComCrate,
        ts_config : &TypeSystemConfig,
    ) -> String {

        let type_name = self.get_name();
        match type_name.as_str() {
            "RawComPtr" => "*void".to_owned(),
            "InBSTR" | "OutBSTR" => "BSTR".to_owned(),
            "usize" => "size_t".to_owned(),
            "u64" => "uint64".to_owned(),
            "i64" => "int64".to_owned(),
            "u32" => "uint32".to_owned(),
            "i32" => "int32".to_owned(),
            "u16" => "uint16".to_owned(),
            "i16" => "int16".to_owned(),
            "u8" => "uint8".to_owned(),
            "i8" => "int8".to_owned(),
            "f64" => "double".to_owned(),
            "f32" => "float".to_owned(),
            "c_void" => "void".to_owned(),
            t => IdlTypeInfo::get_idl_name_for_custom_type( krate, t, ts_config ),
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
            PassBy::Value => false,
            PassBy::Reference | PassBy::Ptr => true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn crate_to_idl() {

        let krate = model::ComCrate::parse( "com_library", &[ r#"
            #[com_library( libid = "11112222-3333-4444-5555-666677778888", CoClass )]
            struct S;

            #[com_interface( com_iid = "22223333-4444-5555-6666-777788889999", base = NO_BASE )]
            trait IInterface {
                fn method( &self, a : u32 ) -> ComResult<bool>;
            }

            #[com_class( clsid = "33334444-5555-6666-7777-888899990000", CoClass, IInterface )]
            struct CoClass;

            #[com_interface( com_iid = "44445555-6666-7777-8888-99990000AAAA" )]
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

        let expected_idl = IdlModel {
            lib_id : "11112222-3333-4444-5555-666677778888".to_owned(),
            lib_name : "ComLibrary".to_owned(),
            interfaces : vec![
                IdlInterface {
                    name : "IInterface".to_owned(),
                    base : None,
                    iid : "22223333-4444-5555-6666-777788889999".to_owned(),
                    methods : vec![
                        IdlMethod {
                            name : "Method".to_owned(),
                            idx : 0,
                            ret_type : "HRESULT".to_owned(),
                            args : vec![
                                IdlArg {
                                    name : "a".to_owned(),
                                    arg_type : "uint32".to_owned(),
                                    attributes : "in".to_owned(),
                                },
                                IdlArg {
                                    name : "__out".to_owned(),
                                    arg_type : "bool*".to_owned(),
                                    attributes : "out, retval".to_owned(),
                                },
                            ]
                        }
                    ]
                },
                IdlInterface {
                    name : "ICoClass".to_owned(),
                    base : Some( "IUnknown".to_owned() ),
                    iid : "44445555-6666-7777-8888-99990000AAAA".to_owned(),
                    methods : vec![
                        IdlMethod {
                            name : "ComMethod".to_owned(),
                            idx : 0,
                            ret_type : "void".to_owned(),
                            args : vec![
                                IdlArg {
                                    name : "b".to_owned(),
                                    arg_type : "uint32".to_owned(),
                                    attributes : "in".to_owned(),
                                },
                            ]
                        }
                    ]
                },
                IdlInterface {
                    name : "IAllocator".to_owned(),
                    base : Some( "IUnknown".to_owned() ),
                    iid : "18EE22B3-B0C6-44A5-A94A-7A417676FB66".to_owned(),
                    methods : vec![
                        IdlMethod {
                            name : "AllocBstr".to_owned(),
                            idx : 0,
                            ret_type : "BSTR".to_owned(),
                            args : vec![
                                IdlArg {
                                    name : "text".to_owned(),
                                    arg_type : "uint16*".to_owned(),
                                    attributes : "in".to_owned(),
                                },
                                IdlArg {
                                    name : "len".to_owned(),
                                    arg_type : "uint32".to_owned(),
                                    attributes : "in".to_owned(),
                                },
                            ]
                        },
                        IdlMethod {
                            name : "FreeBstr".to_owned(),
                            idx : 1,
                            ret_type : "void".to_owned(),
                            args : vec![
                                IdlArg {
                                    name : "bstr".to_owned(),
                                    arg_type : "BSTR".to_owned(),
                                    attributes : "in".to_owned(),
                                },
                            ]
                        },
                        IdlMethod {
                            name : "Alloc".to_owned(),
                            idx : 2,
                            ret_type : "void*".to_owned(),
                            args : vec![
                                IdlArg {
                                    name : "len".to_owned(),
                                    arg_type : "size_t".to_owned(),
                                    attributes : "in".to_owned(),
                                },
                            ]
                        },
                        IdlMethod {
                            name : "Free".to_owned(),
                            idx : 3,
                            ret_type : "void".to_owned(),
                            args : vec![
                                IdlArg {
                                    name : "ptr".to_owned(),
                                    arg_type : "void*".to_owned(),
                                    attributes : "in".to_owned(),
                                },
                            ]
                        },
                    ]
                },
            ],
            coclasses : vec![
                IdlCoClass {
                    name : "CoClass".to_owned(),
                    clsid : "33334444-5555-6666-7777-888899990000".to_owned(),
                    interfaces : vec![
                        "ICoClass".to_owned(),
                        "IInterface".to_owned(),
                    ],
                },
                IdlCoClass {
                    name : "Allocator".to_owned(),
                    clsid : "EC444090-9CDC-31A4-4023-D0458C5CD45C".to_owned(),
                    interfaces : vec![
                        "IAllocator".to_owned(),
                    ]
                },
            ],
        };

        let actual_idl = IdlModel::from_crate( &krate, false ).unwrap();

        assert_eq!( expected_idl, actual_idl );
    }
}
