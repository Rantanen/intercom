
extern crate com_runtime;

use super::*;
use syn::*;
use quote::Tokens;

use error::MacroError;

pub fn trace( t : &str, n : &str ) {
    println!( "Added {}: {}", t, n );
}

pub fn parse_inputs(
    attr_name: &str,
    attr_tokens: &TokenStream,
    item_tokens: &TokenStream,
) -> Result<( Vec<Tokens>, Attribute, Item ), MacroError>
{
    let attr_rendered = format!( "#[{}{}]", attr_name, attr_tokens.to_string() );
    let attr = match syn::parse_outer_attr( &attr_rendered ) {
        Ok(t) => t,
        Err(e) => error(
                format!( "Could not parse [{}] attribute", attr_name ),
                attr_tokens ),
    };
    let item = match syn::parse_item( &item_tokens.to_string() ) {
        Ok(t) => t,
        Err(e) => error(
                format!( "Could not parse [{}] item", attr_name ),
                &attr ),
    };

    Ok( ( vec![ quote!( #item ) ], attr, item ) )
}

pub fn tokens_to_tokenstream<T: IntoIterator<Item=Tokens>>(
    tokens : T,
) -> Result<TokenStream, LexError>
{
    TokenStream::from_str(
            &tokens.into_iter()
                .map( |t| t.parse::<String>().unwrap() )
                .fold( String::new(), |prev, next| prev + &next ) )
}

pub fn flatten<'a, I: Iterator<Item=&'a Tokens>>(
    tokens: I
) -> Tokens
{
    let mut all_tokens = quote::Tokens::new();
    all_tokens.append_all( tokens );
    all_tokens
}

pub fn get_ident_and_fns(
    item : &Item
) -> Option< ( &Ident, Vec<(&Ident, &MethodSig)> ) >
{
    match item.node {
        ItemKind::Impl( .., ref trait_ref, ref ty, ref items ) => {
            let ( _, struct_ident, items ) =
                    get_impl_data_raw( trait_ref, ty, items );
            Some( ( struct_ident, items ) )
        },
        ItemKind::Trait( .., ref items ) => {

            let methods : Option< Vec< (&Ident, &MethodSig) > > = items
                    .into_iter()
                    .map( |i| get_trait_method( i ).map( |m| ( &i.ident, m ) ) )
                    .collect();

            match methods {
                Some( m ) => Some( ( &item.ident, m ) ),
                None => None
            }
        },
        _ => None
    }
}

pub fn get_impl_data<'a>(
    item : &'a Item
) -> Option< ( Option<&'a Ident>, &'a Ident, Vec< ( &'a Ident, &'a MethodSig ) > ) >
{
    if let ItemKind::Impl( .., ref trait_ref, ref ty, ref items ) = item.node {
        return Some( get_impl_data_raw( trait_ref, ty, items ) );
    }
    None
}

fn get_impl_data_raw<'a>(
    trait_ref : &'a Option<Path>,
    struct_ty : &'a Ty,
    items : &'a [ImplItem]
) -> ( Option<&'a Ident>, &'a Ident, Vec< ( &'a Ident, &'a MethodSig ) > )
{

    let struct_ident = match get_ty_ident( struct_ty ) {
        Some( ty_ident ) => ty_ident,
        None => panic!()
    };

    let trait_ident = match trait_ref {
        &Some( ref tr ) => Some( path_to_ident( &tr ) ),
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
) -> &Ident
{
    &p.segments.last().unwrap().ident
}

pub fn get_struct_ident_from_annotatable(
    item : &Item
) -> &Ident
{
    &item.ident
}

pub fn get_attr_params(
    attr : &Attribute
) -> Option< &Vec<NestedMetaItem> >
{
    if let MetaItem::List( _, ref v ) = attr.value {
        return Some( v );
    }

    None
}

pub fn get_ty_ident(
    ty : &Ty
) -> Option<&Ident>
{
    match ty {
        &Ty::Path( _, ref p ) =>
            p.segments.last().map( |l| &l.ident ),
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
    p : &NestedMetaItem
) -> Result< com_runtime::GUID, &'static str >
{
    if let &NestedMetaItem::Literal( Lit::Str( ref s, _ ) ) = p {
        return com_runtime::GUID::parse( &s.as_str() );
    }

    return Err( "GUID parameter must be literal string" );
}

pub fn get_method_args(
    m : &MethodSig
) -> Option<(
    Vec<Tokens>,
    Vec<Tokens>,
)>
{
    m.decl.inputs
        .split_first()
        .and_then( | (self_arg, other_args ) | {

            // Get the self arg. This is always a ComPtr.
            let mut args = vec![
                quote!( self_vtable : com_runtime::RawComPtr, )
            ];

            // Process the remaining args into the args and params arrays.
            let mut params : Vec<Tokens> = vec![];
            for arg_ref in other_args {

                // Get the type handler.
                let arg_ty = arg_to_ty( &arg_ref );
                let param = paramhandlers::get_param_handler(
                    &arg_ty );
                let ty = param.arg_ty( &arg_ty );
                let arg_ident = match arg_to_ident( arg_ref ) {
                    Ok(i) => i, Err(e) => panic!(e.msg)
                };

                // Construct the arguemnt. quote_tokens! has difficulties parsing
                // arguments on their own so construct the Arg using quote_arg!
                // and then push the tokens using quote_tokens!.
                let arg = quote!( #arg_ident : #ty );
                args.push( quote!( #arg, ) );

                // Get the call parameter.
                let call_param = param.for_call( &arg_ident, &arg_ty );
                params.push( quote!( #call_param, ) );
            }

            // Add the [retval] arg if one exists and isn't ().
            if let Some( outs ) = get_out_and_ret( m ) {
                if let ( Some( out_ty ), _ ) = outs {
                    if ! is_unit( &out_ty ) {
                        let param = paramhandlers::get_param_handler(
                                &out_ty );
                        let ty = param.arg_ty( &out_ty );
                        args.push( quote!( __out : *mut #ty ) );
                    }
                }
            } else {
                return None
            };

            // Ensure the first parameter is &self.
            // Static methods don't count here.
            if let &FnArg::SelfRef(..) = self_arg {
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
    tk : &Ty
) -> bool
{
    if let &Ty::Tup( ref v ) = tk {
        if v.len() == 0 {
            return true
        }
    }
    false
}

pub fn get_ret_types(
    ret_ty : &Ty
) -> Result< ( Option<Ty>, Tokens ), &'static str >
{
    // Get the path type on the return value.
    let path = match ret_ty {
        &Ty::Path( _, ref p ) => p,
        _ => return Err( "Use Result as a return type" )
    };

    // Find the last path segment.
    let last_segment = path.segments.last().unwrap();

    // Check the last segment has angle bracketed parameters.
    if let PathParameters::AngleBracketed( ref data ) = last_segment.parameters {
        if data.types.len() > 0 {
            // Angle bracketed parameters exist. We're assuming this is
            // some kind of Result<ok> or Result<ok, err>. In either case
            // we can take the first parameter as the 'ok' type.
            //
            // TODO: Figure out whether we can ask the compiler whether
            // the type matches Result<S,E> type.
            return Ok( (
                data.types.first().and_then( |x| Some( x.clone() ) ),
                quote!( com_runtime::HRESULT )
            ) )
        }
    }

    // Default value. We get here only if we didn't return a type from
    // the if statements above.
    Ok( ( None, quote!( #path ) ) )
}

pub fn get_out_and_ret(
    m : &MethodSig
) -> Option< ( Option<Ty>, Tokens ) >
{
    let output = &m.decl.output;
    let result_ty = match output {
        &FunctionRetTy::Ty( ref ty ) => ty,
        _ => return None
    };

    get_ret_types( &result_ty ).ok()
}

pub fn get_method_rvalues(
    m : &MethodSig
) -> Option< ( Tokens, Tokens ) >
{
    let ( out_ty, ret_ty ) = match get_out_and_ret( m ) {
        Some( s ) => s,
        None => return None,
    };

    Some( match out_ty {
        // Result<(), _>. Ignore the [retval] value but handle the Err
        // as the method return value.
        Some( ref unit ) if is_unit( &unit ) => (
            ret_ty,
            quote!(
                match result {
                    Ok( _ ) => com_runtime::S_OK,
                    Err( e ) => e
                } ) ),

        // Result<_, _>. Ok() -> __out + S_OK, Err() -> E_*
        Some( ref out_ty ) => {
            let param = paramhandlers::get_param_handler( &out_ty );
            let out_ident = Ident::from( "__out" );
            let write_out = param.write_out( &out_ident, &out_ty );
            let write_null = param.write_null( &out_ident, &out_ty );
            (
                ret_ty,
                quote!(
                    match result {
                        Ok( r ) => { #write_out; com_runtime::S_OK },
                        Err( e ) => { #write_null; e },
                    } ) )
        },

        // Not a Result<..>, assume we can return the return value as is.
        None => (
            ret_ty, quote!( return result ) ),
    } )
}

fn arg_to_ident(
    arg : &FnArg,
) -> Result<Ident, MacroError> {
    Ok( match arg {
        &FnArg::SelfRef(..) | &FnArg::SelfValue(..)
            => Ident::from( "self" ),
        &FnArg::Captured( ref pat, _ ) => match pat {
            &Pat::Ident( _, ref i, _ ) => i.clone(),
            _ => Err( format!( "Unsupported argument: {:?}", arg ) )?,
        },
        &FnArg::Ignored(..) => Ident::from( "_" ),
    } )
}

fn arg_to_ty(
    arg : &FnArg,
) -> Ty
{
    match arg {
        &FnArg::Captured( _, ref ty )
            | &FnArg::Ignored( ref ty )
            => ty.clone(),
        &FnArg::SelfRef( ref life, m )
            => Ty::Rptr( life.clone(), Box::new( MutTy {
                mutability: m,
                ty: self_ty()
            } ) ),
        &FnArg::SelfValue( m )
            => self_ty(),
    }
}

fn self_ty() -> Ty {
    Ty::Path(
        None,
        Path::from( PathSegment::from( Ident::from( "Self" ) ) )
    )
}

pub fn parameter_to_ident(
    p : &NestedMetaItem
) -> Option<&Ident>
{
    match p {
        &NestedMetaItem::MetaItem( ref mi ) => Some( match mi {
            &MetaItem::Word( ref i ) => i,
            &MetaItem::List( ref i, .. ) => i,
            &MetaItem::NameValue( ref i, .. ) => i,
        } ),
        _ => None
    }
}

pub fn get_guid_tokens(
    g : &com_runtime::GUID
) -> Tokens
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
    quote!( 
        com_runtime::GUID {
            data1: #d1, data2: #d2, data3: #d3,
            data4: [ #d4_0, #d4_1, #d4_2, #d4_3, #d4_4, #d4_5, #d4_6, #d4_7 ]
        }
    )
}
