
use syn::*;
use quote::{ToTokens, Tokens};

use error::MacroError;
use super::*;

use ast_converters::*;

pub fn parse_attr_tokens(
    attr_name: &str,
    attr_tokens: &str,
) -> Result< Attribute, MacroError >
{
    let attr_rendered = format!( "#[{}{}]", attr_name, attr_tokens );
    Ok( match syn::parse_outer_attr( &attr_rendered ) {
        Ok(t) => t,
        Err(_) => Err(
                format!( "Could not parse [{}] attribute", attr_name ) )?,
    } )
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum InterfaceType { Trait, Struct }

pub type InterfaceData<'a> = (
    Ident,
    Vec< ( &'a Ident, &'a MethodSig ) >,
    InterfaceType,
    syn::Unsafety,
);

pub fn get_ident_and_fns(
    item : &Item
) -> Option< InterfaceData >
{
    match item.node {
        ItemKind::Impl( unsafety, .., ref trait_ref, ref ty, ref items ) => {
            let ( _, struct_ident, items ) =
                    get_impl_data_raw( trait_ref, ty, items );
            Some( ( struct_ident, items, InterfaceType::Struct, unsafety ) )
        },
        ItemKind::Trait( unsafety, .., ref items ) => {

            let methods : Option< Vec< (&Ident, &MethodSig) > > = items
                    .into_iter()
                    .map( |i| get_trait_method( i ).map( |m| ( &i.ident, m ) ) )
                    .collect();

            match methods {
                Some( m ) => Some( (
                        item.ident.clone(),
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
    Vec< ( &'a Ident, &'a MethodSig ) >
);

pub fn get_impl_data(
    item : &Item
) -> Option< ImplData >
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
) -> ImplData<'a>
{

    let struct_ident = struct_ty.get_ident().unwrap();
    let trait_ident = match *trait_ref {
        Some( ref tr ) => tr.get_ident().ok(),
        None => None
    };

    let methods_opt : Option< Vec< (&Ident, &MethodSig) > > = items
            .into_iter()
            .map( |i| get_impl_method( i ).map( |m| ( &i.ident, m ) ) )
            .collect();
    let methods = methods_opt.unwrap_or_else( || vec![] );

    ( trait_ident, struct_ident, methods )
}

#[derive(PartialEq, Eq, Debug)]
pub enum AttrParam {
    Literal( syn::Lit ),
    Word( syn::Ident ),
}

pub fn iter_parameters(
    attr : &syn::Attribute
) -> Box< Iterator<Item = AttrParam > >
{
    match attr.value {

        // Attributes without parameter lists don't have params.
        syn::MetaItem::Word(..)
            | syn::MetaItem::NameValue(..) => Box::new( std::iter::empty() ),

        syn::MetaItem::List( _, ref l ) =>
            Box::new( l.to_owned().into_iter().map( |i| {

                match i {

                    syn::NestedMetaItem::MetaItem( mi ) =>

                            AttrParam::Word( match mi {
                                syn::MetaItem::Word( i )
                                    | syn::MetaItem::List( i, _ )
                                    | syn::MetaItem::NameValue( i, _ )
                                    => i,
                            } ),

                    syn::NestedMetaItem::Literal( l ) =>
                            AttrParam::Literal( l ),
                }
            } ) ),
    }
}

pub fn get_ty_ident(
    ty : &Ty
) -> Option<&Ident>
{
    match *ty {
        Ty::Path( _, ref p ) =>
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

const AUTO_GUID_BASE : guid::GUID = guid::GUID {
    data1: 0x4449_494C,
    data2: 0xDE1F,
    data3: 0x4525,
    data4: [ 0xB9, 0x57, 0x89, 0xD6, 0x0C, 0xE9, 0x34, 0x77 ]
};

pub fn parameter_to_guid(
    p : &AttrParam,
    crate_name : &str,
    item_name : &str,
    item_type : &str,
) -> Result< Option< guid::GUID >, String >
{
    if let AttrParam::Word( ref i ) = *p {
        return Ok( match i.as_ref() {
            "AUTO_GUID" =>
                Some( generate_guid( crate_name, item_name, item_type ) ),
            "NO_GUID" =>
                None,
            _ => return Err( format!( "Invalid GUID: {:?}", i ) ),
        } );
    }

    if let AttrParam::Literal( Lit::Str( ref s, _ ) ) = *p {
        return Ok( Some( guid::GUID::parse( s.as_str() )? ) );
    }

    Err( "GUID parameter must be literal string".to_owned() )
}

pub fn generate_guid(
    crate_name : &str,
    item_name : &str,
    item_type : &str,
) -> guid::GUID
{
    // Hash the name. The name will be hashed in a form similar to:
    // AUTO_GUID_BASE + "CLSID:random_rust_crate:FooBar"
    let mut hash = sha1::Sha1::new();
    hash.update( AUTO_GUID_BASE.as_bytes() );
    hash.update( item_type.as_bytes() );
    hash.update( b":" );
    hash.update( crate_name.as_bytes() );
    hash.update( b":" );
    hash.update( item_name.as_bytes() );

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

pub fn ty_to_string( ty : &syn::Ty ) -> String
{
    let mut tokens = Tokens::new();
    ty.to_tokens( &mut tokens );
    tokens.to_string().replace( " ", "" ).replace( ",", ", " )
}

pub fn is_unit(
    tk : &Ty
) -> bool
{
    if let Ty::Tup( ref v ) = *tk {
        v.is_empty()
    } else {
        false
    }
}

pub fn unit_ty() -> Ty
{
    Ty::Tup( vec![] )
}

pub fn get_guid_tokens(
    g : &guid::GUID
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
        ::intercom::GUID {
            data1: #d1, data2: #d2, data3: #d3,
            data4: [ #d4_0, #d4_1, #d4_2, #d4_3, #d4_4, #d4_5, #d4_6, #d4_7 ]
        }
    )
}

/// Convert the Rust identifier from `snake_case` to `PascalCase`
pub fn pascal_case( input : &str ) -> String {

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

    /// Tests the `ty_to_string` by converting parameter to Ty and back to
    /// String to ensure they equal.
    fn test_ty( ty_str : &str ) {
        let ty = parse_type( ty_str ).unwrap();
        let as_string = ty_to_string( &ty );
        assert_eq!( ty_str, as_string );
    }

    #[test] fn path_to_test() { test_ty( "::path::Foo" ) }
    #[test] fn generics_to_test() { test_ty( "Result<Foo, Bar>" ) }
    #[test] fn unit_to_test() { test_ty( "()" ) }
}
