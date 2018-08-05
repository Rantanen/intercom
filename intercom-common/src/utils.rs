
use prelude::*;
use tyhandlers::TypeSystem;
use syn::*;
use syn::synom::Parser;

use error::MacroError;
use super::*;

use ast_converters::*;

pub fn parse_attr_tokens(
    attr_name: &str,
    attr_tokens: &str,
) -> Result< Attribute, MacroError >
{
    let attr_rendered = format!( "#[{}({})]", attr_name, attr_tokens );
    if let Ok( tt ) = attr_rendered.parse() {
        if let Ok( t ) = Attribute::parse_outer.parse2( tt ) {
            return Ok( t )
        }
    }

    Err( MacroError {
        msg: format!( "Could not parse [{}] attribute", attr_name )
    } )
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum InterfaceType { Trait, Struct }

pub type InterfaceData<'a> = (
    Ident,
    Vec< &'a MethodSig >,
    InterfaceType,
    Option<Token!(unsafe)>,
);

pub fn get_ident_and_fns(
    item : &Item
) -> Option< InterfaceData >
{
    match *item {
        Item::Impl( ItemImpl {
                ref unsafety,
                ref trait_,
                ref self_ty,
                ref items,
                .. } )
            => {
                let ( _, struct_ident, items ) =
                        get_impl_data_raw( trait_, self_ty, items );
                Some( (
                        struct_ident,
                        items,
                        InterfaceType::Struct,
                        *unsafety ) )
            },
        Item::Trait( ItemTrait {
                ref ident,
                unsafety,
                ref items,
                .. } )
            => {

            let methods : Option< Vec< &MethodSig > > = items
                    .into_iter()
                    .map( |i| get_trait_method( i ) )
                    .collect();

            match methods {
                Some( m ) => Some( (
                        ident.clone(),
                        m,
                        InterfaceType::Trait,
                        unsafety,
                    ) ),
                None => None
            }
        },
        _ => None
    }
}

pub type ImplData<'a> = (
    Option<Ident>,
    Ident,
    Vec< &'a MethodSig >
);

pub fn get_impl_data(
    item : &Item
) -> Option< ImplData >
{
    if let Item::Impl( ItemImpl { ref trait_, ref self_ty, ref items, .. } ) = *item {
        return Some( get_impl_data_raw( trait_, self_ty, items ) );
    }
    None
}

fn get_impl_data_raw<'a>(
    trait_ref : &'a Option<( Option<Token!(!)>, Path, Token!(for) )>,
    struct_ty : &'a Type,
    items : &'a [ImplItem]
) -> ImplData<'a>
{

    let struct_ident = struct_ty.get_ident().unwrap();
    let trait_ident = match *trait_ref {
        Some( ( _, ref path, _ ) ) => path.get_ident().ok(),
        None => None
    };

    let methods_opt : Option< Vec< &MethodSig > > = items
            .into_iter()
            .map( |i| get_impl_method( i ) )
            .collect();
    let methods = methods_opt.unwrap_or_else( || vec![] );

    ( trait_ident, struct_ident, methods )
}

#[derive(PartialEq, Eq, Debug)]
pub enum AttrParam {
    Literal( Lit ),
    Word( Ident ),
}

pub fn iter_parameters(
    attr : &syn::Attribute
) -> Box< Iterator<Item = AttrParam > >
{
    match attr.interpret_meta() {

        Some( Meta::List( MetaList { ref nested, .. } ) ) =>
            Box::new( nested.to_owned().into_iter().map( |i| {

                match i {

                    NestedMeta::Meta( meta ) =>
                        AttrParam::Word( match meta {
                            Meta::Word( i ) => i,
                            Meta::List( l ) => l.ident,
                            Meta::NameValue( nv ) => nv.ident,
                        } ),

                    syn::NestedMeta::Literal( l ) =>
                            AttrParam::Literal( l ),
                }
            } ) ),

        // Attributes without parameter lists don't have params.
        None
            | Some( Meta::Word(..) )
            | Some( Meta::NameValue(..) ) => Box::new( std::iter::empty() ),

    }
}

pub fn get_impl_method(
    i : &ImplItem
) -> Option< &MethodSig >
{
    match *i {
        ImplItem::Method( ref itm ) => Some( &itm.sig ),
        _ => None
    }
}

pub fn get_trait_method(
    i : &TraitItem
) -> Option< &MethodSig >
{
    match *i {
        TraitItem::Method( ref tim ) => Some( &tim.sig ),
        _ => None
    }
}

const AUTO_GUID_BASE : guid::GUID = guid::GUID {
    data1: 0x4449_494C,
    data2: 0xDE1F,
    data3: 0x4525,
    data4: [ 0xB9, 0x57, 0x89, 0xD6, 0x0C, 0xE9, 0x34, 0x77 ]
};

