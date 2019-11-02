use super::*;
use crate::prelude::*;

use syn::{Fields, Ident, Visibility};

/// Details of a struct marked with `#[com_class]` attribute.
#[derive(Debug, PartialEq)]
pub struct ComStruct
{
    pub name: Ident,
    pub vis: Visibility,
    pub fields: Fields,
}

impl ComStruct
{
    /// Creates ComStruct from AST elements.
    pub fn parse(
        _crate_name: &str,
        _attr_params: TokenStream,
        item: TokenStream,
    ) -> ParseResult<ComStruct>
    {
        // Parse the inputs.
        let item: ::syn::ItemStruct = ::syn::parse2(item)
            .map_err(|_| ParseError::ComStruct("<Unknown>".into(), "Item syntax error".into()))?;

        Ok(ComStruct {
            name: item.ident.clone(),
            vis: item.vis,
            fields: item.fields,
        })
    }
}

#[cfg(test)]
mod test
{
    use super::*;
}
