use super::*;
use crate::prelude::*;

use crate::guid::GUID;
use crate::idents::SomeIdent;
use syn::{LitStr, Path};

intercom_attribute!(
    ClsidMacro<ClsidMacroParam, Path> {
        clsid: LitStr,
    }
);

/// Details of a struct marked with `#[com_class]` attribute.
#[derive(Debug, PartialEq)]
pub struct Clsid
{
    pub path: Path,
    pub name: Ident,
    pub clsid: GUID,
}

impl Clsid
{
    /// Creates ComClass from AST elements.
    pub fn parse(crate_name: &str, macro_params: TokenStream) -> ParseResult<Clsid>
    {
        let params: ClsidMacro = ::syn::parse2(macro_params)
            .map_err(|e| ParseError::Clsid(format!("Attribute syntax error: {}", e)))?;

        if params.args().len() != 1 {
            return Err(ParseError::Clsid(
                "clsid! must have exactly one path specified".to_string(),
            ));
        }

        let item_path = params.args()[0];
        let name = item_path
            .get_some_ident()
            .ok_or_else(|| ParseError::Clsid("Could not resolve ident".to_string()))?;

        // First attribute parameter is the CLSID. Parse it.
        let clsid_attr = params.clsid().map_err(|msg| {
            ParseError::Clsid(format!("Failed to parse clsid parameter: {}", msg))
        })?;
        let clsid = match clsid_attr {
            None => crate::utils::generate_clsid(crate_name, &name.to_string()),
            Some(clsid) => GUID::parse(&clsid.value())
                .map_err(|_| ParseError::Clsid(format!("Bad CLSID format on {}", name)))?,
        };

        Ok(Clsid {
            path: item_path.clone(),
            name,
            clsid,
        })
    }
}

#[cfg(test)]
mod test
{
    use super::*;
}
