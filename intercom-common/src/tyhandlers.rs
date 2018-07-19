
use std::rc::Rc;
use syn::*;
use quote::Tokens;

use methodinfo::Direction;

use ast_converters::*;

/// Type usage context.
pub struct TypeContext {
    dir: Direction,
}

impl TypeContext {
    pub fn new( dir : Direction ) -> TypeContext {
        TypeContext { dir }
    }

    pub fn input() -> TypeContext {
        TypeContext { dir: Direction::In }
    }

    pub fn output() -> TypeContext {
        TypeContext { dir: Direction::Out }
    }

    pub fn retval() -> TypeContext {
        TypeContext { dir: Direction::Retval }
    }
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
    ) -> Tokens
    {
        quote!( #ident.into() )
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
struct StringParam { ty: Type, context: TypeContext }
impl TypeHandler for StringParam
{
    fn rust_ty( &self ) -> Type { self.ty.clone() }

    fn com_ty( &self ) -> Type
    {
        match self.context.dir {
            Direction::In => parse_quote!( ::intercom::raw::InBSTR ),
            Direction::Out | Direction::Retval => parse_quote!( ::intercom::raw::OutBSTR ),
        }
    }

    fn com_to_rust( &self, ident : Ident ) -> Tokens
    {
        match self.context.dir {

            Direction::In => {

                let rust_ty = self.rust_ty();
                quote!(
                    < #rust_ty as ::intercom::FromWithTemporary<&::intercom::BStr> >::from_temporary(
                        &mut < #rust_ty as ::intercom::FromWithTemporary<&::intercom::BStr> >::to_temporary(
                            ::intercom::BStr::from_ptr( #ident ) )? )?
                )
            },
            Direction::Out | Direction::Retval => {
                quote!( ::intercom::BString::from_ptr( #ident ).com_into()? )
            },
        }
    }

    fn rust_to_com( &self, ident : Ident ) -> Tokens
    {
        match self.context.dir {

            Direction::In => {

                let rust_ty = self.rust_ty();
                quote!(
                    < &::intercom::BStr as ::intercom::FromWithTemporary<#rust_ty> >::from_temporary(
                        &mut < &::intercom::BStr as ::intercom::FromWithTemporary<#rust_ty> >::to_temporary( #ident ) ?
                    )?.as_ptr()
                )
            },
            Direction::Out | Direction::Retval => {
                quote!( ::intercom::BString::from( #ident ).into_ptr() )
            },
        }
    }

    /// Gets the default value for the type.
    fn default_value( &self ) -> Tokens
    {
        quote!( ::std::ptr::null_mut() )
    }
}

/// Resolves the `TypeHandler` to use.
pub fn get_ty_handler(
    arg_ty : &Type,
    context : TypeContext,
) -> Rc<TypeHandler>
{
    let type_info = ::type_parser::parse( arg_ty )
            .unwrap_or_else( || panic!( "Type {:?} could not be parsed.", arg_ty ) );

    map_by_name(
            type_info.get_name().as_ref(), type_info.original.clone(),
            context )
}

/// Selects type handler based on the name of the type.
fn map_by_name(
    name: &str,
    original_type: Type,
    context: TypeContext,
) -> Rc<TypeHandler> {

    match name {

        "ComItf" => Rc::new( ComItfParam( original_type ) ),
        "BString" | "BStr" | "String" | "str" =>
            Rc::new( StringParam { ty: original_type, context } ),
        // "str" => Rc::new( StringRefParam( original_type ) ),

        // Unknown. Use IdentityParam.
        _ => Rc::new( IdentityParam( original_type ) )
    }

}
