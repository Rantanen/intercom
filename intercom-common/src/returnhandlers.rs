
use syn::*;
use quote::Tokens;
use methodinfo::RustArg;
use tyhandlers;
use utils;

/// Defines return handler for handling various different return type schemes.
pub trait ReturnHandler {

    /// The return type of the original Rust method.
    fn rust_ty( &self ) -> Ty;

    /// The return type for COM implementation.
    fn com_ty( &self ) -> Ty
    {
        tyhandlers::get_ty_handler( &self.rust_ty() ).com_ty()
    }

    /// Gets the return statement for converting the COM result into Rust
    /// return.
    fn com_to_rust_return( &self, _result : &Ident ) -> Tokens { quote!() }

    /// Gets the return statement for converting the Rust result into COM
    /// outputs.
    fn rust_to_com_return( &self, _result : &Ident ) -> Tokens { quote!() }

    /// Gets the COM out arguments that result from the Rust return type.
    fn com_out_args( &self ) -> Vec<RustArg> { vec![] }
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
        quote!( #result.into() )
    }

    fn rust_to_com_return( &self, result : &Ident ) -> Tokens {
        quote!( #result.into() )
    }

    fn com_out_args( &self ) -> Vec<RustArg> { vec![] }
}

/// Result type that supports error info for the `Err` value. Converted to
/// `[retval]` on success or `HRESULT` + `IErrorInfo` on error.
struct ErrorResultHandler { retval_ty: Ty, return_ty: Ty }
impl ReturnHandler for ErrorResultHandler {

    fn rust_ty( &self ) -> Ty { self.return_ty.clone() }
    fn com_ty( &self ) -> Ty { parse_type( "::intercom::HRESULT" ).unwrap() }

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
            if #result == ::intercom::S_OK {
                Ok( #ok_tokens )
            } else {
                Err( ::intercom::get_last_error( #result ) )
            }
        )
    }

    fn rust_to_com_return( &self, result : &Ident ) -> Tokens {

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
                Ty::Tup( _ ) => quote!( ( #( #rok_idents ),* ) ),

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

    fn com_out_args( &self ) -> Vec<RustArg> {
        get_out_args_for_result( &self.retval_ty )
    }
}

fn get_out_args_for_result( retval_ty : &Ty ) -> Vec<RustArg> {

    match *retval_ty {

        // Tuples map to multiple out args, no [retval].
        Ty::Tup( ref v ) =>
            v.iter()
                .enumerate()
                .map( |( idx, ty )| RustArg::new(
                                    Ident::from( format!( "__out{}", idx + 1) ),
                                    ty.clone() ) )
                .collect::<Vec<_>>(),
        _ => vec![ RustArg::new( Ident::from( "__out" ), retval_ty.clone() ) ],
    }
}

fn write_out_values(
    idents : &[Ident],
    out_args : Vec<RustArg>,
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
    out_args : Vec<RustArg>
) -> Vec<Tokens>
{
    let mut tokens = vec![];
    for out_arg in out_args {

        let arg_value = out_arg.handler.rust_to_com( &out_arg.name );
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
