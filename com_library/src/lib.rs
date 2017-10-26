#![crate_type="dylib"]
#![feature(quote, plugin_registrar, rustc_private)]
#![allow(unused_imports)]
#![feature(catch_expr)]
#![feature(type_ascription)]

mod utils;
mod idents;

extern crate syntax;
extern crate syntax_pos;
extern crate rustc;
extern crate rustc_plugin;
extern crate com_runtime;

use syntax::ptr::P;
use syntax::symbol::Symbol;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::{MultiDecorator};
use syntax::tokenstream::TokenTree;
use syntax::ast::{
    Ident,
    Item, ItemKind, ImplItemKind,
    MetaItem, MetaItemKind, NestedMetaItemKind, LitKind,
    MutTy, Ty, TyKind, FunctionRetTy,
    PathParameters, Mutability,
    Attribute,
};
use rustc_plugin::Registry;
use syntax::print::pprust;

/// Implements the `com_impl` decorator.
pub fn try_expand_com_impl(
    cx: &mut ExtCtxt,
    _sp: Span,
    _mi: &MetaItem,
    item: &Annotatable,
    push: &mut FnMut( Annotatable )
) -> Result< (), &'static str >
{
    // Get the item info the attribute is bound to.
    let ( itf_ident_opt, struct_ident, fns )
            = utils::get_impl_data( item )
                .ok_or( "[com_impl] must be applied to an impl" )?;
    let itf_ident = itf_ident_opt.unwrap_or( struct_ident.clone() );
    let vtable_struct_ident = idents::vtable_struct( &itf_ident );
    let vtable_instance_ident = idents::vtable_instance( &struct_ident, &itf_ident );
    let vtable_offset = idents::vtable_offset(
        &struct_ident,
        &itf_ident );

    let query_interface_ident = idents::method_impl(
            &struct_ident, &itf_ident, "query_interface" );
    let add_ref_ident = idents::method_impl(
            &struct_ident, &itf_ident, "add_ref" );
    let release_ident = idents::method_impl(
            &struct_ident, &itf_ident, "release" );

    let iunk_ident = Ident::from_str( "IUnknown" );
    let primary_query_interface_ident = idents::method_impl(
            &struct_ident, &iunk_ident, "query_interface" );
    let primary_add_ref_ident = idents::method_impl(
            &struct_ident, &iunk_ident, "add_ref" );
    let primary_release_ident = idents::method_impl(
            &struct_ident, &iunk_ident, "release" );

    /////////////////////
    // $itf::QueryInterface, AddRef & Release
    //
    // The primary add_ref and release. As these are on the IUnknown interface
    // the self_vtable here points to the start of the ComRef structure.
    push( Annotatable::Item( quote_item!( cx,
            #[allow(non_snake_case)]
            pub unsafe extern "stdcall" fn $query_interface_ident(
                self_vtable : com_runtime::RawComPtr,
                riid : com_runtime::REFIID,
                out : *mut com_runtime::RawComPtr
            ) -> com_runtime::HRESULT
            {
                // Get the primary iunk interface by offsetting the current
                // self_vtable with the vtable offset. Once we have the primary
                // pointer we can delegate the call to the primary implementation.
                let iunk_void = ( self_vtable as usize - $vtable_offset() ) as com_runtime::RawComPtr;
                $primary_query_interface_ident( iunk_void, riid, out )
            }
        ).unwrap() ) );
    push( Annotatable::Item( quote_item!( cx,
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            pub unsafe extern "stdcall" fn $add_ref_ident(
                self_vtable : com_runtime::RawComPtr
            ) -> u32 {
                com_runtime::ComBox::< $struct_ident >::add_ref(
                        &mut *(( self_vtable as usize - $vtable_offset() ) as *mut _ ) )
            }
        ).unwrap() ) );
    push( Annotatable::Item( quote_item!( cx,
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            pub unsafe extern "stdcall" fn $release_ident(
                self_vtable : com_runtime::RawComPtr
            ) -> u32 {
                com_runtime::ComBox::< $struct_ident >::release_ptr(
                        ( self_vtable as usize - $vtable_offset() ) as *mut _ )
            }
        ).unwrap() ) );

    // Start the vtable with the IUnknown implementation.
    //
    // Note that the actual methods implementation for these bits differs from
    // the primary IUnknown methods. When the methods are being called through
    // this vtable, the self_vtable pointer will point to this vtable and not
    // the start of the CoClass instance.
    let mut vtable_fields = vec![
        quote_tokens!( cx,
            __base : com_runtime::IUnknownVtbl {
                query_interface : $query_interface_ident,
                add_ref : $add_ref_ident,
                release : $release_ident,
            },
        ) ];


    // Implement the delegating calls for the coclass.
    for ( method_ident, method_sig ) in fns {
        let _res : Result<(), ()> = do catch {

            // Get the self argument and the remaining args.
            let ( args, params ) =
                    utils::get_method_args( cx, method_sig ).ok_or(())?;
            let ( ret_ty, return_statement ) =
                    utils::get_method_rvalues( cx, &method_sig ).ok_or(())?;

            let method_impl_ident = idents::method_impl(
                &struct_ident,
                &itf_ident,
                &method_ident.name.as_str() );

            // Define the delegating method implementation.
            //
            // Note the self_vtable here will be a pointer to the start of the
            // vtable for the current interface. To get the coclass and thus
            // the actual 'data' struct, we'll need to offset the self_vtable
            // with the vtable offset.
            push( Annotatable::Item( quote_item!( cx,
                #[allow(non_snake_case)]
                #[allow(dead_code)]
                pub unsafe extern "stdcall" fn $method_impl_ident( $args ) -> $ret_ty {
                    // Acquire the reference to the ComBox. For this we need
                    // to offset the current 'self_vtable' vtable pointer.
                    let self_comptr = ( self_vtable as usize - $vtable_offset() )
                            as *mut com_runtime::ComBox< $struct_ident >;
                    let result = (*self_comptr).$method_ident( $params );
                    $return_statement
                }
            ).unwrap() ) );
            vtable_fields.push( quote_tokens!( cx,
                    $method_ident : $method_impl_ident, ) );


            Ok(())
        };
    }

    push( Annotatable::Item( quote_item!( cx,
            #[allow(non_upper_case_globals)]
            const $vtable_instance_ident : $vtable_struct_ident
                    = $vtable_struct_ident { $vtable_fields };
        ).unwrap() ) );

    Ok(())
}

