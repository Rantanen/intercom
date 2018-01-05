#![feature(proc_macro)]
#![allow(unused_imports)]
#![feature(catch_expr)]
#![recursion_limit="128"]

use std::iter;
use std::env;
use std::str::FromStr;
use std::iter::FromIterator;

extern crate intercom_common;
use intercom_common::idents;
use intercom_common::utils;
use intercom_common::ast_converters::*;
use intercom_common::error::MacroError;
use intercom_common::methodinfo::{ComMethodInfo, Direction};
use intercom_common::model;

extern crate proc_macro;
use proc_macro::{TokenStream, LexError};
extern crate syn;
#[macro_use]
extern crate quote;

use syn::*;

/// Resolve the name of the package being compiled.
fn lib_name() -> String {

    // Cargo stores the currently compiled package in the CARGO_PKG_NAME
    // environment variable.
    env::var( "CARGO_PKG_NAME" )
        .expect( "Could not resolve package name. \
                 Ensure CARGO_PKG_NAME environment variable is defined." )
}

// Note the rustdoc comments on the [proc_macro_attribute] functions document
// "attributes", not "functions".
//
// While at "com_interface" function creates virtual tables, etc. when it is
// invoked, the attribute doesn't "creates" these. Instead the attribute just
// "defines" the trait/impl as a COM interface.
//
// The runtime documentation for developers is present in the expand_...
// methods below.

/// Defines a COM interface.
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

/// Defines an implementation of a COM interface.
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

/// Defines a COM class that implements one or more COM interfaces.
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

/// Defines the COM library.
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

