use crate::prelude::*;
use proc_macro2::Span;
use std::rc::Rc;
use syn::{spanned::Spanned, FnArg, PathArguments, Receiver, ReturnType, Signature, Type};

use crate::ast_converters::*;
use crate::returnhandlers::{get_return_handler, ReturnHandler};
use crate::tyhandlers::{get_ty_handler, Direction, ModelTypeSystem, TypeContext, TypeHandler};
use crate::utils;

#[derive(Debug, PartialEq)]
pub enum ComMethodInfoError
{
    TooFewArguments,
    BadSelfArg,
    BadArg(Box<FnArg>),
    BadReturnType,
}

#[derive(Clone)]
pub struct RustArg
{
    /// Name of the Rust argument.
    pub name: Ident,

    /// Rust type of the COM argument.
    pub ty: Type,

    /// Rust type span.
    pub span: Span,

    /// Type handler.
    pub handler: Rc<TypeHandler>,
}

impl PartialEq for RustArg
{
    fn eq(&self, other: &RustArg) -> bool
    {
        self.name == other.name && self.ty == other.ty
    }
}

impl ::std::fmt::Debug for RustArg
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
    {
        write!(f, "{}: {:?}", self.name, self.ty)
    }
}

impl RustArg
{
    pub fn new(name: Ident, ty: Type, span: Span, type_system: ModelTypeSystem) -> RustArg
    {
        let tyhandler = get_ty_handler(&ty, TypeContext::new(type_system));
        RustArg {
            name,
            ty,
            span,
            handler: tyhandler,
        }
    }
}

pub struct ComArg
{
    /// Name of the argument.
    pub name: Ident,

    /// Rust type of the raw COM argument.
    pub ty: Type,

    /// Type handler.
    pub handler: Rc<TypeHandler>,

    // Rust span that sources this COM argument.
    pub span: Span,

    /// Argument direction. COM uses OUT params while Rust uses return values.
    pub dir: Direction,
}

impl ComArg
{
    pub fn new(
        name: Ident,
        ty: Type,
        span: Span,
        dir: Direction,
        type_system: ModelTypeSystem,
    ) -> ComArg
    {
        let tyhandler = get_ty_handler(&ty, TypeContext::new(type_system));
        ComArg {
            name,
            ty,
            dir,
            span,
            handler: tyhandler,
        }
    }

    pub fn from_rustarg(rustarg: RustArg, dir: Direction, type_system: ModelTypeSystem) -> ComArg
    {
        let tyhandler = get_ty_handler(&rustarg.ty, TypeContext::new(type_system));
        ComArg {
            name: rustarg.name,
            ty: rustarg.ty,
            dir,
            span: rustarg.span,
            handler: tyhandler,
        }
    }
}

impl PartialEq for ComArg
{
    fn eq(&self, other: &ComArg) -> bool
    {
        self.name == other.name && self.ty == other.ty && self.dir == other.dir
    }
}

impl ::std::fmt::Debug for ComArg
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result
    {
        write!(f, "{}: {:?} {:?}", self.name, self.dir, self.ty)
    }
}

#[derive(Debug, Clone)]
pub struct ComMethodInfo
{
    /// The display name used in public places that do not require an unique name.
    pub name: Ident,

    /// True if the self parameter is not mutable.
    pub is_const: bool,

    /// Rust self argument.
    pub rust_self_arg: Receiver,

    /// Rust return type.
    pub rust_return_ty: Type,

    /// COM retval out parameter type, such as the value of Result<...>.
    pub retval_type: Option<Type>,

    /// COM return type, such as the error value of Result<...>.
    pub return_type: Option<Type>,

    /// Span for the method signature.
    pub signature_span: Span,

    /// Return value handler.
    pub returnhandler: Rc<dyn ReturnHandler>,

    /// Method arguments.
    pub args: Vec<RustArg>,

    /// True if the Rust method is unsafe.
    pub is_unsafe: bool,

    /// Type system.
    pub type_system: ModelTypeSystem,

