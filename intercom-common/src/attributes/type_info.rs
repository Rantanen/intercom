
use prelude::*;

use std::iter;

use tyhandlers::{Direction};
use idents;
use model;

extern crate proc_macro;
use self::proc_macro::TokenStream;


/// Expands the `BidirectionalTypeInfo` derive attribute.
///
/// The attribute expansion results in the following items:
///
/// - Implementation of the BidirectionalTypeInfo trait.
pub fn expand_bidirectional_type_info(
    item_tokens: TokenStreamNightly,
) -> Result<TokenStreamNightly, syn::Error>
{
    // Get the name of the type we want to implement the trait for.
    let input: syn::DeriveInput = syn::parse( item_tokens )?;
    let name = &input.ident;

    // Immpl requires the the generics in particular way.
    let (impl_generics, ty_generics, where_clause ) = input.generics.split_for_impl();
    let result = quote!{ impl #impl_generics ::intercom::type_system::BidirectionalTypeInfo for #name #ty_generics #where_clause {

                /// The default name is the name of the type.
                fn type_name() -> &'static str { stringify!( #name ) }
            } };

    Ok( result.into() )
}
