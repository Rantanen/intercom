use super::macros::*;
use super::*;
use crate::prelude::*;

use crate::ast_converters::*;
use crate::guid::GUID;
use crate::idents::{self, SomeIdent};
use crate::methodinfo::ComMethodInfo;
use crate::quote::ToTokens;
use crate::tyhandlers::ModelTypeSystem;
use indexmap::IndexMap;
use proc_macro2::Span;
use std::iter::FromIterator;
use syn::{
    Attribute, Ident, ImplItem, Item, ItemImpl, ItemTrait, LitStr, Path, Signature, TraitItem,
    Type, TypePath, Visibility,
};

intercom_attribute!(
    ComInterfaceAttr< ComInterfaceAttrParam, NoParams > {
        com_iid : LitStr,
        raw_iid : LitStr,
        base : Path,
        vtable_of: Path,
        implemented_by: Path,
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
    pub base_interface: Option<Path>,
    pub variants: IndexMap<ModelTypeSystem, ComInterfaceVariant>,
    pub item_type: InterfaceType,
    pub span: Span,
    pub is_unsafe: bool,
    pub itf_ref: TokenStream,
    pub vtable_of: Option<Path>,
    pub implemented_by: Option<Path>,
}

#[derive(Debug, PartialEq)]
pub struct ComInterfaceVariant
{
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
        let (path, fns, itf_type, unsafety) = get_ident_and_fns(&item).ok_or_else(|| {
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
                if b.get_ident().map(|i| i == "NO_BASE") == Some(true) {
                    None
                } else {
                    Some(b.to_owned())
                }
            }
            None => Some(syn::parse2(quote!(intercom::IUnknown)).unwrap()),
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
                        .map(|data| ComMethodInfo::new(data.sig, data.attrs, ts))
                        .filter_map(Result::ok)
                        .collect::<Vec<_>>();

                    Ok((
                        ts,
                        ComInterfaceVariant {
                            type_system: ts,
                            iid,
                            methods,
                        },
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?,
        );

        let itf_ref = match itf_type {
            InterfaceType::Trait => quote_spanned!(ident.span() => dyn #path),
            InterfaceType::Struct => quote_spanned!(ident.span() => #path),
        };

        Ok(ComInterface {
            base_interface: base,
            item_type: itf_type,
            is_unsafe: unsafety.is_some(),
            span: ident.span(),
            vtable_of: attr
                .vtable_of()
                .map_err(|e| ParseError::ComInterface(ident.to_string(), e))?
                .cloned(),
            implemented_by: attr
                .implemented_by()
                .map_err(|e| ParseError::ComInterface(ident.to_string(), e))?
                .cloned(),
            path,
            ident,
            visibility,
            variants,
            itf_ref,
        })
    }

    pub fn vtable(&self, ts: ModelTypeSystem) -> TypePath
    {
        let ts_type_tokens = ts.as_typesystem_type(self.ident.span());
        let tt = match &self.vtable_of {
            Some(path) => {
                quote_spanned!(self.ident.span() => <dyn #path as intercom::attributes::ComInterfaceVariant<#ts_type_tokens>>::VTable)
            }
            None => {
                let ident = idents::vtable(&self.ident, ts);
                quote!(#ident)
            }
        };
        syn::parse2(tt).unwrap()
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum InterfaceType
{
    Trait,
    Struct,
}

pub struct MethodData<'a>
{
    attrs: &'a Vec<Attribute>,
    sig: &'a Signature,
}

pub type InterfaceData<'a> = (
    Path,
    Vec<MethodData<'a>>,
    InterfaceType,
    Option<Token!(unsafe)>,
);

pub type ImplData<'a> = (Option<Path>, Path, Vec<MethodData<'a>>);

fn get_ident_and_fns(item: &Item) -> Option<InterfaceData>
{
    match *item {
        Item::Impl(ItemImpl {
            ref unsafety,
            ref trait_,
            ref self_ty,
            ref items,
            ..
        }) => {
            let (_, struct_ident, items) = get_impl_data_raw(trait_, self_ty, items);
            Some((struct_ident, items, InterfaceType::Struct, *unsafety))
        }
        Item::Trait(ItemTrait {
            ref ident,
            unsafety,
            ref items,
            ..
        }) => {
            let methods: Option<Vec<MethodData>> =
                items.iter().map(|i| get_trait_method(i)).collect();
            let path = syn::Path::from(ident.clone());

            match methods {
                Some(m) => Some((path, m, InterfaceType::Trait, unsafety)),
                None => None,
            }
        }
        _ => None,
    }
}

fn get_impl_data_raw<'a>(
    trait_ref: &'a Option<(Option<Token!(!)>, Path, Token!(for))>,
    struct_ty: &'a Type,
    items: &'a [ImplItem],
) -> ImplData<'a>
{
    let struct_path = match struct_ty {
        syn::Type::Path(typepath) => {
            if typepath.qself.is_some() {
                panic!("#[com_interface] cannot use associated types");
            }
            typepath.path.clone()
        }
        _ => panic!("#[com_interface] must be defined for Path"),
    };

    let trait_path = trait_ref.as_ref().map(|(_, path, _)| path.clone());

    let methods_opt: Option<Vec<MethodData>> = items.iter().map(|i| get_impl_method(i)).collect();
    let methods = methods_opt.unwrap_or_else(Vec::new);

    (trait_path, struct_path, methods)
}

fn get_impl_method(i: &ImplItem) -> Option<MethodData>
{
    match *i {
        ImplItem::Method(ref item) => Some(MethodData {
            sig: &item.sig,
            attrs: &item.attrs,
        }),
        _ => None,
    }
}

fn get_trait_method(i: &TraitItem) -> Option<MethodData>
{
    match *i {
        TraitItem::Method(ref item) => Some(MethodData {
            sig: &item.sig,
            attrs: &item.attrs,
        }),
        _ => None,
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
        assert_eq!(
            itf.base_interface.as_ref().unwrap(),
            &parse_quote!(intercom::IUnknown)
        );

        let variant = &itf.variants[&Automation];
        assert_eq!(
            variant.iid,
            GUID::parse("12345678-1234-1234-1234-567890ABCDEF").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].name, "foo");
        assert_eq!(variant.methods[1].name, "bar");

        let variant = &itf.variants[&Raw];
        assert_eq!(
            variant.iid,
            GUID::parse("12345678-1234-1234-1234-567890FEDCBA").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].name, "foo");
        assert_eq!(variant.methods[1].name, "bar");
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
        assert_eq!(
            itf.base_interface.as_ref().unwrap(),
            &parse_quote!(intercom::IUnknown)
        );

        let variant = &itf.variants[&Automation];
        assert_eq!(
            variant.iid,
            GUID::parse("82B905D9-D292-3531-452F-E04722F567DD").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].name, "one");
        assert_eq!(variant.methods[1].name, "two");

        let variant = &itf.variants[&Raw];
        assert_eq!(
            variant.iid,
            GUID::parse("E16EEA74-C0E0-34DE-6F51-1D949883DE06").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].name, "one");
        assert_eq!(variant.methods[1].name, "two");
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
        assert_eq!(itf.base_interface.as_ref().unwrap(), &parse_quote!(IBase));

        let variant = &itf.variants[&ModelTypeSystem::Automation];
        assert_eq!(
            variant.iid,
            GUID::parse("82B905D9-D292-3531-452F-E04722F567DD").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].name, "one");
        assert_eq!(variant.methods[1].name, "two");

        let variant = &itf.variants[&ModelTypeSystem::Raw];
        assert_eq!(
            variant.iid,
            GUID::parse("E16EEA74-C0E0-34DE-6F51-1D949883DE06").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].name, "one");
        assert_eq!(variant.methods[1].name, "two");
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
        assert_eq!(variant.methods[0].name, "one");
        assert_eq!(variant.methods[1].name, "two");

        let variant = &itf.variants[&Raw];
        assert_eq!(
            variant.iid,
            GUID::parse("E16EEA74-C0E0-34DE-6F51-1D949883DE06").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].name, "one");
        assert_eq!(variant.methods[1].name, "two");
    }
}
