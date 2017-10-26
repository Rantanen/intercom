
extern crate com_runtime;

use syntax::ptr::P;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::tokenstream::TokenTree;
use syntax::ast::*;
use syntax::print::pprust;

pub fn trace( t : &str, n : &str ) {
    println!( "Added {}: {}", t, n );
}

pub fn get_ident_and_fns(
    a : &Annotatable
) -> Option< ( Ident, Vec<(&Ident, &MethodSig)> ) >
{
    // Get the annotated item.
    let item = match a {
        &Annotatable::Item( ref item ) => item,
        _ => return None
    };

    match item.node {
        ItemKind::Impl( .., ref trait_ref, ref ty, ref items ) => {
            let ( _, struct_ident, items ) = get_impl_data_raw( trait_ref, ty, items );
            Some( ( struct_ident, items ) )
        },
        ItemKind::Trait( .., ref items ) => {

            let methods : Option< Vec< (&Ident, &MethodSig) > > = items
                    .into_iter()
                    .map( |i| get_trait_method( i ).map( |m| ( &i.ident, m ) ) )
                    .collect();

            match methods {
                Some( m ) => Some( ( item.ident, m ) ),
                None => None
            }
        },
        _ => None
    }
}

pub fn get_impl_data(
    a : &Annotatable
) -> Option< ( Option< Ident >, Ident, Vec< ( &Ident, &MethodSig ) > ) >
{
    if let &Annotatable::Item( ref item ) = a {
        if let ItemKind::Impl( .., ref trait_ref, ref ty, ref items ) = item.node {
            return Some( get_impl_data_raw( trait_ref, ty, items ) );
        }
    }
    None
}

fn get_impl_data_raw<'a>(
    trait_ref : &'a Option<TraitRef>,
    struct_ty : &'a P<Ty>,
    items : &'a [ImplItem]
) -> ( Option<Ident>, Ident, Vec< ( &'a Ident, &'a MethodSig ) > )
{

    let struct_ident = match get_ty_ident( struct_ty ) {
        Some( ty_ident ) => ty_ident,
        None => panic!()
    };

    let trait_ident = match trait_ref {
        &Some( ref tr ) => Some( path_to_ident( &tr.path ) ),
        &None => None
    };

    let methods_opt : Option< Vec< (&Ident, &MethodSig) > > = items
            .into_iter()
            .map( |i| get_impl_method( i ).map( |m| ( &i.ident, m ) ) )
            .collect();
    let methods = methods_opt.unwrap_or( vec![] );

    ( trait_ident, struct_ident, methods )
}

pub fn path_to_ident(
    p : &Path
) -> Ident
{
    p.segments.last().unwrap().identifier.clone()
}


pub fn get_struct_ident_from_annotatable(
    a : &Annotatable
) -> Option< Ident >
{
    // Get the annotated item.
    if let &Annotatable::Item( ref item ) = a {
        return Some( item.ident )
    }

    None
}

pub fn get_metaitem_params(
    mi : &MetaItem
) -> Option< Vec< &NestedMetaItemKind > >
{
    if let MetaItemKind::List( ref v ) = mi.node {
        return Some( v.into_iter().map( |sp| &sp.node ).collect() );
    }

    None
}

pub fn get_ty_ident(
    ty : &Ty
) -> Option<Ident>
{
    match ty.node {
        TyKind::Path( _, ref p ) =>
            p.segments.last().map( |l| l.identifier.clone() ),
        _ => None
    }
}

pub fn get_impl_method(
    i : &ImplItem
) -> Option< &MethodSig >
{
    if let ImplItemKind::Method( ref method_sig, _ ) = i.node {
        return Some( method_sig );
    }
    None
}

pub fn get_trait_method(
    i : &TraitItem
) -> Option< &MethodSig >
{
    if let TraitItemKind::Method( ref method_sig, _ ) = i.node {
        return Some( method_sig );
    }
    None
}

pub fn parameter_to_guid(
    p : &NestedMetaItemKind
) -> Result< com_runtime::GUID, &'static str >
{
    if let &NestedMetaItemKind::Literal( ref l ) = p {
        if let LitKind::Str( ref s, _ ) = l.node {
            return com_runtime::GUID::parse( &s.as_str() );
        }
    }

    return Err( "GUID parameter must be literal string" );
}

