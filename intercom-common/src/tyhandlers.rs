
use super::utils;

use syn::*;
use quote::Tokens;

/// Defines Type-specific logic for handling the various parameter types in the
/// Rust/COM interface.
pub trait TyHandler {

    /// The Rust type.
    fn rust_ty( &self ) -> &Ty;

    /// The COM type.
    fn com_ty( &self ) -> Tokens
    {
        let ty = &self.rust_ty();
        quote!( #ty )
    }

    /// Converts a COM parameter named by the ident into a Rust type.
    fn com_to_rust(
        &self, ident : &Ident
    ) -> Tokens
    {
        quote!( #ident.into() )
    }

    /// Converts a Rust parameter named by the ident into a COM type.
    fn rust_to_com(
        &self, ident : &Ident
    ) -> Tokens
    {
        quote!( #ident.into() )
    }

    /// Gets the default value for the type.
    fn default_value( &self) -> Tokens
    {
        match *self.rust_ty() {
            Ty::Path( _, ref p ) => {
                let name : &str = p.segments.last().unwrap().ident.as_ref();
                match name {
                    "c_void"
                        | "RawComPtr"
                        | "ComRc"
                        => quote!( ::std::ptr::null_mut() ),
                    _ => quote!( Default::default() )
                }
            },
            _ => quote!( Default::default() )
        }
    }
}

/// Identity parameter handler.
///
/// No special logic.
struct IdentityParam( Ty );

impl TyHandler for IdentityParam {
    fn rust_ty( &self ) -> &Ty { &self.0 }
}


/// `ComRc` parameter handler. Supports `ComRc` Rust type and converts this
/// to/from `RawComPtr` COM type.
struct ComRcParam( Ty );

impl TyHandler for ComRcParam {

    fn rust_ty( &self ) -> &Ty { &self.0 }

    fn com_ty( &self ) -> Tokens
    {
        quote!( ::intercom::RawComPtr )
    }

    fn rust_to_com( &self, ident : &Ident ) -> Tokens
    {
        // Get the parameter data from the type.
        let comrc_params = match self.0 {
            Ty::Path( _, ref p ) => {
                let last_segment = &p.segments.last().unwrap();
                match last_segment.parameters {
                    PathParameters::AngleBracketed( ref data ) => data,
                    _ => panic!( "ComRc doesn't have <> params" ),
                }
            }
            _ => unreachable!( "ComRcParam should only be used for Ty::Path" ),
        };

        // Get the interface type.
        let itf_ty = match comrc_params.types.first() {
            Some( ty ) => ty,
            _ => panic!( "ComRc doesn't have type parameters" ),
        };

        // Name the interface.
        let itf_ident = match utils::get_ty_ident( itf_ty ) {
            Some( ty_ident ) => ty_ident,
            _ => panic!( "Could not resolve name of {:?}", itf_ty ),
        };

        // Conversion is done with query_interface, which requires the
        // IID of the ComRc interface type.
        let iid_ident = super::idents::iid( itf_ident );
        quote!( ::intercom::ComRc::query_interface( &#ident, &#iid_ident )
                .expect( "ComRc<T> does not support interface T" ) )
    }
}

/// String parameter handler. Converts between Rust String and COM BSTR types.
struct StringParam( Ty );
impl TyHandler for StringParam
{
    fn rust_ty( &self ) -> &Ty { &self.0 }

    fn com_ty( &self ) -> Tokens
    {
        quote!( ::intercom::BStr )
    }

    fn com_to_rust( &self, ident : &Ident ) -> Tokens
    {
        quote!( #ident.bstr_to_string() )
    }

    fn rust_to_com( &self, ident : &Ident ) -> Tokens
    {
        quote!( *#ident = ::intercom::BStr::string_to_bstr( &r ) )
    }
}

/// Resolves the `TyHandler` to use.
pub fn get_ty_handler(
    arg_ty : &Ty,
) -> Box<TyHandler>
{
    // The ParamHandler needs an owned Ty so clone it here.
    let ty = arg_ty.clone();

    // The match is done using the original ty so we can borrow it while we
    // yield ownership to the cloned 'ty'.
    match *arg_ty {

        // Ty::Path represents various qualified type names, such as structs
        // and traits.
        Ty::Path( .., ref p ) => {

            // Match based on the last segment. We can't rely on the fully
            // qualified name to be in the previous segments thanks to use-
            // statements.
            let name : &str = p.segments.last().unwrap().ident.as_ref();
            match name {

                "ComRc" => Box::new( ComRcParam( ty ) ),
                "String" => Box::new( StringParam( ty ) ),

                // Unknown. Use IdentityParam.
                _ => Box::new( IdentityParam( ty ) )
            }
        },

        // Default to identity param.
        _ => Box::new( IdentityParam( ty ) )
    }
}
