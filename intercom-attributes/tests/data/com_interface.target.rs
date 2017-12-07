#![feature(prelude_import)]
#![no_std]
#![feature(proc_macro)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate intercom;
use intercom::*;

trait Foo {
    fn static_method(a: u16, b: i16);

    fn simple_method(&self);

    fn arg_method(&self, a: u16);

    fn simple_result_method(&self) -> u16;
    fn com_result_method(&self) -> ComResult<u16>;
    fn rust_result_method(&self) -> Result<u16, i32>;

    fn string_method(&self, msg: String) -> String;

    fn complete_method(&mut self, a: u16, b: i16) -> ComResult<bool>;
}

// Interface ID
//
#[allow(non_upper_case_globals)]
pub const IID_Foo: ::intercom::IID =
        ::intercom::GUID {
            data1: 0u32, data2: 0u16, data3: 0u16,
            data4: [ 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8 ]
        };

// Virtual table
//
// Contains the base interface and an entry for each method.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct __FooVtbl {

    // Base interface.
    pub __base: ::intercom::IUnknownVtbl,

    // Method entries.

    pub simple_method: unsafe extern "stdcall" fn(
        self_vtable: ::intercom::RawComPtr
    ) -> (),

    pub arg_method: unsafe extern "stdcall" fn(
        self_vtable: ::intercom::RawComPtr,
        a: u16
    ) -> (),

    pub simple_result_method: unsafe extern "stdcall" fn(
        self_vtable: ::intercom::RawComPtr
    ) -> u16,

    pub com_result_method: unsafe extern "stdcall" fn(
        self_vtable: ::intercom::RawComPtr,
        __out: *mut u16
    ) -> ::intercom::HRESULT,

    pub rust_result_method: unsafe extern "stdcall" fn(
        self_vtable: ::intercom::RawComPtr,
        __out: *mut u16
    ) -> ::intercom::HRESULT,

    pub string_method: unsafe extern "stdcall" fn(
        self_vtable: ::intercom::RawComPtr,
        msg: ::intercom::BStr
    ) -> ::intercom::BStr,

    pub complete_method: unsafe extern "stdcall" fn(
        self_vtable: ::intercom::RawComPtr,
        a: u16,
        b: i16,
        __out: *mut bool
    ) -> ::intercom::HRESULT,
}

// Reverse delegates.
//
// Implement the trait for the ComItf. This allows calling the COM interface
// using the Rust trait methods.
impl Foo for ::intercom::ComItf<Foo>
{
    fn simple_method(&self) -> ()
    {
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;
        unsafe {
            let __result = ((**vtbl).simple_method)(comptr);
        }
    }

    fn arg_method(&self, a: u16) -> ()
    {
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;

        unsafe {
            let __result = ((**vtbl).arg_method)(comptr, a.into());
        }
    }

    fn simple_result_method(&self) -> u16
    {
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;

        unsafe {
            let __result = ((**vtbl).simple_result_method)(comptr);

            // Return the result as is.
            __result.into()
        }
    }

    fn com_result_method(&self) -> ComResult<u16>
    {
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;

        unsafe {
            let mut __out: u16 = Default::default();
            let __result = ((**vtbl).com_result_method)(comptr, &mut __out);

            // ComResults convert the __result HRESULT into Err, S_OK gets
            // converted into Ok.
            //
            // TODO: We'll need to support other SUCCESS values in the future
            //       for Ok(..) as well.
            if __result == ::intercom::S_OK {
                Ok(__out.into())
            } else {
                Err(__result)
            }
        }
    }

    fn rust_result_method(&self) -> Result<u16, i32>
    {
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;

        unsafe {
            let mut __out: u16 = Default::default();
            let __result = ((**vtbl).rust_result_method)(comptr, &mut __out);

            // Normal Result, not a ComResult. The Ok-case goes as before.
            //
            // The Err-case will use IErrorInfo to construct the original
            // error type.
            if __result == ::intercom::S_OK {
                Ok(__out.into())
            } else {
                Err(::intercom::get_last_error())
            }
        }
    }

    fn string_method(&self, msg: String) -> String
    {
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;

        unsafe {
            let __result = ((**vtbl).string_method)( comptr, msg.into() );
            __result.into()
        }
    }

    fn complete_method(
        &mut self,
        a: u16,
        b: i16
    ) -> ComResult<bool>
    {
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;
        unsafe {

            let mut __out: bool = Default::default();
            let __result = ((**vtbl).complete_method)(
                    comptr, a.into(), b.into(), &mut __out);

            if __result == ::intercom::S_OK {
                Ok(__out.into())
            } else {
                Err(__result)
            }
        }
    }
}
