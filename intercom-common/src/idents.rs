
use prelude::*;
use syn::*;
use tyhandlers::TypeSystem;

pub fn with_ts(
    ident: &Ident,
    ts: TypeSystem,
) -> Ident
{
    Ident::new( &format!( "{}_{:?}", ident, ts ), Span::call_site() )
}

pub fn clsid(
    struct_name: &Ident
) -> Ident
{
    new_ident( &format!( "CLSID_{}", struct_name ) )
}

pub fn iid(
    itf_name: &Ident
) -> Ident
{
    new_ident( &format!( "IID_{}", itf_name ) )
}

pub fn method_impl(
    struct_ident : &Ident,
    itf_ident : &Ident,
    method_name: &str
) -> Ident
{
    new_ident( &format!( "__{}_{}_{}",
            struct_ident, itf_ident, method_name ) )
}

pub fn vtable_struct(
    itf_ident : &Ident
) -> Ident
{
    new_ident( &format!( "__{}Vtbl", itf_ident ) )
}

pub fn vtable_instance(
    struct_name : &Ident,
    itf_ident : &Ident,
) -> Ident
{
    new_ident( &format!( "__{}_{}Vtbl_INSTANCE",
                        struct_name,
                        itf_ident ) )
}

pub fn vtable_list(
    struct_ident : &Ident
) -> Ident
{
    new_ident( &format!( "__{}VtblList", struct_ident ) )
}

pub fn vtable_offset(
    s : &Ident,
    i : &Ident
) -> Ident
{
    new_ident( &format!( "__{}_{}Vtbl_offset", s, i ) )
}


fn new_ident( s : &str ) -> Ident {
    Ident::new( s, Span::call_site() )
}
