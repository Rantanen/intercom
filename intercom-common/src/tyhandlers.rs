use crate::prelude::*;
use proc_macro2::Span;
use std::rc::Rc;
use syn::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction
{
    In,
    Out,
    Retval,
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct ModelTypeSystemConfig
{
    pub effective_system: ModelTypeSystem,
    pub is_default: bool,
}

impl ModelTypeSystemConfig
{
    pub fn get_unique_name(&self, base: &str) -> String
    {
        match self.is_default {
            true => base.to_string(),
            false => format!("{}_{:?}", base, self.effective_system),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum ModelTypeSystem
{
    /// COM Automation compatible type system.
    Automation,

    /// Raw type system.
    Raw,
}

impl ModelTypeSystem
{
    /// Converts the model type system into public type system tokens.
    pub fn as_tokens(self) -> TokenStream
    {
        match self {
            ModelTypeSystem::Automation => quote!(Automation),
            ModelTypeSystem::Raw => quote!(Raw),
        }
    }

    /// Converts the model type system into public type system tokens.
    pub fn as_typesystem_tokens(self, span: Span) -> TokenStream
    {
        match self {
            ModelTypeSystem::Automation => {
                quote_spanned!(span=> intercom::type_system::TypeSystemName::Automation)
            }
            ModelTypeSystem::Raw => {
                quote_spanned!(span=> intercom::type_system::TypeSystemName::Raw)
            }
        }
    }

    /// Returns the intercom type that represents the type system.
    pub fn as_typesystem_type(self, span: Span) -> Type
    {
        syn::parse2(match self {
            ModelTypeSystem::Automation => {
                quote_spanned!(span=> intercom::type_system::AutomationTypeSystem)
            }
            ModelTypeSystem::Raw => quote_spanned!(span=> intercom::type_system::RawTypeSystem),
        })
        .unwrap()
    }
}

impl quote::IdentFragment for ModelTypeSystem
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}

/// Type usage context.
pub struct TypeContext
{
    type_system: ModelTypeSystem,
}

impl TypeContext
{
    pub fn new(type_system: ModelTypeSystem) -> TypeContext
    {
        TypeContext { type_system }
    }
}

/// Defines Type-specific logic for handling the various parameter types in the
/// Rust/COM interface.
pub struct TypeHandler
{
    ty: Type,
    context: TypeContext,
}

impl TypeHandler
{
    /// The Rust type.
    pub fn rust_ty(&self) -> Type
    {
        self.ty.clone()
    }

    /// The COM type.
    pub fn com_ty(&self, span: Span, dir: Direction, infallible: bool) -> Type
    {
        // Construct bits for the quote.
        let ty = &self.ty;
        let ts = self.context.type_system.as_typesystem_type(span);
        let (tr, _unwrap) = resolve_type_handling(dir, infallible, span);
        syn::parse2(quote_spanned!(span => <#ty as #tr<#ts>>::ForeignType)).unwrap()
    }

    /// Converts a COM parameter named by the ident into a Rust type.
    pub fn com_to_rust(
        &self,
        ident: &Ident,
        span: Span,
        dir: Direction,
        infallible: bool,
    ) -> TokenStream
    {
        // Construct bits for the quote.
        let ty = &self.ty;
        let ts = self.context.type_system.as_typesystem_type(span);
        let (tr, unwrap) = resolve_type_handling(dir, infallible, span);
        let (maybe_ref, maybe_as_ref) = resolve_ref(ty);
        match dir {
            Direction::In => quote_spanned!(span=>
                    #maybe_ref <#ty as #tr<#ts>>
                        ::from_foreign_parameter(#ident)#unwrap#maybe_as_ref),

            // Output variables do not use #unwrap to avoid jumping out before all parameters have
            // been converted from foreign output. This ensures all parameters are brought under
            // Rust's memory management.
            Direction::Out | Direction::Retval => quote_spanned!(span=>
                    <#ty as #tr<#ts>>::from_foreign_output(#ident)),
        }
    }

    /// Converts a Rust parameter named by the ident into a COM type.
    pub fn rust_to_com(
        &self,
        ident: &Ident,
        span: Span,
        dir: Direction,
        infallible: bool,
    ) -> TokenStream
    {
        // Construct bits for the quote.
        let ty = &self.ty;
        let ts = self.context.type_system.as_typesystem_type(span);
        let (tr, unwrap) = resolve_type_handling(dir, infallible, span);
        match dir {
            Direction::In => quote_spanned!(span=>
                    <#ty as #tr<#ts>>
                        ::into_foreign_parameter(#ident)#unwrap.0),
            Direction::Out | Direction::Retval => quote_spanned!(span=>
                    <#ty as #tr<#ts>>
                        ::into_foreign_output(#ident)#unwrap),
        }
    }

    /// Gets the default value for the type.
    pub fn default_value(&self) -> TokenStream
    {
        quote!(intercom::type_system::ExternDefault::extern_default())
    }
}

fn resolve_ref(ty: &Type) -> (TokenStream, TokenStream)
{
    if let syn::Type::Reference(..) = ty {
        return (quote!(&), quote!());
    }

    if has_ref(ty) {
        (quote!(), quote!(.as_ref()))
    } else {
        (quote!(), quote!())
    }
}

fn has_ref(ty: &Type) -> bool
{
    match ty {
        syn::Type::Reference(..) => true,
        syn::Type::Path(p) => {
            let last_segment = p.path.segments.last().expect("Path was empth");
            match &last_segment.arguments {
                syn::PathArguments::None | syn::PathArguments::Parenthesized(..) => false,
                syn::PathArguments::AngleBracketed(generics) => {
                    generics.args.iter().any(|g| match g {
                        syn::GenericArgument::Type(t) => has_ref(t),
                        _ => false,
                    })
                }
            }
        }
        _ => false,
    }
}

fn resolve_type_handling(dir: Direction, infallible: bool, span: Span)
    -> (TokenStream, TokenStream)
{
    let dir_part = match dir {
        Direction::In => "Input",
        Direction::Out | Direction::Retval => "Output",
    };
    let (fallibility, unwrap) = match infallible {
        true => ("Infallible", quote!()),
        false => ("", quote!(?)),
    };

    let ident = Ident::new(&format!("{}Extern{}", fallibility, dir_part), span);
    (
        quote_spanned!(span => intercom::type_system::#ident),
        unwrap,
    )
}

/// Resolves the `TypeHandler` to use.
pub fn get_ty_handler(arg_ty: &Type, context: TypeContext) -> Rc<TypeHandler>
{
    Rc::new(TypeHandler {
        ty: arg_ty.clone(),
        context,
    })
}
