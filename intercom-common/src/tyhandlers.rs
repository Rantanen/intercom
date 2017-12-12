
use syn::*;
use quote::Tokens;

/// Defines Type-specific logic for handling the various parameter types in the
/// Rust/COM interface.
pub trait TyHandler {

    /// The Rust type.
    fn rust_ty( &self ) -> Ty;

    /// The COM type.
    fn com_ty( &self ) -> Ty
    {
        self.rust_ty()
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
    fn default_value( &self ) -> Tokens
    {
        match self.rust_ty() {
            Ty::Path( _, ref p ) => {
                let name : &str = p.segments.last().unwrap().ident.as_ref();
                match name {
                    "c_void"
                        | "RawComPtr"
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
    fn rust_ty( &self ) -> Ty { self.0.clone() }
}


/// `ComItf` parameter handler. Supports `ComItf` Rust type and ensures the this
/// to/from `RawComPtr` COM type.
struct ComItfParam( Ty );

impl TyHandler for ComItfParam {

    fn rust_ty( &self ) -> Ty { self.0.clone() }

    /// Gets the default value for the type.
    fn default_value( &self ) -> Tokens
    {
        quote!( ComItf::null_itf() )
    }
}

/// String parameter handler. Converts between Rust String and COM BSTR types.
struct StringParam( Ty );
impl TyHandler for StringParam
{
    fn rust_ty( &self ) -> Ty { self.0.clone() }

    fn com_ty( &self ) -> Ty
    {
        parse_type( "::intercom::BStr" ).unwrap()
    }

    fn com_to_rust( &self, ident : &Ident ) -> Tokens
    {
        quote!( #ident.into() )
    }

    fn rust_to_com( &self, ident : &Ident ) -> Tokens
    {
        quote!( #ident.into() )
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

                "ComItf" => Box::new( ComItfParam( ty ) ),
                "String" => Box::new( StringParam( ty ) ),

                // Unknown. Use IdentityParam.
                _ => Box::new( IdentityParam( ty ) )
            }
        },

        // Default to identity param.
        _ => Box::new( IdentityParam( ty ) )
    }
}
