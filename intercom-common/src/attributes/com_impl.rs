use super::common::*;
use crate::prelude::*;

use std::iter;

use crate::idents;
use crate::model;
use crate::tyhandlers::Direction;

extern crate proc_macro;
use self::proc_macro::TokenStream;

/// Expands the `com_impl` attribute.
///
/// The attribute expansion results in the following items:
///
/// - Implementation for the delegating methods when calling the Rust methods
///   from COM.
/// - Virtual table instance for the COM type.
pub fn expand_com_impl(
    _attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, model::ParseError>
{
    // Parse the attribute.
    let mut output = vec![];
    let imp = model::ComImpl::parse(&item_tokens.to_string())?;
    let struct_ident = imp.struct_name();
    let itf_ident = imp.interface_name();

    for (_, impl_variant) in imp.variants() {
        let itf_unique_ident = impl_variant.interface_unique_name();
        let vtable_struct_ident = idents::vtable_struct(&itf_unique_ident);
        let vtable_instance_ident = idents::vtable_instance(&struct_ident, &itf_unique_ident);
        let vtable_offset = idents::vtable_offset(&struct_ident, &itf_unique_ident);

        /////////////////////
        // #itf::QueryInterface, AddRef & Release
        //
        // Note that the actual methods implementation for these bits differs from
        // the primary IUnknown methods. When the methods are being called through
        // this vtable, the self_vtable pointer will point to this vtable and not
        // the start of the CoClass instance.
        //
        // We can convert these to the ComBox references by offsetting the pointer
        // by the known vtable offset.

        // QueryInterface
        let calling_convetion = get_calling_convetion();
        let query_interface_ident =
            idents::method_impl(&struct_ident, &itf_unique_ident, "query_interface");
        output.push(quote!(
            #[allow(non_snake_case)]
            #[doc(hidden)]
            unsafe extern #calling_convetion fn #query_interface_ident(
                self_vtable : intercom::RawComPtr,
                riid : <intercom::REFIID as intercom::type_system::ExternType<
                        intercom::type_system::AutomationTypeSystem>>
                            ::ExternInputType,
                out : *mut <intercom::RawComPtr as intercom::type_system::ExternType<
                        intercom::type_system::AutomationTypeSystem>>
                            ::ExternOutputType,
            ) -> <intercom::raw::HRESULT as intercom::type_system::ExternType<
                    intercom::type_system::AutomationTypeSystem>>
                        ::ExternOutputType
            {
                // Get the primary iunk interface by offsetting the current
                // self_vtable with the vtable offset. Once we have the primary
                // pointer we can delegate the call to the primary implementation.
                intercom::ComBox::< #struct_ident >::query_interface(
                        &mut *(( self_vtable as usize - #vtable_offset() ) as *mut _ ),
                        riid,
                        out )
            }
        ));

        // AddRef
        let add_ref_ident = idents::method_impl(&struct_ident, &itf_unique_ident, "add_ref");
        output.push(quote!(
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            #[doc(hidden)]
            unsafe extern #calling_convetion fn #add_ref_ident(
                self_vtable : intercom::RawComPtr
            ) -> <u32 as intercom::type_system::ExternType<
                    intercom::type_system::AutomationTypeSystem>>
                        ::ExternOutputType
            {
                intercom::ComBox::< #struct_ident >::add_ref(
                        &mut *(( self_vtable as usize - #vtable_offset() ) as *mut _ ) )
            }
        ));

        // Release
        let release_ident = idents::method_impl(&struct_ident, &itf_unique_ident, "release");
        output.push(quote!(
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            #[doc(hidden)]
            unsafe extern #calling_convetion fn #release_ident(
                self_vtable : intercom::RawComPtr
            ) -> <u32 as intercom::type_system::ExternType<
                    intercom::type_system::AutomationTypeSystem>>
                        ::ExternOutputType
            {
                intercom::ComBox::< #struct_ident >::release_ptr(
                        ( self_vtable as usize - #vtable_offset() ) as *mut _ )
            }
        ));

        // Start the definition fo the vtable fields. The base interface is always
        // IUnknown at this point. We might support IDispatch later, but for now
        // we only support IUnknown.
        let mut vtable_fields = vec![quote!(
            __base : intercom::IUnknownVtbl {
                query_interface_Automation : #query_interface_ident,
                add_ref_Automation : #add_ref_ident,
                release_Automation : #release_ident,
            },
        )];

        // Process the impl items. This gathers all COM-visible methods and defines
        // delegating calls for them. These delegating calls are the ones that are
        // invoked by the clients. The calls then convert everything to the RUST
        // interface.
        //
        // The impl may have various kinds of items - we only support the ones that
        // seem okay. So in case we encounter any errors we'll just skip the method
        // silently. This is done by breaking out of the 'catch' before adding the
        // method to the vtable fields.
        for method_info in impl_variant.methods() {
            let method_ident = &method_info.unique_name;
            let method_rust_ident = &method_info.display_name;
            let method_impl_ident =
                idents::method_impl(&struct_ident, &itf_unique_ident, &method_ident.to_string());

            let in_out_args = method_info.raw_com_args().into_iter().map(|com_arg| {
                let name = &com_arg.name;
                let com_ty = &com_arg.handler.com_ty(com_arg.dir);
                let dir = match com_arg.dir {
                    Direction::In => quote!(),
                    Direction::Out | Direction::Retval => quote!( *mut ),
                };
                quote!( #name : #dir #com_ty )
            });
            let self_arg = quote!(self_vtable: intercom::RawComPtr);
            let args = iter::once(self_arg).chain(in_out_args);

            // Format the in and out parameters for the Rust call.
            let in_params: Vec<_> = method_info
                .args
                .iter()
                .map(|ca| ca.handler.com_to_rust(&ca.name, Direction::In))
                .collect();

            let return_ident = Ident::new("__result", Span::call_site());
            let return_statement = method_info.returnhandler.rust_to_com_return(&return_ident);

            // Define the delegating method implementation.
            //
            // Note the self_vtable here will be a pointer to the start of the
            // vtable for the current interface. To get the coclass and thus
            // the actual 'data' struct, we'll need to offset the self_vtable
            // with the vtable offset.
            let ret_ty = method_info.returnhandler.com_ty();
            let self_struct_stmt = if method_info.is_const {
                quote!( let self_struct : &#itf_ident = &**self_combox )
            } else {
                quote!( let self_struct : &mut #itf_ident = &mut **self_combox )
            };

            output.push(quote!(
                #[allow(non_snake_case)]
                #[allow(dead_code)]
                #[doc(hidden)]
                unsafe extern #calling_convetion fn #method_impl_ident(
                    #( #args ),*
                ) -> #ret_ty {
                    use intercom::type_system::{IntercomFrom, IntercomInto};
                    let result : Result< #ret_ty, intercom::ComError > = ( || {
                        // Acquire the reference to the ComBox. For this we need
                        // to offset the current 'self_vtable' vtable pointer.
                        let self_combox = ( self_vtable as usize - #vtable_offset() )
                                as *mut intercom::ComBox< #struct_ident >;

                        #self_struct_stmt;
                        let #return_ident = self_struct.#method_rust_ident( #( #in_params ),* );

                        Ok( { #return_statement } )
                    } )();

                    use intercom::ErrorValue;
                    match result {
                        Ok( v ) => v,
                        Err( err ) => < #ret_ty as ErrorValue >::from_error(
                                intercom::store_error( err ) ),
                    }
                }
            ));

            // Include the delegating method in the virtual table fields.
            vtable_fields.push(quote!( #method_ident : #method_impl_ident, ));
        }

        // Now that we've gathered all the virtual table fields, we can finally
        // emit the virtual table instance.
        output.push(quote!(
            #[allow(non_upper_case_globals)]
            const #vtable_instance_ident : #vtable_struct_ident
                    = #vtable_struct_ident { #( #vtable_fields )* };
        ));
    }

    output.push(quote!(
        impl intercom::HasInterface< #itf_ident > for #struct_ident {}
    ));

    Ok(tokens_to_tokenstream(item_tokens, output))
}
