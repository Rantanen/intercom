
//! Enables the generation of IDL file that describes intercom library.

use std::io::Write;
use std::path::Path;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::borrow::Cow;

use super::GeneratorError;
use super::{ModelOptions, TypeSystemOptions, pascal_case};

use handlebars::Handlebars;
use serde_derive::Serialize;

use intercom::typelib::{TypeLib, TypeInfo, CoClass, Interface, InterfaceVariant, Method, Arg, Direction};

#[derive(Debug, Serialize)]
struct IdlLibrary {
    pub lib_name: String,
    pub lib_id: String,
    pub interfaces: Vec<IdlInterface>,
    pub coclasses: Vec<IdlClass>,
}

#[derive(Debug, Serialize)]
struct IdlInterface {
    name: String,
    base: Option<String>,
    iid: String,
    methods: Vec<IdlMethod>,
}

#[derive(Debug, Serialize)]
struct IdlMethod {
    pub name: String,
    pub idx: usize,
    pub ret_type: String,
    pub args: Vec<IdlArg>,
}

#[derive(Debug, Serialize)]
struct IdlArg {
    pub name: String,
    pub arg_type: String,
    pub attributes: String,
}

#[derive(Debug, Serialize)]
struct IdlClass {
    pub name : String,
    pub clsid : String,
    pub interfaces : Vec<String>,
}

struct LibraryContext<'a> {
    pub itfs_by_ref: &'a HashMap<String, &'a Interface>,
    pub itfs_by_name: &'a HashMap<String, &'a Interface>,
}

impl IdlLibrary {

    fn try_from(
        lib : TypeLib,
        opts: &ModelOptions
    ) -> Result<Self, GeneratorError> {

        let itfs_by_name : HashMap<String, &Interface>
            = lib.types.iter()
                .filter_map( |t| match t {
                    TypeInfo::Interface(itf) => Some(itf),
                    _ => None
                } )
                .map( |itf| ( itf.as_ref().name.to_string(), &**(itf.as_ref()) ) )
                .collect();
        let itfs_by_ref : HashMap<String, &Interface>
            = lib.types.iter()
                .filter_map( |t| match t {
                    TypeInfo::Class(cls) => Some(cls),
                    _ => None
                } )
                .flat_map( |cls| &cls.as_ref().interfaces )
                .map( |itf_ref| ( itf_ref.name.to_string(), itfs_by_name[ itf_ref.name.as_ref() ] ) )
                .collect();
        let context = LibraryContext {
            itfs_by_name: &itfs_by_name,
            itfs_by_ref: &itfs_by_ref };

        let mut interfaces = vec![];
        let mut coclasses = vec![];
        for t in &lib.types {
            match t {
                TypeInfo::Class(cls)
                    => coclasses.push(IdlClass::try_from(cls.as_ref(), opts, &context)?),
                TypeInfo::Interface(itf)
                    => interfaces.push(IdlInterface::gather(itf.as_ref(), opts, &context)?),
            }
        }
        let interfaces = interfaces
                .into_iter()
                .flatten()
                .collect::<Vec<IdlInterface>>();

        Ok( Self {
            lib_name: pascal_case( lib.name ),
            lib_id: format!( "{:-X}", lib.libid ),
            interfaces,
            coclasses,
        } )
    }
}

impl IdlInterface {

    fn gather(
        itf: &Interface,
        opts: &ModelOptions,
        ctx: &LibraryContext,
    ) -> Result<Vec<Self>, GeneratorError>
    {
        Ok( opts.type_systems.iter().map( |ts_opts| {
            match itf.variants.iter().find(|v| v.as_ref().ts == ts_opts.ts) {
                Some(v) => Some( IdlInterface::try_from(&itf, v.as_ref(), ts_opts, ctx) ),
                None => None
            }
        } ).filter_map(|i| i).collect::<Result<Vec<_>, _>>()? )
    }