/*
pub fn parameter_to_guid(
    p : &AttrParam,
    crate_name : &str,
    item_name : &str,
    item_type : &str,
) -> Result< Option< guid::GUID >, String >
{
    if let AttrParam::Word( ref i ) = *p {
        return Ok( match i.to_string().as_ref() {
            "AUTO_GUID" =>
                Some( generate_guid( crate_name, item_name, item_type ) ),
            "NO_GUID" =>
                None,
            _ => return Err( format!( "Invalid GUID: {:?}", i ) ),
        } );
    }

    if let AttrParam::Literal( Lit::Str( ref s ) ) = *p {
        return Ok( Some( guid::GUID::parse( &s.value() )? ) );
    }

    Err( "GUID parameter must be literal string".to_owned() )
}
*/

pub fn generate_iid(
    crate_name : &str,
    item_name : &str,
    type_system : TypeSystem,
) -> guid::GUID
{
    generate_guid( &[
            "IID",
            crate_name,
            item_name,
            match type_system {
                TypeSystem::Automation => "automation",
                TypeSystem::Raw => "raw",
                TypeSystem::Invariant => "invariant",
            }
        ].join( ":" ) )
}

pub fn generate_libid(
    crate_name : &str,
) -> guid::GUID
{
    generate_guid( &[
            "LIBID",
            crate_name,
        ].join( ":" ) )
}

pub fn generate_clsid(
    crate_name : &str,
    item_name : &str,
) -> guid::GUID
{
    generate_guid( &[
            "CLSID",
            crate_name,
            item_name,
        ].join( ":" ) )
}

pub fn generate_guid(
    key : &str,
) -> guid::GUID
{
    // Hash the name. The name will be hashed in a form similar to:
    // AUTO_GUID_BASE + "CLSID:random_rust_crate:FooBar"
    let mut hash = sha1::Sha1::new();
    hash.update( AUTO_GUID_BASE.as_bytes() );
    hash.update( key.as_bytes() );

    let digest = hash.digest();
    let bytes = digest.bytes();

    // Set the GUID bytes according to RFC-4122, section 4.3.
    let time_low : u32
        = ( u32::from( bytes[0] ) << 24 )
        + ( u32::from( bytes[1] ) << 16 )
        + ( u32::from( bytes[2] ) << 8 )
        + u32::from( bytes[3] );
    let time_mid : u16
        = ( u16::from( bytes[4] ) << 8 )
        + ( u16::from( bytes[5] ) );
    let time_hi_and_version : u16
        = (
            (
                ( u16::from( bytes[6] ) << 8 ) + u16::from( bytes[7] )
            ) & 0x0fff
        ) | 0x3000;
    let clk_seq_hi_res : u8
        = ( bytes[8] & 0b0011_1111 ) | 0b0100_0000;
    let clk_seq_low : u8 = bytes[9];

    guid::GUID {
        data1: time_low,
        data2: time_mid,
        data3: time_hi_and_version,
        data4: [
            clk_seq_hi_res, clk_seq_low,
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15] ]
    }
}

pub fn ty_to_string( ty : &syn::Type ) -> String
{
    quote!( #ty ).to_string().replace( " ", "" ).replace( ",", ", " )
}

pub fn is_unit(
    tk : &Type
) -> bool
{
    if let Type::Tuple( ref t ) = *tk {
        t.elems.is_empty()
    } else {
        false
    }
}

pub fn unit_ty() -> Type
{
    parse_quote!( () )
}

pub fn get_guid_tokens(
    g : &guid::GUID
) -> TokenStream
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
        ::intercom::GUID {
            data1: #d1, data2: #d2, data3: #d3,
            data4: [ #d4_0, #d4_1, #d4_2, #d4_3, #d4_4, #d4_5, #d4_6, #d4_7 ]
        }
    )
}

/// Convert the Rust identifier from `snake_case` to `PascalCase`
pub fn pascal_case<T: AsRef<str>>( input : T ) -> String {
    let input = input.as_ref();

    // Allocate the output string. We'll never increase the amount of
    // characters so we can reserve string buffer using the input string length.
    let mut output = String::new();
    output.reserve( input.len() );

    // Process each character from the input.
    let mut capitalize = true;
    for c in input.chars() {

        // Check the capitalization requirement.
        if c == '_' {

            // Skip '_' but capitalize the following character.
            capitalize = true;

        } else if capitalize {

            // Capitalize. Add the uppercase characters.
            for c_up in c.to_uppercase() {
                output.push( c_up )
            }

            // No need to capitalize any more.
            capitalize = false;

        } else {

            // No need to capitalize. Just add the character as is.
            output.push( c );
        }

    }
    output
}

#[cfg(test)]
mod test
{
    use super::*;

    /// Tests the `ty_to_string` by converting parameter to Type and back to
    /// String to ensure they equal.
    fn test_ty( ty_str : &str ) {
        let ty = parse_str( ty_str ).unwrap();
        let as_string = ty_to_string( &ty );
        assert_eq!( ty_str, as_string );
    }

    #[test] fn path_to_test() { test_ty( "::path::Foo" ) }
    #[test] fn generics_to_test() { test_ty( "Result<Foo, Bar>" ) }
    #[test] fn unit_to_test() { test_ty( "()" ) }
}
