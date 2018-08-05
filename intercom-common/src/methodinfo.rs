
use prelude::*;
use std::rc::Rc;
use syn::*;

use ast_converters::*;
use tyhandlers::{Direction, TypeContext, TypeSystem, TypeHandler, get_ty_handler};
use returnhandlers::{ReturnHandler, get_return_handler};
use utils;

#[derive(Debug, PartialEq)]
pub enum ComMethodInfoError {
    TooFewArguments,
    BadSelfArg,
    BadArg(Box<FnArg>),
    BadReturnType,
}

#[derive(Clone)]
pub struct RustArg {

    /// Name of the Rust argument.
    pub name: Ident,

    /// Rust type of the COM argument.
    pub ty: Type,

    /// Type handler.
    pub handler: Rc<TypeHandler>,
}

impl PartialEq for RustArg {

    fn eq(&self, other: &RustArg) -> bool
    {
        self.name == other.name
            && self.ty == other.ty
    }
}

impl ::std::fmt::Debug for RustArg {
    fn fmt( &self, f: &mut ::std::fmt::Formatter ) -> ::std::fmt::Result {
        write!( f, "{}: {:?}", self.name, self.ty )
    }
}

impl RustArg {

    pub fn new( name: Ident, ty: Type, type_system: TypeSystem ) -> RustArg {

        let tyhandler = get_ty_handler(
                &ty, TypeContext::new( Direction::In, type_system ) );
        RustArg {
            name,
            ty,
            handler: tyhandler,
        }
    }
}

pub struct ComArg {

    /// Name of the argument.
    pub name: Ident,

    /// Rust type of the raw COM argument.
    pub ty: Type,

    /// Type handler.
    pub handler: Rc<TypeHandler>,

    /// Argument direction. COM uses OUT params while Rust uses return values.
    pub dir : Direction
}

impl ComArg {

    pub fn new(
        name: Ident,
        ty: Type,
        dir: Direction,
        type_system: TypeSystem
    ) -> ComArg {

        let tyhandler = get_ty_handler(
                &ty, TypeContext::new( dir, type_system ) );
        ComArg {
            name,
            ty,
            dir,
            handler: tyhandler,
        }
    }

    pub fn from_rustarg(
        rustarg: RustArg,
        dir: Direction,
        type_system: TypeSystem,
    ) -> ComArg {

        let tyhandler = get_ty_handler(
                &rustarg.ty, TypeContext::new( dir, type_system ) );
        ComArg {
            name: rustarg.name,
            ty: rustarg.ty,
            dir,
            handler: tyhandler,
        }
    }
}

impl PartialEq for ComArg {

    fn eq(&self, other: &ComArg) -> bool
    {
        self.name == other.name
            && self.ty == other.ty
            && self.dir == other.dir
    }
}

impl ::std::fmt::Debug for ComArg {
    fn fmt( &self, f: &mut ::std::fmt::Formatter ) -> ::std::fmt::Result {
        write!( f, "{}: {:?} {:?}", self.name, self.dir, self.ty )
    }
}


#[derive(Debug)]
pub struct ComMethodInfo {

    /// The display name used in public places that do not require an unique name.
    pub display_name: Ident,

    /// Unique name that differentiates between different type systems.
    pub unique_name: Ident,

    /// True if the self parameter is not mutable.
    pub is_const: bool,

    /// Rust self argument.
    pub rust_self_arg: ArgSelfRef,

    /// Rust return type.
    pub rust_return_ty: Type,

    /// COM retval out parameter type, such as the value of Result<...>.
    pub retval_type: Option<Type>,

    /// COM return type, such as the error value of Result<...>.
    pub return_type: Option<Type>,

    /// Return value handler.
    pub returnhandler: Box<ReturnHandler>,

    /// Method arguments.
    pub args: Vec<RustArg>,

    /// True if the Rust method is unsafe.
    pub is_unsafe: bool,

    /// Type system.
    pub type_system : TypeSystem,
}

impl PartialEq for ComMethodInfo {

