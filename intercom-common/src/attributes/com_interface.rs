
use prelude::*;
use super::common::*;

use std::iter;

use idents;
use utils;
use tyhandlers::Direction;
use model;

extern crate proc_macro;
use self::proc_macro::TokenStream;
use syn::*;

/// Expands the `com_interface` attribute.
///
/// The attribute expansion results in the following items:
///
/// - Global IID for the interface.
/// - Virtual table struct definition for the interface.
/// - Implementation for the delegating methods when calling the COM interface
///   from Rust.
pub fn expand_com_interface(
    attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, model::ParseError>
{
    // Parse the attribute.
    let mut output = vec![];
    let itf = model::ComInterface::parse(
            &lib_name(),
            &attr_tokens.to_string(),
            &item_tokens.to_string() )?;
    let itf_ident = itf.name();
    let visibility = itf.visibility();
    let iid_ident = idents::iid( itf.name() );
    let vtable_ident = idents::vtable_struct( itf.name() );

    // IID_IInterface GUID.
    let iid_tokens = utils::get_guid_tokens( itf.aut().iid() );
    let iid_doc = format!( "`{}` interface ID.", itf_ident );
    output.push( quote!(
        #[doc = #iid_doc]
        #[allow(non_upper_case_globals)]
        #visibility const #iid_ident : ::intercom::IID = #iid_tokens;
    ) );

    // IidOf implementation.
    let iidof_doc = format!( "Returns `{}`.", iid_ident );
    output.push( quote!(
        impl ::intercom::IidOf for #itf_ident {
            #[doc = #iidof_doc]
            fn iid() -> &'static ::intercom::IID {
                & #iid_ident
            }
        }
    ) );

    // Create a vector for the virtual table fields and insert the base
    // interface virtual table in it if required.
    let mut fields = vec![];
    if let Some( ref base ) = *itf.base_interface() {
        let vtbl = match base.to_string().as_ref() {
            "IUnknown" => quote!( ::intercom::IUnknownVtbl ),
            _ => { let vtbl = idents::vtable_struct( &base ); quote!( #vtbl ) }
        };
        fields.push( quote!( pub __base : #vtbl ) );
    }

    // Process the trait items. Each COM-callable method on the trait will
    // result in a field in the virtual table.
    //
    // We will also create the delegating call from Rust to COM for these
    // methods.
    //
    // NOTE: Currently we are skipping methods that aren't "COM compatible".
    //       However as we need to be able to delegate the calls from Rust
    //       to COM and this requires implementing the trait for a random
    //       COM pointer, we might need to fail the traits that have COM
    //       incompatible functions instead.
    let calling_convention = get_calling_convetion();
    let mut impls = vec![];
    for method_info in itf.aut().methods() {

        let method_ident = &method_info.display_name;
        let in_out_args = method_info.raw_com_args()
                .into_iter()
                .map( |com_arg| {
                    let name = &com_arg.name;
                    let com_ty = &com_arg.handler.com_ty();
                    let dir = match com_arg.dir {
                        Direction::In => quote!(),
                        Direction::Out | Direction::Retval => quote!( *mut )
                    };
                    quote!( #name : #dir #com_ty )
                } );
        let self_arg = quote!( self_vtable : ::intercom::RawComPtr );
        let args = iter::once( self_arg ).chain( in_out_args );

        // Create the vtable field and add it to the vector of fields.
        let ret_ty = method_info.returnhandler.com_ty();
        fields.push( quote!(
            pub #method_ident :
                unsafe extern #calling_convention fn( #( #args ),* ) -> #ret_ty
        ) );

        // COM delegate implementation.

        // Format the method arguments into tokens.
        let impl_args = method_info.args.iter().map( |ca| {
            let name = &ca.name;
            let ty = &ca.ty;
            quote!( #name : #ty )
        } );

        // The COM out-arguments that mirror the Rust return value will
        // require temporary variables during the COM call. Format their
        // declarations.
        let out_arg_declarations = method_info.returnhandler.com_out_args()
                .iter()
                .map( |ca| {
                    let ident = &ca.name;
                    let ty = &ca.handler.com_ty();
                    let default = ca.handler.default_value();
                    quote!( let mut #ident : #ty = #default; )
                } ).collect::<Vec<_>>();

        // Format the in and out parameters for the COM call.
        let ( temporaries, params ) : ( Vec<_>, Vec<_> ) = method_info.raw_com_args()
                .into_iter()
                .map( |com_arg| {
                    let name = com_arg.name;
                    match com_arg.dir {
                        Direction::In => {
                            let param = com_arg.handler.rust_to_com( &name );
                            ( param.temporary, param.value )
                        },
                        Direction::Out | Direction::Retval
                            => ( None, quote!( &mut #name ) ),
                    }
                } )
                .unzip();

        // Combine the parameters into the final parameter list.
        // This includes the 'this' pointer and both the IN and OUT
        // parameters.
        let params = iter::once( quote!( comptr ) ).chain( params );

        // Create the return statement. 
        let return_ident = Ident::new( "__result", Span::call_site() );
        let return_statement = method_info
                .returnhandler
                .com_to_rust_return( &return_ident );

        // Create the method implementation using the bits defined above.
        let self_arg = &method_info.rust_self_arg;
        let return_ty = &method_info.rust_return_ty;
        let unsafety = if method_info.is_unsafe { quote!( unsafe ) } else { quote!() };
        impls.push( quote!(
            #unsafety fn #method_ident(
                #self_arg, #( #impl_args ),*
            ) -> #return_ty
            {
                #[allow(unused_imports)]
                use ::intercom::ComInto;

                let comptr = ::intercom::ComItf::ptr( self );
                let vtbl = comptr as *const *const #vtable_ident;

                #( #temporaries )*

                #[allow(unused_unsafe)]  // The fn itself _might_ be unsafe.
                let result : Result< #return_ty, ::intercom::ComError > = ( || unsafe {
                    #( #out_arg_declarations )*;
                    let #return_ident = ((**vtbl).#method_ident)( #( #params ),* );

                    Ok( { #return_statement } )
                } )();

                #[allow(unused_imports)]
                use ::intercom::ErrorValue;
                match result {
                    Ok( v ) => v,
                    Err( err ) => < #return_ty as ErrorValue >::from_error(
                            ::intercom::return_hresult( err ) ),
                }
            }
        ) );
    }

    // Create the vtable. We've already gathered all the vtable method
    // pointer fields so defining the struct is simple enough.
    output.push( quote!(
        #[allow(non_camel_case_types)]
        #[repr(C)]
        #[doc(hidden)]
        #visibility struct #vtable_ident { #( #fields, )* }
    ) );

    // If this is a trait (as opposed to an implicit struct `impl`), include
    // the Rust-to-COM call implementations.
    //
    // If the [com_interface] is on an implicit struct `impl` we'd end up with
    // `impl StructName for intercom::ComItf<StructName>`, which is invalid
    // syntax when `StructName` is struct instead of a trait.
    if itf.item_type() == utils::InterfaceType::Trait {
        let unsafety = if itf.is_unsafe() { quote!( unsafe ) } else { quote!() };
        output.push( quote!(
            #unsafety impl #itf_ident for ::intercom::ComItf< #itf_ident > {
                #( #impls )*
            }
        ) );
    }

    // If this is a trait based interface, implement Deref for ComItf.
    //
    // Trait based interfaces can always be Deref'd from ComItf into &Trait.
    // Struct based interfaces can only be Deref'd if the struct has the
    // interface in its vtable list. This is decided in the com_class
    // attribute.
    if itf.item_type() == utils::InterfaceType::Trait {
        output.push( quote!(
            impl ::std::ops::Deref for ::intercom::ComItf< #itf_ident > {
                type Target = #itf_ident;
                fn deref( &self ) -> &Self::Target {
                    self
                }
            }
        ) );
    }

    Ok( tokens_to_tokenstream( item_tokens, output ) )
}