pub fn parameter_to_ident(
    p : &NestedMetaItemKind
) -> Option<Ident>
{
    if let &NestedMetaItemKind::MetaItem( MetaItem { name, .. } ) = p {
        return Some( Ident::from_str( &name.as_str() ) );
    }

    None
}

trait ParamHandler {
    fn get_arg_ty(
        &self,
        cx : &mut ExtCtxt,
        ty : &P<Ty>
    ) -> Vec<TokenTree>;
    fn get_call_param(
        &self,
        cx : &mut ExtCtxt,
        ident : &Ident,
        ty : &P<Ty>
    ) -> Vec<TokenTree>;
    fn write_out_param(
        &self,
        cx : &mut ExtCtxt,
        ident : &Ident,
        ty : &P<Ty>
    ) -> Vec<TokenTree>;
}

struct IdentityParam;
impl ParamHandler for IdentityParam {
    fn get_arg_ty(
        &self,
        cx : &mut ExtCtxt,
        ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, $ty )
    }
    fn get_call_param(
        &self,
        cx : &mut ExtCtxt,
        ident : &Ident,
        ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, $ident )
    }
    fn write_out_param(
        &self,
        cx : &mut ExtCtxt,
        ident : &Ident,
        ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, *__out = r.into(); )
    }
}

struct ComRcParam;
impl ParamHandler for ComRcParam {
    fn get_arg_ty(
        &self,
        cx : &mut ExtCtxt,
        ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, com_runtime::RawComPtr )
    }
    fn get_call_param(
        &self,
        cx : &mut ExtCtxt,
        ident : &Ident,
        ty : &P<Ty>
    ) -> Vec<TokenTree>
    {
        quote_tokens!( cx, $ident )
    }
    fn write_out_param(
        &self,
        cx : &mut ExtCtxt,
        ident : &Ident,
        ty : &P<Ty>
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

        let itf_ident = match get_ty_ident( itf_ty ) {
            Some( ty_ident ) => ty_ident,
            None => panic!()
        };

        let iid_ident = super::idents::iid( &itf_ident );
        quote_tokens!( cx,
            com_runtime::ComRc::query_interface( &r, &$iid_ident, __out ) )
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
                _ => Box::new( IdentityParam )
            }
        },

        // Default to identity param.
        _ => Box::new( IdentityParam )
    }
}

pub fn get_method_args(
    cx : &mut ExtCtxt,
    m : &MethodSig
) -> Option<(
    Vec<Vec<TokenTree>>,
    Vec<Vec<TokenTree>>,
)>
{
    m.decl.inputs
        .split_first()
        .and_then( | (self_arg, other_args ) | {

            // Get the self arg. This is always a ComPtr.
            let mut args = vec![
                quote_tokens!( cx, self_vtable : com_runtime::RawComPtr, )
            ];

            // Process the remaining args into the args and params arrays.
            let mut params : Vec<Vec<TokenTree>> = vec![];
            for arg_ref in other_args {

                // Get the type handler.
                let handler = get_param_handler( &arg_ref.ty );
                let ty = handler.get_arg_ty( cx, &arg_ref.ty );
                let arg_ident = Ident::from_str(
                        &pprust::pat_to_string( &(*arg_ref.pat) ) );

                // Construct the arguemnt. quote_tokens! has difficulties parsing
                // arguments on their own so construct the Arg using quote_arg!
                // and then push the tokens using quote_tokens!.
                let arg = quote_arg!( cx, $arg_ident : $ty );
                args.push( quote_tokens!( cx, $arg, ) );

                // Get the call parameter.
                let call_param = handler.get_call_param( cx, &arg_ident, &arg_ref.ty );
                params.push( call_param );
            }

            // Add the [retval] arg if one exists and isn't ().
            if let Some( outs ) = get_out_and_ret( cx, m ) {
                if let ( Some( out_ty ), _ ) = outs {
                    if ! is_unit( &out_ty.node ) {
                        let handler = get_param_handler( &out_ty );
                        let ty = handler.get_arg_ty( cx, &out_ty );
                        args.push( quote_tokens!( cx, __out : *mut $ty ) );
                    }
                }
            } else {
                return None
            };

            // Ensure the first parameter is &self.
            // Static methods don't count here.
            if let TyKind::Rptr( _, _ ) = self_arg.ty.node {
            } else {
                return None
            }

            Some( (
                args,
                params,
            ) )
        } )
}

