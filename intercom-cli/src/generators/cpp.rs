//! Enables the generation of header and source files for using intercom
//! libraries from C++ projects.

extern crate std;

use std::borrow::Cow;
use std::io::Write;

use super::GeneratorError;
use super::{pascal_case, LibraryContext, ModelOptions, TypeSystemOptions};

use intercom::typelib::{
    Arg, CoClass, Direction, Interface, InterfaceVariant, Method, TypeInfo, TypeLib,
};

use handlebars::Handlebars;
use serde_derive::Serialize;

#[derive(PartialEq, Serialize, Debug)]
pub struct CppLibrary
{
    pub lib_name: String,
    pub interfaces: Vec<CppInterface>,
    pub coclass_count: usize,
    pub coclasses: Vec<CppClass>,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct CppInterface
{
    pub name: String,
    pub iid_struct: String,
    pub base: Option<String>,
    pub methods: Vec<CppMethod>,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct CppMethod
{
    pub name: String,
    pub ret_type: String,
    pub args: Vec<CppArg>,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct CppArg
{
    pub name: String,
    pub arg_type: String,
}

#[derive(PartialEq, Serialize, Debug)]
pub struct CppClass
{
    pub name: String,
    pub clsid_struct: String,
    pub interface_count: usize,
    pub interfaces: Vec<String>,
}

impl CppLibrary
{
    fn try_from(lib: TypeLib, opts: &ModelOptions) -> Result<Self, GeneratorError>
    {
        let ctx = LibraryContext::try_from(&lib)?;

        let mut interfaces = vec![];
        let mut coclasses = vec![];
        for t in &lib.types {
            match t {
                TypeInfo::Class(cls) => {
                    coclasses.push(CppClass::try_from(cls.as_ref(), opts, &ctx)?)
                }
                TypeInfo::Interface(itf) => {
                    interfaces.push(CppInterface::gather(itf.as_ref(), opts, &ctx)?)
                }
            }
        }
        let interfaces = interfaces
            .into_iter()
            .flatten()
            .collect::<Vec<CppInterface>>();

        Ok(Self {
            lib_name: lib.name.to_string(),
            interfaces,
            coclass_count: coclasses.len(),
            coclasses,
        })
    }
}

impl CppInterface
{
    fn gather(
        itf: &Interface,
        opts: &ModelOptions,
        ctx: &LibraryContext,
    ) -> Result<Vec<Self>, GeneratorError>
    {
        Ok(opts
            .type_systems
            .iter()
            .map(
                |ts_opts| match itf.variants.iter().find(|v| v.as_ref().ts == ts_opts.ts) {
                    Some(v) => Some(CppInterface::try_from(&itf, v.as_ref(), ts_opts, ctx)),
                    None => None,
                },
            )
            .filter_map(|i| i)
            .collect::<Result<Vec<_>, _>>()?)
    }

    fn try_from(
        itf: &Interface,
        itf_variant: &InterfaceVariant,
        ts_opts: &TypeSystemOptions,
        ctx: &LibraryContext,
    ) -> Result<Self, GeneratorError>
    {
        Ok(Self {
            name: Self::final_name(&itf, ts_opts),
            iid_struct: guid_as_struct(&itf_variant.iid),
            base: Some("IUnknown".to_string()),
            methods: itf_variant
                .methods
                .iter()
                .map(|m| CppMethod::try_from(m.as_ref(), ts_opts, ctx))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn final_name(itf: &Interface, opts: &TypeSystemOptions) -> String
    {
        let base_name = if itf.options.class_impl_interface {
            Cow::from(format!("I{}", itf.name))
        } else {
            itf.name.clone()
        };

        match opts.use_full_name {
            true => format!("{}_{:?}", base_name, opts.ts),
            false => base_name.to_string(),
        }
    }
}

impl CppMethod
{
    fn try_from(
        method: &Method,
        opts: &TypeSystemOptions,
        ctx: &LibraryContext,
    ) -> Result<Self, GeneratorError>
    {
        Ok(Self {
            name: pascal_case(&method.name),
            ret_type: CppArg::cpp_type(&method.return_type, opts, ctx),
            args: method
                .parameters
                .iter()
                .map(|arg| CppArg::try_from(arg, opts, ctx))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl CppArg
{
    fn try_from(
        arg: &Arg,
        opts: &TypeSystemOptions,
        ctx: &LibraryContext,
    ) -> Result<Self, GeneratorError>
    {
        let mut attrs = vec![];
        match arg.direction {
            Direction::In => attrs.push("in"),
            Direction::Out => attrs.push("out"),
            Direction::Retval => {
                attrs.push("out");
                attrs.push("retval");
            }
            Direction::Return => {
                return Err("Direction::Return is invalid direction for arguments"
                    .to_string()
                    .into());
            }
        }

        Ok(Self {
            name: arg.name.to_string(),
            arg_type: Self::cpp_type(arg, opts, ctx),
        })
    }

    fn cpp_type(arg: &Arg, opts: &TypeSystemOptions, ctx: &LibraryContext) -> String
    {
        let base_name = ctx
            .itfs_by_name
            .get(arg.ty.as_ref())
            .map(|itf| CppInterface::final_name(itf, opts))
            .unwrap_or_else(|| arg.ty.to_string());
        let indirection = match arg.direction {
            Direction::In | Direction::Return => arg.indirection_level,
            Direction::Out | Direction::Retval => arg.indirection_level + 1,
        };

        let base_name = match base_name.as_ref() {
            "std::ffi::c_void" => "void".to_string(),
            "HRESULT" => "intercom::HRESULT".to_string(),
            other => other.to_string(),
        };

        format!("{}{}", base_name, "*".repeat(indirection as usize))
    }
}

impl CppClass
{
    fn try_from(
        cls: &CoClass,
        opts: &ModelOptions,
        ctx: &LibraryContext,
    ) -> Result<Self, GeneratorError>
    {
        let interfaces = cls
            .interfaces
            .iter()
            .flat_map(|itf_ref| {
                opts.type_systems
                    .iter()
                    .map(|opt| {
                        let itf = ctx.itfs_by_ref[itf_ref.name.as_ref()];
                        CppInterface::final_name(itf, opt)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(CppClass {
            name: cls.name.to_string(),
            clsid_struct: guid_as_struct(&cls.clsid),
            interface_count: interfaces.len(),
            interfaces,
        })
    }
}

/// Generates the manifest content.
///
/// - `out` - The writer to use for output.
pub fn write(
    lib: intercom::typelib::TypeLib,
    opts: ModelOptions,
    out_header: Option<&mut dyn Write>,
    out_source: Option<&mut dyn Write>,
) -> Result<(), GeneratorError>
{
    let mut reg = Handlebars::new();
    reg.register_template_string("cpp_header", include_str!("cpp_header.hbs"))
        .expect("Error in the built-in C++ template.");
    reg.register_template_string("cpp_source", include_str!("cpp_source.hbs"))
        .expect("Error in the built-in C++ template.");

    let cpp_model = CppLibrary::try_from(lib, &opts)?;

    if let Some(out_header) = out_header {
        let rendered = reg
            .render("cpp_header", &cpp_model)
            .expect("Rendering a valid ComCrate to C++ failed");
        write!(out_header, "{}", rendered)?;
    }

    if let Some(out_source) = out_source {
        let rendered = reg
            .render("cpp_source", &cpp_model)
            .expect("Rendering a valid ComCrate to C++ failed");
        write!(out_source, "{}", rendered)?;
    }

    Ok(())
}

/// Converts a guid to binarys representation.
pub fn guid_as_struct(g: &intercom::GUID) -> String
{
    format!( "{{0x{:08x},0x{:04x},0x{:04x},{{0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x}}}}}",
            g.data1, g.data2, g.data3,
            g.data4[0], g.data4[1], g.data4[2], g.data4[3],
            g.data4[4], g.data4[5], g.data4[6], g.data4[7] )
}