    /// Is the method infallible.
    pub infallible: bool,
}

impl PartialEq for ComMethodInfo
{
    fn eq(&self, other: &ComMethodInfo) -> bool
    {
        self.name == other.name
            && self.is_const == other.is_const
            && self.rust_self_arg == other.rust_self_arg
            && self.rust_return_ty == other.rust_return_ty
            && self.retval_type == other.retval_type
            && self.return_type == other.return_type
            && self.args == other.args
    }
}

impl ComMethodInfo
{
    /// Constructs new COM method info from a Rust method signature.
    pub fn new(
        decl: &Signature,
        type_system: ModelTypeSystem,
    ) -> Result<ComMethodInfo, ComMethodInfoError>
    {
        // Process all the function arguments.
        // In Rust this includes the 'self' argument and the actual function
        // arguments. For COM the self is implicit so we'll handle it
        // separately.
        let n = decl.ident.clone();
        let unsafety = decl.unsafety.is_some();
        let mut iter = decl.inputs.iter();
        let rust_self_arg = iter.next().ok_or(ComMethodInfoError::TooFewArguments)?;

        let (is_const, rust_self_arg) = match *rust_self_arg {
            FnArg::Receiver(ref self_arg) => (self_arg.mutability.is_none(), self_arg.clone()),
            _ => return Err(ComMethodInfoError::BadSelfArg),
        };

        // Process other arguments.
        let args = iter
            .map(|arg| {
                let ty = arg
                    .get_ty()
                    .map_err(|_| ComMethodInfoError::BadArg(Box::new(arg.clone())))?;
                let ident = arg
                    .get_ident()
                    .map_err(|_| ComMethodInfoError::BadArg(Box::new(arg.clone())))?;

                Ok(RustArg::new(ident, ty, arg.span(), type_system))
            })
            .collect::<Result<_, _>>()?;

        // Get the output.
        let rust_return_ty = match decl.output {
            ReturnType::Default => syn::parse2(quote_spanned!(decl.span() => ())).unwrap(),
            ReturnType::Type(_, ref ty) => (**ty).clone(),
        };

        // Resolve the return type and retval type.
        let (retval_type, return_type, retval_span) = if utils::is_unit(&rust_return_ty) {
            (None, None, decl.span())
        } else if let Some((retval, ret)) = try_parse_result(&rust_return_ty) {
            (Some(retval), Some(ret), decl.output.span())
        } else {
            (None, Some(rust_return_ty.clone()), decl.output.span())
        };

        let returnhandler =
            get_return_handler(&retval_type, &return_type, retval_span, type_system)
                .or(Err(ComMethodInfoError::BadReturnType))?;
        Ok(ComMethodInfo {
            name: n,
            infallible: returnhandler.is_infallible(),
            returnhandler: returnhandler.into(),
            signature_span: decl.span(),
            is_const,
            rust_self_arg,
            rust_return_ty,
            retval_type,
            return_type,
            args,
            is_unsafe: unsafety,
            type_system,
        })
    }

    pub fn raw_com_args(&self) -> Vec<ComArg>
    {
        let in_args = self
            .args
            .iter()
            .map(|ca| ComArg::from_rustarg(ca.clone(), Direction::In, self.type_system));
        let out_args = self.returnhandler.com_out_args();

        in_args.chain(out_args).collect()
    }

