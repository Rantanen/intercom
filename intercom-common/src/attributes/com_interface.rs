
use crate::prelude::*;
use super::common::*;

use std::iter;
use std::collections::BTreeMap;

use crate::idents;
use crate::utils;
use crate::tyhandlers::{Direction, ModelTypeSystem};
use crate::model;
use crate::methodinfo::ComMethodInfo;

extern crate proc_macro;

/// Interface level output.
#[derive(Default)]
struct InterfaceOutput {
    iid_arms : Vec<TokenStream>,
    method_impls : BTreeMap< String, MethodImpl >,
}

struct MethodImpl {

    /// _Some_ method info.
    ///
    /// This should not be depended upon for anything type system specific.
    info : ComMethodInfo,

    /// Type system specific implementation for the method.
    impls : BTreeMap< ModelTypeSystem, TokenStream >,
}

impl MethodImpl {
    pub fn new( mi : ComMethodInfo ) -> Self {
        MethodImpl {
            info: mi,
            impls : Default::default(),
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
    let itf = model::ComInterface::parse(
            &lib_name(),
            attr_tokens.into(),
            &item_tokens.to_string() )?;
    let itf_ident = itf.name();

    let mut itf_output = InterfaceOutput::default();
    for ( &ts, itf_variant ) in itf.variants() {
        process_itf_variant(
                &itf, ts, itf_variant,
                &mut output, &mut itf_output );
    }

    if itf.item_type() == utils::InterfaceType::Trait {
        let mut impls = vec![];
        for ( _, method ) in itf_output.method_impls.iter() {

            let mut impl_branches = vec![];
            for ( ts, method_ts_impl ) in method.impls.iter() {

                let ts_tokens = ts.as_typesystem_type();
                impl_branches.push( quote!(
                    if let Some( comptr ) = ComItf::maybe_ptr::< #ts_tokens >( self ) {
                        #method_ts_impl
                    }
                ) );
            }

            // Format the method arguments into tokens.
            let impl_args = method.info.args.iter().map( |ca| {
                let name = &ca.name;
                let ty = &ca.ty;
                quote!( #name : #ty )
            } );

            let unsafety = if method.info.is_unsafe { quote!( unsafe ) } else { quote!() };
            let self_arg = &method.info.rust_self_arg;
            let method_rust_ident = &method.info.display_name;
            let return_ty = &method.info.rust_return_ty;

            // Rust to COM implementation.
            impls.push( quote!(
                #unsafety fn #method_rust_ident(
                    #self_arg, #( #impl_args ),*
                ) -> #return_ty {

                    #[allow(unused_imports)]
                    use intercom::ComInto;
                    #[allow(unused_imports)]
                    use intercom::ErrorValue;

                    // Try the available type systems.
                    #( #impl_branches )*

                    // None of the type system pointers were available,
                    // which means this is a null reference.
                    < #return_ty as intercom::ErrorValue >::from_com_error(
                            intercom::ComError::E_POINTER.into() )
                }
            ) );
        }