/// Expands the `com_interface` attribute.
///
/// The attribute expansion results in the following items:
///
/// - Global IID for the interface.
/// - Virtual table struct definition for the interface.
/// - Implementation for the delegating methods when calling the COM interface
///   from Rust.
fn expand_com_interface(
    attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, MacroError>
{
    // Parse the attribute.
    let mut output = vec![];
    let itf = model::ComInterface::parse(
            &lib_name(), &attr_tokens.to_string(), &item_tokens.to_string() )?;
    let itf_ident = itf.name();
    let visibility = itf.visibility();
    let iid_ident = idents::iid( itf.name() );
    let vtable_ident = idents::vtable_struct( itf.name() );

    // IID_IInterface GUID.
    let iid_tokens = utils::get_guid_tokens( itf.iid() );
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
    if let &Some( ref base ) = itf.base_interface() {
        let vtbl = match base.as_ref() {
            "IUnknown" => quote!( ::intercom::IUnknownVtbl ),
            _ => { let vtbl = idents::vtable_struct( base ); quote!( #vtbl ) }
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
    for method_info in itf.methods() {

        let method_ident = &method_info.name;
        let in_out_args = method_info.raw_com_args()
                .into_iter()
                .map( |com_arg| {
                    let name = &com_arg.arg.name;
                    let com_ty = &com_arg.arg.handler.com_ty();
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
        let params = method_info.raw_com_args()
                .into_iter()
                .map( |com_arg| {
                    let name = &com_arg.arg.name;
                    match com_arg.dir {
                        Direction::In => {
                            let param = com_arg.arg.handler.rust_to_com( name );
                            quote!( #param )
                        },
                        Direction::Out | Direction::Retval
                            => quote!( &mut #name ),
                    }
                } );

        // Combine the parameters into the final parameter list.
        // This includes the 'this' pointer and both the IN and OUT
        // parameters.
        let params = iter::once( quote!( comptr ) ).chain( params );

        // Create the return statement. 
        let return_ident = Ident::from( "__result" );
        let return_statement = method_info
                .returnhandler
                .com_to_rust_return( &return_ident );

        // Create the method implementation using the bits defined above.
        let self_arg = &method_info.rust_self_arg;
        let return_ty = &method_info.rust_return_ty;
        impls.push( quote!(
            fn #method_ident( #self_arg, #( #impl_args ),* ) -> #return_ty {
                let comptr = ::intercom::ComItf::ptr( self );
                let vtbl = comptr as *const *const #vtable_ident;
                unsafe {
                    #( #out_arg_declarations )*;
                    let #return_ident = ((**vtbl).#method_ident)( #( #params ),* );
                    #return_statement
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
        output.push( quote!(
            impl #itf_ident for ::intercom::ComItf< #itf_ident > {
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

    Ok( tokens_to_tokenstream( item_tokens, output )? )
}

/// Expands the `com_impl` attribute.
///
/// The attribute expansion results in the following items:
///
/// - Implementation for the delegating methods when calling the Rust methods
///   from COM.
/// - Virtual table instance for the COM type.
fn expand_com_impl(
    _attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, MacroError>
{
    // Parse the attribute.
    let mut output = vec![] ;
    let imp = model::ComImpl::parse( &item_tokens.to_string() )?;
    let struct_ident = imp.struct_name();
    let itf_ident = imp.interface_name();
    let vtable_struct_ident = idents::vtable_struct( itf_ident );
    let vtable_instance_ident = idents::vtable_instance( struct_ident, itf_ident );
    let vtable_offset = idents::vtable_offset( struct_ident, itf_ident );

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
    let query_interface_ident = idents::method_impl(
            &struct_ident, &itf_ident, "query_interface" );
    output.push( quote!(
            #[allow(non_snake_case)]
            #[doc(hidden)]
            unsafe extern #calling_convetion fn #query_interface_ident(
                self_vtable : ::intercom::RawComPtr,
                riid : ::intercom::REFIID,
                out : *mut ::intercom::RawComPtr
            ) -> ::intercom::HRESULT
            {
                // Get the primary iunk interface by offsetting the current
                // self_vtable with the vtable offset. Once we have the primary
                // pointer we can delegate the call to the primary implementation.
                ::intercom::ComBox::< #struct_ident >::query_interface(
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
            #[doc(hidden)]
            unsafe extern #calling_convetion fn #add_ref_ident(
                self_vtable : ::intercom::RawComPtr
            ) -> u32 {
                ::intercom::ComBox::< #struct_ident >::add_ref(
                        &mut *(( self_vtable as usize - #vtable_offset() ) as *mut _ ) )
            }
        ) );

    // Release
    let release_ident = idents::method_impl(
            &struct_ident, &itf_ident, "release" );
    output.push( quote!(
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            #[doc(hidden)]
            unsafe extern #calling_convetion fn #release_ident(
                self_vtable : ::intercom::RawComPtr
            ) -> u32 {
                ::intercom::ComBox::< #struct_ident >::release_ptr(
                        ( self_vtable as usize - #vtable_offset() ) as *mut _ )
            }
        ) );

    // Start the definition fo the vtable fields. The base interface is always
    // IUnknown at this point. We might support IDispatch later, but for now
    // we only support IUnknown.
    let mut vtable_fields = vec![
        quote!(
            __base : ::intercom::IUnknownVtbl {
                query_interface : #query_interface_ident,
                add_ref : #add_ref_ident,
                release : #release_ident,
            },
        ) ];

    // Process the impl items. This gathers all COM-visible methods and defines
    // delegating calls for them. These delegating calls are the ones that are
    // invoked by the clients. The calls then convert everything to the RUST
    // interface.
    //
    // The impl may have various kinds of items - we only support the ones that
    // seem okay. So in case we encounter any errors we'll just skip the method
    // silently. This is done by breaking out of the 'catch' before adding the
    // method to the vtable fields.
    for method_info in imp.methods() {

        let method_ident = &method_info.name;
        let method_impl_ident = idents::method_impl(
                &struct_ident, &itf_ident, &method_ident.as_ref() );

        let in_out_args = method_info.raw_com_args()
                .into_iter()
                .map( |com_arg| {
                    let name = &com_arg.arg.name;
                    let com_ty = &com_arg.arg.handler.com_ty();
                    let dir = match com_arg.dir {
                        Direction::In => quote!(),
                        Direction::Out | Direction::Retval => quote!( *mut )
                    };
                    quote!( #name : #dir #com_ty )
                } );
        let self_arg = quote!( self_vtable : ::intercom::RawComPtr );
        let args = iter::once( self_arg ).chain( in_out_args );

        // Format the in and out parameters for the Rust call.
        let in_params = method_info.args
                .iter()
                .map( |ca| {
                    let param = ca.handler.com_to_rust( &ca.name );
                    quote!( #param )
                } );

        let return_ident = Ident::from( "__result" );
        let return_statement = method_info
                .returnhandler
                .rust_to_com_return( &return_ident );

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

        output.push( quote!(
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            #[doc(hidden)]
            unsafe extern #calling_convetion fn #method_impl_ident(
                #( #args ),*
            ) -> #ret_ty {
                // Acquire the reference to the ComBox. For this we need
                // to offset the current 'self_vtable' vtable pointer.
                let self_combox = ( self_vtable as usize - #vtable_offset() )
                        as *mut ::intercom::ComBox< #struct_ident >;

                #self_struct_stmt;
                let #return_ident = self_struct.#method_ident( #( #in_params ),* );

                #return_statement
            }
        ) );

        // Include the delegating method in the virtual table fields.
        vtable_fields.push( quote!( #method_ident : #method_impl_ident, ) );
    }

    // Now that we've gathered all the virtual table fields, we can finally
    // emit the virtual table instance.
    output.push( quote!(
            #[allow(non_upper_case_globals)]
            const #vtable_instance_ident : #vtable_struct_ident
                    = #vtable_struct_ident { #( #vtable_fields )* };
        ) );

    Ok( tokens_to_tokenstream( item_tokens, output )? )
}

/// Expands the `com_class` attribute.
///
/// The attribute expansion results in the following items:
///
/// - Virtual table offset values for the different interfaces.
/// - IUnknown virtual table instance.
/// - CoClass trait implementation.
fn expand_com_class(
    attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, MacroError>
{
    // Parse the attribute.
    let mut output = vec![];
    let cls = model::ComStruct::parse(
            &lib_name(), &attr_tokens.to_string(), &item_tokens.to_string() )?;
    let struct_ident = cls.name();

    // IUnknown vtable match. As the primary query_interface is implemented
    // on the root IUnknown interface, the self_vtable here should already be
    // the IUnknown we need.
    let mut query_interface_match_arms = vec![
        quote!(
            ::intercom::IID_IUnknown =>
                ( &vtables._ISupportErrorInfo )
                    as *const &::intercom::ISupportErrorInfoVtbl
                    as *mut &::intercom::ISupportErrorInfoVtbl
                    as ::intercom::RawComPtr
        ),
        quote!(
            ::intercom::IID_ISupportErrorInfo =>
                ( &vtables._ISupportErrorInfo )
                    as *const &::intercom::ISupportErrorInfoVtbl
                    as *mut &::intercom::ISupportErrorInfoVtbl
                    as ::intercom::RawComPtr
        ) ];
    let mut support_error_info_match_arms = vec![] ;

    // Gather the virtual table list struct field definitions and their values.
    // The definitions are needed when we define the virtual table list struct,
    // which is different for each com_class. The values are needed when we
    // construct the virtual table list.
    //
    // The primary IUnknown virtual table _MUST_ be at the beginning of the list.
    // This is done to ensure the IUnknown pointer matches the ComBox pointer.
    // We ensure this by defining the primary IUnknown methods on the
    // ISupportErrorInfo virtual table and having that at the beginning.
    let isupporterrorinfo_ident = Ident::from( "ISupportErrorInfo".to_owned() );
    let isupporterrorinfo_vtable_instance_ident =
            idents::vtable_instance( &struct_ident, &isupporterrorinfo_ident );
    let mut vtable_list_field_decls = vec![
        quote!( _ISupportErrorInfo : &'static ::intercom::ISupportErrorInfoVtbl ) ];
    let mut vtable_list_field_values = vec![
        quote!( _ISupportErrorInfo : &#isupporterrorinfo_vtable_instance_ident ) ];

    // Create the vtable data for the additional interfaces.
    // The data should include the match-arms for the primary query_interface
    // and the vtable offsets used for the delegating query_interface impls.
    for itf in cls.interfaces() {

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
                        &::intercom::ComBox::< #struct_ident >::null_vtable().#itf
                                as *const _ as usize
                    }
                }
        ) );

        // Add the interface in the vtable list.
        vtable_list_field_decls.push(
                quote!( #itf : &'static #vtable_struct_ident ) );
        vtable_list_field_values.push(
                quote!( #itf : &#vtable_instance_ident ) );

        // Define the query_interface match arm for the current interface.
        // This just gets the correct interface vtable reference from the list
        // of vtables.
        query_interface_match_arms.push( quote!(
            self::#iid_ident => &vtables.#itf
                    as *const &#vtable_struct_ident
                    as *mut &#vtable_struct_ident
                    as ::intercom::RawComPtr
        ) );

        // Define the support error info match arms.
        support_error_info_match_arms.push( quote!(
            self::#iid_ident => true
        ) );

        // ComStruct (which is what the struct should be constructed to)
        // can be .into()'d into ComRc and ComItf. Generate the impls for this.
        let into_expect_msg = format!(
            "query_interface( {} ) failed for {}",
            iid_ident, itf );
        output.push( quote!(
            impl From< ::intercom::ComStruct< #struct_ident > > for
                    ::intercom::ComRc< #itf > {

                fn from( source : ::intercom::ComStruct< #struct_ident >) -> Self
                {
                    // into ComItf will leave the ref count dangling.
                    // This means we can just attach to get a proper ComRc.
                    let itf : ::intercom::ComItf< #itf > = source.into();
                    ::intercom::ComRc::attach( itf )
                }
            }
        ) );
        output.push( quote!(
            impl From< ::intercom::ComStruct< #struct_ident > > for
                    ::intercom::ComItf< #itf > {

                fn from( source : ::intercom::ComStruct< #struct_ident >) -> Self
                {
                    unsafe {

                        // ComBox::query_interface is contracted to return
                        // pointer to the correct interface. We can attach
                        // safely.
                        let itf = ::intercom::ComItf::wrap(

                            // Query interface the ComBox.
                            < #struct_ident as ::intercom::CoClass >
                                ::query_interface(
                                    ::intercom::ComBox::vtable( &source ),
                                    &#iid_ident
                                ).expect( #into_expect_msg )
                        );

                        // Forget the source. We did not increment the
                        // reference count when attaching to ComRc so we must
                        // not decrement when ComStruct drops.
                        std::mem::forget( source );

                        itf
                    }
                }
            }
        ) );

        // Check if the current interface is the implicit struct interface.
        if struct_ident == &itf {

            // Implicit interface.
            //
            // This interface is unimplementable from Rust perspective as it
            // represents a struct instead of a trait. Deref on ComItf will
            // deref into the struct, which we'll do through the ComBox.
            //
            // ComBox already derefs into the struct so we'll just get the
            // ComBox here and deref that.
            output.push( quote!(
                impl ::std::ops::Deref for ::intercom::ComItf< #itf > {
                    type Target = #itf;
                    fn deref( &self ) -> &Self::Target {
                        unsafe {
                            let self_combox =
                                    ( ::intercom::ComItf::ptr( self ) as usize
                                            - #offset_ident() )
                                    as *mut ::intercom::ComBox< #struct_ident >;

                            &**self_combox
                        }
                    }
                }
            ) );
        }
    }

    /////////////////////
    // ISupportErrorInfo virtual table instance.
    //
    // The primary IUnknown virtual table is embedded in this one.
    output.push( quote!(
            #[allow(non_upper_case_globals)]
            const #isupporterrorinfo_vtable_instance_ident
                    : ::intercom::ISupportErrorInfoVtbl
                    = ::intercom::ISupportErrorInfoVtbl {
                        __base : ::intercom::IUnknownVtbl {
                            query_interface
                                : ::intercom::ComBox::< #struct_ident >
                                    ::query_interface_ptr,
                            add_ref
                                : ::intercom::ComBox::< #struct_ident >
                                    ::add_ref_ptr,
                            release
                                : ::intercom::ComBox::< #struct_ident >
                                    ::release_ptr,
                        },
                        interface_supports_error_info
                            : ::intercom::ComBox::< #struct_ident >
                                ::interface_supports_error_info_ptr,
                    };
        ) );

    // The CoClass implementation.
    //
    // Define the vtable list struct first. This lists the vtables of all the
    // interfaces that the coclass implements.

    // VTableList struct definition.
    let vtable_list_ident = idents::vtable_list( &struct_ident );
    let visibility = cls.visibility();
    output.push( quote!(
            #[allow(non_snake_case)]
            #[doc(hidden)]
            #visibility struct #vtable_list_ident {
                #( #vtable_list_field_decls ),*
            }
        ) );

    // The actual CoClass implementation.
    output.push( quote!(
            impl ::intercom::CoClass for #struct_ident {
                type VTableList = #vtable_list_ident;
                fn create_vtable_list() -> Self::VTableList {
                    #vtable_list_ident {
                        #( #vtable_list_field_values ),*
                    }
                }
                fn query_interface(
                    vtables : &Self::VTableList,
                    riid : ::intercom::REFIID,
                ) -> ::intercom::ComResult< ::intercom::RawComPtr > {
                    if riid.is_null() { return Err( ::intercom::E_NOINTERFACE ) }
                    Ok( match *unsafe { &*riid } {
                        #( #query_interface_match_arms ),*,
                        _ => return Err( ::intercom::E_NOINTERFACE )
                    } )
                }

                fn interface_supports_error_info(
                    riid : ::intercom::REFIID
                ) -> bool
                {
                    match *unsafe { &*riid } {
                        #( #support_error_info_match_arms ),*,
                        _ => false
                    }
                }
            }
        ) );

    // CLSID constant for the class.
    let clsid_ident = idents::clsid( &struct_ident );
    if let &Some( ref guid ) = cls.clsid() {
        let clsid_guid_tokens = utils::get_guid_tokens( guid );
        let clsid_doc = format!( "`{}` class ID.", struct_ident );
        let clsid_const = quote!(
            #[allow(non_upper_case_globals)]
            #[doc = #clsid_doc ]
            const #clsid_ident : ::intercom::CLSID = #clsid_guid_tokens;
        );
        output.push( clsid_const );
    }

    Ok( tokens_to_tokenstream( item_tokens, output )? )
}

/// Expands the `com_library` attribute.
///
/// The attribute expansion results in the following items:
///
/// - DllGetClassObject extern function implementation.
fn expand_com_library(
    attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, MacroError>
{
    let mut output = vec![];
    let lib = model::ComLibrary::parse( &lib_name(), &attr_tokens.to_string() )?;

    // Create the match-statmeent patterns for each supposedly visible COM class.
    let mut match_arms = vec![];
    for struct_ident in lib.coclasses() {

        // Construct the match pattern.
        let clsid_name = idents::clsid( &struct_ident );
        match_arms.push( quote!(
            self::#clsid_name =>
                Ok( ::intercom::ComBox::new(
                        #struct_ident::new()
                    ) as ::intercom::RawComPtr )
        ) );
    }

    // Implement DllGetClassObject.
    //
    // This is more or less the only symbolic entry point that the COM
    // infrastructure uses. The COM client uses this method to acquire
    // the IClassFactory interfaces that are then used to construct the
    // actual coclasses.
    let calling_convetion = get_calling_convetion();
    let dll_get_class_object = quote!(
        #[no_mangle]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[doc(hidden)]
        pub unsafe extern #calling_convetion fn DllGetClassObject(
            rclsid : ::intercom::REFCLSID,
            riid : ::intercom::REFIID,
            pout : *mut ::intercom::RawComPtr
        ) -> ::intercom::HRESULT
        {
            // Create new class factory.
            // Specify a create function that is able to create all the
            // contained coclasses.
            let mut com_struct = ::intercom::ComStruct::new(
                ::intercom::ClassFactory::new( rclsid, | clsid | {

                    match *clsid {
                        #( #match_arms, )*
                        _ => Err( ::intercom::E_NOINTERFACE ),
                    }
                } ) );
            ::intercom::ComBox::query_interface(
                    com_struct.as_mut(),
                    riid,
                    pout );

            // com_struct will drop here and decrement the referenc ecount.
            // This is okay, as the query_interface incremented it, leaving
            // it at two at this point.

            ::intercom::S_OK
        }
    );
    output.push( dll_get_class_object );

    Ok( tokens_to_tokenstream( item_tokens, output )? )
}

/// Reports errors during attribute expansion.
///
/// The proc macros don't have any sane way to report errors. The "recommended"
/// way to do this is by panicing during compilation.
fn error<E,T>(
    e: E,
    _attr: &T
) -> !
    where MacroError: From<E>
{
    panic!( "{}", MacroError::from( e ).msg )
}

fn tokens_to_tokenstream<T: IntoIterator<Item=quote::Tokens>>(
    original : TokenStream,
    tokens : T,
) -> Result<TokenStream, MacroError>
{
    Ok( TokenStream::from_iter(
        std::iter::once( original )
            .chain( std::iter::once(
                TokenStream::from_str(
                        &tokens.into_iter()
                            .map( |t| t.parse::<String>().unwrap() )
                            .fold( String::new(), |prev, next| prev + &next ) )
                    .map_err( |_| "Failed to parse generated code." )?
            ) ) ) )
}

// https://msdn.microsoft.com/en-us/library/984x0h58.aspx
#[cfg(windows)]
fn get_calling_convetion() -> &'static str { "stdcall" }

#[cfg(not(windows))]
fn get_calling_convetion() -> &'static str { "C" }

