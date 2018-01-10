
use std::rc::Rc;
use syn::*;

use ast_converters::*;
use tyhandlers::{TyHandler, get_ty_handler};
use returnhandlers::{ReturnHandler, get_return_handler};
use utils;

#[derive(Debug, PartialEq)]
pub enum ComMethodInfoError {
    TooFewArguments,
    BadSelfArg,
    BadArg(Box<FnArg>),
    BadReturnTy,
}

#[derive(Clone)]
pub struct RustArg {
    pub name: Ident,
    pub ty: Ty,
    pub handler: Rc<TyHandler>,
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

    pub fn new( name: Ident, ty: Ty ) -> RustArg {

        let tyhandler = get_ty_handler( &ty );
        RustArg {
            name: name,
            ty: ty,
            handler: tyhandler,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction { In, Out, Retval }

#[derive(Debug, PartialEq)]
pub struct ComArg {
    pub arg : RustArg,
    pub dir : Direction
}

#[derive(Debug)]
pub struct ComMethodInfo {

    pub name: Ident,

    pub is_const: bool,
    pub rust_self_arg: FnArg,
    pub rust_return_ty: Ty,

    pub retval_type: Option<Ty>,
    pub return_type: Option<Ty>,

    pub returnhandler: Box<ReturnHandler>,
    pub args: Vec<RustArg>,

    pub is_unsafe: bool,
}

impl PartialEq for ComMethodInfo {

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

impl ComMethodInfo {

    /// Constructs new COM method info from a Rust method signature.
    pub fn new(
        n: &Ident,
        m : &MethodSig
    ) -> Result<ComMethodInfo, ComMethodInfoError>
    {
        // Process all the function arguments.
        // In Rust this includes the 'self' argument and the actual function
        // arguments. For COM the self is implicit so we'll handle it
        // separately.
        let ( is_const, rust_self_arg, com_args ) = m.decl.inputs
            .split_first()
            .ok_or( ComMethodInfoError::TooFewArguments )
            .and_then( | ( self_arg, other_args ) | {

                // Resolve the self argument.
                let ( is_const, rust_self_arg ) = match *self_arg {
                    FnArg::SelfRef(.., m) => (
                        m == Mutability::Immutable,
                        self_arg.clone(),
                    ),
                    _ => return Err( ComMethodInfoError::BadSelfArg ),
                };

                // Process other arguments.
                let args = other_args.iter().map( | arg | {
                    let ty = arg.get_ty()
                        .or_else( |_| Err(
                            ComMethodInfoError::BadArg( Box::new( arg.clone() ) )
                        ) )?;
                    let ident = arg.get_ident()
                        .or_else( |_| Err(
                            ComMethodInfoError::BadArg( Box::new( arg.clone() ) )
                        ) )?;

                    Ok( RustArg::new( ident, ty ) )
                } ).collect::<Result<_,_>>()?;

                Ok( ( is_const, rust_self_arg, args ) )
            } )?;

        // Get the output.
        let output = &m.decl.output;
        let rust_return_ty = output.get_ty()
                .or( Err( ComMethodInfoError::BadReturnTy ) )?;

        // Resolve the return type and retval type.
        let ( retval_type, return_type ) = if utils::is_unit( &rust_return_ty ) {
            ( None, None )
        } else if let Some( ( retval, ret ) ) = try_parse_result( &rust_return_ty ) {
            ( Some( retval ), Some( ret ) )
        } else {
            ( None, Some( rust_return_ty.clone() ) )
        };

        let returnhandler = get_return_handler( &retval_type, &return_type )
                .or( Err( ComMethodInfoError::BadReturnTy ) )?;
        Ok( ComMethodInfo {
            name: n.clone(),
            is_const: is_const,
            rust_self_arg: rust_self_arg,
            rust_return_ty: rust_return_ty,
            retval_type: retval_type,
            return_type: return_type,
            returnhandler: returnhandler,
            args: com_args,
            is_unsafe: m.unsafety == Unsafety::Unsafe,
        } )
    }

    pub fn raw_com_args( &self ) -> Vec<ComArg>
    {
        let out_dir = if let Some( Ty::Tup(_) ) = self.retval_type {
                            Direction::Out
                        } else {
                            Direction::Retval
                        };

        let in_args = self.args
                .iter()
                .map( |ca| {
                    ComArg { arg: ca.clone(), dir: Direction::In }
                } );
        let out_args = self.returnhandler.com_out_args()
                .into_iter()
                .map( |ca| {
                    ComArg { arg: ca, dir: out_dir }
                } );

        in_args.chain( out_args ).collect()
    }
}

fn try_parse_result( ty : &Ty ) -> Option<( Ty, Ty )>
{
    let path = match *ty {
        Ty::Path( _, ref p ) => p,
        _ => return None,
    };

    // Ensure the type name contains 'Result'. We don't really have
    // good ways to ensure it is an actual Result type but at least we can
    // use this to discount things like Option<>, etc.
    let last_segment = path.segments.last()?;
    if ! last_segment.ident.to_string().contains( "Result" ) {
        return None;
    }

    // Ensure the Result has angle bracket arguments.
    if let PathParameters::AngleBracketed( ref data )
            = last_segment.parameters {

        // The returned types depend on how many arguments the Result has.
        return Some( match data.types.len() {
            1 => ( data.types[ 0 ].clone(), hresult_ty() ),
            2 => ( data.types[ 0 ].clone(), data.types[ 1 ].clone() ),
            _ => return None,
        } )
    }

    // We couldn't find a valid type. Return nothing.
    None
}

fn hresult_ty() -> Ty {
    Ty::Path(
        None,
        Path {
            global: true,
            segments: vec![
                PathSegment::from( Ident::from( "intercom" ) ),
                PathSegment::from( Ident::from( "HRESULT" ) ),
            ]
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args_or_return_value() {

        let info = test_info( "fn foo( &self ) {}" );

        assert_eq!( info.is_const, true );
        assert_eq!( info.name, "foo" );
        assert_eq!( info.args.len(), 0 );
        assert_eq!( info.retval_type.is_none(), true );
        assert_eq!( info.return_type.is_none(), true );
    }

    #[test]
    fn basic_return_value() {

        let info = test_info( "fn foo( &self ) -> bool {}" );

        assert_eq!( info.is_const, true );
        assert_eq!( info.name, "foo" );
        assert_eq!( info.args.len(), 0 );
        assert_eq!( info.retval_type.is_none(), true );
        assert_eq!(
                info.return_type,
                parse_type( "bool" ).ok() );
    }

    #[test]
    fn result_return_value() {

        let info = test_info( "fn foo( &self ) -> Result<String, f32> {}" );

        assert_eq!( info.is_const, true );
        assert_eq!( info.name, "foo" );
        assert_eq!( info.args.len(), 0 );
        assert_eq!(
                info.retval_type,
                parse_type( "String" ).ok() );
        assert_eq!(
                info.return_type,
                parse_type( "f32" ).ok() );
    }

    #[test]
    fn comresult_return_value() {

        let info = test_info( "fn foo( &self ) -> ComResult<String> {}" );

        assert_eq!( info.is_const, true );
        assert_eq!( info.name, "foo" );
        assert_eq!( info.args.len(), 0 );
        assert_eq!(
                info.retval_type,
                parse_type( "String" ).ok() );
        assert_eq!(
                info.return_type,
                parse_type( "::intercom::HRESULT" ).ok() );
    }

    #[test]
    fn basic_arguments() {

        let info = test_info( "fn foo( &self, a : u32, b : f32 ) {}" );

        assert_eq!( info.is_const, true );
        assert_eq!( info.name, "foo" );
        assert_eq!( info.retval_type.is_none(), true );
        assert_eq!( info.return_type.is_none(), true );

        assert_eq!( info.args.len(), 2 );

        assert_eq!( info.args[0].name, Ident::from( "a" ) );
        assert_eq!( info.args[0].ty, parse_type( "u32" ).unwrap() );

        assert_eq!( info.args[1].name, Ident::from( "b" ) );
        assert_eq!( info.args[1].ty, parse_type( "f32" ).unwrap() );
    }

    fn test_info( code : &str ) -> ComMethodInfo {

        let item = parse_item( code ).unwrap();
        ComMethodInfo::new(
            &item.ident,
            match item.node {
                ItemKind::Fn( ref fn_decl, .. ) => fn_decl,
                _ => panic!( "Code isn't function" ),
            } ).unwrap()
    }
}
