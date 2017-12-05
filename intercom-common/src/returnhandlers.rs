
use syn::*;
use quote::Tokens;
use methodinfo::ComArg;

/// Defines return handler for handling various different return type schemes.
pub trait ReturnHandler {

    /// Gets the return statement for converting the COM result into Rust
    /// return.
    fn return_statement( &self, _result : &Ident ) -> Tokens { quote!() }

    /// Gets the COM out arguments that result from the Rust return type.
    fn com_out_args( &self ) -> Vec<ComArg> { vec![] }
}

/// Void return type.
struct VoidHandler;
impl ReturnHandler for VoidHandler {
}

/// Simple return type with the return value as the immediate value.
struct ReturnOnlyHandler;
impl ReturnHandler for ReturnOnlyHandler {

    fn return_statement( &self, result : &Ident ) -> Tokens {
        quote!( #result )
    }

    fn com_out_args( &self ) -> Vec<ComArg> { vec![] }
}

/// Result return type that is converted into COM [retval] and HRESULT return.
struct ResultHandler { retval_ty: Ty }
impl ReturnHandler for ResultHandler {

    fn return_statement( &self, result : &Ident ) -> Tokens {

        // Return statement checks for S_OK (should be is_success) HRESULT and
        // yields either Ok or Err Result based on that.
        //
        // Note we are using .into() here as we know the type of the temporary
        // parameter is compatible with the Result type. Any Rust/COM type
        // conversions take place during the method invocation.
        //
        // NOTE: This is assumption is probably faulty as the method invocation
        //       is really dumb and just does &mut Foo for the out values.
        //       We'll probably need to use the TyHandler of the com_out_arg
        //       ComArg here to convert the com_ty into rust type.
        quote!(
            if #result == intercom::S_OK {
                Ok( __out.into() )
            } else {
                Err( #result )
            }
        )
    }

    fn com_out_args( &self ) -> Vec<ComArg> {

        // Since we have only one out value in any case, we can hardcode the
        // __out here. The only thing that relies on it being hardcoded is the
        // ResultHandler itself.
        vec![
            ComArg::new(
                Ident::from( "__out" ),
                self.retval_ty.clone()
            )
        ]
    }
}

/// Resolves the correct return handler to use.
pub fn get_return_handler(
    retval_ty : &Option< Ty >,
    return_ty : &Option< Ty >
) -> Result< Box<ReturnHandler>, () >
{
    Ok( match ( retval_ty, return_ty ) {
        ( &None, &None ) => Box::new( VoidHandler ),
        ( &None, &Some(..) )
            => Box::new( ReturnOnlyHandler ),
        ( &Some( ref rv ), &Some(..) )
            => Box::new( ResultHandler { retval_ty: rv.clone() } ),

        // Unsupported return scheme. Note we are using Result::Err instead of
        // Option::None here because having no return handler is unsupported
        // error case.
        _ => return Err( () ),
    } )
}
