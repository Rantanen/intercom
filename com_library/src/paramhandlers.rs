
use super::utils;

use syntax::ast::*;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ptr::P;
use syntax::tokenstream::TokenTree;

pub trait ParamHandler {
    fn arg_ty(
        &self, cx : &mut ExtCtxt, ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, $ty )
    }

    fn for_call(
        &self, cx : &mut ExtCtxt, ident : &Ident, _ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, $ident.into() )
    }

    fn write_out(
        &self, cx : &mut ExtCtxt, ident : &Ident, _ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, *$ident = r.into(); )
    }

    fn write_null(
        &self, cx : &mut ExtCtxt, ident : &Ident, _ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, *$ident = Default::default(); )
    }
}

struct IdentityParam;
impl ParamHandler for IdentityParam { }

struct ComRcParam;
impl ParamHandler for ComRcParam {
    fn arg_ty(
        &self, cx : &mut ExtCtxt, _ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, com_runtime::RawComPtr )
    }

    fn write_out(
        &self, cx : &mut ExtCtxt, ident : &Ident, ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        let none_tokens = quote_tokens!( cx, );
        let comrc_params = match ty.node {
            TyKind::Path( _, ref p ) => {
                let last_segment = &p.segments.last().unwrap();
                match last_segment.parameters {
                    Some( ref p ) => match **p {
                        PathParameters::AngleBracketed( ref data ) => data,
                        _ => return none_tokens
                    },
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
        quote_tokens!( cx,
            com_runtime::ComRc::query_interface(
                    &r, &$iid_ident, $ident ) )
    }
}

struct StringParam;
impl ParamHandler for StringParam
{
    fn arg_ty(
        &self, cx : &mut ExtCtxt, _ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, com_runtime::BStr )
    }

    fn for_call(
        &self, cx : &mut ExtCtxt, ident : &Ident, _ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, $ident.bstr_to_string() )
    }

    fn write_out(
        &self, cx : &mut ExtCtxt, ident : &Ident, _ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, *$ident = com_runtime::BStr::string_to_bstr( &r ) )
    }
}

pub fn get_param_handler(
    arg_ty : &P<Ty>,
) -> Box<ParamHandler>
{
    match arg_ty.node {

        TyKind::Path( _, ref p ) => {
            let name : &str = &p.segments.last().unwrap().identifier.name.as_str();
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