    fn eq(&self, other: &ComMethodInfo) -> bool
    {
        self.display_name == other.display_name
            && self.unique_name == other.unique_name
            && self.is_const == other.is_const
            && self.rust_self_arg == other.rust_self_arg
            && self.rust_return_ty == other.rust_return_ty
            && self.retval_type == other.retval_type
            && self.return_type == other.return_type
            && self.args == other.args
    }
}

impl ComMethodInfo {

    /// Constructs new COM method info from a Rust method signature.
    pub fn new(
        m : &MethodSig,
        type_system : TypeSystem,
    ) -> Result<ComMethodInfo, ComMethodInfoError>
    {
        Self::new_from_parts( m.ident.clone(), &m.decl, m.unsafety.is_some(), type_system )
    }

    pub fn new_from_parts(
        n: Ident,
        decl: &FnDecl,
        unsafety: bool,
        type_system : TypeSystem,
    ) -> Result<ComMethodInfo, ComMethodInfoError>
    {
        // Process all the function arguments.
        // In Rust this includes the 'self' argument and the actual function
        // arguments. For COM the self is implicit so we'll handle it
        // separately.
        let mut iter = decl.inputs.iter();
        let rust_self_arg = iter.next()
                .ok_or_else( || ComMethodInfoError::TooFewArguments )?;

        let ( is_const, rust_self_arg ) = match *rust_self_arg {
            FnArg::SelfRef( ref self_arg ) => (
                self_arg.mutability.is_none(),
                self_arg.clone()
            ),
            _ => return Err( ComMethodInfoError::BadSelfArg ),
        } ;

        // Process other arguments.
        let args = iter.map( | arg | {
            let ty = arg.get_ty()
                .or_else( |_| Err(
                    ComMethodInfoError::BadArg( Box::new( arg.clone() ) )
                ) )?;
            let ident = arg.get_ident()
                .or_else( |_| Err(
                    ComMethodInfoError::BadArg( Box::new( arg.clone() ) )
                ) )?;

            Ok( RustArg::new( ident, ty, type_system ) )
        } ).collect::<Result<_,_>>()?;

        // Get the output.
        let rust_return_ty = match decl.output {
            ReturnType::Default => parse_quote!( () ),
            ReturnType::Type( _, ref ty ) => (**ty).clone(),
        };

        // Resolve the return type and retval type.
        let ( retval_type, return_type ) = if utils::is_unit( &rust_return_ty ) {
            ( None, None )
        } else if let Some( ( retval, ret ) ) = try_parse_result( &rust_return_ty ) {
            ( Some( retval ), Some( ret ) )
        } else {
            ( None, Some( rust_return_ty.clone() ) )
        };

        let returnhandler = get_return_handler(
                    &retval_type, &return_type, type_system )
                .or( Err( ComMethodInfoError::BadReturnType ) )?;
        Ok( ComMethodInfo {
            unique_name: Ident::new( &format!( "{}_{:?}", n, type_system ), Span::call_site() ),
            display_name: n,
            is_const,
            rust_self_arg,
            rust_return_ty,
            retval_type,
            return_type,
            returnhandler,
            args,
            is_unsafe: unsafety,
            type_system
        } )
    }

    pub fn raw_com_args( &self ) -> Vec<ComArg>
    {
        let in_args = self.args
                .iter()
                .map( |ca| {
                    ComArg::from_rustarg( ca.clone(), Direction::In, self.type_system )
                } );
        let out_args = self.returnhandler.com_out_args();

        in_args.chain( out_args ).collect()
    }
}

