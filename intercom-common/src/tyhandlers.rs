
use crate::prelude::*;
use std::rc::Rc;
use syn::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction { In, Out, Retval }

pub struct TypeConversion {

    /// Possible temporary values that need to be kept alive for the duration
    /// of the conversion result usage.
    pub temporary: Option<TokenStream>,

    /// Conversion result value. Possibly referencing the temporary value.
    pub value : TokenStream,
}

#[derive(PartialEq, Eq, Debug, Hash)]
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

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum ModelTypeSystem {

    /// COM Automation compatible type system.
    Automation,

    /// Raw type system.
    Raw,
}

impl ModelTypeSystem {

    /// Converts the model type system into public type system tokens.
    pub fn as_tokens( self ) -> TokenStream {
        match self {
            ModelTypeSystem::Automation => quote!( Automation ),
            ModelTypeSystem::Raw => quote!( Raw ),
        }
    }

    /// Converts the model type system into public type system tokens.
    pub fn as_typesystem_tokens( self ) -> TokenStream {
        match self {
            ModelTypeSystem::Automation =>
                    quote!( intercom::type_system::TypeSystemName::Automation ),
            ModelTypeSystem::Raw =>
                    quote!( intercom::type_system::TypeSystemName::Raw ),
        }
    }

    /// Returns the intercom type that represents the type system.
    pub fn as_typesystem_type( self ) -> Type {
        match self {
            ModelTypeSystem::Automation =>
                    parse_quote!( intercom::type_system::AutomationTypeSystem ),
            ModelTypeSystem::Raw =>
                    parse_quote!( intercom::type_system::RawTypeSystem ),
        }
    }
}

/// Type usage context.
pub struct TypeContext {
    type_system: ModelTypeSystem,
}

impl TypeContext {
    pub fn new( type_system : ModelTypeSystem ) -> TypeContext {
        TypeContext { type_system }
    }
}

/// Defines Type-specific logic for handling the various parameter types in the
/// Rust/COM interface.
pub trait TypeHandler {

    /// The Rust type.
    fn rust_ty( &self ) -> Type;

    /// The COM type.
    fn com_ty( &self, _dir: Direction ) -> Type
    {
        self.rust_ty()
    }

    /// Converts a COM parameter named by the ident into a Rust type.
    fn com_to_rust(
        &self,
        ident : &Ident,
        _dir: Direction,
    ) -> TypeConversion
    {
        TypeConversion {
            temporary: None,
            value: quote!( #ident.into() ),
        }
    }

    /// Converts a Rust parameter named by the ident into a COM type.
    fn rust_to_com(
        &self, ident : &Ident, _dir: Direction
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
        quote!( intercom::type_system::ExternDefault::extern_default() )
    }
}

/// Replacement type handler that uses the Rust type system for representing
/// the various type system type conversions.
struct TypeSystemParam {
    ty : Type,
    context : TypeContext,
}
impl TypeHandler for TypeSystemParam {
    fn rust_ty( &self ) -> Type {
        self.ty.clone()
    }

    fn com_ty( &self, dir: Direction ) -> Type {

        // Construct bits for the quote.
        let ty = &self.ty;
        let ts = self.context.type_system.as_typesystem_type();
        let ts_trait = quote!(
            <#ty as intercom::type_system::ExternType< #ts > > );

        // Get the final type based on the parameter direction.
        match dir {
            Direction::In
                => parse_quote!( #ts_trait::ExternInputType ),
            Direction::Out | Direction::Retval
                => parse_quote!( #ts_trait::ExternOutputType ),
        }
    }

    fn com_to_rust( &self, ident : &Ident, dir: Direction ) -> TypeConversion
    {
        // Construct bits for the quote.
        let ty = &self.ty;
        let ts = self.context.type_system.as_typesystem_type();
        let ts_trait = quote!(
            <#ty as intercom::type_system::ExternType< #ts > > );

        TypeConversion {
            temporary: None,
            value: match dir {
                Direction::In => {
                    // Input arguments may use an intermediate type.
                    let intermediate = quote!(
                        < #ts_trait::OwnedNativeType >::intercom_from( #ident )? );
                    quote!( ( & #intermediate ).intercom_into()? )
                },
                Direction::Out | Direction::Retval => {
                    // Output arguments must not use an intermediate type
                    // as these must outlive the current function.
                    quote!( #ident.intercom_into()? )
                },
            }
        }
    }

    fn rust_to_com( &self, ident : &Ident, dir: Direction ) -> TypeConversion
    {
        // Construct bits for the quote.
        let ty = &self.ty;
        let ts = self.context.type_system.as_typesystem_type();
        let ts_trait = quote!(
            <#ty as intercom::type_system::ExternType< #ts > > );

        TypeConversion {
            temporary: None,
            value: match dir {
                Direction::In => {
                    let intermediate = quote!(
                        #ts_trait::OwnedExternType::intercom_from( #ident )? );
                    quote!( ( & #intermediate ).intercom_into()? )
                },
                Direction::Out | Direction::Retval => {
                    quote!( #ident.intercom_into()? )
                },
            }
        }
    }
}

/// Resolves the `TypeHandler` to use.
pub fn get_ty_handler(
    arg_ty : &Type,
    context : TypeContext,
) -> Rc<dyn TypeHandler>
{
    return Rc::new( TypeSystemParam {
        ty: arg_ty.clone(),
        context
    } )
}