    fn try_from(
        itf: &Interface,
        itf_variant: &InterfaceVariant,
        ts_opts: &TypeSystemOptions,
        ctx: &LibraryContext,
    ) -> Result<Self, GeneratorError>
    {
        Ok( Self {
            name: Self::final_name( &itf, ts_opts ),
            iid: format!( "{:-X}", itf_variant.iid ),
            base: Some("IUnknown".to_string()),
            methods: itf_variant.methods
                .iter()
                .enumerate()
                .map(|(i, m)| IdlMethod::try_from(i, m.as_ref(), ts_opts, ctx))
                .collect::<Result<Vec<_>, _>>()?
        } )
    }

    pub fn final_name(
        itf: &Interface,
        opts: &TypeSystemOptions,
    ) -> String {
        let base_name = if itf.options.class_impl_interface {
            Cow::from( format!("I{}", itf.name) )
        } else {
            itf.name.clone()
        };

        match opts.use_full_name {
            true => format!( "{}_{:?}", base_name, opts.ts ),
            false => base_name.to_string(),
        }
    }
}

impl IdlMethod {
    fn try_from(
        idx: usize,
        method: &Method,
        opts: &TypeSystemOptions,
        ctx: &LibraryContext,
    ) -> Result<Self, GeneratorError>
    {
        Ok( Self {
            name: pascal_case( &method.name ),
            idx,
            ret_type: IdlArg::idl_type(&method.return_type, opts, ctx),
            args: method.parameters
                .iter()
                .map(|arg| IdlArg::try_from(arg, opts, ctx))
                .collect::<Result<Vec<_>, _>>()?
        } )
    }
}

impl IdlArg {
    fn try_from(
        arg: &Arg,
        opts: &TypeSystemOptions,
        ctx: &LibraryContext,
    ) -> Result<Self, GeneratorError>
    {
        let mut attrs = vec![];
        match arg.direction {
            Direction::In => attrs.push( "in" ),
            Direction::Out => attrs.push( "out" ),
            Direction::Retval => {
                attrs.push( "out" );
                attrs.push( "retval" );
            },
            Direction::Return => {
                Err( "Direction::Return is invalid direction for arguments".to_string() )?
            }
        }

        Ok( Self {
            name: arg.name.to_string(),
            arg_type: Self::idl_type(arg, opts, ctx),
            attributes: attrs.join(", "),
        } )
    }

    fn idl_type(
        arg: &Arg,
        opts: &TypeSystemOptions,
        ctx: &LibraryContext,
    ) -> String {
        let base_name = ctx.itfs_by_name.get(arg.ty.as_ref())
            .map(|itf| IdlInterface::final_name(itf, opts))
            .unwrap_or_else(|| arg.ty.to_string());
        let mut indirection = match arg.direction {
            Direction::In | Direction::Return => arg.indirection_level,
            Direction::Out | Direction::Retval => arg.indirection_level + 1,
        };

        let base_name = match base_name.as_ref() {
            "std::ffi::c_void" => "void".to_string(),
            other => other.to_string()
        };

        format!( "{}{}", base_name, "*".repeat(indirection as usize) )
    }
}

impl IdlClass {
    fn try_from(
        cls: &CoClass,
        opts: &ModelOptions,
        ctx: &LibraryContext,
    ) -> Result<Self, GeneratorError>
    {
        let interfaces = cls.interfaces
            .iter()
            .flat_map(|itf_ref| {
                opts.type_systems.iter().map(|opt| {
                    let itf = ctx.itfs_by_ref[itf_ref.name.as_ref()];
                    IdlInterface::final_name(itf, opt)
                }).collect::<Vec<_>>()
            }).collect();
        Ok(IdlClass {
            name: cls.name.to_string(),
            clsid: format!("{:-X}", cls.clsid),
            interfaces: interfaces,
        })
    }
}

/// Generates the manifest content.
///
/// - `out` - The writer to use for output.
pub fn write(
    lib : intercom::typelib::TypeLib,
    opts: ModelOptions,
    out : &mut dyn Write,
) -> Result<(), GeneratorError> {

    let mut reg = Handlebars::new();
    reg.register_template_string( "idl", include_str!( "idl.hbs" ) )
            .expect( "Error in the built-in IDL template." );

    let idl_model = IdlLibrary::try_from( lib, &opts )?;

    let rendered = reg
            .render( "idl", &idl_model )
            .expect( "Rendering a valid ComCrate to IDL failed" );
    write!( out, "{}", rendered )?;

    Ok(())
}
