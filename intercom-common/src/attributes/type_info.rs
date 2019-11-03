use crate::prelude::*;

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
    let input: syn::DeriveInput = syn::parse(item_tokens)?;
    let name = &input.ident;

    // Immpl requires the the generics in particular way.
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let result = quote! { impl #impl_generics intercom::type_system::BidirectionalTypeInfo for #name #ty_generics #where_clause {

        /// The default name is the name of the type.
        fn type_name() -> &'static str { stringify!( #name ) }
    } };

    Ok(result.into())
}

/// Expands the `ExternParameter` derive attribute.
///
/// The attribute expansion results in the following items:
///
/// - Implementation of the ExternParameter trait.
pub fn expand_derive_extern_parameter(
    item_tokens: TokenStreamNightly,
) -> Result<TokenStreamNightly, syn::Error>
{
    // Get the name of the type we want to implement the trait for.
    let input: syn::DeriveInput = syn::parse(item_tokens)?;
    let name = &input.ident;

    // Immpl requires the the generics in particular way.
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let result = quote! { impl<TS: intercom::type_system::TypeSystem> #impl_generics intercom::type_system::ExternParameter<TS> for #name #ty_generics #where_clause {

        type ForeignType = #name;
        type IntoTemporary = ();

        #[inline(always)]
        fn into_foreign_parameter(self) -> intercom::ComResult<(Self::ForeignType, Self::IntoTemporary)> {
            Ok((self, ()))
        }

        type OwnedParameter = #name;

        #[inline(always)]
        unsafe fn from_foreign_parameter(source: Self::ForeignType) -> intercom::ComResult<Self::OwnedParameter> {
            Ok(source)
        }
    } };

    Ok(result.into())
}

/// Expands the `ExternOutput` derive attribute.
///
/// The attribute expansion results in the following items:
///
/// - Implementation of the ExternOutput trait.
pub fn expand_derive_extern_output(
    item_tokens: TokenStreamNightly,
) -> Result<TokenStreamNightly, syn::Error>
{
    // Get the name of the type we want to implement the trait for.
    let input: syn::DeriveInput = syn::parse(item_tokens)?;
    let name = &input.ident;

    // Immpl requires the the generics in particular way.
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let result = quote! { impl<TS: intercom::type_system::TypeSystem> #impl_generics intercom::type_system::ExternOutput<TS> for #name #ty_generics #where_clause {

        type ForeignType = #name;

        #[inline(always)]
        fn into_foreign_output(self) -> intercom::ComResult<Self::ForeignType> {
            Ok(self)
        }

        #[inline(always)]
        unsafe fn from_foreign_output(source: Self::ForeignType) -> intercom::ComResult<Self> {
            Ok(source)
        }
    } };

    Ok(result.into())
}
