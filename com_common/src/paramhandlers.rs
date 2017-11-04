
use super::utils;

use syn::*;
use quote::Tokens;

pub trait ParamHandler {
    fn arg_ty(
        &self, ty : &Ty
    ) -> Tokens
    {
        quote!( #ty )
    }

    fn for_call(
        &self, ident : &Ident, _ty : &Ty
    ) -> Tokens
    {
        quote!( #ident.into() )
    }

    fn write_out(
        &self, ident : &Ident, _ty : &Ty
    ) -> Tokens
    {
        quote!( *#ident = r.into(); )
    }

    fn write_null(
        &self, ident : &Ident, ty : &Ty
    ) -> Tokens
    {
        match ty {
            &Ty::Path( _, ref p ) => {
                let name : &str = &p.segments.last().unwrap().ident.as_ref();
                match name {
                    "c_void" => quote!( *#ident = std::ptr::null_mut() ),
                    "RawComPtr" => quote!( *#ident = std::ptr::null_mut() ),
                    "ComRc" => quote!( *#ident = std::ptr::null_mut() ),
                    _ => quote!( *#ident = Default::default(); )
                }
            },
            _ => quote!( *#ident = Default::default(); )
        }
    }
}

struct IdentityParam;
impl ParamHandler for IdentityParam { }

struct ComRcParam;
impl ParamHandler for ComRcParam {
    fn arg_ty(
        &self, ty : &Ty
    ) -> Tokens
    {
        quote!( com_runtime::RawComPtr )
    }

    fn write_out(
        &self, ident : &Ident, ty : &Ty
    ) -> Tokens
    {
        let none_tokens = quote!( );
        let comrc_params = match ty {
            &Ty::Path( _, ref p ) => {
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
        quote!(
            com_runtime::ComRc::query_interface(
                    &r, &#iid_ident, #ident ) )
    }
}

struct StringParam;
impl ParamHandler for StringParam
{
    fn arg_ty(
        &self, ty : &Ty
    ) -> Tokens
    {
        quote!( com_runtime::BStr )
    }

    fn for_call(
        &self, ident : &Ident, _ty : &Ty
    ) -> Tokens
    {
        quote!( #ident.bstr_to_string() )
    }

    fn write_out(
        &self, ident : &Ident, _ty : &Ty
    ) -> Tokens
    {
        quote!( *#ident = com_runtime::BStr::string_to_bstr( &r ) )
    }
}

pub fn get_param_handler(
    arg_ty : &Ty,
) -> Box<ParamHandler>
{
    match arg_ty {

        &Ty::Path( _, ref p ) => {
            let name : &str = &p.segments.last().unwrap().ident.as_ref();
            match name {
                "ComRc" => Box::new( ComRcParam ),
                "String" => Box::new( StringParam ),
                _ => Box::new( IdentityParam )
            }
        },

        // Default to identity param.
        _ => Box::new( IdentityParam )
    }
}
