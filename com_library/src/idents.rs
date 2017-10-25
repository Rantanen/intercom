
use syntax::ast::Ident;

pub fn coclass(
    struct_name: &Ident
) -> Ident
{
    Ident::from_str( &format!( "__{}CoClass", struct_name ) )
}

pub fn clsid(
    struct_name: &Ident
) -> Ident
{
    Ident::from_str( &format!( "CLSID_{}", struct_name ) )
}

pub fn iid(
    itf_name: &Ident
) -> Ident
{
    Ident::from_str( &format!( "IID_{}", itf_name ) )
}

pub fn method_impl(
    struct_ident : &Ident,
    itf_ident : &Ident,
    method_name: &str
) -> Ident
{
    Ident::from_str( &format!( "__{}_{}_{}",
            struct_ident.name, itf_ident.name, method_name ) )
}

pub fn vtable_struct(
    itf_ident : &Ident
) -> Ident
{
    Ident::from_str( &format!( "__{}Vtbl", itf_ident.name ).as_str() )
}

pub fn vtable_instance(
    struct_name : &Ident,
    itf_ident : &Ident,
) -> Ident
{
    Ident::from_str( &format!( "__{}_{}Vtbl_INSTANCE",
                        struct_name.name,
                        itf_ident.name ).as_str() )
}

pub fn vtable_list(
    struct_ident : &Ident
) -> Ident
{
    Ident::from_str( &format!( "__{}VtblList", struct_ident.name ).as_str() )
}

pub fn vtable_list_instance(
    struct_ident : &Ident
) -> Ident
{
    Ident::from_str( &format!( "__{}VtblList_INSTANCE", struct_ident.name ).as_str() )
}

pub fn vtable_offset(
    s : &Ident,
    i : &Ident
) -> Ident
{
    Ident::from_str( &format!( "__{}_{}Vtbl_offset", s.name, i.name ) )
}