fn try_parse_result( ty : &Type ) -> Option<( Type, Type )>
{
    let path = match *ty {
        Type::Path( ref p ) => &p.path,
        _ => return None,
    };

    // Ensure the type name contains 'Result'. We don't really have
    // good ways to ensure it is an actual Result type but at least we can
    // use this to discount things like Option<>, etc.
    let last_segment = path.segments.last()?;
    if ! last_segment.value().ident.to_string().contains( "Result" ) {
        return None;
    }

    // Ensure the Result has angle bracket arguments.
    if let PathArguments::AngleBracketed( ref data )
            = last_segment.value().arguments {

        // The returned types depend on how many arguments the Result has.
        return Some( match data.args.len() {
            1 => ( data.args[ 0 ].get_ty().ok()?, hresult_ty() ),
            2 => ( data.args[ 0 ].get_ty().ok()?, data.args[ 1 ].get_ty().ok()? ),
            _ => return None,
        } )
    }

    // We couldn't find a valid type. Return nothing.
    None
}

fn hresult_ty() -> Type {
    parse_quote!( ::intercom::HRESULT )
}

#[cfg(test)]
mod tests {

    use super::*;
    use tyhandlers::TypeSystem::*;

    #[test]
    fn no_args_or_return_value() {

        let info = test_info( "fn foo( &self ) {}", Automation );

        assert_eq!( info.is_const, true );
        assert_eq!( info.display_name, "foo" );
        assert_eq!( info.unique_name, "foo_Automation" );
        assert_eq!( info.args.len(), 0 );
        assert_eq!( info.retval_type.is_none(), true );
        assert_eq!( info.return_type.is_none(), true );
    }

    #[test]
    fn basic_return_value() {

        let info = test_info( "fn foo( &self ) -> bool {}", Raw );

        assert_eq!( info.is_const, true );
        assert_eq!( info.display_name, "foo" );
        assert_eq!( info.unique_name, "foo_Raw" );
        assert_eq!( info.args.len(), 0 );
        assert_eq!( info.retval_type.is_none(), true );
        assert_eq!(
                info.return_type,
                Some( parse_quote!( bool ) ) );
    }

    #[test]
    fn result_return_value() {

        let info = test_info( "fn foo( &self ) -> Result<String, f32> {}", Automation );

        assert_eq!( info.is_const, true );
        assert_eq!( info.display_name, "foo" );
        assert_eq!( info.unique_name, "foo_Automation" );
        assert_eq!( info.args.len(), 0 );
        assert_eq!(
                info.retval_type,
                Some( parse_quote!( String ) ) );
        assert_eq!(
                info.return_type,
                Some( parse_quote!( f32 ) ) );
    }

    #[test]
    fn comresult_return_value() {

        let info = test_info( "fn foo( &self ) -> ComResult<String> {}", Automation );

        assert_eq!( info.is_const, true );
        assert_eq!( info.display_name, "foo" );
        assert_eq!( info.unique_name, "foo_Automation" );
        assert_eq!( info.args.len(), 0 );
        assert_eq!(
                info.retval_type,
                Some( parse_quote!( String ) ) );
        assert_eq!(
                info.return_type,
                Some( parse_quote!( ::intercom::HRESULT ) ) );
    }

    #[test]
    fn basic_arguments() {

        let info = test_info( "fn foo( &self, a : u32, b : f32 ) {}", Raw );

        assert_eq!( info.is_const, true );
        assert_eq!( info.display_name, "foo" );
        assert_eq!( info.unique_name, "foo_Raw" );
        assert_eq!( info.retval_type.is_none(), true );
        assert_eq!( info.return_type.is_none(), true );

        assert_eq!( info.args.len(), 2 );

        assert_eq!( info.args[0].name, Ident::new( "a", Span::call_site() ) );
        assert_eq!( info.args[0].ty, parse_quote!( u32 ) );

        assert_eq!( info.args[1].name, Ident::new( "b", Span::call_site() ) );
        assert_eq!( info.args[1].ty, parse_quote!( f32 ) );
    }

    fn test_info( code : &str, ts : TypeSystem) -> ComMethodInfo {

        let item = parse_str( code ).unwrap();
        let ( ident, decl, unsafety ) = match item {
            Item::Fn( ref f ) => ( f.ident.clone(), f.decl.as_ref(), f.unsafety.is_some() ),
            _ => panic!( "Code isn't function" ),
        };
        ComMethodInfo::new_from_parts(
                ident, decl, unsafety, ts ).unwrap()
    }
}
