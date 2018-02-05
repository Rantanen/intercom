
use std::rc::Rc;
use syn::*;

use ast_converters::*;
use tyhandlers::{TypeHandler, get_ty_handler};
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
    pub name: Ident,
    pub ty: Type,
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

    pub fn new( name: Ident, ty: Type ) -> RustArg {

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
    pub rust_self_arg: ArgSelfRef,
    pub rust_return_ty: Type,

    pub retval_type: Option<Type>,
    pub return_type: Option<Type>,

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
        m : &MethodSig
    ) -> Result<ComMethodInfo, ComMethodInfoError>
    {
        Self::new_from_parts( &m.ident, &m.decl, m.unsafety.is_some() )
    }

    pub fn new_from_parts(
        n: &Ident,
        decl: &FnDecl,
        unsafety: bool,
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

            Ok( RustArg::new( ident, ty ) )
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

        let returnhandler = get_return_handler( &retval_type, &return_type )
                .or( Err( ComMethodInfoError::BadReturnType ) )?;
        Ok( ComMethodInfo {
            name: *n,
            is_const: is_const,
            rust_self_arg: rust_self_arg,
            rust_return_ty: rust_return_ty,
            retval_type: retval_type,
            return_type: return_type,
            returnhandler: returnhandler,
            args: args,
            is_unsafe: unsafety,
        } )
    }

    pub fn raw_com_args( &self ) -> Vec<ComArg>
    {
        let out_dir = if let Some( Type::Tuple(_) ) = self.retval_type {
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
                Some( parse_quote!( bool ) ) );
    }

    #[test]
    fn result_return_value() {

        let info = test_info( "fn foo( &self ) -> Result<String, f32> {}" );

        assert_eq!( info.is_const, true );
        assert_eq!( info.name, "foo" );
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

        let info = test_info( "fn foo( &self ) -> ComResult<String> {}" );

        assert_eq!( info.is_const, true );
        assert_eq!( info.name, "foo" );
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

        let info = test_info( "fn foo( &self, a : u32, b : f32 ) {}" );

        assert_eq!( info.is_const, true );
        assert_eq!( info.name, "foo" );
        assert_eq!( info.retval_type.is_none(), true );
        assert_eq!( info.return_type.is_none(), true );

        assert_eq!( info.args.len(), 2 );

        assert_eq!( info.args[0].name, Ident::from( "a" ) );
        assert_eq!( info.args[0].ty, parse_quote!( u32 ) );

        assert_eq!( info.args[1].name, Ident::from( "b" ) );
        assert_eq!( info.args[1].ty, parse_quote!( f32 ) );
    }

    fn test_info( code : &str ) -> ComMethodInfo {

        let item = parse_str( code ).unwrap();
        let ( ident, decl, unsafety ) = match item {
            Item::Fn( ref f ) => ( f.ident, f.decl.as_ref(), f.unsafety.is_some() ),
            _ => panic!( "Code isn't function" ),
        };
        ComMethodInfo::new_from_parts( &ident, decl, unsafety ).unwrap()
    }
}
