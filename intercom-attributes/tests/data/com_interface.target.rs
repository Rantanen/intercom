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
    fn simple_result_method(&self)
    -> u16;
    fn com_result_method(&self)
    -> ComResult<u16>;
    fn rust_result_method(&self)
    -> Result<u16, i32>;
    fn complete_method(&mut self, a: u16, b: i16)
    -> ComResult<bool>;
}
#[allow(non_upper_case_globals)]
const IID_Foo: intercom::IID =
    intercom::GUID{data1: 0u32,
                   data2: 0u16,
                   data3: 0u16,
                   data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],};
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct __FooVtbl {
    __base: intercom::IUnknownVtbl,
    simple_method: unsafe extern "stdcall" fn(self_vtable:
                                                  intercom::RawComPtr) -> (),
    arg_method: unsafe extern "stdcall" fn(self_vtable: intercom::RawComPtr,
                                           a: u16) -> (),
    simple_result_method: unsafe extern "stdcall" fn(self_vtable:
                                                         intercom::RawComPtr)
                              -> u16,
    com_result_method: unsafe extern "stdcall" fn(self_vtable:
                                                      intercom::RawComPtr,
                                                  __out: *mut u16)
                           -> ::intercom::HRESULT,
    rust_result_method: unsafe extern "stdcall" fn(self_vtable:
                                                       intercom::RawComPtr,
                                                   __out: *mut u16)
                            -> ::intercom::HRESULT,
    complete_method: unsafe extern "stdcall" fn(self_vtable:
                                                    intercom::RawComPtr,
                                                a: u16, b: i16,
                                                __out: *mut bool)
                         -> ::intercom::HRESULT,
}
impl Foo for intercom::ComItf<Foo> {
    fn simple_method(&self) -> () {
        let comptr = intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;
        unsafe { let __result = ((**vtbl).simple_method)(comptr); }
    }
    fn arg_method(&self, a: u16) -> () {
        let comptr = intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;
        unsafe { let __result = ((**vtbl).arg_method)(comptr, a.into()); }
    }
    fn simple_result_method(&self) -> u16 {
        let comptr = intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;
        unsafe {
            let __result = ((**vtbl).simple_result_method)(comptr);
            __result
        }
    }
    fn com_result_method(&self) -> ComResult<u16> {
        let comptr = intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;
        unsafe {
            let mut __out: u16 = Default::default();
            let __result = ((**vtbl).com_result_method)(comptr, &mut __out);
            if __result == intercom::S_OK {
                Ok(__out.into())
            } else { Err(__result) }
        }
    }
    fn rust_result_method(&self) -> Result<u16, i32> {
        let comptr = intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;
        unsafe {
            let mut __out: u16 = Default::default();
            let __result = ((**vtbl).rust_result_method)(comptr, &mut __out);
            if __result == intercom::S_OK {
                Ok(__out.into())
            } else { Err(intercom::get_last_error()) }
        }
    }
    fn complete_method(&mut self, a: u16, b: i16) -> ComResult<bool> {
        let comptr = intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __FooVtbl;
        unsafe {
            let mut __out: bool = Default::default();
            let __result =
                ((**vtbl).complete_method)(comptr, a.into(), b.into(),
                                           &mut __out);
            if __result == intercom::S_OK {
                Ok(__out.into())
            } else { Err(__result) }
        }
    }
}
