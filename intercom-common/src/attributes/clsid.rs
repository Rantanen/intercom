use super::common::*;
use crate::model;
use crate::prelude::*;
use crate::utils::{get_guid_tokens, snake_case};

/// Expands the `ForeignType` derive attribute.
///
/// The attribute expansion results in the following items:
///
/// - Implementation of the ForeignType trait.
pub fn expand_clsid(
    macro_tokens: TokenStreamNightly,
) -> Result<TokenStreamNightly, model::ParseError>
{
    // Get the name of the type we want to implement the trait for.
    let model = model::Clsid::parse(&lib_name(), macro_tokens.into())?;
    let name = &model.name;

    let snake_case_name = snake_case(name.to_string()).to_uppercase();
    let ident = format_ident!("CLSID_{}", snake_case_name);
    let guid_tokens = get_guid_tokens(&model.clsid, Span::call_site());

    // Immpl requires the the generics in particular way.
    let result = quote! {
        pub const #ident: intercom::GUID = #guid_tokens;
    };

    Ok(result.into())
}
