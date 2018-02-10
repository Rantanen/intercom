
use super::common::*;

use idents;
use utils;
use model;
use builtin_model;

extern crate proc_macro;
use self::proc_macro::TokenStream;

/// Expands the `com_library` attribute.
///
/// The attribute expansion results in the following items:
///
/// - `DllGetClassObject` extern function implementation.
pub fn expand_com_library(
    attr_tokens: &TokenStream,
    item_tokens: TokenStream,
) -> Result<TokenStream, model::ParseError>
{
    let mut output = vec![];
    let lib = model::ComLibrary::parse( &lib_name(), &attr_tokens.to_string() )?;

    // Create the match-statmeent patterns for each supposedly visible COM class.
    let mut match_arms = vec![];
    for struct_ident in lib.coclasses() {

        // Construct the match pattern.
        let clsid_name = idents::clsid( struct_ident );
        match_arms.push( quote!(
            self::#clsid_name =>
                Ok( ::intercom::ComBox::new(
                        #struct_ident::new()
                    ) as ::intercom::RawComPtr )
        ) );
    }

    // Generate built-in type data.
    for bti in builtin_model::builtin_intercom_types( lib.name() ) {

        // CLSID
        let clsid_tokens = utils::get_guid_tokens(
                bti.class.clsid().as_ref().unwrap() );
        let clsid_doc = format!( "Built-in {} class ID.", bti.class.name() );
        let builtin_clsid = idents::clsid( bti.class.name() );
        output.push( quote!(
            #[allow(non_upper_case_globals)]
            #[doc = #clsid_doc ]
            pub const #builtin_clsid : ::intercom::CLSID = #clsid_tokens;
        ) );

        // Match arm
        let ctor = bti.ctor;
        match_arms.push( quote!(
            self::#builtin_clsid =>
                Ok( ::intercom::ComBox::new( #ctor ) as ::intercom::RawComPtr )
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
    Ok( tokens_to_tokenstream( item_tokens, output ) )
}
