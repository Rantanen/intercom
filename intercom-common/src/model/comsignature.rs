use super::*;
use crate::prelude::*;

use syn::{Ident, LitInt, LitStr};

#[derive(Debug, Clone)]
pub enum SignatureItem
{
    Input(Ident),
    Output(u32),
}

impl syn::parse::Parse for SignatureItem
{
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self>
    {
        let ident: syn::Ident = input.parse()?;
        match ident {
            out if out == "OUT" => {
                let content;
                bracketed!(content in input);
                let value: LitInt = content.parse()?;
                Ok(SignatureItem::Output(value.base10_parse()?))
            }
            ident => Ok(SignatureItem::Input(ident)),
        }
    }
}

intercom_attribute!(ComSignatureAttr < ComSignatureAttrParam, SignatureItem > {
    com_iid : LitStr,
});

#[derive(Debug)]
pub struct ComSignature
{
    pub params: Vec<SignatureItem>,
}

impl ComSignature
{
    /// Creates ComInterface from AST elements.
    pub fn from_ast(attr: TokenStream, item: &Ident) -> ParseResult<ComSignature>
    {
        let attr: ComSignatureAttr = ::syn::parse2(attr).map_err(|_| {
            ParseError::ComSignature(item.to_string(), "Attribute syntax error".into())
        })?;

        Ok(ComSignature {
            params: attr.args().into_iter().cloned().collect(),
        })
    }
}
