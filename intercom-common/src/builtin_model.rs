//!
//! Defines the default Intercom type model.
//!

use prelude::*;
use model::{ ComInterface, ComStruct, ComImpl };

pub struct BuiltinTypeInfo {
    pub interface: ComInterface,
    pub class: ComStruct,
    pub implementation: ComImpl,
    pub ctor : TokenStream,
}

pub fn builtin_intercom_types( lib_name: &str ) -> Vec<BuiltinTypeInfo> {
    vec![
        BuiltinTypeInfo {
            interface: allocator_interface( lib_name ),
            class: allocator_class( lib_name ),
            implementation: allocator_impl(),
            ctor: quote!( ::intercom::alloc::Allocator::default() ),
        },
    ]
}

fn allocator_interface( lib_name: &str ) -> ComInterface {
    ComInterface::parse(
            lib_name,
            quote!( (
                com_iid = "18EE22B3-B0C6-44A5-A94A-7A417676FB66",
                raw_iid = "7A6F6564-04B5-4455-A223-EA0512B8CC63",
            ) ),
            allocator_impl_code() ).unwrap()
}

fn allocator_impl() -> ComImpl {
    ComImpl::parse( allocator_impl_code() ).unwrap()
}

fn allocator_class( lib_name: &str ) -> ComStruct {
    ComStruct::parse(
            lib_name,
            quote!( Allocator ),
            "pub struct Allocator;" ).unwrap()
}

fn allocator_impl_code() -> &'static str {
    r#"
    impl Allocator {
        unsafe fn alloc_bstr( &self, text : *const u16, len : u32 ) -> BString {
            os::alloc_bstr( text, len )
        }

        unsafe fn free_bstr( &self, bstr : &BStr ) {
            os::free_bstr( bstr )
        }

        unsafe fn alloc( &self, len : usize ) -> *mut raw::c_void {
            os::alloc( len )
        }

        unsafe fn free( &self, ptr : *mut raw::c_void ) {
            os::free( ptr )
        }
    }
    "#
}

