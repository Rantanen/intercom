
use syn::*;
use quote::Tokens;
use methodinfo::ComArg;

pub trait ReturnHandler {
    fn return_statement( &self, _result : &Ident ) -> Tokens { quote!() }
    fn com_out_args( &self ) -> Vec<ComArg> { vec![] }
}

struct VoidHandler;
impl ReturnHandler for VoidHandler {
}

struct ReturnOnlyHandler;
impl ReturnHandler for ReturnOnlyHandler {

    fn return_statement( &self, result : &Ident ) -> Tokens {
        quote!( #result )
    }

    fn com_out_args( &self ) -> Vec<ComArg> { vec![] }
}

struct ResultHandler { retval_ty: Ty }
impl ReturnHandler for ResultHandler {

    fn return_statement( &self, result : &Ident ) -> Tokens {
        quote!(
            if #result == intercom::S_OK {
                Ok( __out.into() )
            } else {
                Err( #result )
            }
        )
    }

    fn com_out_args( &self ) -> Vec<ComArg> {
        vec![
            ComArg::new(
                Ident::from( "__out" ),
                self.retval_ty.clone()
            )
        ]
    }
}

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
        _ => return Err( () ),
    } )
}
