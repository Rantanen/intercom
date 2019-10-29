use super::common::*;
use crate::prelude::*;

use crate::idents;
use crate::model;
use crate::utils;

use crate::tyhandlers::ModelTypeSystem;

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
) -> Result<TokenStreamNightly, model::ParseError>
{
    // Parse the attribute.
    let mut output = vec![];
    let cls = model::ComStruct::parse(&lib_name(), attr_tokens.into(), item_tokens.clone().into())?;
    let struct_ident = &cls.name;
    let struct_name = struct_ident.to_string();

    // IUnknown vtable match. As the primary query_interface is implemented
    // on the root IUnknown interface, the self_vtable here should already be
    // the IUnknown we need.
    let support_error_info_vtbl = quote!(
        <dyn intercom::ISupportErrorInfo as intercom::attributes::ComInterface<
            intercom::type_system::AutomationTypeSystem,
        >>::VTable
    );
    let mut query_interface_match_arms = vec![
        quote!(
            if riid == <dyn intercom::IUnknown as intercom::attributes::ComInterface<intercom::type_system::AutomationTypeSystem>>::iid() {
                intercom::logging::trace(format_args!(
                    "[{:p}] {}::query_interface({:-X}) -> IUnknown", vtables, #struct_name, riid));
                intercom::logging::trace(format_args!(
                    "-> IUnknown"));
                ( &vtables._ISupportErrorInfo )
                    as *const &#support_error_info_vtbl
                    as *mut &#support_error_info_vtbl
                    as intercom::RawComPtr
            } else
        ),
        quote!(
            if riid == <dyn intercom::ISupportErrorInfo as intercom::attributes::ComInterface<intercom::type_system::AutomationTypeSystem>>::iid() {
                intercom::logging::trace(format_args!(
                    "[{:p}] {}::query_interface({:-X}) -> ISupportErrorInfo", vtables, #struct_name, riid));
                ( &vtables._ISupportErrorInfo )
                    as *const &#support_error_info_vtbl
                    as *mut &#support_error_info_vtbl
                    as intercom::RawComPtr
            } else
        ),
    ];
    let mut support_error_info_match_arms = vec![];

    // Gather the virtual table list struct field definitions and their values.
    // The definitions are needed when we define the virtual table list struct,
    // which is different for each com_class. The values are needed when we
    // construct the virtual table list.
    //
    // The primary IUnknown virtual table _MUST_ be at the beginning of the list.
    // This is done to ensure the IUnknown pointer matches the ComBoxData pointer.
    // We ensure this by defining the primary IUnknown methods on the
    // ISupportErrorInfo virtual table and having that at the beginning.
    let mut vtable_list_field_decls = vec![quote!(
        _ISupportErrorInfo:
            &'static <dyn intercom::ISupportErrorInfo as intercom::attributes::ComInterface<
                intercom::type_system::AutomationTypeSystem,
            >>::VTable
    )];
    let mut vtable_list_field_values = vec![quote!( _ISupportErrorInfo :
                <#struct_ident as intercom::attributes::ComImpl<
                    dyn intercom::ISupportErrorInfo, intercom::type_system::AutomationTypeSystem>>::vtable() )];

    // Create the vtable data for the additional interfaces.
    // The data should include the match-arms for the primary query_interface
    // and the vtable offsets used for the delegating query_interface impls.
    for itf in &cls.interfaces {
        for &ts in &[ModelTypeSystem::Automation, ModelTypeSystem::Raw] {
            // Various idents.
            let itf_variant = Ident::new(&format!("{}_{:?}", itf, ts), itf.span());
            let ts_type = ts.as_typesystem_type(itf.span());
            let maybe_dyn = match itf == struct_ident {
                true => quote!(),
                false => quote_spanned!(itf.span() => dyn),
            };

            // Store the field offset globally. We need this offset when implementing
            // the delegating query_interface methods. The only place where we know
            // the actual layout of the vtable is here. Thus we need to store this
            // offset somewhere where the com_impl's can access it.
            //
            // Rust doesn't allow pointer derefs or conversions in consts so we'll
            // use an inline fn instead. LLVM should be able to reduce this into a
            // constant expression during compilation.
            output.push(quote!(
                #[allow(non_snake_case)]
                impl intercom::attributes::ComClass<
                    #maybe_dyn #itf, #ts_type> for #struct_ident {

                    #[inline(always)]
                    fn offset() -> usize {
                        unsafe {
                            &intercom::ComBoxData::< #struct_ident >::null_vtable().#itf_variant
                                    as *const _ as usize
                        }
                    }
                }
            ));

            let itf_attrib_data = quote!(
                <#maybe_dyn #itf as intercom::attributes::ComInterface<#ts_type>>);
            let impl_attrib_data = quote!(
                <#struct_ident as intercom::attributes::ComImpl<#maybe_dyn #itf, #ts_type>>);

            // Add the interface in the vtable list.
            vtable_list_field_decls.push(quote!( #itf_variant : &'static #itf_attrib_data::VTable));
            vtable_list_field_values.push(quote!( #itf_variant : #impl_attrib_data::vtable()));

            // Define the query_interface match arm for the current interface.
            // This just gets the correct interface vtable reference from the list
            // of vtables.
            let itf_name = itf.to_string();
            let ts_name = format!("{:?}", ts);
            query_interface_match_arms.push(quote!(
                if riid == #itf_attrib_data::iid() {
                    intercom::logging::trace(format_args!(
                        "[{:p}] {}::query_interface({:-X}) -> {} ({})", vtables, #struct_name, riid, #itf_name, #ts_name));
                    &vtables.#itf_variant
                        as *const &#itf_attrib_data::VTable
                        as *mut &#itf_attrib_data::VTable
                        as intercom::RawComPtr
                } else
            ));

            // Define the support error info match arms.
            support_error_info_match_arms.push(quote!(
                if riid == <#maybe_dyn #itf as intercom::attributes::ComInterface<#ts_type>>::iid() {
                    true
                } else
            ));
        }
    }

    /////////////////////
    // ISupportErrorInfo virtual table instance.
    //
    // The primary IUnknown virtual table is embedded in this one.
    let iunknown_vtbl = quote!(
        <dyn intercom::IUnknown as intercom::attributes::ComInterface<
            intercom::type_system::AutomationTypeSystem,
        >>::VTable
    );
    output.push(quote!(
        #[allow(non_upper_case_globals)]
        impl intercom::attributes::ComImpl<
            intercom::ISupportErrorInfo, intercom::type_system::AutomationTypeSystem>
            for #struct_ident {

            fn vtable() -> &'static <dyn intercom::ISupportErrorInfo as intercom::attributes::ComInterface<intercom::type_system::AutomationTypeSystem>>::VTable
            {
                type T = <dyn intercom::ISupportErrorInfo as intercom::attributes::ComInterface<intercom::type_system::AutomationTypeSystem>>::VTable;
                & T {
                    __base : {
                        type Vtbl = #iunknown_vtbl;
                        Vtbl {
                            query_interface
                                : intercom::ComBoxData::< #struct_ident >
                                    ::query_interface_ptr,
                            add_ref
                                : intercom::ComBoxData::< #struct_ident >
                                    ::add_ref_ptr,
                            release
                                : intercom::ComBoxData::< #struct_ident >
                                    ::release_ptr,
                        }
                    },
                    interface_supports_error_info
                        : intercom::ComBoxData::< #struct_ident >
                            ::interface_supports_error_info_ptr,
                }
            }
        }
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
    let vtable_list_ident = Ident::new(
        &format!("__intercom_vtable_for_{}", struct_ident),
        Span::call_site(),
    );
    let visibility = &cls.visibility;
    output.push(quote!(
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #visibility struct #vtable_list_ident {
            #( #vtable_list_field_decls ),*
        }
    ));

    // The actual CoClass implementation.
    output.push(quote!(
        #[allow(clippy::all)]
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
                if riid.is_null() {
                    intercom::logging::error(format_args!(
                        "[{:p}] {}::query_interface(NULL)", vtables, #struct_name));
                    return Err( intercom::raw::E_NOINTERFACE );
                }
                unsafe {
                    let riid = &*riid;
                    intercom::logging::trace(format_args!(
                        "[{:p}] {}::query_interface({:-X})", vtables, #struct_name, riid));
                    Ok(
                        #( #query_interface_match_arms )*
                        {
                            intercom::logging::trace(format_args!(
                                "[{:p}] {}::query_interface({:-X}) -> E_NOINTERFACe", vtables, #struct_name, riid));
                            return Err( intercom::raw::E_NOINTERFACE )
                        }
                    )
                }
            }

            fn interface_supports_error_info(
                riid : intercom::REFIID
            ) -> bool
            {
                if riid.is_null() { return false; }
                unsafe {
                    let riid = &*riid;
                    #( #support_error_info_match_arms )*
                    { false }
                }
            }
        }
    ));

    // CLSID constant for the class.
    let clsid_ident = idents::clsid(struct_ident);
    if let Some(ref guid) = cls.clsid {
        let clsid_guid_tokens = utils::get_guid_tokens(guid, Span::call_site());
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
            .map_err(|e| model::ParseError::ComStruct(cls.name.to_string(), e))?,
    );

    Ok(tokens_to_tokenstream(item_tokens, output))
}

fn create_get_typeinfo_function(cls: &model::ComStruct) -> Result<TokenStream, String>
{
    let fn_name = Ident::new(
        &format!("get_intercom_coclass_info_for_{}", cls.name),
        Span::call_site(),
    );
    let cls_ident = &cls.name;
    let cls_name = cls.name.to_string();
    let clsid = match &cls.clsid {
        Some(guid) => guid,
        None => {
            return Ok(quote!(
                pub(crate) fn #fn_name() -> Vec<intercom::typelib::TypeInfo>
                { vec![] }
            ))
        }
    };
    let clsid_tokens = utils::get_guid_tokens(&clsid, Span::call_site());
    let (interfaces, interface_info): (Vec<_>, Vec<_>) = cls
        .interfaces
        .iter()
        .map(|itf_ident| {
            let itf_name = itf_ident.to_string();
            let maybe_dyn = match itf_ident == &cls.name {
                true => quote!(),
                false => quote_spanned!(itf_ident.span() => dyn),
            };
            (
                quote!( intercom::typelib::InterfaceRef {
                    name: #itf_name.into(),
                    iid_automation: <#maybe_dyn #itf_ident as intercom::attributes::ComInterface<intercom::type_system::AutomationTypeSystem>>::iid().clone(),
                    iid_raw: <#maybe_dyn #itf_ident as intercom::attributes::ComInterface<intercom::type_system::RawTypeSystem>>::iid().clone(),
                } ),
                quote!(
                    r.extend(<#maybe_dyn #itf_ident as intercom::attributes::InterfaceHasTypeInfo>::gather_type_info());
                ),
            )
        })
        .unzip();
    Ok(quote!(
        impl intercom::attributes::HasTypeInfo for #cls_ident
        {
            fn gather_type_info() -> Vec<intercom::typelib::TypeInfo>
            {
                let mut r = vec![ intercom::typelib::TypeInfo::Class(
                    intercom::ComBox::new( intercom::typelib::CoClass::__new(
                        #cls_name.into(),
                        #clsid_tokens,
                        vec![ #( #interfaces ),* ]
                    ) ) )
                ];
                #( #interface_info )*
                r
            }
        }
    ))
}
