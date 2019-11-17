use super::common::*;
use crate::prelude::*;

use crate::idents;
use crate::idents::SomeIdent;
use crate::model;
use crate::utils;

use crate::tyhandlers::ModelTypeSystem;

use syn::spanned::Spanned;

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
    let cls = model::ComClass::parse(&lib_name(), attr_tokens.into(), item_tokens.clone().into())?;
    let cls_ident = &cls.name;
    let cls_name = cls_ident.to_string();
    let (impl_generics, ty_generics, where_clause) = cls.generics.split_for_impl();

    // IUnknown vtable match. As the primary query_interface is implemented
    // on the root IUnknown interface, the self_vtable here should already be
    // the IUnknown we need.
    let support_error_info_vtbl = quote!(
        <dyn intercom::ISupportErrorInfo as intercom::attributes::ComInterfaceVariant<
            intercom::type_system::AutomationTypeSystem,
        >>::VTable
    );
    let mut query_interface_match_arms = vec![
        quote!(
            if riid == <dyn intercom::IUnknown as intercom::attributes::ComInterfaceVariant<intercom::type_system::AutomationTypeSystem>>::iid() {
                let ptr = ( &vtables._ISupportErrorInfo )
                    as *const &#support_error_info_vtbl
                    as *mut &#support_error_info_vtbl
                    as intercom::raw::RawComPtr;
                intercom::logging::trace(|l| l(module_path!(), format_args!(
                    "[{:p}] {}::query_interface({:-X}) -> IUnknown [{:p}]",
                    vtables, #cls_name, riid, ptr)));
                ptr
            } else
        ),
        quote!(
            if riid == <dyn intercom::ISupportErrorInfo as intercom::attributes::ComInterfaceVariant<intercom::type_system::AutomationTypeSystem>>::iid() {
                let ptr = ( &vtables._ISupportErrorInfo )
                    as *const &#support_error_info_vtbl
                    as *mut &#support_error_info_vtbl
                    as intercom::raw::RawComPtr;
                intercom::logging::trace(|l| l(module_path!(), format_args!(
                    "[{:p}] {}::query_interface({:-X}) -> ISupportErrorInfo [{:p}]",
                    vtables, #cls_name, riid, ptr)));
                ptr
            } else
        ),
    ];
    let mut support_error_info_match_arms = vec![];

    output.push(quote!(
        impl #impl_generics intercom::IUnknown for #cls_ident #ty_generics #where_clause {}
    ));

    // Gather the virtual table list struct field definitions and their values.
    // The definitions are needed when we define the virtual table list struct,
    // which is different for each com_class. The values are needed when we
    // construct the virtual table list.
    //
    // The primary IUnknown virtual table _MUST_ be at the beginning of the list.
    // This is done to ensure the IUnknown pointer matches the ComBoxData pointer.
    // We ensure this by defining the primary IUnknown methods on the
    // ISupportErrorInfo virtual table and having that at the beginning.
    let mut vtable_list_field_defs = vec![];
    let mut vtable_list_field_decls = vec![quote!(
        _ISupportErrorInfo:
            &'static <dyn intercom::ISupportErrorInfo as intercom::attributes::ComInterfaceVariant<
                intercom::type_system::AutomationTypeSystem,
            >>::VTable
    )];
    let mut vtable_list_field_values = vec![];
    let mut vtable_list_field_ptrs = vec![quote!(
                 _ISupportErrorInfo :
                &<dyn intercom::ISupportErrorInfo as intercom::attributes::ComInterfaceVTableFor<
                    dyn intercom::ISupportErrorInfo,
                    #cls_ident #ty_generics,
                    intercom::type_system::AutomationTypeSystem>>::VTABLE
    )];

    // Create the vtable data for the additional interfaces.
    // The data should include the match-arms for the primary query_interface
    // and the vtable offsets used for the delegating query_interface impls.
    for itf in &cls.interfaces {
        let maybe_dyn = match cls.is_self_path(itf) {
            true => quote!(),
            false => quote_spanned!(itf.span() => dyn),
        };
        output.push(quote_spanned!(itf.span() =>
            impl #impl_generics intercom::attributes::HasInterface<#maybe_dyn #itf> for #cls_ident #ty_generics #where_clause {}
        ));

        for &ts in &[ModelTypeSystem::Automation, ModelTypeSystem::Raw] {
            // Various idents.
            let itf_ident = itf.get_some_ident().expect("#[com_interface] had no ident");
            let itf_variant = Ident::new(&format!("{}_{:?}", itf_ident, ts), itf.span());
            let ts_type = ts.as_typesystem_type(itf.span());

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
                impl #impl_generics intercom::attributes::ComClassInterface<
                    #maybe_dyn #itf, #ts_type> for #cls_ident #ty_generics #where_clause {

                    #[inline(always)]
                    fn offset() -> usize {
                        unsafe {
                            &intercom::ComBoxData::< #cls_ident #ty_generics >::null_vtable().#itf_variant
                                    as *const _ as usize
                        }
                    }
                }
            ));

            let itf_attrib_data = quote!(
                <#maybe_dyn #itf as intercom::attributes::ComInterfaceVariant<#ts_type>>);
            let itf_vtable_for = quote!(
                <#maybe_dyn #itf as intercom::attributes::ComInterfaceVTableFor<#maybe_dyn #itf, #cls_ident #ty_generics, #ts_type>>);

            // Add the interface in the vtable list.
            vtable_list_field_defs.push(quote!( #itf_variant : #itf_attrib_data::VTable));
            vtable_list_field_decls.push(quote!( #itf_variant : &'static #itf_attrib_data::VTable));
            vtable_list_field_values.push(quote!( #itf_variant : #itf_vtable_for::VTABLE));
            vtable_list_field_ptrs.push(quote!( #itf_variant : &#itf_vtable_for::VTABLE));

            // Define the query_interface match arm for the current interface.
            // This just gets the correct interface vtable reference from the list
            // of vtables.
            let itf_name = itf_ident.to_string();
            let ts_name = format!("{:?}", ts);
            query_interface_match_arms.push(quote!(
                if riid == #itf_attrib_data::iid() {
                    let ptr = &vtables.#itf_variant
                        as *const &#itf_attrib_data::VTable
                        as *mut &#itf_attrib_data::VTable
                        as intercom::raw::RawComPtr;
                    intercom::logging::trace(|l| l(module_path!(), format_args!(
                        "[{:p}] {}::query_interface({:-X}) -> {} ({}) [{:p}]",
                        vtables, #cls_name, riid, #itf_name, #ts_name, ptr)));
                    ptr
                } else
            ));

            // Define the support error info match arms.
            support_error_info_match_arms.push(quote!(
                if riid == <#maybe_dyn #itf as intercom::attributes::ComInterfaceVariant<#ts_type>>::iid() {
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
        <dyn intercom::IUnknown as intercom::attributes::ComInterfaceVariant<
            intercom::type_system::AutomationTypeSystem,
        >>::VTable
    );
    output.push(quote!(
        #[allow(non_snake_case)]
        impl #impl_generics intercom::attributes::ComClassInterface<
            dyn intercom::ISupportErrorInfo,
            intercom::type_system::AutomationTypeSystem>
        for #cls_ident #ty_generics #where_clause {

            #[inline(always)]
            fn offset() -> usize { 0 }
        }
    ));

    // Mark the struct as having IUnknown.
    output.push(quote!(
        impl #impl_generics intercom::attributes::HasInterface< intercom::IUnknown > for #cls_ident #ty_generics #where_clause {}
    ));

    // The ComClass implementation.
    //
    // Define the vtable list struct first. This lists the vtables of all the
    // interfaces that the coclass implements.

    // VTableList struct definition.
    let vtable_list_ident = Ident::new(
        &format!("__intercom_vtable_for_{}", cls_ident),
        Span::call_site(),
    );
    let visibility = &cls.visibility;
    output.push(quote!(
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[derive(Clone, Copy)]
        #visibility struct #vtable_list_ident {
            #( #vtable_list_field_decls ),*
        }
    ));

    // The actual ComClass implementation.
    let vtable_static_ident = Ident::new(
        &format!("Static{}", vtable_list_ident),
        vtable_list_ident.span(),
    );
    output.push(quote!(
        #[allow(non_snake_case)]
        #visibility struct #vtable_static_ident {
            #( #vtable_list_field_defs ),*
        }

        #[allow(clippy::all)]
        impl #impl_generics intercom::attributes::ComClass for #cls_ident #ty_generics #where_clause {
            type VTableList = #vtable_list_ident;
            const VTABLE : Self::VTableList = #vtable_list_ident {
                #( #vtable_list_field_ptrs ),*
            };
            fn query_interface(
                vtables : &Self::VTableList,
                riid : intercom::REFIID,
            ) -> intercom::RawComResult< intercom::raw::RawComPtr > {
                if riid.is_null() {
                    intercom::logging::error(|l| l(module_path!(), format_args!(
                        "[{:p}] {}::query_interface(NULL)", vtables, #cls_name)));
                    return Err( intercom::raw::E_NOINTERFACE );
                }
                unsafe {
                    let riid = &*riid;
                    intercom::logging::trace(|l| l(module_path!(), format_args!(
                        "[{:p}] {}::query_interface({:-X})", vtables, #cls_name, riid)));
                    Ok(
                        #( #query_interface_match_arms )*
                        {
                            intercom::logging::trace(|l| l(module_path!(), format_args!(
                                "[{:p}] {}::query_interface({:-X}) -> E_NOINTERFACE", vtables, #cls_name, riid)));
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
    let clsid_ident = idents::clsid(cls_ident);
    if let Some(ref guid) = cls.clsid {
        let clsid_guid_tokens = utils::get_guid_tokens(guid, Span::call_site());
        let clsid_doc = format!("`{}` class ID.", cls_ident);
        let clsid_const = quote!(
            #[allow(non_upper_case_globals)]
            #[doc = #clsid_doc ]
            pub const #clsid_ident : intercom::CLSID = #clsid_guid_tokens;
        );
        output.push(clsid_const);
    }

    output.push(
        create_get_typeinfo_function(&cls)
            .map_err(|e| model::ParseError::ComClass(cls.name.to_string(), e))?,
    );

    Ok(tokens_to_tokenstream(item_tokens, output))
}

fn create_get_typeinfo_function(cls: &model::ComClass) -> Result<TokenStream, String>
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
    let (impl_generics, ty_generics, where_clause) = cls.generics.split_for_impl();
    let (interfaces, interface_info): (Vec<_>, Vec<_>) = cls
        .interfaces
        .iter()
        .map(|itf_path| {
            let itf_name = itf_path.get_some_ident().expect("#[com_interface] had no ident").to_string();
            let maybe_dyn = match cls.is_self_path(itf_path) {
                true => quote!(),
                false => quote_spanned!(itf_path.span() => dyn),
            };
            (
                quote!( intercom::typelib::InterfaceRef {
                    name: #itf_name.into(),
                    iid_automation: <#maybe_dyn #itf_path as intercom::attributes::ComInterfaceVariant<intercom::type_system::AutomationTypeSystem>>::iid().clone(),
                    iid_raw: <#maybe_dyn #itf_path as intercom::attributes::ComInterfaceVariant<intercom::type_system::RawTypeSystem>>::iid().clone(),
                } ),
                quote!(
                    r.extend(<#maybe_dyn #itf_path as intercom::attributes::ComInterfaceTypeInfo>::gather_type_info());
                ),
            )
        })
        .unzip();
    Ok(quote!(
        impl #impl_generics intercom::attributes::ComClassTypeInfo for #cls_ident #ty_generics #where_clause
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