    pub fn get_parameters_tokenstream(&self) -> TokenStream
    {
        let in_out_args = self.raw_com_args().into_iter().map(|com_arg| {
            let name = &com_arg.name;
            let com_ty = &com_arg.handler.com_ty(com_arg.span);
            let dir = match com_arg.dir {
                Direction::In => quote!(),
                Direction::Out | Direction::Retval => quote_spanned!(com_arg.span => *mut ),
            };
            quote_spanned!(com_arg.span => #name : #dir #com_ty )
        });
        let self_arg = quote_spanned!(self.rust_self_arg.span()=>
            self_vtable: intercom::raw::RawComPtr);
        let args = std::iter::once(self_arg).chain(in_out_args);
        quote!(#(#args),*)
    }
}

fn try_parse_result(ty: &Type) -> Option<(Type, Type)>
{
    let path = match *ty {
        Type::Path(ref p) => &p.path,
        _ => return None,
    };

    // Ensure the type name contains 'Result'. We don't really have
    // good ways to ensure it is an actual Result type but at least we can
    // use this to discount things like Option<>, etc.
    let last_segment = path.segments.last()?;
    if !last_segment.ident.to_string().contains("Result") {
        return None;
    }

    // Ensure the Result has angle bracket arguments.
    if let PathArguments::AngleBracketed(ref data) = last_segment.arguments {
        // The returned types depend on how many arguments the Result has.
        return Some(match data.args.len() {
            1 => (data.args[0].get_ty().ok()?, hresult_ty(ty.span())),
            2 => (data.args[0].get_ty().ok()?, data.args[1].get_ty().ok()?),
            _ => return None,
        });
    }

    // We couldn't find a valid type. Return nothing.
    None
}

fn hresult_ty(span: Span) -> Type
{
    syn::parse2(quote_spanned!(span => intercom::raw::HRESULT)).unwrap()
}

#[cfg(test)]
mod tests
{

    use syn::Item;

    use super::*;
    use crate::tyhandlers::ModelTypeSystem::*;

    #[test]
    fn no_args_or_return_value()
    {
        let info = test_info("fn foo( &self ) {}", Automation);

        assert_eq!(info.is_const, true);
        assert_eq!(info.name, "foo");
        assert_eq!(info.args.len(), 0);
        assert_eq!(info.retval_type.is_none(), true);
        assert_eq!(info.return_type.is_none(), true);
    }

    #[test]
    fn basic_return_value()
    {
        let info = test_info("fn foo( &self ) -> bool {}", Raw);

        assert_eq!(info.is_const, true);
        assert_eq!(info.name, "foo");
        assert_eq!(info.args.len(), 0);
        assert_eq!(info.retval_type.is_none(), true);
        assert_eq!(info.return_type, Some(parse_quote!(bool)));
    }

    #[test]
    fn result_return_value()
    {
        let info = test_info("fn foo( &self ) -> Result<String, f32> {}", Automation);

        assert_eq!(info.is_const, true);
        assert_eq!(info.name, "foo");
        assert_eq!(info.args.len(), 0);
        assert_eq!(info.retval_type, Some(parse_quote!(String)));
        assert_eq!(info.return_type, Some(parse_quote!(f32)));
    }

    #[test]
    fn comresult_return_value()
    {
        let info = test_info("fn foo( &self ) -> ComResult<String> {}", Automation);

        assert_eq!(info.is_const, true);
        assert_eq!(info.name, "foo");
        assert_eq!(info.args.len(), 0);
        assert_eq!(info.retval_type, Some(parse_quote!(String)));
        assert_eq!(info.return_type, Some(parse_quote!(intercom::raw::HRESULT)));
    }

    #[test]
    fn basic_arguments()
    {
        let info = test_info("fn foo( &self, a : u32, b : f32 ) {}", Raw);

        assert_eq!(info.is_const, true);
        assert_eq!(info.name, "foo");
        assert_eq!(info.retval_type.is_none(), true);
        assert_eq!(info.return_type.is_none(), true);

        assert_eq!(info.args.len(), 2);

        assert_eq!(info.args[0].name, Ident::new("a", Span::call_site()));
        assert_eq!(info.args[0].ty, parse_quote!(u32));

        assert_eq!(info.args[1].name, Ident::new("b", Span::call_site()));
        assert_eq!(info.args[1].ty, parse_quote!(f32));
    }

    fn test_info(code: &str, ts: ModelTypeSystem) -> ComMethodInfo
    {
        let item = syn::parse_str(code).unwrap();
        let sig = match item {
            Item::Fn(ref f) => &f.sig,
            _ => panic!("Code isn't function"),
        };
        ComMethodInfo::new(sig, ts).unwrap()
    }
}
