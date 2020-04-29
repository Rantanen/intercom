use super::common::*;
use crate::prelude::*;

use std::collections::BTreeMap;
use std::iter;

use crate::idents::{self, SomeIdent};
use crate::methodinfo::ComMethodInfo;
use crate::model;
use crate::tyhandlers::{Direction, ModelTypeSystem};
use crate::utils;

use syn::spanned::Spanned;

extern crate proc_macro;

/// Interface level output.
#[derive(Default)]
struct InterfaceOutput
{
    iid_arms: Vec<TokenStream>,
    method_impls: BTreeMap<String, MethodImpl>,
}

struct MethodImpl
{
    /// _Some_ method info.
    ///
    /// This should not be depended upon for anything type system specific.
    info: ComMethodInfo,

    /// Type system specific implementation for the method.
    impls: BTreeMap<ModelTypeSystem, TokenStream>,
}

impl MethodImpl
{
    pub fn new(mi: ComMethodInfo) -> Self
    {
        MethodImpl {
            info: mi,
            impls: Default::default(),
        }
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
pub fn expand_com_interface(
    attr_tokens: TokenStreamNightly,
    item_tokens: TokenStreamNightly,
) -> Result<TokenStreamNightly, model::ParseError>
{
    // Parse the attribute.
    let mut output = vec![];
    let itf =
        model::ComInterface::from_ast(&lib_name(), attr_tokens.into(), item_tokens.clone().into())?;
    let itf_path = &itf.path;
    let itf_name = itf.ident.to_string();
    let itf_ref = &itf.itf_ref;

    let mut itf_output = InterfaceOutput::default();
    for (ts, itf_variant) in &itf.variants {
        process_itf_variant(&itf, *ts, itf_variant, &mut output, &mut itf_output);
    }

    if itf.item_type == utils::InterfaceType::Trait {
        let mut impls = vec![];
        for (_, method) in itf_output.method_impls.iter() {
            let method_rust_ident = &method.info.name;
            let method_name = method_rust_ident.to_string();
            let mut impl_branches = vec![];
            for (ts, method_ts_impl) in method.impls.iter() {
                let ts_tokens = ts.as_typesystem_type(method.info.signature_span);
                let ts_name = format!("{:?}", ts);
                impl_branches.push(quote_spanned!(method.info.signature_span =>
                    if let Some( comptr ) = intercom::ComItf::ptr::<#ts_tokens>( self ) {
                        intercom::logging::trace(|l| l(module_path!(), format_args!(
                            "[{:p}, with {:p}] Calling {}::{}, type system: {}",
                            self, comptr.ptr, #itf_name, #method_name, #ts_name)));

                        #method_ts_impl
                    }
                ));
            }

            // Format the method arguments into tokens.
            let impl_args = method.info.args.iter().map(|ca| {
                let name = &ca.name;
                let ty = &ca.ty;
                quote_spanned!(ca.span => #name : #ty )
            });

            let unsafety = if method.info.is_unsafe {
                quote_spanned!(method.info.signature_span => unsafe)
            } else {
                quote!()
            };
            let self_arg = &method.info.rust_self_arg;
            let return_ty = &method.info.rust_return_ty;

            // Rust to COM implementation.
            impls.push(quote_spanned!(method.info.signature_span =>
                #unsafety fn #method_rust_ident(
                    #self_arg, #( #impl_args ),*
                ) -> #return_ty {

                    intercom::logging::trace(|l| l(module_path!(), format_args!(
                        "[{:p}] Calling {}::{}", self, #itf_name, #method_name)));

                    #[allow(unused_imports)]
                    use intercom::ErrorValue;

                    // Try the available type systems.
                    #( #impl_branches )*

                    // The ComItf invariant states it has at least one pointer
                    // available so we should never get here.
                    //
                    // Also since this is Rust-to-COM call we are allowed to
                    // panic here.
                    unreachable!();
                }
            ));
        }

        let unsafety = if itf.is_unsafe {
            quote_spanned!(itf.span => unsafe)
        } else {
            quote!()
        };
        output.push(quote_spanned!(itf.span =>
            #[allow(clippy::all)]
            #[allow(unused_braces)]
            #unsafety impl<I: intercom::attributes::ComInterface + #itf_path + ?Sized> #itf_path for intercom::ComItf<I> {
                #( #impls )*
            }
        ));
    }

    // Implement the ComInterface for the trait.
    let iid_arms = itf_output.iid_arms;
    let (deref_impl, deref_ret) = if itf.item_type == utils::InterfaceType::Trait {
        (
            quote_spanned!(itf.span => com_itf),
            quote_spanned!(itf.span => &( dyn #itf_path + 'static ) ),
        )
    } else {
        // Note this is _extremely_ dangerous.
        //
        // Essentially we are assuming here that every #itf_path pointer represents
        // a ComBox structure that we have created. This will fail the moment
        // the user code implements #itf_path interface on their own and passes
        // that back to us.
        //
        // There's no real way to get this to work and we might want to just remove
        // the possibility to do 'implicit' interfaces by just impling the struct.
        (
            quote_spanned!(itf.span =>
                let some_iunk : &intercom::ComItf<dyn intercom::interfaces::RawIUnknown> = com_itf.as_raw_iunknown();
                let iunknown_iid = intercom::IUnknown::iid(
                        intercom::type_system::TypeSystemName::Automation )
                            .expect( "IUnknown must have Automation IID" );
                let primary_iunk = some_iunk.query_interface( iunknown_iid )
                        .expect( "All types must implement IUnknown" );

                let combox : *mut intercom::ComBoxData< #itf_path > =
                        primary_iunk as *mut intercom::ComBoxData< #itf_path >;
                unsafe {

                    // We are already holding a reference to the 'self', which should
                    // keep this alive. We don't need to maintain a lifetime of the
                    // queried interface.
                    intercom::ComBoxData::release( combox );

                    // Deref.
                    use std::ops::Deref;
                    (*combox).deref()
                }
            ),
            quote_spanned!(itf.span => & #itf_path ),
        )
    };

    output.push(quote_spanned!(itf.span =>
        impl intercom::attributes::ComInterface for #itf_ref {

            type TSelf = #itf_ref;

            #[doc = "Returns the IID of the requested interface."]
            fn iid_ts<TS: intercom::type_system::TypeSystem>() -> &'static intercom::IID
                where Self: intercom::attributes::ComInterfaceVariant<TS>
            {
                <Self as intercom::attributes::ComInterfaceVariant<TS>>::iid()
            }

            fn iid(
                ts : intercom::type_system::TypeSystemName
            ) -> Option< &'static intercom::IID >
            {
                match ts {
                    #( #iid_arms ),*
                }
            }

            fn deref(
                com_itf : &intercom::ComItf<#itf_ref>
            ) -> #deref_ret {
                #deref_impl
            }
        }
    ));

    // Implement type info for the interface.
    output.push(quote_spanned!(itf.span =>

        impl intercom::type_system::ForeignType for #itf_ref {

            /// The name of the type.
            fn type_name() -> &'static str { stringify!( #itf_path )  }
        }

    ));

    // Create runtime type info.
    output.push(create_get_typeinfo_function(&itf).map_err(|e| {
        model::ParseError::ComInterface(itf_path.get_some_ident().unwrap().to_string(), e)
    })?);

    Ok(tokens_to_tokenstream(item_tokens, output))
}

/// Processes the interface type system variant.
///
/// # Arguments
///
/// * `itf` - Interface details.
/// * `ts` - Type system the variant represents.
/// * `itf_variant` - Interface variant details.
/// * `output` - Direct output emitted for each interface variant.
/// * `itf_output` - Interface variant data for the interface level output.
fn process_itf_variant(
    itf: &model::ComInterface,
    ts: ModelTypeSystem,
    itf_variant: &model::ComInterfaceVariant,
    output: &mut Vec<TokenStream>,
    itf_output: &mut InterfaceOutput,
)
{
    let itf_path = &itf.path;
    let itf_ident = &itf.ident;
    let visibility = &itf.visibility;
    let ts_value_tokens = ts.as_typesystem_tokens(itf.span);
    let ts_type_tokens = ts.as_typesystem_type(itf.span);
    let itf_ref = &itf.itf_ref;
    let vtable_path = itf.vtable(ts);
    let attr_cominterfacevariant = quote_spanned!(itf_ident.span() =>
        intercom::attributes::ComInterfaceVariant<#ts_type_tokens>);
    let attr_cominterfacevtablefor = quote_spanned!(itf_ident.span() =>
        intercom::attributes::ComInterfaceVTableFor<I, S, #ts_type_tokens>);

    // Construct the iid(ts) match arm for this type system.
    itf_output
        .iid_arms
        .push(quote_spanned!(itf.span => #ts_value_tokens => Some( <Self as #attr_cominterfacevariant>::iid() ) ));

    // Create a vector for the virtual table fields and insert the base
    // interface virtual table in it if required.
    let mut vtbl_fields = vec![];
    let mut vtbl_values = vec![];
    if let Some(ref base) = itf.base_interface {
        vtbl_values.push(quote_spanned!(itf.span =>
                __base : <dyn #base as #attr_cominterfacevtablefor>::VTABLE));
        vtbl_fields.push(quote_spanned!(itf.span =>
                pub __base : <dyn #base as #attr_cominterfacevariant>::VTable));
    }

    // Gather all the trait methods for the remaining vtable fields.
    for method_info in &itf_variant.methods {
        // Create the vtable field and add it to the vector of fields.
        let (vtbl_field, vtbl_value) = format_method_vtable_entries(itf, method_info, ts);
        vtbl_fields.push(vtbl_field);
        vtbl_values.push(vtbl_value);

        // Ensure the MethodImpl exists for the method.
        // These are shared by type systems so only one type system needs to
        // add them here.
        let method_name = method_info.name.to_string();
        if !itf_output.method_impls.contains_key(&method_name) {
            itf_output
                .method_impls
                .insert(method_name.clone(), MethodImpl::new(method_info.clone()));
        }

        let method_impl = &mut itf_output
            .method_impls
            .get_mut(&method_name)
            .expect("We just ensured this exists three lines up... ;_;");
        method_impl.impls.insert(
            itf_variant.type_system,
            rust_to_com_delegate(itf, itf_variant, &method_info),
        );

        output.push(create_virtual_method(itf, method_info, ts));
    }

    // Create the vtable. We've already gathered all the vtable method
    // pointer fields so defining the struct is simple enough.
    let itf_bound = match itf.item_type {
        utils::InterfaceType::Struct => quote!(),
        utils::InterfaceType::Trait if itf.implemented_by.is_some() => quote!(),
        utils::InterfaceType::Trait => quote!(+ #itf_ident),
    };
    if itf.vtable_of.is_none() {
        output.push(quote_spanned!(itf.span =>
            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            #[allow(clippy::all)]
            #[repr(C)]
            #[doc(hidden)]
            #[derive(Clone, Copy)]
            #visibility struct #vtable_path { #( #vtbl_fields, )* }

            #[allow(unused)]
            impl<I, S> intercom::attributes::ComInterfaceVTableFor<I, S, #ts_type_tokens> for #itf_ref
            where I: ?Sized,
                  S: intercom::attributes::ComClassInterface<I, #ts_type_tokens> + intercom::attributes::ComClass #itf_bound,
            {
                const VTABLE: #vtable_path = #vtable_path {
                    #( #vtbl_values, )*
                };
            }
        ));
    }

    let iid_tokens = utils::get_guid_tokens(&itf_variant.iid, itf.span);
    output.push(quote_spanned!(itf_path.span() =>
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        #[allow(clippy::all)]
        #[doc(hidden)]
        impl #attr_cominterfacevariant for #itf_ref {
            type VTable = #vtable_path;
            fn iid() -> &'static intercom::IID {
                & #iid_tokens
            }
        }
    ));
}

/// Creates the functions responsible for delegating calls from Rust to COM
/// interfaces.
///
/// # Arguments
///
/// * `itf` - Interface details.
/// * `itf_variant` - Interface variant details.
/// * `method_info` - Method to delegate.
fn rust_to_com_delegate(
    itf: &model::ComInterface,
    itf_variant: &model::ComInterfaceVariant,
    method_info: &ComMethodInfo,
) -> TokenStream
{
    // The COM out-arguments that mirror the Rust return value will
    // require temporary variables during the COM call. Format their
    // declarations.
    let infallible = method_info.returnhandler.is_infallible();
    let out_arg_declarations = method_info
        .returnhandler
        .com_out_args()
        .iter()
        .map(|ca| {
            let ident = &ca.name;
            let ty = &ca.handler.com_ty(ca.span, Direction::Retval, infallible);
            let default = ca.handler.default_value();
            quote_spanned!(ca.span => let mut #ident : #ty = #default; )
        })
        .collect::<Vec<_>>();

    // Format the in and out parameters for the COM call.
    let params: Vec<_> = method_info
        .raw_com_args()
        .into_iter()
        .map(|com_arg| {
            let name = com_arg.name;
            match com_arg.dir {
                Direction::In => {
                    com_arg
                        .handler
                        .rust_to_com(&name, com_arg.span, Direction::In, infallible)
                }
                Direction::Out | Direction::Retval => quote_spanned!(com_arg.span => &mut #name ),
            }
        })
        .collect();

    // Combine the parameters into the final parameter list.
    // This includes the 'this' pointer and both the IN and OUT
    // parameters.
    let params =
        iter::once(quote_spanned!(method_info.rust_self_arg.span() => comptr.ptr.as_ptr()))
            .chain(params);

    // Create the return statement.
    let return_ident = Ident::new("__result", Span::call_site());
    let return_statement = method_info.returnhandler.com_to_rust_return(&return_ident);

    // Resolve some of the fields needed for quote.
    let method_ident = &method_info.name;
    let return_ty = &method_info.rust_return_ty;
    let iid_tokens = utils::get_guid_tokens(&itf_variant.iid, method_info.signature_span);
    let itf_ref = &itf.itf_ref;
    let ts_type = itf_variant.type_system.as_typesystem_type(itf.span);

    // Construct the final method.
    if infallible {
        quote_spanned!(method_info.signature_span =>
            #[allow(unused_imports)]
            let vtbl = comptr.ptr.as_ptr() as *const *const <#itf_ref as
                intercom::attributes::ComInterfaceVariant<#ts_type>>::VTable;

            #[allow(unused_unsafe)]  // The fn itself _might_ be unsafe.
            unsafe {
                #( #out_arg_declarations )*
                let #return_ident = ((**vtbl).#method_ident)( #( #params ),* );

                let __intercom_iid = #iid_tokens;
                return { #return_statement };
            }
        )
    } else {
        quote_spanned!(method_info.signature_span =>
            #[allow(unused_imports)]
            let vtbl = comptr.ptr.as_ptr() as *const *const <#itf_ref as
                intercom::attributes::ComInterfaceVariant<#ts_type>>::VTable;

            // Use an IIFE to act as a try/catch block. The various template
            // substitutions might end up using ?'s for error handling. The IIFE allows
            // us to handle the results here immediately.
            #[allow(unused_unsafe)]  // The fn itself _might_ be unsafe.
            let __intercom_result : Result< #return_ty, intercom::ComError > = ( || unsafe {
                #( #out_arg_declarations )*
                let #return_ident = ((**vtbl).#method_ident)( #( #params ),* );

                let __intercom_iid = #iid_tokens;
                Ok( { #return_statement } )
            } )();

            return match __intercom_result {
                Ok( v ) => v,
                Err( err ) => < #return_ty as intercom::ErrorValue >::from_error( err ),
            };
        )
    }
}

fn format_method_vtable_entries(
    itf: &model::ComInterface,
    method_info: &ComMethodInfo,
    ts: ModelTypeSystem,
) -> (TokenStream, TokenStream)
{
    let itf_ident = &itf.ident;
    let method_ident = &method_info.name;
    let method_impl_ident = idents::com_to_rust_method_impl(itf_ident, method_ident, ts);
    let ret_ty = method_info.returnhandler.com_ty();
    let generics = match itf.item_type {
        utils::InterfaceType::Struct => quote!(),
        utils::InterfaceType::Trait => quote!(::<I, S>),
    };
    let params = method_info.get_parameters_tokenstream();

    (
        quote_spanned!(method_info.signature_span =>
            pub #method_ident : unsafe extern "system" fn(#params) -> #ret_ty),
        quote_spanned!(method_info.signature_span =>
            #method_ident: #method_impl_ident #generics),
    )
}

fn create_virtual_method(
    itf: &model::ComInterface,
    method_info: &ComMethodInfo,
    ts: ModelTypeSystem,
) -> TokenStream
{
    let itf_ident = &itf.ident;
    let method_ident = &method_info.name;
    let method_name = method_ident.to_string();
    let method_impl_ident = idents::com_to_rust_method_impl(itf_ident, method_ident, ts);
    let infallible = method_info.returnhandler.is_infallible();
    let itf_ident = &itf.ident;
    let itf_path = &itf.path;
    let ts_type_tokens = ts.as_typesystem_type(itf.span);
    let params = method_info.get_parameters_tokenstream();
    let attr_comclassinterface = quote!(intercom::attributes::ComClassInterface);

    // Format the in and out parameters for the Rust call.
    let in_args: Vec<_> = method_info
        .args
        .iter()
        .map(|ca| {
            ca.handler
                .com_to_rust(&ca.name, ca.span, Direction::In, infallible)
        })
        .collect();

    let return_ident = Ident::new("__result", Span::call_site());
    let return_statement = method_info.returnhandler.rust_to_com_return(&return_ident);
    let ret_ty = method_info.returnhandler.com_ty();

    // Figure out how to get the self struct reference.
    let self_struct_expr = if itf.implemented_by.is_some() {
        quote!(&*self_combox)
    } else if method_info.is_const {
        quote!(&**self_combox)
    } else {
        quote!(&mut **self_combox)
    };

    // The implemented_by option affects the actual method implementation
    // as well as the interface bounds. Interfaces implemented manually
    // do not require the interface as a bound.
    let (required_itf, call) = match &itf.implemented_by {
        Some(path) => (
            quote!(),
            quote!(#path::#method_ident(self_struct, #( #in_args ),*)),
        ),
        None => (
            quote!(+ #itf_ident),
            quote!(self_struct.#method_ident( #( #in_args ),* )),
        ),
    };

    // Since "+ Struct" is an invalid bound and wouldn't really make sense
    // anywya, we won't use generic parameters on struct impl based implicit
    // interfaces.
    let (generics, bounds, s_ref, i_ref) = match itf.item_type {
        utils::InterfaceType::Struct => (quote!(), quote!(), quote!(#itf_path), quote!(#itf_path)),
        utils::InterfaceType::Trait => (
            quote!(<I, S>),
            quote!(
                where I: ?Sized,
                      S: #attr_comclassinterface<I, #ts_type_tokens> + intercom::attributes::ComClass #required_itf
            ),
            quote!(S),
            quote!(I),
        ),
    };

    // Format the payload depending on whether the method is infallible or not.
    let payload = if infallible {
        quote!(
            let self_struct = #self_struct_expr;
            let #return_ident = #call;

            intercom::logging::trace(|l| l(module_path!(), format_args!(
                "[{:p}, through {:p}] Serving {}::{}, OK",
                self_combox, self_vtable, std::any::type_name::<#s_ref>(), #method_name)));

            #return_statement
        )
    } else {
        // Fallible methods require an error-catching closure and error handling.
        quote!(
            let result : Result< #ret_ty, intercom::ComError > = ( || {
                let self_struct = #self_struct_expr;
                let #return_ident = #call;
                Ok( { #return_statement } )
            } )();

            match result {
                Ok( v ) => {
                    intercom::logging::trace(|l| l(module_path!(), format_args!(
                        "[{:p}, through {:p}] Serving {}::{}, OK",
                        self_combox, self_vtable,
                        std::any::type_name::<#s_ref>(), #method_name)));
                    v
                },
                Err( err ) => {
                    intercom::logging::trace(|l| l(module_path!(), format_args!(
                        "[{:p}, through {:p}] Serving {}::{}, ERROR",
                        self_combox, self_vtable,
                        std::any::type_name::<#s_ref>(), #method_name)));
                    <#ret_ty as intercom::ErrorValue>::from_error(
                        intercom::store_error(err))
                },
            }
        )
    };

    quote!(
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[doc(hidden)]
        unsafe extern "system" fn #method_impl_ident #generics(
            #params
        ) -> #ret_ty
        #bounds
        {
            // Acquire the reference to the ComBoxData. For this we need
            // to offset the current 'self_vtable' vtable pointer.
            let offset = <#s_ref as #attr_comclassinterface<#i_ref, #ts_type_tokens>>::offset();
            let self_combox = ( self_vtable as usize - offset )
                    as *mut intercom::ComBoxData<#s_ref>;

            intercom::logging::trace(|l| l(module_path!(), format_args!(
                "[{:p}, through {:p}] Serving {}::{}",
                self_combox, self_vtable,
                std::any::type_name::<#s_ref>(), #method_name)));

            #payload
        }
    )
}

fn create_get_typeinfo_function(itf: &model::ComInterface) -> Result<TokenStream, String>
{
    let itf_name = itf.ident.to_string();
    let itf_ref = &itf.itf_ref;
    let mut variant_tokens = vec![];
    for (ts, variant) in &itf.variants {
        variant_tokens.push(create_typeinfo_for_variant(itf, *ts, &variant)?);
    }
    let is_impl_interface = itf.item_type == utils::InterfaceType::Struct;

    Ok(quote_spanned!(itf.span =>
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        impl intercom::attributes::ComInterfaceTypeInfo for #itf_ref
        {
            fn gather_type_info() -> Vec<intercom::typelib::TypeInfo>
            {
                let variants = vec![ #( #variant_tokens ),* ];

                vec![ intercom::typelib::TypeInfo::Interface(
                    intercom::ComBox::new( intercom::typelib::Interface {
                        name: #itf_name.into(),
                        variants: variants,
                        options: intercom::typelib::InterfaceOptions {
                            class_impl_interface: #is_impl_interface,
                            ..Default::default()
                        }
                    })
                ) ]
            }
        }
    ))
}

fn create_typeinfo_for_variant(
    itf: &model::ComInterface,
    ts: ModelTypeSystem,
    itf_variant: &model::ComInterfaceVariant,
) -> Result<TokenStream, String>
{
    let ts_tokens = ts.as_typesystem_tokens(itf.span);
    let ts_type = ts.as_typesystem_type(itf.span);
    let iid_tokens = utils::get_guid_tokens(&itf_variant.iid, itf.span);
    let methods = itf_variant.methods.iter().map( |m| {
        let infallible = m.returnhandler.is_infallible();
        let method_name = m.name.to_string();
        let return_type = match &m.return_type {
            Some(rt) => quote_spanned!(m.signature_span =>
                intercom::typelib::Arg {
                    name: "".into(),
                    ty: <
                        <#rt as intercom::type_system::ExternOutput<#ts_type>>::ForeignType
                        as intercom::type_system::ForeignType>::type_name().into(),
                    indirection_level: <
                        <#rt as intercom::type_system::ExternOutput<#ts_type>>::ForeignType
                        as intercom::type_system::ForeignType>::indirection_level(),
                    direction: intercom::typelib::Direction::Return,
                }),
            None => quote_spanned!(m.signature_span => intercom::typelib::Arg {
                name: "".into(),
                ty: "void".into(),
                indirection_level: 0,
                direction: intercom::typelib::Direction::Return,
            } ),
        };

        let params = m.raw_com_args().into_iter().map(|arg| {
            let com_ty = arg.handler.com_ty(arg.span, arg.dir, infallible);
            let arg_name = arg.name.to_string();
            let dir_ident = Ident::new(match arg.dir {
                Direction::In => "In",
                Direction::Out => "Out",
                Direction::Retval => "Retval"
            }, arg.span);

            let ty_info_trait = Ident::new(match arg.dir {
                Direction::Out | Direction::Retval => "ForeignType",
                Direction::In => "ForeignType",
            }, arg.span);

            quote_spanned!(arg.span => intercom::typelib::Arg {
                name: #arg_name.into(),
                ty: <#com_ty as intercom::type_system::#ty_info_trait>::type_name().into(),
                indirection_level: <#com_ty as intercom::type_system::#ty_info_trait>::indirection_level(),
                direction: intercom::typelib::Direction::#dir_ident,
            })
        }).collect::<Vec<_>>();

        quote_spanned!(m.signature_span =>
            intercom::ComBox::new(intercom::typelib::Method {
                name: #method_name.into(),
                return_type: #return_type,
                parameters: vec![ #( #params ),* ],
            })
        )
    }).collect::<Vec<_>>();

    Ok(quote_spanned!(itf.span =>
        intercom::ComBox::new( intercom::typelib::InterfaceVariant {
            ts: #ts_tokens,
            iid: #iid_tokens,
            methods: vec![ #( #methods ),* ],
        })
    ))
}
