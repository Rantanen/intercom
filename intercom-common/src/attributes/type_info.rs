use crate::prelude::*;

/// Expands the `ForeignType` derive attribute.
///
/// The attribute expansion results in the following items:
///
/// - Implementation of the ForeignType trait.
pub fn expand_bidirectional_type_info(
    item_tokens: TokenStreamNightly,
) -> Result<TokenStreamNightly, syn::Error>
{
    // Get the name of the type we want to implement the trait for.
    let input: syn::DeriveInput = syn::parse(item_tokens)?;
    let name = &input.ident;

    // Immpl requires the the generics in particular way.
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let result = quote! { impl #impl_generics intercom::type_system::ForeignType for #name #ty_generics #where_clause {

        /// The default name is the name of the type.
        fn type_name() -> &'static str { stringify!( #name ) }
    } };

    Ok(result.into())
}

/// Expands the `ExternInput` derive attribute.
///
/// The attribute expansion results in the following items:
///
/// - Implementation of the ExternInput trait.
pub fn expand_derive_extern_parameter(
    item_tokens: TokenStreamNightly,
) -> Result<TokenStreamNightly, syn::Error>
{
    // Get the name of the type we want to implement the trait for.
    let input: syn::DeriveInput = syn::parse(item_tokens)?;
    let name = &input.ident;

    // Immpl requires the the generics in particular way.
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let result = quote! {
        unsafe impl<TS: intercom::type_system::TypeSystem> #impl_generics intercom::type_system::ExternInput<TS> for #name #ty_generics #where_clause {

            type ForeignType = #name;
            type Lease = ();

            #[inline(always)]
            unsafe fn into_foreign_parameter(self) -> intercom::ComResult<(Self::ForeignType, Self::Lease)> {
                Ok((self, ()))
            }

            type Owned = #name;

            #[inline(always)]
            unsafe fn from_foreign_parameter(source: Self::ForeignType) -> intercom::ComResult<Self::Owned> {
                Ok(source)
            }
        }

        unsafe impl<TS: intercom::type_system::TypeSystem> #impl_generics intercom::type_system::InfallibleExternInput<TS> for #name #ty_generics #where_clause {

            type ForeignType = #name;
            type Lease = ();

            #[inline(always)]
            unsafe fn into_foreign_parameter(self) -> (Self::ForeignType, Self::Lease) {
                (self, ())
            }

            type Owned = #name;

            #[inline(always)]
            unsafe fn from_foreign_parameter(source: Self::ForeignType) -> Self::Owned {
                source
            }
        }
    };

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

    // Impl requires the the generics in particular way.
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let result = quote! {
        unsafe impl<TS: intercom::type_system::TypeSystem> #impl_generics intercom::type_system::ExternOutput<TS> for #name #ty_generics #where_clause {

            type ForeignType = #name;

            #[inline(always)]
            fn into_foreign_output(self) -> intercom::ComResult<Self::ForeignType> {
                Ok(self)
            }

            #[inline(always)]
            unsafe fn from_foreign_output(source: Self::ForeignType) -> intercom::ComResult<Self> {
                Ok(source)
            }
        }

        unsafe impl<TS: intercom::type_system::TypeSystem> #impl_generics intercom::type_system::InfallibleExternOutput<TS> for #name #ty_generics #where_clause {

            type ForeignType = #name;

            #[inline(always)]
            fn into_foreign_output(self) -> Self::ForeignType {
                self
            }

            #[inline(always)]
            unsafe fn from_foreign_output(source: Self::ForeignType) -> Self {
                source
            }
        }
    };

    Ok(result.into())
}
