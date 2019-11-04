use super::*;
use crate::prelude::*;

use crate::ast_converters::*;
use crate::methodinfo::ComMethodInfo;
use crate::tyhandlers::ModelTypeSystem;
use ::indexmap::IndexMap;
use ::std::iter::FromIterator;

use crate::idents::SomeIdent;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::Path;

#[derive(Debug)]
pub struct ComImpl
{
    pub struct_path: Path,
    pub struct_ident: Ident,
    pub interface_path: Path,
    pub interface_ident: Ident,
    pub is_trait_impl: bool,
    pub variants: IndexMap<ModelTypeSystem, ComImplVariant>,
    pub impl_span: Span,
}

#[derive(Debug, PartialEq)]
pub struct ComImplVariant
{
    pub type_system: ModelTypeSystem,
    pub methods: Vec<ComMethodInfo>,
}

impl ComImpl
{
    /// Parses the associated item of the #[com_impl] attribute.
    pub fn parse(item: TokenStream) -> ParseResult<ComImpl>
    {
        // Get the item details from the associated item.
        let item: ::syn::Item = ::syn::parse2(item).map_err(|_| {
            ParseError::ComImpl(
                "<Unknown>".into(),
                "<Unknown>".into(),
                "Could not parse [com_impl]".into(),
            )
        })?;

        // Resolve the idents and functions.
        let (itf_path_opt, struct_path, fns) =
            crate::utils::get_impl_data(&item).ok_or_else(|| {
                ParseError::ComImpl(
                    item.get_ident().expect("Item had no ident").to_string(),
                    "<Unknown>".into(),
                    "Unsupported associated item".into(),
                )
            })?;
        let is_trait_impl = itf_path_opt.is_some();
        let interface_path = itf_path_opt.unwrap_or_else(|| struct_path.clone());

        let variants = IndexMap::from_iter(
            [ModelTypeSystem::Automation, ModelTypeSystem::Raw]
                .iter()
                .map(|&ts| {
                    // Turn the impl methods into MethodInfo.
                    //
                    // TODO: Currently we ignore invalid methods. We should probably do
                    //       something smarter.
                    let methods = fns
                        .iter()
                        .map(|sig| ComMethodInfo::new(sig, ts).map_err(|_| sig.ident.clone()))
                        .filter_map(Result::ok)
                        .collect::<Vec<_>>();

                    (
                        ts,
                        ComImplVariant {
                            type_system: ts,
                            methods,
                        },
                    )
                }),
        );

        let impl_span = match item {
            syn::Item::Impl(i) => i.impl_token.span().join(i.self_ty.span()),
            _ => None,
        }
        .unwrap_or_else(Span::call_site);
        Ok(ComImpl {
            struct_ident: struct_path.get_some_ident().expect("Type had no name"),
            interface_ident: interface_path.get_some_ident().expect("Trait had no name"),
            struct_path,
            interface_path,
            impl_span,
            variants,
            is_trait_impl,
        })
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use crate::tyhandlers::ModelTypeSystem::*;

    #[test]
    fn parse_com_impl_for_struct()
    {
        let itf = ComImpl::parse(quote!(impl Foo { fn foo( &self ) {} fn bar( &self ) {} }))
            .expect("com_impl attribute parsing failed");

        assert_eq!(itf.struct_path, parse_quote!(Foo));
        assert_eq!(itf.interface_path, parse_quote!(Foo));
        assert_eq!(itf.is_trait_impl, false);
        assert_eq!(itf.variants[&Automation].methods.len(), 2);
        assert_eq!(itf.variants[&Automation].methods[0].name, "foo");
        assert_eq!(itf.variants[&Automation].methods[1].name, "bar");
        assert_eq!(itf.variants[&Raw].methods.len(), 2);
        assert_eq!(itf.variants[&Raw].methods[0].name, "foo");
        assert_eq!(itf.variants[&Raw].methods[1].name, "bar");
    }

    #[test]
    fn parse_com_impl_for_trait()
    {
        let itf =
            ComImpl::parse(quote!(impl IFoo for Bar { fn one( &self ) {} fn two( &self ) {} }))
                .expect("com_impl attribute parsing failed");

        assert_eq!(itf.struct_path, parse_quote!(Bar));
        assert_eq!(itf.interface_path, parse_quote!(IFoo));
        assert_eq!(itf.is_trait_impl, true);
        assert_eq!(itf.variants[&Automation].methods.len(), 2);
        assert_eq!(itf.variants[&Automation].methods[0].name, "one");
        assert_eq!(itf.variants[&Automation].methods[1].name, "two");
        assert_eq!(itf.variants[&Raw].methods.len(), 2);
        assert_eq!(itf.variants[&Raw].methods[0].name, "one");
        assert_eq!(itf.variants[&Raw].methods[1].name, "two");
    }
}
