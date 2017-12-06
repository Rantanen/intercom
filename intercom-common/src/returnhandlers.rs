
use syn::*;
use quote::Tokens;
use methodinfo::ComArg;
use ast_converters::*;
use utils;

/// Defines return handler for handling various different return type schemes.
pub trait ReturnHandler {

    /// The return type of the original Rust method.
    fn rust_ty( &self ) -> Ty;

    /// The return type for COM implementation.
    fn com_ty( &self ) -> Ty { self.rust_ty() }

    /// Gets the return statement for converting the COM result into Rust
    /// return.
    fn com_to_rust_return( &self, _result : &Ident ) -> Tokens { quote!() }

    /// Gets the return statement for converting the Rust result into COM
    /// outputs.
    fn rust_to_com_return( &self, _result : &Ident ) -> Tokens { quote!() }

    /// Gets the COM out arguments that result from the Rust return type.
    fn com_out_args( &self ) -> Vec<ComArg> { vec![] }
}

/// Void return type.
struct VoidHandler;
impl ReturnHandler for VoidHandler {
    fn rust_ty( &self ) -> Ty { utils::unit_ty() }
}

/// Simple return type with the return value as the immediate value.
struct ReturnOnlyHandler( Ty );
impl ReturnHandler for ReturnOnlyHandler {

    fn rust_ty( &self ) -> Ty { self.0.clone() }

    fn com_to_rust_return( &self, result : &Ident ) -> Tokens {
        quote!( #result )
    }

    fn rust_to_com_return( &self, result : &Ident ) -> Tokens {
        quote!( #result )
    }

    fn com_out_args( &self ) -> Vec<ComArg> { vec![] }
}

/// Result return type that is converted into COM [retval] and HRESULT return.
struct HResultHandler { retval_ty: Ty, return_ty: Ty }
impl ReturnHandler for HResultHandler {

    fn rust_ty( &self ) -> Ty { self.return_ty.clone() }

    fn com_to_rust_return( &self, result : &Ident ) -> Tokens {

        // Return statement checks for S_OK (should be is_success) HRESULT and
        // yields either Ok or Err Result based on that.
        let ok_values = get_ok_values( self.com_out_args() );
        let ok_tokens = if ok_values.len() != 1 {
                quote!( ( #( #ok_values ),* ) )
            } else {
                quote!( #( #ok_values )* )
            };
        quote!(
            if #result == intercom::S_OK {
                Ok( #ok_tokens )
            } else {
                Err( #result )
            }
        )
    }

    fn rust_to_com_return( &self, result : &Ident ) -> Tokens {

        let ( ok_writes, err_writes ) = write_out_values(
            vec![ Ident::from( "v" ) ],
            self.com_out_args() );
        quote!(
            match #result {
                Ok( v ) => { #( #ok_writes );*; intercom::S_OK },
                Err( e ) => { #( #err_writes );*; e },
            }
        )
    }

    fn com_out_args( &self ) -> Vec<ComArg> {

        if utils::is_unit( &self.retval_ty ) {
            return vec![];
        }

        // Since we have only one out value in any case, we can hardcode the
        // __out here. The only thing that relies on it being hardcoded is the
        // HResultHandler itself.
        vec![
            ComArg::new(
                Ident::from( "__out" ),
                self.retval_ty.clone()
            )
        ]
    }
}

/// Result type that supports error info for the Err value. Converted to
/// [retval] on success or HRESULT + IErrorInfo on error.
struct ErrorResultHandler { retval_ty: Ty, return_ty: Ty }
impl ReturnHandler for ErrorResultHandler {

    fn rust_ty( &self ) -> Ty { self.return_ty.clone() }
    fn com_ty( &self ) -> Ty { parse_type( "intercom::HRESULT" ).unwrap() }

    fn com_to_rust_return( &self, result : &Ident ) -> Tokens {

        // Return statement checks for S_OK (should be is_success) HRESULT and
        // yields either Ok or Err Result based on that.
        let ok_values = get_ok_values( self.com_out_args() );
        let ok_tokens = if ok_values.len() != 1 {
                quote!( ( #( #ok_values ),* ) )
            } else {
                quote!( #( #ok_values )* )
            };
        quote!(
            if #result == intercom::S_OK {
                Ok( #ok_tokens )
            } else {
                Err( intercom::get_last_error() )
            }
        )
    }

    fn rust_to_com_return( &self, result : &Ident ) -> Tokens {

        let ( ok_writes, err_writes ) = write_out_values(
            vec![ Ident::from( "v" ) ],
            self.com_out_args() );
        quote!(
            match #result {
                Ok( v ) => { #( #ok_writes );*; intercom::S_OK },
                Err( e ) => {
                    #( #err_writes );*;
                    intercom::return_hresult( e )
                },
            }
        )
    }

    fn com_out_args( &self ) -> Vec<ComArg> {

        if utils::is_unit( &self.retval_ty ) {
            return vec![];
        }

        // Since we have only one out value in any case, we can hardcode the
        // __out here. The only thing that relies on it being hardcoded is the
        // HResultHandler itself.
        vec![
            ComArg::new(
                Ident::from( "__out" ),
                self.retval_ty.clone()
            )
        ]
    }
}

fn write_out_values(
    idents : Vec<Ident>,
    out_args : Vec<ComArg>
) -> ( Vec<Tokens>, Vec<Tokens> )
{
    let mut ok_tokens = vec![];
    let mut err_tokens = vec![];
    for ( ident, out_arg ) in idents.iter().zip( out_args ) {

        let arg_name = out_arg.name;
        let ok_value = out_arg.handler.rust_to_com( ident );
        let err_value = out_arg.handler.default_value();
        ok_tokens.push( quote!( *#arg_name = #ok_value ) );
        err_tokens.push( quote!( *#arg_name = #err_value ) );
    }

    ( ok_tokens, err_tokens )
}

fn get_ok_values(
    out_args : Vec<ComArg>
) -> Vec<Tokens>
{
    let mut tokens = vec![];
    for out_arg in out_args {

        let arg_value = out_arg.handler.rust_to_com(
                &Ident::from( out_arg.name ) );
        tokens.push( quote!( #arg_value ) );
    }
    tokens
}

/// Resolves the correct return handler to use.
pub fn get_return_handler(
    retval_ty : &Option< Ty >,
    return_ty : &Option< Ty >
) -> Result< Box<ReturnHandler>, () >
{
    Ok( match ( retval_ty, return_ty ) {
        ( &None, &None ) => Box::new( VoidHandler ),
        ( &None, &Some( ref ty ) )
            => Box::new( ReturnOnlyHandler( ty.clone() ) ),
        ( &Some( ref rv ), &Some( ref rt ) )
            if match rt {
                &Ty::Path( _, ref return_ty ) 
                    if return_ty.get_ident() == Ok( Ident::from( "HRESULT" ) )
                    => true,
                _ => false
            }
            => Box::new( HResultHandler {
                retval_ty: rv.clone(),
                return_ty: rt.clone()
            } ),
        ( &Some( ref rv ), &Some( ref rt ) )
            => Box::new( ErrorResultHandler {
                retval_ty: rv.clone(),
                return_ty: rt.clone(),
            } ),

        // Unsupported return scheme. Note we are using Result::Err instead of
        // Option::None here because having no return handler is unsupported
        // error case.
        _ => return Err( () ),
    } )
}
