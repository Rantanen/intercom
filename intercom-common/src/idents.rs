
use syn::*;

pub fn clsid(
    struct_name: Ident
) -> Ident
{
    Ident::from( format!( "CLSID_{}", struct_name ) )
}

pub fn iid(
    itf_name: Ident
) -> Ident
{
    Ident::from( format!( "IID_{}", itf_name ) )
}

pub fn method_impl(
    struct_ident : Ident,
    itf_ident : Ident,
    method_name: &str
) -> Ident
{
    Ident::from( format!( "__{}_{}_{}",
            struct_ident, itf_ident, method_name ) )
}

pub fn vtable_struct(
    itf_ident : Ident
) -> Ident
{
    Ident::from( format!( "__{}Vtbl", itf_ident ) )
}

pub fn vtable_instance(
    struct_name : Ident,
    itf_ident : Ident,
) -> Ident
{
    Ident::from( format!( "__{}_{}Vtbl_INSTANCE",
                        struct_name,
                        itf_ident ) )
}

pub fn vtable_list(
    struct_ident : Ident
) -> Ident
{
    Ident::from( format!( "__{}VtblList", struct_ident ) )
}

pub fn vtable_offset(
    s : Ident,
    i : Ident
) -> Ident
{
    Ident::from( format!( "__{}_{}Vtbl_offset", s, i ) )
}

