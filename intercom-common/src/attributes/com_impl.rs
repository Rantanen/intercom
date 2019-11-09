use super::common::*;
use crate::prelude::*;

use std::iter;

use crate::idents;
use crate::model;
use crate::tyhandlers::Direction;

extern crate proc_macro;
use self::proc_macro::TokenStream;
use syn::spanned::Spanned;

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
    let imp = model::ComImpl::parse(item_tokens.clone().into())?;
    let struct_path = imp.struct_path;
    let struct_ident = imp.struct_ident;
    let itf_path = imp.interface_path;
    let itf_ident = imp.interface_ident;
    let maybe_dyn = match imp.is_trait_impl {
        true => quote_spanned!(itf_path.span() => dyn),
        false => quote!(),
    };

    for (_, impl_variant) in imp.variants {
        let ts = impl_variant.type_system;
        let ts_tokens = impl_variant
            .type_system
            .as_typesystem_type(struct_path.span());
        let vtable_offset = quote!(
            <#struct_path as intercom::attributes::ComClass<#maybe_dyn #itf_path, #ts_tokens>>::offset()
        );

        /////////////////////
        // #itf::QueryInterface, AddRef & Release
        //
        // Note that the actual methods implementation for these bits differs from
        // the primary IUnknown methods. When the methods are being called through
        // this vtable, the self_vtable pointer will point to this vtable and not
        // the start of the CoClass instance.
        //
        // We can convert these to the ComBoxData references by offsetting the pointer
        // by the known vtable offset.

        // QueryInterface
        let query_interface_ident =
            idents::method_impl(&struct_ident, &itf_ident, "query_interface", ts);
        let struct_name = struct_ident.to_string();
        output.push(quote_spanned!(struct_path.span() =>
            #[allow(non_snake_case)]
            #[doc(hidden)]
            unsafe extern "system" fn #query_interface_ident(
                self_vtable : intercom::raw::RawComPtr,
                riid : <intercom::REFIID as intercom::type_system::ExternInput<
                        intercom::type_system::AutomationTypeSystem>>
                            ::ForeignType,
                out : *mut <intercom::raw::RawComPtr as intercom::type_system::ExternOutput<
                        intercom::type_system::AutomationTypeSystem>>
                            ::ForeignType,
            ) -> <intercom::raw::HRESULT as intercom::type_system::ExternOutput<
                    intercom::type_system::AutomationTypeSystem>>
                        ::ForeignType
            {
                // Get the primary iunk interface by offsetting the current
                // self_vtable with the vtable offset. Once we have the primary
                // pointer we can delegate the call to the primary implementation.
                let self_ptr = ( self_vtable as usize - #vtable_offset ) as *mut _;
                intercom::logging::trace(|l| l(module_path!(), format_args!(
                    "[{:p}, through {:p}] Serving {}::query_interface",
                    self_ptr, self_vtable, #struct_name)));
                intercom::ComBoxData::< #struct_path >::query_interface(
                        &mut *self_ptr,
                        riid,
                        out )
            }
        ));

        // AddRef
        let add_ref_ident = idents::method_impl(&struct_ident, &itf_ident, "add_ref", ts);
        output.push(quote_spanned!(struct_path.span() =>
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            #[doc(hidden)]
            unsafe extern "system" fn #add_ref_ident(
                self_vtable : intercom::raw::RawComPtr
            ) -> <u32 as intercom::type_system::ExternOutput<
                    intercom::type_system::AutomationTypeSystem>>
                        ::ForeignType
            {
                let self_ptr = ( self_vtable as usize - #vtable_offset ) as *mut _;
                intercom::logging::trace(|l| l(module_path!(), format_args!(
                    "[{:p}, through {:p}] Serving {}::add_ref",
                    self_ptr, self_vtable, #struct_name)));
                intercom::ComBoxData::< #struct_path >::add_ref_ptr(self_ptr)
            }
        ));

        // Release
        let release_ident = idents::method_impl(&struct_ident, &itf_ident, "release", ts);
        output.push(quote_spanned!(struct_path.span() =>
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            #[doc(hidden)]
            unsafe extern "system" fn #release_ident(
                self_vtable : intercom::raw::RawComPtr
            ) -> <u32 as intercom::type_system::ExternOutput<
                    intercom::type_system::AutomationTypeSystem>>
                        ::ForeignType
            {
                let self_ptr = ( self_vtable as usize - #vtable_offset ) as *mut _;
                intercom::logging::trace(|l| l(module_path!(), format_args!(
                    "[{:p}, through {:p}] Serving {}::release_ptr",
                    self_ptr, self_vtable, #struct_name)));
                intercom::ComBoxData::< #struct_path >::release_ptr(self_ptr)
            }
        ));

        // Start the definition fo the vtable fields. The base interface is always
        // IUnknown at this point. We might support IDispatch later, but for now
        // we only support IUnknown.
        let iunk_vtbl_type = quote_spanned!(itf_path.span() =>
            <dyn intercom::IUnknown as intercom::attributes::ComInterface<intercom::type_system::AutomationTypeSystem>>::VTable);
        let mut vtable_fields = vec![quote_spanned!(struct_path.span() =>
            __base : {
                type TVtbl = #iunk_vtbl_type;
                TVtbl {
                    query_interface : #query_interface_ident,
                    add_ref : #add_ref_ident,
                    release : #release_ident,
                }
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
        for method_info in impl_variant.methods {
            let method_ident = &method_info.name;
            let method_rust_ident = &method_info.name;
            let method_impl_ident =
                idents::method_impl(&struct_ident, &itf_ident, &method_ident, ts);
            let infallible = method_info.returnhandler.is_infallible();

            let in_out_args = method_info.raw_com_args().into_iter().map(|com_arg| {
                let name = &com_arg.name;
                let com_ty = &com_arg
                    .handler
                    .com_ty(com_arg.span, com_arg.dir, infallible);
                let dir = match com_arg.dir {
                    Direction::In => quote!(),
                    Direction::Out | Direction::Retval => quote!( *mut ),
                };
                quote!( #name : #dir #com_ty )
            });
            let self_arg = quote!(self_vtable: intercom::raw::RawComPtr);
            let args = iter::once(self_arg).chain(in_out_args);

            // Format the in and out parameters for the Rust call.
            let in_params: Vec<_> = method_info
                .args
                .iter()
                .map(|ca| {
                    ca.handler
                        .com_to_rust(&ca.name, ca.span, Direction::In, infallible)
                })
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
                quote!( let self_struct : &#maybe_dyn #itf_path = &**self_combox )
            } else {
                quote!( let self_struct : &mut #maybe_dyn #itf_path = &mut **self_combox )
            };

            let method_name = method_ident.to_string();
            if infallible {
                output.push(quote!(
                    #[allow(non_snake_case)]
                    #[allow(dead_code)]
                    #[doc(hidden)]
                    unsafe extern "system" fn #method_impl_ident(
                        #( #args ),*
                    ) -> #ret_ty {
                        // Acquire the reference to the ComBoxData. For this we need
                        // to offset the current 'self_vtable' vtable pointer.
                        let self_combox = ( self_vtable as usize - #vtable_offset )
                                as *mut intercom::ComBoxData< #struct_path >;

                        intercom::logging::trace(|l| l(module_path!(), format_args!(
                            "[{:p}, through {:p}] Serving {}::{}",
                            self_combox, self_vtable, #struct_name, #method_name)));

                        #self_struct_stmt;
                        let #return_ident = self_struct.#method_rust_ident( #( #in_params ),* );

                        intercom::logging::trace(|l| l(module_path!(), format_args!(
                            "[{:p}, through {:p}] Serving {}::{}, OK",
                            self_combox, self_vtable, #struct_name, #method_name)));

                        #return_statement
                    }
                ));
            } else {
                output.push(quote!(
                    #[allow(non_snake_case)]
                    #[allow(dead_code)]
                    #[doc(hidden)]
                    unsafe extern "system" fn #method_impl_ident(
                        #( #args ),*
                    ) -> #ret_ty {
                        // Acquire the reference to the ComBoxData. For this we need
                        // to offset the current 'self_vtable' vtable pointer.
                        let self_combox = ( self_vtable as usize - #vtable_offset )
                                as *mut intercom::ComBoxData< #struct_path >;

                        let result : Result< #ret_ty, intercom::ComError > = ( || {
                            intercom::logging::trace(|l| l(module_path!(), format_args!(
                                "[{:p}, through {:p}] Serving {}::{}",
                                self_combox, self_vtable, #struct_name, #method_name)));

                            #self_struct_stmt;
                            let #return_ident = self_struct.#method_rust_ident( #( #in_params ),* );

                            Ok( { #return_statement } )
                        } )();

                        use intercom::ErrorValue;
                        match result {
                            Ok( v ) => {
                                intercom::logging::trace(|l| l(module_path!(), format_args!(
                                    "[{:p}, through {:p}] Serving {}::{}, OK",
                                    self_combox, self_vtable, #struct_name, #method_name)));
                                v
                            },
                            Err( err ) => {
                                intercom::logging::trace(|l| l(module_path!(), format_args!(
                                    "[{:p}, through {:p}] Serving {}::{}, ERROR",
                                    self_combox, self_vtable, #struct_name, #method_name)));
                                <#ret_ty as ErrorValue>::from_error(
                                    intercom::store_error(err))
                            },
                        }
                    }
                ));
            }

            // Include the delegating method in the virtual table fields.
            vtable_fields.push(quote!(
                #[allow(non_snake_case)]
                #method_ident : #method_impl_ident,
            ));
        }

        // Now that we've gathered all the virtual table fields, we can finally
        // emit the virtual table instance.
        let attrib_data = quote_spanned!(itf_path.span() =>
            <#maybe_dyn #itf_path as intercom::attributes::ComInterface<#ts_tokens>>);
        output.push(quote_spanned!(itf_path.span() =>
            #[allow(non_upper_case_globals)]
            impl intercom::attributes::ComImpl<
                #maybe_dyn #itf_path, #ts_tokens>
                for #struct_path
            {
                fn vtable() -> &'static #attrib_data::VTable {
                    type T = #attrib_data::VTable;
                    & T { #( #vtable_fields )* }
                }
            }
        ));
    }

    output.push(quote_spanned!(imp.impl_span =>
        impl intercom::HasInterface<#maybe_dyn #itf_path> for #struct_path {}
    ));

    Ok(tokens_to_tokenstream(item_tokens, output))
}