        let unsafety = if itf.is_unsafe() { quote!( unsafe ) } else { quote!() };
        output.push( quote!(
            #unsafety impl #itf_ident for intercom::ComItf< #itf_ident > {
                #( #impls )*
            }
        ) );
    }

    // Implement the ComInterface for the trait.
    let iid_arms = itf_output.iid_arms;
    let ( deref_impl, deref_ret ) = if itf.item_type() == utils::InterfaceType::Trait {
        (
            quote!( com_itf ),
            quote!( &( #itf_ident + 'static ) )
        )
    } else {

        // Note this is _extremely_ dangerous.
        //
        // Essentially we are assuming here that every #itf_ident pointer represents
        // a ComBox structure that we have created. This will fail the moment
        // the user code implements #itf_ident interface on their own and passes
        // that back to us.
        //
        // There's no real way to get this to work and we might want to just remove
        // the possibility to do 'implicit' interfaces by just impling the struct.
        (
            quote!(
                let some_iunk : &intercom::ComItf<intercom::IUnknown> = com_itf.as_ref();
                let iunknown_iid = intercom::IUnknown::iid(
                        intercom::type_system::TypeSystemName::Automation )
                            .expect( "IUnknown must have Automation IID" );
                let primary_iunk = some_iunk.query_interface( iunknown_iid )
                        .expect( "All types must implement IUnknown" );

                let combox : *mut intercom::ComBox< #itf_ident > =
                        primary_iunk as *mut intercom::ComBox< #itf_ident >;
                unsafe {

                    // We are already holding a reference to the 'self', which should
                    // keep this alive. We don't need to maintain a lifetime of the
                    // queried interface.
                    intercom::ComBox::release( combox );

                    // Deref.
                    use std::ops::Deref;
                    (*combox).deref()
                }
            ),
            quote!( & #itf_ident )
        )
    };

    output.push( quote!(
        impl intercom::ComInterface for #itf_ident {

            #[doc = "Returns the IID of the requested interface."]
            fn iid(
                ts : intercom::type_system::TypeSystemName
            ) -> Option< &'static intercom::IID >
            {
                match ts {
                    #( #iid_arms ),*
                }
            }

            fn deref(
                com_itf : &intercom::ComItf< #itf_ident >
            ) -> #deref_ret {
                #deref_impl
            }
        }
    ) );

    // Implement type info for the interface.
    output.push( quote!(

        impl intercom::type_system::BidirectionalTypeInfo for #itf_ident {

            /// The name of the type.
            fn type_name() -> &'static str { stringify!( #itf_ident )  }
        }

    ) );

    // Create runtime type info.
    output.push( create_get_typeinfo_function( &itf )
        .map_err(|e| model::ParseError::ComInterface( itf_ident.to_string(), e ) )? );

    Ok( tokens_to_tokenstream( item_tokens, output ) )
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
    itf : &model::ComInterface,
    ts : ModelTypeSystem,
    itf_variant : &model::ComInterfaceVariant,
    output : &mut Vec<TokenStream>,
    itf_output : &mut InterfaceOutput,
) {

    let itf_ident = itf.name();
    let visibility = itf.visibility();
    let iid_ident = idents::iid( itf_variant.unique_name() );
    let vtable_ident = idents::vtable_struct( itf_variant.unique_name() );

    // IID_IInterface GUID.
    let iid_tokens = utils::get_guid_tokens( itf_variant.iid() );
    let iid_doc = format!( "`{}` interface ID.", itf_ident );
    output.push( quote!(
        #[doc = #iid_doc]
        #[allow(non_upper_case_globals)]
        #visibility const #iid_ident : intercom::IID = #iid_tokens;
    ) );

    // Construct the iid(ts) match arm for this type system.
    let ts_match = ts.as_typesystem_tokens();
    itf_output.iid_arms.push( quote!( #ts_match => Some( & #iid_ident ) ) );

    // Create a vector for the virtual table fields and insert the base
    // interface virtual table in it if required.
    let mut vtbl_fields = vec![];
    if let Some( ref base ) = *itf.base_interface() {
        let vtbl = match base.to_string().as_ref() {
            "IUnknown" => quote!( intercom::IUnknownVtbl ),
            _ => { let vtbl = idents::vtable_struct( &base ); quote!( #vtbl ) }
        };
        vtbl_fields.push( quote!( pub __base : #vtbl ) );
    }

    // Gather all the trait methods for the remaining vtable fields.
    let calling_convention = get_calling_convetion();
    for method_info in itf_variant.methods() {

        let method_ident = &method_info.unique_name;
        let in_out_args = method_info.raw_com_args()
                .into_iter()
                .map( |com_arg| {
                    let name = &com_arg.name;
                    let com_ty = &com_arg.handler.com_ty( com_arg.dir );
                    let dir = match com_arg.dir {
                        Direction::In => quote!(),
                        Direction::Out | Direction::Retval => quote!( *mut )
                    };
                    quote!( #name : #dir #com_ty )
                } );
        let self_arg = quote!( self_vtable : intercom::RawComPtr );
        let args = iter::once( self_arg ).chain( in_out_args );

        // Create the vtable field and add it to the vector of fields.
        let ret_ty = method_info.returnhandler.com_ty();
        vtbl_fields.push( quote!(
            pub #method_ident :
                unsafe extern #calling_convention fn( #( #args ),* ) -> #ret_ty
        ) );

        let method_name = method_info.display_name.to_string();
        if ! itf_output.method_impls.contains_key( &method_name ) {
            itf_output.method_impls.insert(
                    method_name.clone(),
                    MethodImpl::new( method_info.clone() ) );
        }

        let method_impl = &mut itf_output.method_impls.get_mut( &method_name )
                .expect( "We just ensured this exists three lines up... ;_;" );
        method_impl.impls.insert(
                itf_variant.type_system(),
                rust_to_com_delegate( itf_variant, method_info, &vtable_ident ) );
    }

    // Create the vtable. We've already gathered all the vtable method
    // pointer fields so defining the struct is simple enough.
    output.push( quote!(
        #[allow(non_camel_case_types)]
        #[repr(C)]
        #[doc(hidden)]
        #visibility struct #vtable_ident { #( #vtbl_fields, )* }
    ) );
}

/// Creates the functions responsible for delegating calls from Rust to COM
/// interfaces.
///
/// # Arguments
///
/// * `itf_variant` - Interface variant details.
/// * `method_info` - Method to delegate.
/// * `vtable_ident` - Vtable to use for the delegation.
fn rust_to_com_delegate(
    itf_variant : &model::ComInterfaceVariant,
    method_info : &ComMethodInfo,
    vtable_ident : &Ident,
) -> TokenStream {

    // The COM out-arguments that mirror the Rust return value will
    // require temporary variables during the COM call. Format their
    // declarations.
    let out_arg_declarations = method_info.returnhandler.com_out_args()
            .iter()
            .map( |ca| {
                let ident = &ca.name;
                let ty = &ca.handler.com_ty( Direction::Retval );
                let default = ca.handler.default_value();
                quote!( let mut #ident : #ty = #default; )
            } ).collect::<Vec<_>>();

    // Format the in and out parameters for the COM call.
    let params : Vec<_> = method_info.raw_com_args()
            .into_iter()
            .map( |com_arg| {
                let name = com_arg.name;
                match com_arg.dir {
                    Direction::In => {
                        com_arg.handler.rust_to_com( &name, Direction::In )
                    },
                    Direction::Out | Direction::Retval
                        => quote!( &mut #name ),
                }
            } )
            .collect();

    // Combine the parameters into the final parameter list.
    // This includes the 'this' pointer and both the IN and OUT
    // parameters.
    let params = iter::once( quote!( comptr.ptr ) ).chain( params );

    // Create the return statement.
    let return_ident = Ident::new( "__result", Span::call_site() );
    let return_statement = method_info
            .returnhandler
            .com_to_rust_return( &return_ident );

    // Resolve some of the fields needed for quote.
    let method_ident = &method_info.unique_name;
    let return_ty = &method_info.rust_return_ty;
    let iid_tokens = utils::get_guid_tokens( itf_variant.iid() );

    // Construct the final method.
    quote!(
        use intercom::type_system::{IntercomFrom, IntercomInto};
        let vtbl = comptr.ptr as *const *const #vtable_ident;

        // Use an IIFE to act as a try/catch block. The various template
        // substitutions might end up using ?'s for error handling. The IIFE allows
        // us to handle the results here immediately.
        #[allow(unused_unsafe)]  // The fn itself _might_ be unsafe.
        let result : Result< #return_ty, intercom::ComError > = ( || unsafe {
            #( #out_arg_declarations )*
            let #return_ident = ((**vtbl).#method_ident)( #( #params ),* );

            let INTERCOM_iid = #iid_tokens;
            Ok( { #return_statement } )
        } )();

        return match result {
            Ok( v ) => v,
            Err( err ) => < #return_ty as intercom::ErrorValue >::from_com_error( err ),
        };
    )
}

fn create_get_typeinfo_function(
    itf: &model::ComInterface,
) -> Result<TokenStream, String>
{
    let fn_name = Ident::new(
            &format!("get_intercom_interface_info_for_{}", itf.name() ),
            Span::call_site() );
    let itf_name = itf.name().to_string();
    let mut variant_tokens = vec![];
    for (ts, variant) in itf.variants() {
        variant_tokens.push( create_typeinfo_for_variant(itf, *ts, variant)? );
    }
    let is_impl_interface = itf.item_type() == utils::InterfaceType::Struct;

    Ok(quote!(
        pub(crate) fn #fn_name() -> intercom::typelib::TypeInfo
        {
            let mut variants = vec![ #( #variant_tokens ),* ];

            intercom::typelib::TypeInfo::Interface(
                intercom::ComStruct::new( intercom::typelib::Interface {
                    name: #itf_name.into(),
                    variants: variants,
                    options: intercom::typelib::InterfaceOptions {
                        class_impl_interface: #is_impl_interface,
                        ..Default::default()
                    }
                })
            )
        }
    ))
}

fn create_typeinfo_for_variant(
    _itf: &model::ComInterface,
    ts: ModelTypeSystem,
    itf_variant: &model::ComInterfaceVariant,
) -> Result<TokenStream, String>
{
    let ts_tokens = ts.as_typesystem_tokens();
    let ts_type = ts.as_typesystem_type();
    let iid_tokens = utils::get_guid_tokens( itf_variant.iid() );
    let methods = itf_variant.methods().iter().map( |m| {
        let method_name = m.display_name.to_string();
        let return_type = match &m.return_type {
            Some(rt) => quote!( intercom::typelib::Arg {
                name: "".into(),
                ty: <
                    <#rt as intercom::type_system::ExternType<#ts_type>>::ExternOutputType
                    as intercom::type_system::OutputTypeInfo>::type_name().into(),
                indirection_level: <
                    <#rt as intercom::type_system::ExternType<#ts_type>>::ExternOutputType
                    as intercom::type_system::OutputTypeInfo>::indirection_level(),
                direction: intercom::typelib::Direction::Return,
            }),
            None => quote!( intercom::typelib::Arg {
                name: "".into(),
                ty: "void".into(),
                indirection_level: 0,
                direction: intercom::typelib::Direction::Return,
            } ),
        };

        let params = m.raw_com_args().into_iter().map(|arg| {
            let com_ty = arg.handler.com_ty(arg.dir);
            let arg_name = arg.name.to_string();
            let dir_ident = Ident::new( match arg.dir {
                Direction::In => "In",
                Direction::Out => "Out",
                Direction::Retval => "Retval"
            }, Span::call_site() );

            let ty_info_trait = Ident::new( match arg.dir {
                Direction::Out | Direction::Retval => "OutputTypeInfo",
                Direction::In => "InputTypeInfo",
            }, Span::call_site() );

            quote!( intercom::typelib::Arg {
                name: #arg_name.into(),
                ty: <#com_ty as intercom::type_system::#ty_info_trait>::type_name().into(),
                indirection_level: <#com_ty as intercom::type_system::#ty_info_trait>::indirection_level(),
                direction: intercom::typelib::Direction::#dir_ident,
            })
        }).collect::<Vec<_>>();

        quote!(
            intercom::ComStruct::new(intercom::typelib::Method {
                name: #method_name.into(),
                return_type: #return_type,
                parameters: vec![ #( #params ),* ],
            })
        )
    }).collect::<Vec<_>>();

    Ok(quote!(
        intercom::ComStruct::new( intercom::typelib::InterfaceVariant {
            ts: #ts_tokens,
            iid: #iid_tokens,
            methods: vec![ #( #methods ),* ],
        })
    ))
}
