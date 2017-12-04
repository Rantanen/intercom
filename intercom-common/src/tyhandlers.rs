
use super::utils;

use syn::*;
use quote::Tokens;

pub trait TyHandler {

    fn rust_ty( &self ) -> &Ty;

    fn com_ty( &self ) -> Tokens
    {
        let ty = &self.rust_ty();
        quote!( #ty )
    }

    fn com_to_rust(
        &self, ident : &Ident
    ) -> Tokens
    {
        quote!( #ident.into() )
    }

    fn rust_to_com(
        &self, ident : &Ident
    ) -> Tokens
    {
        quote!( #ident.into() )
    }

    fn default_value( &self) -> Tokens
    {
        match self.rust_ty() {
            &Ty::Path( _, ref p ) => {
                let name : &str = &p.segments.last().unwrap().ident.as_ref();
                match name {
                    "c_void" => quote!( std::ptr::null_mut() ),
                    "RawComPtr" => quote!( std::ptr::null_mut() ),
                    "ComRc" => quote!( std::ptr::null_mut() ),
                    _ => quote!( Default::default() )
                }
            },
            _ => quote!( Default::default() )
        }
    }
}

struct IdentityParam( Ty );
impl TyHandler for IdentityParam {
    fn rust_ty( &self ) -> &Ty { &self.0 }
}


struct ComRcParam( Ty );
impl TyHandler for ComRcParam {

    fn rust_ty( &self ) -> &Ty { &self.0 }

    fn com_ty( &self ) -> Tokens
    {
        quote!( intercom::RawComPtr )
    }

    fn rust_to_com( &self, ident : &Ident ) -> Tokens
    {
        let none_tokens = quote!( );
        let comrc_params = match self.0 {
            Ty::Path( _, ref p ) => {
                let last_segment = &p.segments.last().unwrap();
                match last_segment.parameters {
                    PathParameters::AngleBracketed( ref data ) => data,
                    _ => return none_tokens
                }
            }
            _ => return none_tokens
        };

        let itf_ty = match comrc_params.types.first() {
            Some( ty ) => ty,
            _ => return none_tokens
        };

        let itf_ident = match utils::get_ty_ident( itf_ty ) {
            Some( ty_ident ) => ty_ident,
            None => panic!()
        };

        let iid_ident = super::idents::iid( &itf_ident );
        quote!( intercom::ComRc::query_interface( &#ident, &#iid_ident )
                .expect( "ComRc<T> does not support interface T" ) )
    }
}

struct StringParam( Ty );
impl TyHandler for StringParam
{
    fn rust_ty( &self ) -> &Ty { &self.0 }

    fn com_ty( &self ) -> Tokens
    {
        quote!( intercom::BStr )
    }

    fn com_to_rust( &self, ident : &Ident ) -> Tokens
    {
        quote!( #ident.bstr_to_string() )
    }

    fn rust_to_com( &self, ident : &Ident ) -> Tokens
    {
        quote!( *#ident = intercom::BStr::string_to_bstr( &r ) )
    }
}

pub fn get_ty_handler(
    arg_ty : &Ty,
) -> Box<TyHandler>
{
    let ty = arg_ty.clone();
    match arg_ty {

        &Ty::Path( .., ref p ) => {
            let name : &str = &p.segments.last().unwrap().ident.as_ref();
            match name {
                "ComRc" => Box::new( ComRcParam( ty ) ),
                "String" => Box::new( StringParam( ty ) ),
                _ => Box::new( IdentityParam( ty ) )
            }
        },

        // Default to identity param.
        _ => Box::new( IdentityParam( ty ) )
    }
}
