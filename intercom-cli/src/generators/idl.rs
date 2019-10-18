
//! Enables the generation of IDL file that describes intercom library.

use std::io::Write;
use std::path::Path;
use std::convert::TryFrom;

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

impl IdlLibrary {

    fn try_from(lib : TypeLib, opts: &ModelOptions) -> Result<Self, GeneratorError> {
        let mut interfaces = vec![];
        let mut coclasses = vec![];
        for t in lib.types {
            match t {
                TypeInfo::Class(cls)
                    => coclasses.push(IdlClass::try_from(cls.as_ref(), opts)?),
                TypeInfo::Interface(itf)
                    => interfaces.push(IdlInterface::gather(itf.as_ref(), opts)?),
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
        opts: &ModelOptions
    ) -> Result<Vec<Self>, GeneratorError>
    {
        Ok( opts.type_systems.iter().map( |ts_opts| {
            match itf.variants.iter().find(|v| v.as_ref().ts == ts_opts.ts) {
                Some(v) => Some( IdlInterface::try_from(&itf, v.as_ref(), ts_opts) ),
                None => None
            }
        } ).filter_map(|i| i).collect::<Result<Vec<_>, _>>()? )
    }

    fn try_from(
        itf: &Interface,
        itf_variant: &InterfaceVariant,
        ts_opts: &TypeSystemOptions,
    ) -> Result<Self, GeneratorError>
    {
        Ok( Self {
            name: Self::final_name( &itf.name, ts_opts ),
            iid: format!( "{:-X}", itf_variant.iid ),
            base: Some("IUnknown".to_string()),
            methods: itf_variant.methods
                .iter()
                .enumerate()
                .map(|(i, m)| IdlMethod::try_from(i, m.as_ref()))
                .collect::<Result<Vec<_>, _>>()?
        } )
    }

    fn final_name(base_name: &str, opts: &TypeSystemOptions) -> String {
        match opts.use_full_name {
            true => format!( "{}_{:?}", base_name, opts.ts ),
            false => base_name.to_string(),
        }
    }
}

impl IdlMethod {
    fn try_from(
        idx: usize,
        method: &Method
    ) -> Result<Self, GeneratorError>
    {
        Ok( Self {
            name: pascal_case( &method.name ),
            idx,
            ret_type: IdlArg::idl_type(&method.return_type),
            args: method.parameters
                .iter()
                .map(IdlArg::try_from)
                .collect::<Result<Vec<_>, _>>()?
        } )
    }
}

impl IdlArg {
    fn try_from(
        arg: &Arg
    ) -> Result<Self, GeneratorError>
    {
        let mut attrs = vec![];
        match arg.direction {
            Direction::In => attrs.push( "in" ),
            Direction::Out => attrs.push( "out" ),
            Direction::Retval => {
                attrs.push( "out" );
                attrs.push( "retval" );
            }
        }

        Ok( Self {
            name: arg.name.to_string(),
            arg_type: Self::idl_type(arg),
            attributes: attrs.join(", "),
        } )
    }

    fn idl_type(arg: &Arg) -> String {
        format!( "{}{}", arg.ty, "*".repeat(arg.indirection_level as usize) )
    }
}

impl IdlClass {
    fn try_from(
        cls: &CoClass,
        opts: &ModelOptions
    ) -> Result<Self, GeneratorError>
    {
        Ok(IdlClass {
            name: cls.name.to_string(),
            clsid: format!("{:-X}", cls.clsid),
            interfaces: cls.interfaces.iter().map(|itf_ref| itf_ref.name.to_string()).collect(),
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
