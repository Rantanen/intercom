use super::common::*;
use crate::prelude::*;

use crate::idents;
use crate::model;
use crate::utils;
use std::iter::FromIterator;

extern crate quote;

/// Expands the `com_library` macro.
///
/// The macro expansion results in the following items:
///
/// - `DllGetClassObject` extern function implementation.
/// - `IntercomListClassObjects` extern function implementation.
pub fn expand_com_library(
    arg_tokens: TokenStreamNightly,
) -> Result<TokenStreamNightly, model::ParseError>
{
    let mut output = vec![];
    let lib = model::ComLibrary::parse(&lib_name(), arg_tokens.into())?;

    // Create the match-statmeent patterns for each supposedly visible COM class.
    let mut match_arms = vec![];
    let mut creatable_classes = vec![];
    for struct_path in &lib.coclasses {
        // Construct the match pattern.
        let clsid_path = idents::clsid_path(struct_path);
        match_arms.push(quote!(
            self::#clsid_path =>
                Ok( intercom::ComBoxData::new(
                        #struct_path::new()
                    ) as intercom::RawComPtr )
        ));

        // Collect class identifies of classes that have a guid and
        // which others outside the library can create.
        creatable_classes.push(quote!( #clsid_path ));
    }

    // Generate built-in type data.
    /*
    for bti in builtin_model::builtin_intercom_types( lib.name() ) {

        // CLSID
        let clsid_tokens = utils::get_guid_tokens(
                bti.class.clsid().as_ref().unwrap() );
        let clsid_doc = format!( "Built-in {} class ID.", bti.class.name() );
        let builtin_clsid = idents::clsid( bti.class.name() );
        output.push( quote!(
            #[allow(non_upper_case_globals)]
            #[doc = #clsid_doc ]
            pub const #builtin_clsid : intercom::CLSID = #clsid_tokens;
        ) );

        // Match arm
        let ctor = bti.ctor;
        match_arms.push( quote!(
            self::#builtin_clsid =>
                Ok( intercom::ComBox::new( #ctor ) as intercom::RawComPtr )
        ) );

        // Include also built-in classes. They have a custom CLSID in every library.
        creatable_classes.push( quote!( #builtin_clsid ) );
    }
    */
    for (clsid, path) in &[
        (
            quote!(intercom::alloc::CLSID_Allocator),
            quote!(intercom::alloc::Allocator),
        ),
        (
            quote!(intercom::error::CLSID_ErrorStore),
            quote!(intercom::error::ErrorStore),
        ),
    ] {
        match_arms.push(quote!(
            #clsid =>
                    Ok( intercom::ComBoxData::new( #path::default() )
                        as intercom::RawComPtr ) ));
        creatable_classes.push(quote!( #clsid ));
    }

    // Implement DllGetClassObject.
    //
    // This is more or less the only symbolic entry point that the COM
    // infrastructure uses. The COM client uses this method to acquire
    // the IClassFactory interfaces that are then used to construct the
    // actual coclasses.
    let dll_get_class_object = get_dll_get_class_object_function(&match_arms);
    output.push(dll_get_class_object);

    // Implement get_intercom_typelib()
    let get_typelib_fn =
        create_get_typelib_function(&lib).map_err(model::ParseError::ComLibrary)?;
    output.push(get_typelib_fn);

    // Implement DllListClassObjects
    // DllListClassObjects returns all CLSIDs implemented in the crate.
    let list_class_objects = get_intercom_list_class_objects_function(&creatable_classes);
    output.push(list_class_objects);

    Ok(TokenStream::from_iter(output.into_iter()).into())
}

fn get_dll_get_class_object_function(match_arms: &[TokenStream]) -> TokenStream
{
    let calling_convetion = get_calling_convetion();
    quote!(
        #[no_mangle]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[doc(hidden)]
        pub unsafe extern #calling_convetion fn DllGetClassObject(
            rclsid : intercom::REFCLSID,
            riid : intercom::REFIID,
            pout : *mut intercom::RawComPtr
        ) -> intercom::raw::HRESULT
        {
            // Create new class factory.
            // Specify a create function that is able to create all the
            // contained coclasses.
            let mut com_struct = intercom::ComBox::new(
                intercom::ClassFactory::new( rclsid, | clsid | {
                    match *clsid {
                        #( #match_arms, )*
                        _ => Err( intercom::raw::E_NOINTERFACE ),
                    }
                } ) );
            intercom::ComBoxData::query_interface(
                    com_struct.as_mut(),
                    riid,
                    pout );

            // com_struct will drop here and decrement the referenc ecount.
            // This is okay, as the query_interface incremented it, leaving
            // it at two at this point.

            intercom::raw::S_OK
        }
    )
}

fn create_get_typelib_function(lib: &model::ComLibrary) -> Result<TokenStream, String>
{
    let lib_name = lib_name();
    let libid = utils::get_guid_tokens(&lib.libid, Span::call_site());
    let create_class_typeinfo = lib.coclasses.iter().map(|path| {
        quote!(
            <#path as intercom::attributes::HasTypeInfo>::gather_type_info()
        )
    });
    let create_struct_typeinfo = lib.structs.iter().map(|path| {
        quote!(
            <#path as intercom::attributes::StructHasTypeInfo>::gather_type_info()
        )
    });
    Ok(quote!(
        pub(crate) fn get_intercom_typelib() -> intercom::typelib::TypeLib
        {
            let types = vec![
                <intercom::alloc::Allocator as intercom::attributes::HasTypeInfo>::gather_type_info(),
                <intercom::error::ErrorStore as intercom::attributes::HasTypeInfo>::gather_type_info(),
                #( #create_class_typeinfo, )*
                #( #create_struct_typeinfo, )*
            ].into_iter().flatten().collect::<Vec<_>>();
            intercom::typelib::TypeLib::__new(
                #lib_name.into(),
                #libid,
                "1.0".into(),
                types
            )
        }

        #[no_mangle]
        pub unsafe extern "system" fn IntercomTypeLib(
            type_system: intercom::type_system::TypeSystemName,
            out: *mut intercom::RawComPtr,
        ) -> intercom::raw::HRESULT
        {
            let mut tlib = intercom::ComBox::new( get_intercom_typelib() );
            let rc = intercom::ComRc::<intercom::typelib::IIntercomTypeLib>::from( &tlib );
            let itf = intercom::ComRc::detach(rc);
            *out = match type_system {
                intercom::type_system::TypeSystemName::Automation =>
                    intercom::ComItf::ptr::<intercom::type_system::AutomationTypeSystem>(&itf).ptr,
                intercom::type_system::TypeSystemName::Raw =>
                    intercom::ComItf::ptr::<intercom::type_system::RawTypeSystem>(&itf).ptr,
            };

            intercom::raw::S_OK
        }
    ))
}

fn get_intercom_list_class_objects_function(clsid_tokens: &[TokenStream]) -> TokenStream
{
    let calling_convetion = get_calling_convetion();
    let token_count = clsid_tokens.len();
    quote!(
        #[no_mangle]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[doc(hidden)]
        pub unsafe extern #calling_convetion fn IntercomListClassObjects(
            pcount: *mut usize,
            pclsids: *mut *const intercom::CLSID,
        ) -> intercom::raw::HRESULT
        {
            // Do not crash due to invalid parameters.
            if pcount.is_null() { return intercom::raw::E_POINTER; }
            if pclsids.is_null() { return intercom::raw::E_POINTER; }

            // Store the available CLSID in a static variable so that we can
            // pass them as-is to the caller.
            static AVAILABLE_CLASSES: [::intercom::CLSID; #token_count ] = [
                #( #clsid_tokens, )*
            ];

            // com_struct will drop here and decrement the referenc ecount.
            // This is okay, as the query_interface incremented it, leaving
            // it at two at this point.
            *pcount = #token_count;
            *pclsids = AVAILABLE_CLASSES.as_ptr();

            intercom::raw::S_OK
        }
    )
}
