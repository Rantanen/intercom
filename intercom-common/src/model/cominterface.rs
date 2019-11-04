use super::macros::*;
use super::*;
use crate::prelude::*;

use crate::ast_converters::*;
use crate::guid::GUID;
use crate::idents::SomeIdent;
use crate::methodinfo::ComMethodInfo;
use crate::quote::ToTokens;
use crate::tyhandlers::ModelTypeSystem;
use ::indexmap::IndexMap;
use ::std::iter::FromIterator;
use ::syn::{spanned::Spanned, Ident, LitStr, Path, Visibility};
use proc_macro2::Span;

intercom_attribute!(
    ComInterfaceAttr< ComInterfaceAttrParam, NoParams > {
        com_iid : LitStr,
        raw_iid : LitStr,
        base : Ident,
    }
);

impl ComInterfaceAttr
{
    pub fn iid(&self, ts: ModelTypeSystem) -> Result<Option<&LitStr>, String>
    {
        match ts {
            ModelTypeSystem::Raw => self.raw_iid(),
            ModelTypeSystem::Automation => self.com_iid(),
        }
    }
}

#[derive(Debug)]
pub struct ComInterface
{
    pub path: Path,
    pub ident: Ident,
    pub visibility: Visibility,
    pub base_interface: Option<Ident>,
    pub variants: IndexMap<ModelTypeSystem, ComInterfaceVariant>,
    pub item_type: crate::utils::InterfaceType,
    pub span: Span,
    pub is_unsafe: bool,
}

#[derive(Debug, PartialEq)]
pub struct ComInterfaceVariant
{
    pub unique_base_interface: Option<Ident>,
    pub type_system: ModelTypeSystem,
    pub iid: GUID,
    pub methods: Vec<ComMethodInfo>,
}

