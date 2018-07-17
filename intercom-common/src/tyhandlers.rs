
use std::rc::Rc;
use syn::*;
use quote::Tokens;

use ast_converters::*;

/// Defines tokens for converting COM types into Rust types
pub struct ComToRust
{
    /// Optional expression for storing a temporary value in the stack
    /// for the duration of the Rust call.
    pub stack: Option<Tokens>,

    /// Expression that converts the COM type into Rust type.
    pub conversion: Tokens
}

/// Defines Type-specific logic for handling the various parameter types in the
/// Rust/COM interface.
pub trait TypeHandler {

    /// The Rust type.
    fn rust_ty( &self ) -> Type;

    /// The COM type.
    fn com_ty( &self ) -> Type
    {
        self.rust_ty()
    }

    /// Converts a COM parameter named by the ident into a Rust type.
    fn com_to_rust(
        &self, ident : Ident
    ) -> ComToRust
    {
        ComToRust {
            stack: None,
            conversion: quote!( #ident.into() )
        }
    }

    /// Converts a Rust parameter named by the ident into a COM type.
    fn rust_to_com(
        &self, ident : Ident
    ) -> Tokens
    {
        quote!( #ident.into() )
    }

    /// Gets the default value for the type.
    fn default_value( &self ) -> Tokens
    {
        match self.rust_ty() {
            Type::Path( ref p ) => {
                let ident = p.path.get_ident().unwrap();
                let name = ident.as_ref();
                match name {
                    "c_void"
                        | "RawComPtr"
                        => quote!( ::std::ptr::null_mut() ),
                    _ => quote!( Default::default() )
                }
            },
            _ => quote!( Default::default() )
        }
    }
}

/// Identity parameter handler.
///
/// No special logic.
struct IdentityParam( Type );

impl TypeHandler for IdentityParam {
    fn rust_ty( &self ) -> Type { self.0.clone() }
}


/// `ComItf` parameter handler. Supports `ComItf` Rust type and ensures the this
/// to/from `RawComPtr` COM type.
struct ComItfParam( Type );

impl TypeHandler for ComItfParam {

    fn rust_ty( &self ) -> Type { self.0.clone() }

    /// Gets the default value for the type.
    fn default_value( &self ) -> Tokens
    {
        quote!( ComItf::null_itf() )
    }
}

/// String parameter handler. Converts between Rust String and COM BSTR types.
struct StringParam( Type );
impl TypeHandler for StringParam
{
    fn rust_ty( &self ) -> Type { self.0.clone() }

    fn com_ty( &self ) -> Type
    {
        parse_quote!( ::intercom::BStr )
    }

    fn com_to_rust( &self, ident : Ident ) -> ComToRust
    {
        ComToRust {
            stack: None,
            conversion: quote!( #ident.into() )
        }
    }

    fn rust_to_com( &self, ident : Ident ) -> Tokens
    {
        quote!( #ident.into() )
    }
}

/// String parameter handler. Converts between Rust &str and COM BSTR types.
struct StringRefParam( Type );
impl TypeHandler for StringRefParam
{
    fn rust_ty( &self ) -> Type { self.0.clone() }

    fn com_ty( &self ) -> Type
    {
        parse_quote!( ::intercom::BStr )
    }

    fn com_to_rust( &self, ident : Ident ) -> ComToRust
    {
        // Generate unique name for each stack variable to avoid conflicts with function
        // thay may have multiple parameters.
        let as_string_ident = Ident::from( format!( "{}_as_string", ident ) );
        ComToRust {
            stack: Some( quote!( let #as_string_ident: String = #ident.into(); ) ),
            conversion: quote!( #as_string_ident.as_ref() )
        }
    }

    fn rust_to_com( &self, ident : Ident ) -> Tokens
    {
        quote!( #ident.into() )
    }
}

/// Resolves the `TypeHandler` to use.
pub fn get_ty_handler(
    arg_ty : &Type,
) -> Rc<TypeHandler>
{
    let type_info = ::type_parser::parse( arg_ty )
            .unwrap_or_else( || panic!( "Type {:?} could not be parsed.", arg_ty ) );

    map_by_name( type_info.get_name().as_ref(), type_info.original.clone() )
}

/// Selects type handler based on the name of the type.
fn map_by_name(
    name: &str,
    original_type: Type
) -> Rc<TypeHandler> {

    match name {

        "ComItf" => Rc::new( ComItfParam( original_type ) ),
        "String" => Rc::new( StringParam( original_type ) ),
        "str" => Rc::new( StringRefParam( original_type ) ),

        // Unknown. Use IdentityParam.
        _ => Rc::new( IdentityParam( original_type ) )
    }

}