pub fn is_unit(
    tk : &TyKind
) -> bool
{
    if let &TyKind::Tup( ref v ) = tk {
        if v.len() == 0 {
            return true
        }
    }
    false
}

pub fn get_ret_types(
    cx: &mut ExtCtxt,
    ret_ty : &Ty
) -> Result< ( Option<P<Ty>>, P<Ty> ), &'static str >
{
    // Get the path type on the return value.
    let path = match &ret_ty.node {
        &TyKind::Path( _, ref p ) => p,
        _ => return Err( "Use Result as a return type" )
    };

    // Find the last path segment.
    let last_segment = path.segments.last().unwrap();

    // Check the last segment has angle bracketed parameters.
    if let &Some( ref p ) = &last_segment.parameters {
        if let PathParameters::AngleBracketed( ref data ) = **p {

            // Angle bracketed parameters exist. We're assuming this is
            // some kind of Result<ok> or Result<ok, err>. In either case
            // we can take the first parameter as the 'ok' type.
            //
            // TODO: Figure out whether we can ask the compiler whether
            // the type matches Result<S,E> type.
            return Ok( (
                data.types.first().and_then( |x| Some( x.clone() ) ),
                quote_ty!( cx, com_runtime::HRESULT )
            ) )
        }
    }

    // Default value. We get here only if we didn't return a type from
    // the if statements above.
    Ok( ( None, quote_ty!( cx, $path ) ) )
}

pub fn get_out_and_ret(
    cx : &mut ExtCtxt,
    m : &MethodSig
) -> Option< ( Option< P<Ty> >, P<Ty> ) >
{
    let output = &m.decl.output;
    let result_ty = match output {
        &FunctionRetTy::Ty( ref ty ) => ty,
        _ => return None
    };

    get_ret_types( cx, &result_ty ).ok()
}

pub fn get_method_rvalues(
    cx : &mut ExtCtxt,
    m : &MethodSig
) -> Option< ( P<Ty>, Vec<TokenTree> ) >
{
    let ( out_ty, ret_ty ) = match get_out_and_ret( cx, m ) {
        Some( s ) => s,
        None => return None,
    };

    Some( match out_ty {
        // Result<(), _>. Ignore the [retval] value but handle the Err
        // as the method return value.
        Some( ref unit ) if is_unit( &unit.node ) => (
            ret_ty,
            quote_tokens!( cx,
                match result {
                    Ok( _ ) => com_runtime::S_OK,
                    Err( e ) => e
                } ) ),

        // Result<_, _>. Ok() -> __out + S_OK, Err() -> E_*
        Some( ref out_ty ) => {
            let handler = get_param_handler( &out_ty );
            let out_ident = Ident::from_str( "__out" );
            let write_out = handler.write_out_param( cx, &out_ident, &out_ty );
            (
                ret_ty,
                quote_tokens!( cx,
                    match result {
                        Ok( r ) => { $write_out; com_runtime::S_OK },
                        Err( e ) => e
                    } ) )
        },

        // Not a Result<..>, assume we can return the return value as is.
        None => (
            ret_ty, quote_tokens!( cx, return result ) ),
    } )
}

pub fn get_guid_tokens(
    cx : &mut ExtCtxt,
    g : &com_runtime::GUID
) -> Vec<TokenTree>
{
    let d1 = g.data1;
    let d2 = g.data2;
    let d3 = g.data3;
    let d4_0 = g.data4[ 0 ];
    let d4_1 = g.data4[ 1 ];
    let d4_2 = g.data4[ 2 ];
    let d4_3 = g.data4[ 3 ];
    let d4_4 = g.data4[ 4 ];
    let d4_5 = g.data4[ 5 ];
    let d4_6 = g.data4[ 6 ];
    let d4_7 = g.data4[ 7 ];
    quote_tokens!( cx,
        com_runtime::GUID {
            data1: $d1, data2: $d2, data3: $d3,
            data4: [ $d4_0, $d4_1, $d4_2, $d4_3, $d4_4, $d4_5, $d4_6, $d4_7 ]
        }
    )
}

