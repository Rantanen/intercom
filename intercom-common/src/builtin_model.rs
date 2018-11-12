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
        BuiltinTypeInfo {
            interface: errorstore_interface( lib_name ),
            class: errorstore_class( lib_name ),
            implementation: errorstore_impl(),
            ctor: quote!( ::intercom::error::ErrorStore::default() ),
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
        unsafe fn alloc_bstr( &self, text : *const u16, len : u32 ) -> BString {}
        unsafe fn free_bstr( &self, bstr : &BStr ) { }
        unsafe fn alloc( &self, len : usize ) -> *mut raw::c_void { }
        unsafe fn free( &self, ptr : *mut raw::c_void ) { }
    }
    "#
}

fn errorstore_interface( lib_name: &str ) -> ComInterface {
    ComInterface::parse(
            lib_name,
            quote!( (
                com_iid = "d7f996c5-0b51-4053-82f8-19a7261793a9",
                raw_iid = "7586c49a-abbd-4a06-b588-e3d02b431f01",
            ) ),
            errorstore_impl_code() ).unwrap()
}

fn errorstore_impl() -> ComImpl {
    ComImpl::parse( errorstore_impl_code() ).unwrap()
}

fn errorstore_class( lib_name: &str ) -> ComStruct {
    ComStruct::parse(
            lib_name,
            quote!( ErrorStore ),
            "pub struct ErrorStore;" ).unwrap()
}

fn errorstore_impl_code() -> &'static str {
    r#"
    impl ErrorStore
    {
        fn get_error_info( &self ) -> ComResult<ComItf<IErrorInfo>> { }
        fn set_error_info( &self, info : ComItf<IErrorInfo> ) -> ComResult<()> { }
        fn set_error_message( &self, msg : &str ) -> ComResult<()> { }
    }
    "#
}