impl ComInterface
{
    /// Creates ComInterface from AST elements.
    pub fn from_ast(
        crate_name: &str,
        attr: TokenStream,
        item: TokenStream,
    ) -> ParseResult<ComInterface>
    {
        let item: syn::Item = ::syn::parse2(item).map_err(|_| {
            ParseError::ComInterface("<Unknown>".into(), "Item syntax error".into())
        })?;

        let attr: ComInterfaceAttr = ::syn::parse2(attr).map_err(|_| {
            ParseError::ComInterface(
                item.get_ident().unwrap().to_string(),
                "Attribute syntax error".into(),
            )
        })?;

        // Get the interface details. As [com_interface] can be applied to both
        // impls and traits this handles both of those.
        let (path, fns, itf_type, unsafety) =
            crate::utils::get_ident_and_fns(&item).ok_or_else(|| {
                ParseError::ComInterface(
                    item.get_ident().unwrap().to_string(),
                    "Unsupported associated item".into(),
                )
            })?;

        let ident = path.get_some_ident().ok_or_else(|| {
            ParseError::ComInterface(
                path.to_token_stream().to_string(),
                "Could not resolve ident for".to_string(),
            )
        })?;

        // The second argument is the optional base class. If there's no base
        // class defined, use IUnknown as the default. The value of NO_BASE will
        // construct an interface that has no base class.
        //
        // In practice the NO_BASE should be used ONLY for the IUnknown itself.
        let base = attr
            .base()
            .map_err(|msg| ParseError::ComInterface(item.get_ident().unwrap().to_string(), msg))?;
        let base = match base {
            Some(b) => {
                if b == "NO_BASE" {
                    None
                } else {
                    Some(b.to_owned())
                }
            }
            None => Some(Ident::new("IUnknown", Span::call_site())),
        };

        // Visibility for trait interfaces is the visibility of the trait.
        //
        // For implicit interfaces (impl Struct) the visibility is always public.
        // These interfaces should only exist for COM types that are meant to be
        // called from external sources as they can't be impl'd for random ComItf.
        //
        // Note this may conflict with visibility of the actual [com_class], but
        // nothing we can do for this really.
        let visibility = if let ::syn::Item::Trait(ref t) = item {
            t.vis.clone()
        } else {
            parse_quote!(pub)
        };

        let variants = IndexMap::from_iter(
            [ModelTypeSystem::Automation, ModelTypeSystem::Raw]
                .iter()
                .map(|&ts| {
                    // IUnknown interfaces do not have type system variants.
                    let unique_base = match base {
                        Some(ref iunk) if iunk == "IUnknown" => base.clone(),
                        ref b => b
                            .as_ref()
                            .map(|b| Ident::new(&format!("{}_{:?}", b, ts), Span::call_site())),
                    };

                    let iid_attr = attr.iid(ts).map_err(|msg| {
                        ParseError::ComInterface(item.get_ident().unwrap().to_string(), msg)
                    })?;
                    let iid = match iid_attr {
                        Some(iid) => GUID::parse(&iid.value()).map_err(|_| {
                            ParseError::ComInterface(
                                item.get_ident().unwrap().to_string(),
                                "Bad IID format".into(),
                            )
                        })?,
                        None => crate::utils::generate_iid(crate_name, &ident.to_string(), ts),
                    };

                    // Read the method details.
                    //
                    // TODO: Currently we ignore invalid methods. We should probably do
                    //       something smarter.
                    let methods = fns
                        .iter()
                        .map(|sig| ComMethodInfo::new(sig, ts))
                        .filter_map(Result::ok)
                        .collect::<Vec<_>>();

                    Ok((
                        ts,
                        ComInterfaceVariant {
                            unique_base_interface: unique_base,
                            type_system: ts,
                            iid,
                            methods,
                        },
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?,
        );

        Ok(ComInterface {
            base_interface: base,
            item_type: itf_type,
            is_unsafe: unsafety.is_some(),
            span: item.span(),
            path,
            ident,
            visibility,
            variants,
        })
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use crate::tyhandlers::ModelTypeSystem::*;

    #[test]
    fn parse_com_interface()
    {
        let itf = ComInterface::from_ast(
            "not used",
            quote!(
                com_iid = "12345678-1234-1234-1234-567890ABCDEF",
                raw_iid = "12345678-1234-1234-1234-567890FEDCBA",
            ),
            quote!(
                trait ITrait
                {
                    fn foo(&self);
                    fn bar(&self);
                }
            ),
        )
        .expect("com_interface attribute parsing failed");

        assert_eq!(itf.path, parse_quote!(ITrait));
        assert_eq!(itf.visibility, Visibility::Inherited);
        assert_eq!(itf.base_interface.as_ref().unwrap(), "IUnknown");

        let variant = &itf.variants[&Automation];
        assert_eq!(
            variant.iid,
            GUID::parse("12345678-1234-1234-1234-567890ABCDEF").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].display_name, "foo");
        assert_eq!(variant.methods[1].display_name, "bar");

        let variant = &itf.variants[&Raw];
        assert_eq!(
            variant.iid,
            GUID::parse("12345678-1234-1234-1234-567890FEDCBA").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].display_name, "foo");
        assert_eq!(variant.methods[1].display_name, "bar");
    }

    #[test]
    fn parse_com_interface_with_auto_guid()
    {
        let itf = ComInterface::from_ast(
            "not used",
            quote!(),
            quote!(
                pub trait IAutoGuid
                {
                    fn one(&self);
                    fn two(&self);
                }
            ),
        )
        .expect("com_interface attribute parsing failed");

        assert_eq!(itf.path, parse_quote!(IAutoGuid));

        let pub_visibility: Visibility = parse_quote!(pub);
        assert_eq!(itf.visibility, pub_visibility);
        assert_eq!(itf.base_interface.as_ref().unwrap(), "IUnknown");

        let variant = &itf.variants[&Automation];
        assert_eq!(
            variant.iid,
            GUID::parse("82B905D9-D292-3531-452F-E04722F567DD").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].display_name, "one");
        assert_eq!(variant.methods[1].display_name, "two");

        let variant = &itf.variants[&Raw];
        assert_eq!(
            variant.iid,
            GUID::parse("E16EEA74-C0E0-34DE-6F51-1D949883DE06").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].display_name, "one");
        assert_eq!(variant.methods[1].display_name, "two");
    }

    #[test]
    fn parse_com_interface_with_base_interface()
    {
        let itf = ComInterface::from_ast(
            "not used",
            quote!(base = IBase),
            quote!(
                pub trait IAutoGuid
                {
                    fn one(&self);
                    fn two(&self);
                }
            ),
        )
        .expect("com_interface attribute parsing failed");

        assert_eq!(itf.path, parse_quote!(IAutoGuid));

        let pub_visibility: Visibility = parse_quote!(pub);
        assert_eq!(itf.visibility, pub_visibility);
        assert_eq!(itf.base_interface.as_ref().unwrap(), "IBase");

        let variant = &itf.variants[&ModelTypeSystem::Automation];
        assert_eq!(
            variant.iid,
            GUID::parse("82B905D9-D292-3531-452F-E04722F567DD").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].display_name, "one");
        assert_eq!(variant.methods[0].unique_name, "one_Automation");
        assert_eq!(variant.methods[1].display_name, "two");
        assert_eq!(variant.methods[1].unique_name, "two_Automation");

        let variant = &itf.variants[&ModelTypeSystem::Raw];
        assert_eq!(
            variant.iid,
            GUID::parse("E16EEA74-C0E0-34DE-6F51-1D949883DE06").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].display_name, "one");
        assert_eq!(variant.methods[0].unique_name, "one_Raw");
        assert_eq!(variant.methods[1].display_name, "two");
        assert_eq!(variant.methods[1].unique_name, "two_Raw");
    }

    #[test]
    fn parse_com_interface_with_no_base_interface()
    {
        let itf = ComInterface::from_ast(
            "not used",
            quote!(base = NO_BASE),
            quote!(
                pub trait IAutoGuid
                {
                    fn one(&self);
                    fn two(&self);
                }
            ),
        )
        .expect("com_interface attribute parsing failed");

        assert_eq!(itf.path, parse_quote!(IAutoGuid));

        let pub_visibility: Visibility = parse_quote!(pub);
        assert_eq!(itf.visibility, pub_visibility);
        assert_eq!(itf.base_interface, None);

        let variant = &itf.variants[&Automation];
        assert_eq!(
            variant.iid,
            GUID::parse("82B905D9-D292-3531-452F-E04722F567DD").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].unique_name, "one_Automation");
        assert_eq!(variant.methods[1].unique_name, "two_Automation");

        let variant = &itf.variants[&Raw];
        assert_eq!(
            variant.iid,
            GUID::parse("E16EEA74-C0E0-34DE-6F51-1D949883DE06").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].unique_name, "one_Raw");
        assert_eq!(variant.methods[1].unique_name, "two_Raw");
    }
}
