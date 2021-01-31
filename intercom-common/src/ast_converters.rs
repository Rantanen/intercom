use crate::prelude::*;
use syn::{Attribute, FnArg, GenericArgument, Ident, Item, Pat, Path, Type};

/// Extract the underlying Type from various AST types.
pub trait GetType
{
    /// Gets the Type from the AST element.
    fn get_ty(&self) -> Result<Type, String>;
}

impl GetType for FnArg
{
    fn get_ty(&self) -> Result<Type, String>
    {
        Ok(match *self {
            FnArg::Receiver(_) => self_ty(),
            FnArg::Typed(ref pat_type) => pat_type.ty.as_ref().to_owned(),
        })
    }
}

impl GetType for GenericArgument
{
    fn get_ty(&self) -> Result<Type, String>
    {
        match *self {
            GenericArgument::Type(ref ty) => Ok(ty.clone()),
            _ => Err("Expected type parameter".to_string()),
        }
    }
}

pub trait GetIdent
{
    /// Gets the Ident from the AST element.
    fn get_ident(&self) -> Result<Ident, String>;
}

impl GetIdent for FnArg
{
    fn get_ident(&self) -> Result<Ident, String>
    {
        Ok(match *self {
            FnArg::Receiver(_) => Ident::new("self", Span::call_site()),
            FnArg::Typed(ref pat_type) => match *pat_type.pat {
                Pat::Ident(ref pat_ident) => pat_ident.ident.clone(),
                _ => return Err(format!("Unsupported argument: {:?}", self)),
            },
        })
    }
}

impl GetIdent for Path
{
    fn get_ident(&self) -> Result<Ident, String>
    {
        self.segments
            .last()
            .map(|l| l.ident.clone())
            .ok_or_else(|| "Empty path".to_owned())
    }
}

impl GetIdent for Type
{
    fn get_ident(&self) -> Result<Ident, String>
    {
        match *self {
            Type::Path(ref p) => p
                .path
                .get_ident()
                .cloned()
                .ok_or_else(|| format!("No Ident for {:?}", self)),
            _ => Err(format!("Cannot get Ident for {:?}", self)),
        }
    }
}

impl GetIdent for Item
{
    fn get_ident(&self) -> Result<Ident, String>
    {
        Ok(match *self {
            Item::ExternCrate(ref i) => i.ident.clone(),
            Item::Static(ref i) => i.ident.clone(),
            Item::Const(ref i) => i.ident.clone(),
            Item::Fn(ref i) => i.sig.ident.clone(),
            Item::Mod(ref i) => i.ident.clone(),
            Item::Type(ref i) => i.ident.clone(),
            Item::Struct(ref i) => i.ident.clone(),
            Item::Enum(ref i) => i.ident.clone(),
            Item::Union(ref i) => i.ident.clone(),
            Item::Trait(ref i) => i.ident.clone(),
            Item::Impl(ref i) => i.self_ty.get_ident()?,
            Item::Macro(ref m) => m
                .mac
                .path
                .get_ident()
                .cloned()
                .ok_or_else(|| format!("No ident on {:?}", self))?,
            Item::Macro2(ref i) => i.ident.clone(),
            Item::TraitAlias(ref i) => i.ident.clone(),

            Item::Use(..) | Item::ForeignMod(..) | Item::Verbatim(..) => {
                return Err("Item type not supported for Ident".to_string())
            }
            _ => panic!(),
        })
    }
}

pub trait GetAttributes
{
    /// Gets the Attributes from the AST element.
    fn get_attributes(&self) -> Result<Vec<Attribute>, String>;
}

impl GetAttributes for Item
{
    fn get_attributes(&self) -> Result<Vec<Attribute>, String>
    {
        Ok(match *self {
            Item::ExternCrate(ref i) => i.attrs.clone(),
            Item::Static(ref i) => i.attrs.clone(),
            Item::Const(ref i) => i.attrs.clone(),
            Item::Fn(ref i) => i.attrs.clone(),
            Item::Mod(ref i) => i.attrs.clone(),
            Item::Type(ref i) => i.attrs.clone(),
            Item::Struct(ref i) => i.attrs.clone(),
            Item::Enum(ref i) => i.attrs.clone(),
            Item::Union(ref i) => i.attrs.clone(),
            Item::Trait(ref i) => i.attrs.clone(),
            Item::Impl(ref i) => i.attrs.clone(),
            Item::Macro(ref i) => i.attrs.clone(),
            Item::Macro2(ref i) => i.attrs.clone(),
            Item::Use(ref i) => i.attrs.clone(),
            Item::ForeignMod(ref i) => i.attrs.clone(),
            Item::TraitAlias(ref i) => i.attrs.clone(),
            Item::Verbatim(..) => vec![],
            _ => panic!(),
        })
    }
}

pub trait ReplaceIdent: Sized
{
    fn map_ident(&self, f: impl FnOnce(&Ident) -> String) -> Result<Self, String>;
}

impl ReplaceIdent for syn::Path
{
    fn map_ident(&self, f: impl FnOnce(&Ident) -> String) -> Result<Self, String>
    {
        let mut result = self.clone();
        let mut last = result
            .segments
            .last_mut()
            .ok_or_else(|| format!("Path {:?} is empty", self))?;
        last.ident = Ident::new(&f(&last.ident), Span::call_site());
        Ok(result)
    }
}

fn self_ty() -> Type
{
    parse_quote!(Self)
}
