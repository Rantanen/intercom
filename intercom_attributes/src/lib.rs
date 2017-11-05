#![feature(proc_macro)]
#![allow(unused_imports)]
#![feature(catch_expr)]
#![feature(type_ascription)]

extern crate intercom_common;
use intercom_common::idents;
use intercom_common::utils;
use intercom_common::error::MacroError;

extern crate proc_macro;
use proc_macro::{TokenStream, LexError};
extern crate syn;
#[macro_use]
extern crate quote;

use syn::*;

#[proc_macro_attribute]
pub fn com_interface(
    attr: TokenStream,
    tokens: TokenStream,
) -> TokenStream
{
    match expand_com_interface( &attr, tokens ) {
        Ok(t) => t,
        Err(e) => error(e, &attr),
    }
}

#[proc_macro_attribute]
pub fn com_impl(
    attr: TokenStream,
    tokens: TokenStream,
) -> TokenStream
{
    match expand_com_impl( &attr, tokens ) {
        Ok(t) => t,
        Err(e) => error(e, &attr),
    }
}

#[proc_macro_attribute]
pub fn com_class(
    attr: TokenStream,
    tokens: TokenStream,
) -> TokenStream
{
    match expand_com_class( &attr, tokens ) {
        Ok(t) => t,
        Err(e) => error(e, &attr),
    }
}

#[proc_macro_attribute]
pub fn com_library(
    attr: TokenStream,
    tokens: TokenStream,
) -> TokenStream
{
    match expand_com_library( &attr, tokens ) {
        Ok(t) => t,
        Err(e) => error(e, &attr),
    }
}