pub fn try_expand_com_class(
    cx: &mut ExtCtxt,
    _sp: Span,
    mi: &MetaItem,
    item: &Annotatable,
    push: &mut FnMut( Annotatable )
) -> Result< (), &'static str >
{
    // Get the item info the attribute is bound to.
    let struct_ident = utils::get_struct_ident_from_annotatable( item )
            .ok_or( "[com_class] must be applied to struct" )?;
    let iunk_ident = Ident::from_str( "IUnknown" );
    let query_interface_ident = idents::method_impl(
            &struct_ident, &iunk_ident, "query_interface" );
    let add_ref_ident = idents::method_impl(
            &struct_ident, &iunk_ident, "add_ref" );
    let release_ident = idents::method_impl(
            &struct_ident, &iunk_ident, "release" );

    let ( clsid_guid, itfs ) = utils::get_metaitem_params( mi )
            .as_ref()
            .and_then( |ref params| params.split_first() )
            .ok_or( "[com_class(IID, itfs...)] must specify an IID" )
            .and_then( |( f, itfs )|
                Ok( (
                    utils::parameter_to_guid( f )?,
                    ( itfs.into_iter()
                        .map( |i|
                            utils::parameter_to_ident( i )
                                .ok_or( "Invalid interface" ))
                        .collect() : Result<Vec<Ident>, &'static str> )?
                ) ) )?;

    // IUnknown vtable match. As the primary query_interface is implemented
    // on the root IUnknown interface, the self_vtable here should already be
    // the IUnknown we need.
    let mut match_arms = vec![
        quote_tokens!(
            cx, com_runtime::IID_IUnknown => self_iunk,
        ) ];

    // The vtable fields.
    let iunk_vtable_instance_ident =
            idents::vtable_instance( &struct_ident, &iunk_ident );
    let mut vtable_list_fields = vec![
        quote_tokens!(
            cx, _IUnknown : &'static com_runtime::IUnknownVtbl,
        ) ];
    let mut vtable_list_field_values = vec![
        quote_tokens!(
            cx, _IUnknown : &$iunk_vtable_instance_ident,
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
        push( Annotatable::Item( quote_item!( cx,
                #[inline(always)]
                #[allow(non_snake_case)]
                fn $offset_ident() -> usize {
                    unsafe { 
                        &com_runtime::ComBox::< $struct_ident >::null_vtable().$itf
                                as *const _ as usize
                    }
                };
        ).unwrap() ) );
        utils::trace( "vtable_offset", &offset_ident.name.as_str() );

        // The vtable pointer to for the ComBox vtable list.
        vtable_list_fields.push( quote_tokens!( cx,
                $itf : &'static $vtable_struct_ident,) );
        vtable_list_field_values.push( quote_tokens!( cx,
                $itf : &$vtable_instance_ident,) );

        // As this is the primary IUnknown query_interface, the self_vtable here
        // points to the start of the ComRef structure. The return value should
        // be the vtable corresponding to the given IID so we'll just offset
        // the self_vtable by the vtable offset.
        match_arms.push( quote_tokens!(
            cx, self::$iid_ident =>
                ( self_iunk as usize + $offset_ident() ) as com_runtime::RawComPtr,
        ) );
    }

    /////////////////////
    // IUnknown::QueryInterface, AddRef & Release
    //
    // The primary add_ref and release. As these are on the IUnknown interface
    // the self_vtable here points to the start of the ComRef structure.
    push( Annotatable::Item( quote_item!( cx,
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            pub unsafe extern "stdcall" fn $query_interface_ident(
                self_iunk : com_runtime::RawComPtr,
                riid : com_runtime::REFIID,
                out : *mut com_runtime::RawComPtr
            ) -> com_runtime::HRESULT
            {
                *out = match *riid {
                    $match_arms
                    _ => return com_runtime::E_NOINTERFACE
                };

                // We did *out assignment. Add reference count.
                com_runtime::ComBox::< $struct_ident >::add_ref_ptr( self_iunk );

                com_runtime::S_OK
            }
        ).unwrap() ) );
    push( Annotatable::Item( quote_item!( cx,
            #[allow(non_upper_case_globals)]
            const $iunk_vtable_instance_ident : com_runtime::IUnknownVtbl
                    = com_runtime::IUnknownVtbl {
                        query_interface : $query_interface_ident,
                        add_ref : com_runtime::ComBox::< $struct_ident >::add_ref_ptr,
                        release : com_runtime::ComBox::< $struct_ident >::release_ptr,
                    };
        ).unwrap() ) );

    // The CoClass implementation.
    //
    // Define the vtable list struct first. This lists the vtables of all the
    // interfaces that the coclass implements.
    let vtable_list_ident = idents::vtable_list( &struct_ident );
    push( Annotatable::Item( quote_item!( cx,
            #[allow(non_snake_case)]
            pub struct $vtable_list_ident {
                $vtable_list_fields
            }
        ).unwrap() ) );
    push( Annotatable::Item( quote_item!( cx,
            impl com_runtime::CoClass for $struct_ident {
                type VTableList = $vtable_list_ident;
                fn create_vtable_list() -> Self::VTableList {
                    $vtable_list_ident {
                        $vtable_list_field_values
                    }
                }
            }
        ).unwrap() ) );

    // CLSID constant for the class.
    let clsid_ident = idents::clsid( &struct_ident );
    let clsid_guid_tokens = utils::get_guid_tokens( cx, &clsid_guid );
    let clsid_const = quote_item!(
        cx,
        #[allow(non_upper_case_globals)]
        const $clsid_ident : com_runtime::GUID = $clsid_guid_tokens;
    ).unwrap();
    push( Annotatable::Item( clsid_const ) );

    Ok(())
}

/// `com_class` MultiDecorator handler.
///
/// Delegates to a Result<>ful try_... variant.
pub fn expand_com_class(
    cx: &mut ExtCtxt,
    sp: Span,
    mi: &MetaItem,
    item: &Annotatable,
    push: &mut FnMut( Annotatable )
) {
    if let Err( err ) = try_expand_com_class( cx, sp, mi, item, push ) {
        cx.span_err( mi.span, err );
    }
}

/// Implements the `com_library` decorator.
pub fn try_expand_com_library(
    cx: &mut ExtCtxt,
    _sp: Span,
    mi: &MetaItem,
    _item: &Annotatable,
    push: &mut FnMut( Annotatable )
) -> Result< (), &'static str >
{
    // Get the decorator parameters.
    let params = match mi.node {
        MetaItemKind::List( ref v ) => v,
        _ => return Err( "[com_library(...)] needs visible structs as parameters." )
    };

    // Create the match-statmeent patterns for each supposedly visible COM class.
    let mut match_arms : Vec<Vec<TokenTree>> = vec![];
    for p in params {

        // Extract the class name from the parameter item.
        let struct_ident = match p.node {
            NestedMetaItemKind::MetaItem( ref l ) => &l.name,
            _ => return Err( "Could not parse structs" )
        };

        // Construct the match pattern.
        let struct_ident = Ident::from_str( &struct_ident.as_str() );
        let clsid_name = idents::clsid( &struct_ident );
        match_arms.push( quote_tokens!( cx,
            self::$clsid_name => {
                com_runtime::ComBox::allocate( $struct_ident::new() )
                        .as_ptr() as com_runtime::RawComPtr
            },
        ) );
    }

    // Define the ClassFactory create_instance function.
    // 
    // This function is responsible for constructing all of our COM coclasses.
    let create_instance_ident = Ident::from_str( "__ClassFactory_create_instance" );
    let create_instance_def = quote_item!(
        cx,
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        pub unsafe extern "stdcall" fn $create_instance_ident(
            self_vtable: com_runtime::RawComPtr,
            _outer : com_runtime::RawComPtr,
            _iid : com_runtime::REFIID,
            out : *mut com_runtime::RawComPtr
        ) -> com_runtime::HRESULT
        {
            // Turn the *c_void to a ClassFactory pointer.
            let self_ptr : *mut com_runtime::ClassFactory = std::mem::transmute( self_vtable );

            // Match based on the clsid.
            #[allow(non_upper_case_globals)]
            let out_ptr = match *(*self_ptr).clsid {
                $match_arms
                _ => {
                    return com_runtime::E_NOINTERFACE
                }
            };

            // Return the created instance.
            *out = out_ptr;
            com_runtime::S_OK
        } ).unwrap();
    push( Annotatable::Item( create_instance_def ) );

    // Create the ClassFactory vtable instance.
    // 
    // Most of the vtable is static, but the create_instance needs to be
    // defined per library.
    let vtable_instance_ident = Ident::from_str( "__ClassFactory_vtable_instance" );
    let vtable_instance = quote_item!(
        cx,
        #[allow(non_upper_case_globals)]
        const $vtable_instance_ident : com_runtime::__ClassFactory_vtable =
                com_runtime::__ClassFactory_vtable {
                    __base : com_runtime::IUnknownVtbl {
                        query_interface : com_runtime::ClassFactory::query_interface,
                        add_ref : com_runtime::ClassFactory::add_ref,
                        release : com_runtime::ClassFactory::release,
                    },
                    create_instance: $create_instance_ident,
                    lock_server : com_runtime::ClassFactory::lock_server,
                };
    ).unwrap();
    push( Annotatable::Item( vtable_instance ) );

    // Implement DllGetClassObject.
    //
    // This is more or less the only symbolic entry point that the COM
    // infrastructure uses. The COM client uses this method to acquire
    // the IClassFactory interfaces that are then used to construct the
    // actual coclasses.
    let dll_get_class_object = quote_item!(
        cx,
        #[no_mangle]
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        pub unsafe extern "stdcall" fn DllGetClassObject(
            rclsid : com_runtime::REFCLSID,
            _riid : com_runtime::REFIID,
            pout : *mut com_runtime::RawComPtr
        ) -> com_runtime::HRESULT
        {
            // Create a ClassFactory. Save an add_ref by defining the
            // reference count as 1.
            let classFactory = Box::new( com_runtime::ClassFactory {
                __vtable : &$vtable_instance_ident,
                clsid : rclsid,
                rc : 1
            } );

            // Detach the class factory form the Box. This prevents it from
            // being destroyed when the Box is dropped. We need to handle the
            // destruction in release() manually.
            let ptrClassFactory = Box::into_raw( classFactory );

            // Assign to output and return OK.
            *pout = ptrClassFactory as com_runtime::RawComPtr;
            com_runtime::S_OK
        }
    ).unwrap();
    push( Annotatable::Item( dll_get_class_object ) );

    Ok(())
}

/// `com_library` MultiDecorator handler.
///
/// Delegates to a Result<>ful try_... variant.
pub fn expand_com_library(
    cx: &mut ExtCtxt,
    sp: Span,
    mi: &MetaItem,
    item: &Annotatable,
    push: &mut FnMut( Annotatable )
) {
    if let Err( err ) = try_expand_com_library( cx, sp, mi, item, push ) {
        cx.span_err( mi.span, err );
    }
}

/// Implements the `com_interface` decorator.
pub fn try_expand_com_interface(
    cx: &mut ExtCtxt,
    _sp: Span,
    mi: &MetaItem,
    item: &Annotatable,
    push: &mut FnMut( Annotatable )
) -> Result< (), &'static str >
{
    let ( itf_ident, fns )
            = utils::get_ident_and_fns( item )
                .ok_or( "[com_interface(IID:&str)] must be applied to trait or struct impl" )?;

    let iid_guid = utils::get_metaitem_params( mi )
            .as_ref()
            .and_then( |ref params| params.first() )
            .ok_or( "[com_interface(IID:&str)] must specify an IID" )
            .and_then( |f| utils::parameter_to_guid( f ) )?;

    let iid_tokens = utils::get_guid_tokens( cx, &iid_guid );
    let iid_ident = idents::iid( &itf_ident );

    // IID_IInterface GUID.
    push( Annotatable::Item(
        quote_item!( cx,
            #[allow(non_upper_case_globals)]
            const $iid_ident : com_runtime::GUID = $iid_tokens;
        ).unwrap()
    ) );

    // Create the base vtable field.
    // All of our interfaces inherit from IUnknown.
    let mut fields = vec![
        quote_tokens!( cx, __base : com_runtime::IUnknownVtbl, )
    ];

    // Process the impl items. This gathers all COM-visible methods and defines
    // delegating calls for them. These delegating calls are the ones that are
    // invoked by the clients. The calls then convert everything to the RUST
    // interface.
    //
    // The impl may have various kinds of items - we only support the ones that
    // seem okay so there's a bit of continue'ing involved in the for-loop.
    for ( method_ident, method_sig ) in fns {
        let _res : Result<(), ()> = do catch {

            // Get the self argument and the remaining args.
            let ( args, _) =
                    utils::get_method_args( cx, method_sig ).ok_or(())?;
            let ( ret_ty, _ ) =
                    utils::get_method_rvalues( cx, &method_sig ).ok_or(())?;

            // Create the vtable field and add it to the vector of fields.
            let vtable_method_decl = quote_tokens!(
                cx,
                #[allow(dead_code)]
                $method_ident :
                    unsafe extern "stdcall" fn( $args ) -> $ret_ty,
            );
            fields.push( vtable_method_decl );

            Ok(())
        };
    }

    // Create the vtable. We've already gathered all the vtable method
    // pointer fields so defining the struct is simple enough.
    let vtable_ident = idents::vtable_struct( &itf_ident );
    let vtable = quote_item!(
        cx,
        #[allow(non_camel_case_types)]
        pub struct $vtable_ident { $fields }
    ).unwrap();
    push( Annotatable::Item( vtable ) );

    Ok(())
}

/// `com_interface` MultiDecorator handler.
///
/// Delegates to a Result<>ful try_... variant.
pub fn expand_com_interface(
    cx: &mut ExtCtxt,
    sp: Span,
    mi: &MetaItem,
    item: &Annotatable,
    push: &mut FnMut( Annotatable )
) {
    if let Err( err ) = try_expand_com_interface( cx, sp, mi, item, push ) {
        cx.span_err( mi.span, err );
    }
}


/// `com_impl` MultiDecorator handler.
///
/// Delegates to a Result<>ful try_... variant.
pub fn expand_com_impl(
    cx: &mut ExtCtxt,
    sp: Span,
    mi: &MetaItem,
    item: &Annotatable,
    push: &mut FnMut( Annotatable )
) {
    if let Err( err ) = try_expand_com_impl( cx, sp, mi, item, push ) {
        cx.span_err( mi.span, err );
    }
}


/// Registers the syntax extensions.
#[plugin_registrar]
pub fn registrar( reg: &mut Registry ) {
    reg.register_syntax_extension(
            Symbol::intern("com_library"),
            MultiDecorator( Box::new( expand_com_library ) ) );
    reg.register_syntax_extension(
            Symbol::intern("com_class"),
            MultiDecorator( Box::new( expand_com_class ) ) );
    reg.register_syntax_extension(
            Symbol::intern("com_interface"),
            MultiDecorator( Box::new( expand_com_interface ) ) );
    reg.register_syntax_extension(
            Symbol::intern("com_impl"),
            MultiDecorator( Box::new( expand_com_impl ) ) );
}

/// Prints an item as code.
/// 
/// Not the prettiest output, but should allow some kind of inspection.
#[allow(dead_code)]
fn print_item( i : &P<Item> ) {
    if let Some( ref tt ) = i.tokens {
        println!( "{}", tt );
    };
}
