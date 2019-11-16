use super::common::*;

use crate::model;

extern crate proc_macro;
use self::proc_macro::TokenStream;

/// Expands the `com_impl` attribute.
///
/// The attribute expansion results in the following items:
///
/// - Implementation for the delegating methods when calling the Rust methods
///   from COM.
/// - Virtual table instance for the COM type.
pub fn expand_com_impl(
    _attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, model::ParseError>
{
    let output = vec![];
    Ok(tokens_to_tokenstream(item_tokens, output))
}