fn expand_com_interface(
    attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, MacroError>
{
    let ( mut output, attr, item ) =
            utils::parse_inputs( "com_interface", &attr_tokens, &item_tokens )?;

    let ( itf_ident, fns ) =
            utils::get_ident_and_fns( &item )
                .ok_or( "[com_interface(IID:&str)] must be applied to trait or struct impl" )?;

    let iid_guid = utils::get_attr_params( &attr )
            .as_ref()
            .and_then( |ref params| params.first() )
            .ok_or( "[com_interface(IID:&str)] must specify an IID".to_owned() )
            .and_then( |f| utils::parameter_to_guid( f ) )?;

    let iid_tokens = utils::get_guid_tokens( &iid_guid );
    let iid_ident = idents::iid( &itf_ident );

    // IID_IInterface GUID.
    output.push( quote!(
        #[allow(non_upper_case_globals)]
        const #iid_ident : intercom::IID = #iid_tokens;
    ) );

    // Create the base vtable field.
    // All of our interfaces inherit from IUnknown.
    let mut fields = vec![
        quote!( __base : intercom::IUnknownVtbl, )
    ];

    // Process the impl items. This gathers all COM-visible methods and defines
    // delegating calls for them. These delegating calls are the ones that are
    // invoked by the clients. The calls then convert everything to the RUST
    // interface.
    //
    // The impl may have various kinds of items - we only support the ones that
    // seem okay. So in case we encounter any errors we'll just skip the method
    // silently. This is done by breaking out of the 'catch' before adding the
    // method to the vtable fields.
    for ( method_ident, method_sig ) in fns {
        do catch {

            // Get the self argument and the remaining args.
            let ( args, _) =
                    utils::get_method_args( method_sig )?;
            let ( ret_ty, _ ) =
                    utils::get_method_rvalues( &method_sig )?;

            // Create the vtable field and add it to the vector of fields.
            let arg_tokens = utils::flatten( args.iter() );
            let vtable_method_decl = quote!(
                #[allow(dead_code)]
                #method_ident :
                    unsafe extern "stdcall" fn( #arg_tokens ) -> #ret_ty,
            );

            fields.push( vtable_method_decl );
            Some(())
        };
    }

    // Create the vtable. We've already gathered all the vtable method
    // pointer fields so defining the struct is simple enough.
    let vtable_ident = idents::vtable_struct( &itf_ident );
    let field_tokens = utils::flatten( fields.iter() );
    output.push( quote!(
        #[allow(non_camel_case_types)]
        #[repr(C)]
        pub struct #vtable_ident { #field_tokens }
    ) );

    Ok( utils::tokens_to_tokenstream( output )? )
}

/// Implements the `com_impl` decorator.
fn expand_com_impl(
    attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, MacroError>
{
    let ( mut output, attr, item ) =
            utils::parse_inputs( "com_impl", &attr_tokens, &item_tokens )?;

    // Get the item info the attribute is bound to.
    let ( itf_ident_opt, struct_ident, fns )
            = utils::get_impl_data( &item )
                .ok_or( "[com_impl] must be applied to an impl" )?;
    let itf_ident = itf_ident_opt.unwrap_or( struct_ident );
    let vtable_struct_ident = idents::vtable_struct( &itf_ident );
    let vtable_instance_ident = idents::vtable_instance( &struct_ident, &itf_ident );
    let vtable_offset = idents::vtable_offset(
        &struct_ident,
        &itf_ident );


    /////////////////////
    // #itf::QueryInterface, AddRef & Release
    //
    // The primary add_ref and release. As these are on the IUnknown interface
    // the self_vtable here points to the start of the ComRef structure.

    // QueryInterface
    let query_interface_ident = idents::method_impl(
            &struct_ident, &itf_ident, "query_interface" );
    output.push( quote!(
            #[allow(non_snake_case)]
            pub unsafe extern "stdcall" fn #query_interface_ident(
                self_vtable : intercom::RawComPtr,
                riid : intercom::REFIID,
                out : *mut intercom::RawComPtr
            ) -> intercom::HRESULT
            {
                // Get the primary iunk interface by offsetting the current
                // self_vtable with the vtable offset. Once we have the primary
                // pointer we can delegate the call to the primary implementation.
                intercom::ComBox::< #struct_ident >::query_interface(
                        &mut *(( self_vtable as usize - #vtable_offset() ) as *mut _ ),
                        riid,
                        out )
            }
        ) );

    // AddRef
    let add_ref_ident = idents::method_impl(
            &struct_ident, &itf_ident, "add_ref" );
    output.push( quote!(
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            pub unsafe extern "stdcall" fn #add_ref_ident(
                self_vtable : intercom::RawComPtr
            ) -> u32 {
                intercom::ComBox::< #struct_ident >::add_ref(
                        &mut *(( self_vtable as usize - #vtable_offset() ) as *mut _ ) )
            }
        ) );

    // Release
    let release_ident = idents::method_impl(
            &struct_ident, &itf_ident, "release" );
    output.push( quote!(
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            pub unsafe extern "stdcall" fn #release_ident(
                self_vtable : intercom::RawComPtr
            ) -> u32 {
                intercom::ComBox::< #struct_ident >::release_ptr(
                        ( self_vtable as usize - #vtable_offset() ) as *mut _ )
            }
        ) );

    // Start the vtable with the IUnknown implementation.
    //
    // Note that the actual methods implementation for these bits differs from
    // the primary IUnknown methods. When the methods are being called through
    // this vtable, the self_vtable pointer will point to this vtable and not
    // the start of the CoClass instance.
    let mut vtable_fields = vec![
        quote!(
            __base : intercom::IUnknownVtbl {
                query_interface : #query_interface_ident,
                add_ref : #add_ref_ident,
                release : #release_ident,
            },
        ) ];


    // Implement the delegating calls for the coclass.
    for ( method_ident, method_sig ) in fns {
        do catch {
            let method_impl_ident = idents::method_impl(
                &struct_ident,
                &itf_ident,
                &method_ident.as_ref() );

            // Get the self argument and the remaining args.
            let ( args, params ) =
                    utils::get_method_args( method_sig )?;
            let ( ret_ty, return_statement ) =
                    utils::get_method_rvalues( &method_sig )?;

            // Define the delegating method implementation.
            //
            // Note the self_vtable here will be a pointer to the start of the
            // vtable for the current interface. To get the coclass and thus
            // the actual 'data' struct, we'll need to offset the self_vtable
            // with the vtable offset.
            let arg_tokens = utils::flatten( args.iter() );
            let param_tokens = utils::flatten( params.iter() );
            output.push( quote!(
                #[allow(non_snake_case)]
                #[allow(dead_code)]
                pub unsafe extern "stdcall" fn #method_impl_ident(
                    #arg_tokens
                ) -> #ret_ty {
                    // Acquire the reference to the ComBox. For this we need
                    // to offset the current 'self_vtable' vtable pointer.
                    let self_comptr = ( self_vtable as usize - #vtable_offset() )
                            as *mut intercom::ComBox< #struct_ident >;
                    let result = (*self_comptr).#method_ident( #param_tokens );
                    #return_statement
                }
            ) );

            vtable_fields.push( quote!( #method_ident : #method_impl_ident, ) );
            Some(())
        };
    }

    let vtable_field_tokens = utils::flatten( vtable_fields.iter() );
    output.push( quote!(
            #[allow(non_upper_case_globals)]
            const #vtable_instance_ident : #vtable_struct_ident
                    = #vtable_struct_ident { #vtable_field_tokens };
        ) );

    Ok( utils::tokens_to_tokenstream( output )? )
}

fn expand_com_class(
    attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, MacroError>
{
    let ( mut output, attr, item ) =
            utils::parse_inputs( "com_class", &attr_tokens, &item_tokens )?;

    // Get the item info the attribute is bound to.
    let struct_ident = utils::get_struct_ident_from_annotatable( &item );
    let iunk_ident = Ident::from( "IUnknown".to_owned() );

    let ( clsid_guid, itfs ) = utils::get_attr_params( &attr )
            .as_ref()
            .and_then( |ref params| params.split_first() )
            .ok_or( "[com_class(IID, itfs...)] must specify an IID".to_owned() )
            .and_then( |( f, itfs )|
                Ok( (
                    utils::parameter_to_guid( f )?,
                    ( itfs.into_iter()
                        .map( |i|
                            utils::parameter_to_ident( i )
                                .ok_or( "Invalid interface" ))
                        .collect() : Result<Vec<&Ident>, &'static str> )?
                ) ) )?;

    // IUnknown vtable match. As the primary query_interface is implemented
    // on the root IUnknown interface, the self_vtable here should already be
    // the IUnknown we need.
    let mut match_arms = vec![
        quote!(
            intercom::IID_IUnknown =>
                ( &vtables._IUnknown )
                    as *const &intercom::IUnknownVtbl
                    as *mut &intercom::IUnknownVtbl
                    as intercom::RawComPtr,
        ) ];

    // The vtable fields.
    let iunk_vtable_instance_ident =
            idents::vtable_instance( &struct_ident, &iunk_ident );
    let mut vtable_list_fields = vec![
        quote!(
            _IUnknown : &'static intercom::IUnknownVtbl,
        ) ];
    let mut vtable_list_field_values = vec![
        quote!(
            _IUnknown : &#iunk_vtable_instance_ident,
        ) ];

    // Create the vtable data for the additional interfaces.
    // The data should include the match-arms for the primary query_interface
    // and the vtable offsets used for the delegating query_interface impls.
    for itf in itfs {

        // Various idents.
        let offset_ident = idents::vtable_offset( &struct_ident, &itf );
        let iid_ident = idents::iid( &itf );
        let vtable_struct_ident = idents::vtable_struct( &itf );
        let vtable_instance_ident = idents::vtable_instance( &struct_ident, &itf );

        // Store the field offset globally. We need this offset when implementing
        // the delegating query_interface methods. The only place where we know
        // the actual layout of the vtable is here. Thus we need to store this
        // offset somewhere where the com_impl's can access it.
        //
        // Rust doesn't allow pointer derefs or conversions in consts so we'll
        // use an inline fn instead. LLVM should be able to reduce this into a
        // constant expression during compilation.
        output.push( quote!(
                #[inline(always)]
                #[allow(non_snake_case)]
                fn #offset_ident() -> usize {
                    unsafe { 
                        &intercom::ComBox::< #struct_ident >::null_vtable().#itf
                                as *const _ as usize
                    }
                }
        ) );
        utils::trace( "vtable_offset", &offset_ident.as_ref() );

        // The vtable pointer to for the ComBox vtable list.
        vtable_list_fields.push( quote!(
                #itf : &'static #vtable_struct_ident,) );
        vtable_list_field_values.push( quote!(
                #itf : &#vtable_instance_ident,) );

        // As this is the primary IUnknown query_interface, the self_vtable here
        // points to the start of the ComRef structure. The return value should
        // be the vtable corresponding to the given IID so we'll just offset
        // the self_vtable by the vtable offset.
        match_arms.push( quote!(
            self::#iid_ident => &vtables.#itf
                    as *const &#vtable_struct_ident
                    as *mut &#vtable_struct_ident
                    as intercom::RawComPtr,
        ) );
    }

    /////////////////////
    // IUnknown::QueryInterface, AddRef & Release
    //
    // The primary add_ref and release. As these are on the IUnknown interface
    // the self_vtable here points to the start of the ComRef structure.
    output.push( quote!(
            #[allow(non_upper_case_globals)]
            const #iunk_vtable_instance_ident : intercom::IUnknownVtbl
                    = intercom::IUnknownVtbl {
                        query_interface : intercom::ComBox::< #struct_ident >::query_interface_ptr,
                        add_ref : intercom::ComBox::< #struct_ident >::add_ref_ptr,
                        release : intercom::ComBox::< #struct_ident >::release_ptr,
                    };
        ) );

    // The CoClass implementation.
    //
    // Define the vtable list struct first. This lists the vtables of all the
    // interfaces that the coclass implements.
    let vtable_list_ident = idents::vtable_list( &struct_ident );
    let vtable_field_tokens = utils::flatten( vtable_list_fields.iter() );
    let vtable_value_tokens = utils::flatten( vtable_list_field_values.iter() );
    let match_arm_tokens = utils::flatten( match_arms.iter() );
    output.push( quote!(
            #[allow(non_snake_case)]
            pub struct #vtable_list_ident {
                #vtable_field_tokens
            }
        ) );
    output.push( quote!(
            #[allow(non_snake_case)]
            impl AsRef<intercom::IUnknownVtbl> for #vtable_list_ident {
                fn as_ref( &self ) -> &intercom::IUnknownVtbl {
                    &self._IUnknown
                }
            }
        ) );
    output.push( quote!(
            impl intercom::CoClass for #struct_ident {
                type VTableList = #vtable_list_ident;
                fn create_vtable_list() -> Self::VTableList {
                    #vtable_list_ident {
                        #vtable_value_tokens
                    }
                }
                fn query_interface(
                    vtables : &Self::VTableList,
                    riid : intercom::REFIID,
                ) -> intercom::ComResult< intercom::RawComPtr > {
                    if riid.is_null() { return Err( intercom::E_NOINTERFACE ) }
                    Ok( match *unsafe { &*riid } {
                        #match_arm_tokens
                        _ => return Err( intercom::E_NOINTERFACE )
                    } )
                }
            }
        ) );

    // CLSID constant for the class.
    let clsid_ident = idents::clsid( &struct_ident );
    let clsid_guid_tokens = utils::get_guid_tokens( &clsid_guid );
    let clsid_const = quote!(
        #[allow(non_upper_case_globals)]
        const #clsid_ident : intercom::CLSID = #clsid_guid_tokens;
    );
    output.push( clsid_const );

    Ok( utils::tokens_to_tokenstream( output )? )
}

fn expand_com_library(
    attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, MacroError>
{
    let ( mut output, attr, item ) =
            utils::parse_inputs( "com_library", &attr_tokens, &item_tokens )?;

    // Get the decorator parameters.
    let ( _, libid, coclasses ) = utils::parse_com_lib_tokens( attr_tokens )?;

    // Create the match-statmeent patterns for each supposedly visible COM class.
    let mut match_arms = vec![];
    for class_name in coclasses {

        // Construct the match pattern.
        let struct_ident = Ident::from( class_name );
        let clsid_name = idents::clsid( &struct_ident );
        match_arms.push( quote!(
            self::#clsid_name =>
                Ok( Box::into_raw( intercom::ComBox::new(
                        #struct_ident::new()
                    ) ) as intercom::RawComPtr ),
        ) );
    }

    // Implement DllGetClassObject.
    //
    // This is more or less the only symbolic entry point that the COM
    // infrastructure uses. The COM client uses this method to acquire
    // the IClassFactory interfaces that are then used to construct the
    // actual coclasses.
    let match_arm_tokens = utils::flatten( match_arms.iter() );
    let dll_get_class_object = quote!(
        #[no_mangle]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        pub unsafe extern "stdcall" fn DllGetClassObject(
            rclsid : intercom::REFCLSID,
            riid : intercom::REFIID,
            pout : *mut intercom::RawComPtr
        ) -> intercom::HRESULT
        {
            // Create new class factory.
            // Specify a create function that is able to create all the contained
            // coclasses.
            let mut combox = intercom::ComBox::new(
                intercom::ClassFactory::new( rclsid, | clsid | {

                    match *clsid {
                        #match_arm_tokens
                        _ => Err( intercom::E_NOINTERFACE ),
                    }
                } ) );
            intercom::ComBox::query_interface(
                    combox.as_mut(),
                    riid,
                    pout );

            // We've assigned the interface to pout, we can now
            // detach the Box from Rust memory management.
            Box::into_raw( combox );
            intercom::S_OK
        }
    );
    output.push( dll_get_class_object );

    Ok( utils::tokens_to_tokenstream( output )? )
}

fn error<E,T>(
    e: E,
    _attr: &T
) -> !
    where MacroError: From<E>
{
    panic!( "{}", MacroError::from( e ).msg )
}
