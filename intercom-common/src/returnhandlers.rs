
use syn::*;
use quote::Tokens;
use methodinfo::{ComArg, Direction};
use tyhandlers;
use utils;

/// Defines return handler for handling various different return type schemes.
pub trait ReturnHandler : ::std::fmt::Debug {

    /// The return type of the original Rust method.
    fn rust_ty( &self ) -> Type;

    /// The return type for COM implementation.
    fn com_ty( &self ) -> Type
    {
        tyhandlers::get_ty_handler(
                &self.rust_ty(),
                tyhandlers::TypeContext::retval() ).com_ty()
    }

    /// Gets the return statement for converting the COM result into Rust
    /// return.
    fn com_to_rust_return( &self, _result : Ident ) -> Tokens { quote!() }

    /// Gets the return statement for converting the Rust result into COM
    /// outputs.
    fn rust_to_com_return( &self, _result : Ident ) -> Tokens { quote!() }

    /// Gets the COM out arguments that result from the Rust return type.
    fn com_out_args( &self ) -> Vec<ComArg> { vec![] }
}

/// Void return type.
#[derive(Debug)]
struct VoidHandler;
impl ReturnHandler for VoidHandler {
    fn rust_ty( &self ) -> Type { utils::unit_ty() }
}

/// Simple return type with the return value as the immediate value.
#[derive(Debug)]
struct ReturnOnlyHandler( Type );
impl ReturnHandler for ReturnOnlyHandler {

    fn rust_ty( &self ) -> Type { self.0.clone() }

    fn com_to_rust_return( &self, result : Ident ) -> Tokens {
        let conversion = tyhandlers::get_ty_handler(
                &self.rust_ty(),
                tyhandlers::TypeContext::retval() ).com_to_rust( result );
        if conversion.temporary.is_some() {
            panic!( "Return values cannot depend on temporaries" );
        }

        conversion.value
    }

    fn rust_to_com_return( &self, result : Ident ) -> Tokens {
        let conversion = tyhandlers::get_ty_handler(
                &self.rust_ty(),
                tyhandlers::TypeContext::retval() ).rust_to_com( result );
        if conversion.temporary.is_some() {
            panic!( "Return values cannot depend on temporaries" );
        }

        conversion.value
    }

    fn com_out_args( &self ) -> Vec<ComArg> { vec![] }
}

/// Result type that supports error info for the `Err` value. Converted to
/// `[retval]` on success or `HRESULT` + `IErrorInfo` on error.
#[derive(Debug)]
struct ErrorResultHandler { retval_ty: Type, return_ty: Type }
impl ReturnHandler for ErrorResultHandler {

    fn rust_ty( &self ) -> Type { self.return_ty.clone() }
    fn com_ty( &self ) -> Type { parse_quote!( ::intercom::HRESULT ) }

    fn com_to_rust_return( &self, result : Ident ) -> Tokens {

        // Format the final Ok value.
        // If there is only one, it should be a raw value;
        // If there are multiple value turn them into a tuple.
        let ok_values = get_rust_ok_values( self.com_out_args() );
        let ok_tokens = if ok_values.len() != 1 {
                quote!( ( #( #ok_values ),* ) )
            } else {
                quote!( #( #ok_values )* )
            };

        // Return statement checks for S_OK (should be is_success) HRESULT and
        // yields either Ok or Err Result based on that.
        quote!(
            if #result == ::intercom::S_OK {
                Ok( #ok_tokens )
            } else {
                Err( ::intercom::get_last_error( #result ) )
            }
        )
    }

    fn rust_to_com_return( &self, result : Ident ) -> Tokens {

        // Get the OK idents. We'll use v0, v1, v2, ... depending on the amount
        // of patterns we need for possible tuples.
        let ok_idents = self.com_out_args()
                    .iter()
                    .enumerate()
                    .map( |(idx, _)| Ident::from( format!( "v{}", idx + 1 ) ) )
                    .collect::<Vec<_>>();

        // Generate the pattern for the Ok(..).
        // Tuples get (v0, v1, v2, ..) pattern while everything else is
        // represented with just Ok( v0 ) as there's just one parameter.
        let ok_pattern = {
            // quote! takes ownership of tokens if we allow so let's give them
            // by reference here.
            let rok_idents = &ok_idents;
            match self.retval_ty {
                Type::Tuple( _ ) => quote!( ( #( #rok_idents ),* ) ),

                // Non-tuples should have only one ident. Concatenate the vector.
                _ => quote!( #( #rok_idents )* ),
            }
        };

        let ( ok_writes, err_writes ) = write_out_values(
            &ok_idents,
            self.com_out_args() );
        quote!(
            match #result {
                Ok( #ok_pattern ) => { #( #ok_writes );*; ::intercom::S_OK },
                Err( e ) => {
                    #( #err_writes );*;
                    ::intercom::return_hresult( e )
                },
            }
        )
    }

    fn com_out_args( &self ) -> Vec<ComArg> {
        get_out_args_for_result( &self.retval_ty )
    }
}

fn get_out_args_for_result( retval_ty : &Type ) -> Vec<ComArg> {

    match *retval_ty {

        // Tuples map to multiple out args, no [retval].
        Type::Tuple( ref t ) =>
            t.elems.iter()
                .enumerate()
                .map( |( idx, ty )| ComArg::new(
                            Ident::from( format!( "__out{}", idx + 1) ),
                            ty.clone(),
                            Direction::Out ) )
                .collect::<Vec<_>>(),
        _ => vec![ ComArg::new(
                Ident::from( "__out" ),
                retval_ty.clone(),
                Direction::Retval ) ],
    }
}

fn write_out_values(
    idents : &[Ident],
    out_args : Vec<ComArg>,
) -> ( Vec<Tokens>, Vec<Tokens> )
{
    let mut ok_tokens = vec![];
    let mut err_tokens = vec![];
    for ( ident, out_arg ) in idents.iter().zip( out_args ) {

        let arg_name = out_arg.name;
        let ok_conversion = out_arg.handler.rust_to_com( *ident );
        let err_value = out_arg.handler.default_value();

        if ok_conversion.temporary.is_some() {
            panic!( "Return values cannot depend on temporaries" );
        }

        let ok_value = ok_conversion.value;
        ok_tokens.push( quote!( *#arg_name = #ok_value ) );
        err_tokens.push( quote!( *#arg_name = #err_value ) );
    }

    ( ok_tokens, err_tokens )
}

/// Gets the result as Rust types for a success return value.
fn get_rust_ok_values(
    out_args : Vec<ComArg>
) -> Vec<Tokens>
{
    let mut tokens = vec![];
    for out_arg in out_args {

        let conversion = out_arg.handler.com_to_rust( out_arg.name );
        if conversion.temporary.is_some() {
            panic!( "Return values cannot depend on temporaries" );
        }

        tokens.push( conversion.value );
    }
    tokens
}

/// Resolves the correct return handler to use.
pub fn get_return_handler(
    retval_ty : &Option< Type >,
    return_ty : &Option< Type >
) -> Result< Box<ReturnHandler>, () >
{
    Ok( match ( retval_ty, return_ty ) {
        ( &None, &None ) => Box::new( VoidHandler ),
        ( &None, &Some( ref ty ) )
            => Box::new( ReturnOnlyHandler( ty.clone() ) ),
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
