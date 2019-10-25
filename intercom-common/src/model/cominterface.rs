use super::macros::*;
use super::*;
use crate::prelude::*;

use crate::ast_converters::*;
use crate::guid::GUID;
use crate::methodinfo::ComMethodInfo;
use crate::tyhandlers::ModelTypeSystem;
use ::indexmap::IndexMap;
use ::std::iter::FromIterator;
use ::syn::{spanned::Spanned, Ident, LitStr, Visibility};
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
    display_name: Ident,
    visibility: Visibility,
    base_interface: Option<Ident>,
    variants: IndexMap<ModelTypeSystem, ComInterfaceVariant>,
    item_type: crate::utils::InterfaceType,
    pub span: Span,
    is_unsafe: bool,
}

#[derive(Debug, PartialEq)]
pub struct ComInterfaceVariant
{
    display_name: Ident,
    unique_name: Ident,
    unique_base_interface: Option<Ident>,
    type_system: ModelTypeSystem,
    iid: GUID,
    methods: Vec<ComMethodInfo>,
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
        let (itf_ident, fns, itf_type, unsafety) = crate::utils::get_ident_and_fns(&item)
            .ok_or_else(|| {
                ParseError::ComInterface(
                    item.get_ident().unwrap().to_string(),
                    "Unsupported associated item".into(),
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
                    let itf_unique_ident = Ident::new(
                        &format!("{}_{:?}", itf_ident.to_string(), ts),
                        Span::call_site(),
                    );

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
                        None => crate::utils::generate_iid(
                            crate_name,
                            &itf_unique_ident.to_string(),
                            ts,
                        ),
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
                            display_name: itf_ident.clone(),
                            unique_name: itf_unique_ident,
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
            display_name: itf_ident,
            base_interface: base,
            item_type: itf_type,
            is_unsafe: unsafety.is_some(),
            span: item.span(),
            visibility,
            variants,
        })
    }

    /// Temp accessor for the automation variant.
    pub fn aut(&self) -> &ComInterfaceVariant
    {
        &self.variants[&ModelTypeSystem::Automation]
    }

    /// Interface name.
    pub fn name(&self) -> &Ident
    {
        &self.display_name
    }

    /// Interface visibility.
    pub fn visibility(&self) -> &Visibility
    {
        &self.visibility
    }

    /// The base interface.
    pub fn base_interface(&self) -> &Option<Ident>
    {
        &self.base_interface
    }

    /// Interface variants.
    pub fn variants(&self) -> &IndexMap<ModelTypeSystem, ComInterfaceVariant>
    {
        &self.variants
    }

    /// The type of the associated item for the #[com_interface] attribute.
    ///
    /// Either an impl or a trait.
    pub fn item_type(&self) -> crate::utils::InterfaceType
    {
        self.item_type
    }

    /// True, if the interface requires unsafe impl.
    pub fn is_unsafe(&self) -> bool
    {
        self.is_unsafe
    }
}

impl ComInterfaceVariant
{
    /// Interface unique name.
    pub fn unique_name(&self) -> &Ident
    {
        &self.unique_name
    }

    /// Interface base interface variant unique name.
    pub fn unique_base_interface(&self) -> &Option<Ident>
    {
        &self.unique_base_interface
    }

    /// Implemented methods.
    pub fn methods(&self) -> &Vec<ComMethodInfo>
    {
        &self.methods
    }

    /// Interface IID.
    pub fn iid(&self) -> &GUID
    {
        &self.iid
    }

    /// Gets the type system this interface variant represents.
    pub fn type_system(&self) -> ModelTypeSystem
    {
        self.type_system
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

        assert_eq!(itf.name(), "ITrait");
        assert_eq!(itf.visibility(), &Visibility::Inherited);
        assert_eq!(itf.base_interface().as_ref().unwrap(), "IUnknown");

        let variant = &itf.variants[&Automation];
        assert_eq!(
            variant.iid(),
            &GUID::parse("12345678-1234-1234-1234-567890ABCDEF").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].display_name, "foo");
        assert_eq!(variant.methods[1].display_name, "bar");

        let variant = &itf.variants[&Raw];
        assert_eq!(
            variant.iid(),
            &GUID::parse("12345678-1234-1234-1234-567890FEDCBA").unwrap()
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

        assert_eq!(itf.name(), "IAutoGuid");

        let pub_visibility: Visibility = parse_quote!(pub);
        assert_eq!(itf.visibility(), &pub_visibility);
        assert_eq!(itf.base_interface().as_ref().unwrap(), "IUnknown");

        let variant = &itf.variants[&Automation];
        assert_eq!(
            variant.iid(),
            &GUID::parse("3DC87B73-0998-30B6-75EA-D4F564454D4B").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].display_name, "one");
        assert_eq!(variant.methods[1].display_name, "two");

        let variant = &itf.variants[&Raw];
        assert_eq!(
            variant.iid(),
            &GUID::parse("D552E455-9FB2-34A2-61C0-34BDE0A9095D").unwrap()
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

        assert_eq!(itf.name(), "IAutoGuid");

        let pub_visibility: Visibility = parse_quote!(pub);
        assert_eq!(itf.visibility(), &pub_visibility);
        assert_eq!(itf.base_interface().as_ref().unwrap(), "IBase");

        let variant = &itf.variants[&ModelTypeSystem::Automation];
        assert_eq!(
            variant.iid(),
            &GUID::parse("3DC87B73-0998-30B6-75EA-D4F564454D4B").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].display_name, "one");
        assert_eq!(variant.methods[0].unique_name, "one_Automation");
        assert_eq!(variant.methods[1].display_name, "two");
        assert_eq!(variant.methods[1].unique_name, "two_Automation");

        let variant = &itf.variants[&ModelTypeSystem::Raw];
        assert_eq!(
            variant.iid(),
            &GUID::parse("D552E455-9FB2-34A2-61C0-34BDE0A9095D").unwrap()
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

        assert_eq!(itf.name(), "IAutoGuid");

        let pub_visibility: Visibility = parse_quote!(pub);
        assert_eq!(itf.visibility(), &pub_visibility);
        assert_eq!(itf.base_interface(), &None);

        let variant = &itf.variants[&Automation];
        assert_eq!(
            variant.iid(),
            &GUID::parse("3DC87B73-0998-30B6-75EA-D4F564454D4B").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].unique_name, "one_Automation");
        assert_eq!(variant.methods[1].unique_name, "two_Automation");

        let variant = &itf.variants[&Raw];
        assert_eq!(
            variant.iid(),
            &GUID::parse("D552E455-9FB2-34A2-61C0-34BDE0A9095D").unwrap()
        );
        assert_eq!(variant.methods.len(), 2);
        assert_eq!(variant.methods[0].unique_name, "one_Raw");
        assert_eq!(variant.methods[1].unique_name, "two_Raw");
    }
}
