use super::common::*;
use crate::prelude::*;

use crate::idents;
use crate::model;
use crate::utils;

use crate::tyhandlers::{get_type_systems, ModelTypeSystem};

/// Expands the `com_class` attribute.
///
/// The attribute expansion results in the following items:
///
/// - Virtual table offset values for the different interfaces.
/// - `IUnknown` virtual table instance.
/// - `CoClass` trait implementation.
pub fn expand_com_class(
    attr_tokens: TokenStreamNightly,
    item_tokens: TokenStreamNightly,
) -> Result<TokenStreamNightly, model::ParseError> {
    // Parse the attribute.
    let mut output = vec![];
    let cls = model::ComStruct::parse(&lib_name(), attr_tokens.into(), &item_tokens.to_string())?;
    let struct_ident = cls.name();

    // IUnknown vtable match. As the primary query_interface is implemented
    // on the root IUnknown interface, the self_vtable here should already be
    // the IUnknown we need.
    let mut query_interface_match_arms = vec![
        quote!(
            intercom::IID_IUnknown =>
                ( &vtables._ISupportErrorInfo )
                    as *const &intercom::ISupportErrorInfoVtbl
                    as *mut &intercom::ISupportErrorInfoVtbl
                    as intercom::RawComPtr
        ),
        quote!(
            intercom::IID_ISupportErrorInfo =>
                ( &vtables._ISupportErrorInfo )
                    as *const &intercom::ISupportErrorInfoVtbl
                    as *mut &intercom::ISupportErrorInfoVtbl
                    as intercom::RawComPtr
        ),
    ];
    let mut support_error_info_match_arms = vec![];

    // Gather the virtual table list struct field definitions and their values.
    // The definitions are needed when we define the virtual table list struct,
    // which is different for each com_class. The values are needed when we
    // construct the virtual table list.
    //
    // The primary IUnknown virtual table _MUST_ be at the beginning of the list.
    // This is done to ensure the IUnknown pointer matches the ComBox pointer.
    // We ensure this by defining the primary IUnknown methods on the
    // ISupportErrorInfo virtual table and having that at the beginning.
    let isupporterrorinfo_ident = Ident::new("ISupportErrorInfo", Span::call_site());
    let isupporterrorinfo_vtable_instance_ident =
        idents::vtable_instance(&struct_ident, &isupporterrorinfo_ident);
    let mut vtable_list_field_decls = vec![quote!(
        _ISupportErrorInfo: &'static intercom::ISupportErrorInfoVtbl
    )];
    let mut vtable_list_field_values =
        vec![quote!( _ISupportErrorInfo : &#isupporterrorinfo_vtable_instance_ident )];

    // Create the vtable data for the additional interfaces.
    // The data should include the match-arms for the primary query_interface
    // and the vtable offsets used for the delegating query_interface impls.
    for itf in cls.interfaces() {
        for &ts in &[ModelTypeSystem::Automation, ModelTypeSystem::Raw] {
            // Various idents.
            let itf_variant = Ident::new(&format!("{}_{:?}", itf, ts), Span::call_site());
            let offset_ident = idents::vtable_offset(struct_ident, &itf_variant);
            let iid_ident = idents::iid(&itf_variant);
            let vtable_struct_ident = idents::vtable_struct(&itf_variant);
            let vtable_instance_ident = idents::vtable_instance(struct_ident, &itf_variant);

            // Store the field offset globally. We need this offset when implementing
            // the delegating query_interface methods. The only place where we know
            // the actual layout of the vtable is here. Thus we need to store this
            // offset somewhere where the com_impl's can access it.
            //
            // Rust doesn't allow pointer derefs or conversions in consts so we'll
            // use an inline fn instead. LLVM should be able to reduce this into a
            // constant expression during compilation.
            output.push(quote!(
                    #[inline(always)]
                    #[allow(non_snake_case)]
                    fn #offset_ident() -> usize {
                        unsafe {
                            &intercom::ComBox::< #struct_ident >::null_vtable().#itf_variant
                                    as *const _ as usize
                        }
                    }
            ));

            // Add the interface in the vtable list.
            vtable_list_field_decls.push(quote!( #itf_variant : &'static #vtable_struct_ident ));
            vtable_list_field_values.push(quote!( #itf_variant : &#vtable_instance_ident ));

            // Define the query_interface match arm for the current interface.
            // This just gets the correct interface vtable reference from the list
            // of vtables.
            query_interface_match_arms.push(quote!(
                self::#iid_ident => &vtables.#itf_variant
                        as *const &#vtable_struct_ident
                        as *mut &#vtable_struct_ident
                        as intercom::RawComPtr
            ));

            // Define the support error info match arms.
            support_error_info_match_arms.push(quote!(
                self::#iid_ident => true
            ));
        }
    }

    /////////////////////
    // ISupportErrorInfo virtual table instance.
    //
    // The primary IUnknown virtual table is embedded in this one.
    output.push(quote!(
        #[allow(non_upper_case_globals)]
        const #isupporterrorinfo_vtable_instance_ident
                : intercom::ISupportErrorInfoVtbl
                = intercom::ISupportErrorInfoVtbl {
                    __base : intercom::IUnknownVtbl {
                        query_interface_Automation
                            : intercom::ComBox::< #struct_ident >
                                ::query_interface_ptr,
                        add_ref_Automation
                            : intercom::ComBox::< #struct_ident >
                                ::add_ref_ptr,
                        release_Automation
                            : intercom::ComBox::< #struct_ident >
                                ::release_ptr,
                    },
                    interface_supports_error_info_Automation
                        : intercom::ComBox::< #struct_ident >
                            ::interface_supports_error_info_ptr,
                };
    ));

    // Mark the struct as having IUnknown.
    output.push(quote!(
        impl intercom::HasInterface< intercom::IUnknown > for #struct_ident {}
    ));

    // The CoClass implementation.
    //
    // Define the vtable list struct first. This lists the vtables of all the
    // interfaces that the coclass implements.

    // VTableList struct definition.
    let vtable_list_ident = idents::vtable_list(&struct_ident);
    let visibility = cls.visibility();
    output.push(quote!(
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #visibility struct #vtable_list_ident {
            #( #vtable_list_field_decls ),*
        }
    ));

    // The actual CoClass implementation.
    output.push(quote!(
        impl intercom::CoClass for #struct_ident {
            type VTableList = #vtable_list_ident;
            fn create_vtable_list() -> Self::VTableList {
                #vtable_list_ident {
                    #( #vtable_list_field_values ),*
                }
            }
            fn query_interface(
                vtables : &Self::VTableList,
                riid : intercom::REFIID,
            ) -> intercom::RawComResult< intercom::RawComPtr > {
                if riid.is_null() { return Err( intercom::raw::E_NOINTERFACE ) }
                Ok( match *unsafe { &*riid } {
                    #( #query_interface_match_arms ),*,
                    _ => return Err( intercom::raw::E_NOINTERFACE )
                } )
            }

            fn interface_supports_error_info(
                riid : intercom::REFIID
            ) -> bool
            {
                match *unsafe { &*riid } {
                    #( #support_error_info_match_arms ),*,
                    _ => false
                }
            }
        }
    ));

    // CLSID constant for the class.
    let clsid_ident = idents::clsid(struct_ident);
    if let Some(ref guid) = *cls.clsid() {
        let clsid_guid_tokens = utils::get_guid_tokens(guid);
        let clsid_doc = format!("`{}` class ID.", struct_ident);
        let clsid_const = quote!(
            #[allow(non_upper_case_globals)]
            #[doc = #clsid_doc ]
            pub const #clsid_ident : intercom::CLSID = #clsid_guid_tokens;
        );
        output.push(clsid_const);
    }

    output.push(
        create_get_typeinfo_function(&cls)
            .map_err(|e| model::ParseError::ComStruct(cls.name().to_string(), e))?,
    );

    // Methods for getting stuct descriptos.
    output.push(get_type_descriptor_helpers(struct_ident, cls.interfaces()));

    Ok(tokens_to_tokenstream(item_tokens, output))
}

fn create_get_typeinfo_function(cls: &model::ComStruct) -> Result<TokenStream, String> {
    let fn_name = Ident::new(
        &format!("get_intercom_coclass_info_for_{}", cls.name()),
        Span::call_site(),
    );
    let cls_name = cls.name().to_string();
    let clsid = match cls.clsid() {
        Some(guid) => guid,
        None => {
            return Ok(quote!(
                pub(crate) fn #fn_name() -> Vec<intercom::typelib::TypeInfo>
                { vec![] }
            ))
        }
    };
    let clsid_tokens = utils::get_guid_tokens(clsid);
    let (interfaces, interface_info): (Vec<_>, Vec<_>) = cls
        .interfaces()
        .iter()
        .map(|itf_ident| {
            let itf_name = itf_ident.to_string();
            let itf_automation_iid = idents::iid(&Ident::new(
                &format!("{}_Automation", itf_name),
                Span::call_site(),
            ));
            let itf_raw_iid =
                idents::iid(&Ident::new(&format!("{}_Raw", itf_name), Span::call_site()));
            let itf_fn = Ident::new(
                &format!("get_intercom_interface_info_for_{}", itf_name),
                Span::call_site(),
            );
            (
                quote!( intercom::typelib::InterfaceRef {
                    name: #itf_name.into(),
                    iid_automation: #itf_automation_iid,
                    iid_raw: #itf_raw_iid,
                } ),
                quote!( #itf_fn() ),
            )
        })
        .unzip();
    Ok(quote!(
        pub fn #fn_name() -> Vec<intercom::typelib::TypeInfo>
        {
            vec![ intercom::typelib::TypeInfo::Class(
                intercom::ComStruct::new( intercom::typelib::CoClass::__new(
                    #cls_name.into(),
                    #clsid_tokens,
                    vec![
                        #( #interfaces ),*
                    ]
                ) ) ),
                #( #interface_info ),*
            ]
        }
    ))
}

/// Gets helper functions for serializing the com class.
fn get_type_descriptor_helpers(class_ident: &syn::Ident, interfaces: &[syn::Ident]) -> TokenStream {
    let get_com_class_method = format!("get_com_class_for_{}", class_ident);
    let get_com_class_method = Ident::new(&get_com_class_method, Span::call_site());
    let get_interfaces_method = format!("get_interfaces_for_{}", class_ident);
    let get_interfaces_method = Ident::new(&get_interfaces_method, Span::call_site());

    // Determine method names for getting full interface descriptions.
    let mut get_interface: Vec<syn::Ident> = Vec::new();
    for type_system in get_type_systems() {
        get_interface.extend(interfaces.iter().map(|itf| {
            Ident::new(
                &format!("get_com_interface_for_{}_{}", itf, type_system.name()),
                Span::call_site(),
            )
        }));
    }
    let get_interface = get_interface;

    let result = quote!(

        /// Gets type description of the #class_ident COM class.
        fn #get_com_class_method () -> intercom::serialization::ComClass {
            intercom::serialization::ComClass::new(
                    stringify!( #class_ident ).to_string(), #get_interfaces_method () )
        }

        /// Gets the interfaces of the #class_ident COM class.
        fn #get_interfaces_method () -> Vec<intercom::serialization::ComInterfaceVariant> {
            vec![ #( #get_interface () ),* ]
        }

    );
    // dbg!( result );
    result
}
