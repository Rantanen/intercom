
use ::prelude::*;
use std::rc::Rc;
use syn::*;

use ast_converters::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction { In, Out, Retval }

pub struct TypeConversion {

    /// Possible temporary values that need to be kept alive for the duration
    /// of the conversion result usage.
    pub temporary: Option<TokenStream>,

    /// Conversion result value. Possibly referencing the temporary value.
    pub value : TokenStream,
}

#[derive(PartialEq, Eq, Debug)]
pub struct ModelTypeSystemConfig {
    pub effective_system : ModelTypeSystem,
    pub is_default : bool,
}

impl ModelTypeSystemConfig {
    pub fn get_unique_name( &self, base : &str ) -> String {
        match self.is_default {
            true => base.to_string(),
            false => format!( "{}_{:?}", base, self.effective_system ),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum ModelTypeSystem {

    /// COM Automation compatible type system.
    Automation,

    /// Raw type system.
    Raw,
}

impl ModelTypeSystem {

    /// Converts the model type system into public type system tokens.
    pub fn as_typesystem_tokens( &self ) -> TokenStream {
        match self {
            ModelTypeSystem::Automation =>
                    quote!( ::intercom::TypeSystem::Automation ),
            ModelTypeSystem::Raw =>
                    quote!( ::intercom::TypeSystem::Raw ),
        }
    }
}

/// Type usage context.
pub struct TypeContext {
    dir: Direction,
    type_system: ModelTypeSystem,
}

impl TypeContext {
    pub fn new( dir : Direction, type_system : ModelTypeSystem ) -> TypeContext {
        TypeContext { dir, type_system }
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
        &self, ident : &Ident
    ) -> TypeConversion
    {
        TypeConversion {
            temporary: None,
            value: quote!( #ident.into() ),
        }
    }

    /// Converts a Rust parameter named by the ident into a COM type.
    fn rust_to_com(
        &self, ident : &Ident
    ) -> TypeConversion
    {
        TypeConversion {
            temporary: None,
            value: quote!( #ident.into() )
        }
    }

    /// Gets the default value for the type.
    fn default_value( &self ) -> TokenStream
    {
        match self.rust_ty() {
            Type::Path( ref p ) => {
                let ident = p.path.get_ident().unwrap();
                let name = ident.to_string();
                match name.as_ref() {
                    "c_void"
                        | "RawComPtr"
                        => quote!( ::std::ptr::null_mut() ),
                    _ => quote!( Default::default() )
                }
            },
            _ => quote!( Default::default() )
        }
    }

    /// Gets the sype system the handler serves if the handler is type system specific. Returns
    /// None if the handler is type system agnostic.
    fn type_system( &self ) -> Option<ModelTypeSystem> { None }
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
struct ComItfParam { ty: Type, context: TypeContext }

impl TypeHandler for ComItfParam {

    fn rust_ty( &self ) -> Type { self.ty.clone() }

    /// The COM type.
    fn com_ty( &self ) -> Type
    {
        parse_quote!( ::intercom::RawComPtr )
    }

    fn default_value( &self ) -> TokenStream
    {
        quote!( ::std::ptr::null_mut() )
    }

    /// Converts a COM parameter named by the ident into a Rust type.
    fn com_to_rust(
        &self, ident : &Ident
    ) -> TypeConversion
    {
        let ts = self.context.type_system.as_typesystem_tokens();
        TypeConversion {
            temporary: None,
            value: quote!( ::intercom::ComItf::wrap( #ident, #ts ) ),
        }
    }

    /// Converts a Rust parameter named by the ident into a COM type.
    fn rust_to_com(
        &self, ident : &Ident
    ) -> TypeConversion
    {
        let ts = self.context.type_system.as_typesystem_tokens();
        TypeConversion {
            temporary: None,
            value: quote!( ::intercom::ComItf::ptr( #ident.into(), #ts ) )
        }
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

    fn com_to_rust( &self, ident : &Ident ) -> TypeConversion
    {
        match self.context.dir {

            Direction::In => {

                let target_ty = self.rust_ty();
                let intermediate_ty = quote!( &::intercom::BStr );
                let to_intermediate = quote!( ::intercom::BStr::from_ptr( #ident ) );
                let as_trait = quote!( < #target_ty as ::intercom::FromWithTemporary< #intermediate_ty > > );

                let temp_ident = Ident::new( &format!( "__{}_temporary", ident.to_string() ), Span::call_site() );
                TypeConversion {
                    temporary: Some( quote!( let mut #temp_ident = #as_trait::to_temporary( #to_intermediate )?; ) ),
                    value: quote!( #as_trait::from_temporary( &mut #temp_ident )? ),
                }
            },
            Direction::Out | Direction::Retval => {
                TypeConversion {
                    temporary: None,
                    value: quote!( ::intercom::BString::from_ptr( #ident ).com_into()? ),
                }
            },
        }
    }

    fn rust_to_com( &self, ident : &Ident ) -> TypeConversion
    {
        match self.context.dir {

            Direction::In => {

                let target_ty = self.rust_ty();
                let intermediate_ty = quote!( &::intercom::BStr );
                let as_trait = quote!( < #intermediate_ty as ::intercom::FromWithTemporary< #target_ty > > );

                let temp_ident = Ident::new( &format!( "__{}_temporary", ident.to_string() ), Span::call_site() );
                TypeConversion {
                    temporary: Some( quote!( let mut #temp_ident = #as_trait::to_temporary( #ident )?; ) ),
                    value: quote!( #as_trait::from_temporary( &mut #temp_ident )?.as_ptr() ),
                }
            },
            Direction::Out | Direction::Retval => {
                TypeConversion {
                    temporary: None,
                    value: quote!( ::intercom::BString::from( #ident ).into_ptr() ),
                }
            },
        }
    }

    fn default_value( &self ) -> TokenStream
    {
        quote!( ::std::ptr::null_mut() )
    }

    /// String parameters differ between the type systems.
    fn type_system( &self ) -> Option<ModelTypeSystem> {
        Some( self.context.type_system )
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

        "ComItf" => Rc::new( ComItfParam { ty: original_type, context } ),
        "BString" | "BStr" | "String" | "str" =>
            Rc::new( StringParam { ty: original_type, context } ),
        // "str" => Rc::new( StringRefParam( original_type ) ),

        // Unknown. Use IdentityParam.
        _ => Rc::new( IdentityParam( original_type ) )
    }

}
