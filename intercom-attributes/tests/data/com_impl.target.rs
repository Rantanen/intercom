#![feature(prelude_import)]
#![no_std]
#![feature(proc_macro)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate intercom;
use intercom::*;

struct __FooVtbl;
const IID_Foo: intercom::IID =
    intercom::GUID{data1: 0,
                   data2: 0,
                   data3: 0,
                   data4: [0, 0, 0, 0, 0, 0, 0, 0],};

struct Foo;
#[inline(always)]
#[allow(non_snake_case)]
fn __Foo_FooVtbl_offset() -> usize {
    unsafe {
        &intercom::ComBox::<Foo>::null_vtable().Foo as *const _ as usize
    }
}
#[allow(non_upper_case_globals)]
const __Foo_ISupportErrorInfoVtbl_INSTANCE: intercom::ISupportErrorInfoVtbl =
    intercom::ISupportErrorInfoVtbl{__base:
                                        intercom::IUnknownVtbl{query_interface:
                                                                   intercom::ComBox::<Foo>::query_interface_ptr,
                                                               add_ref:
                                                                   intercom::ComBox::<Foo>::add_ref_ptr,
                                                               release:
                                                                   intercom::ComBox::<Foo>::release_ptr,},
                                    interface_supports_error_info:
                                        intercom::ComBox::<Foo>::interface_supports_error_info_ptr,};
#[allow(non_snake_case)]
pub struct __FooVtblList {
    _ISupportErrorInfo: &'static intercom::ISupportErrorInfoVtbl,
    Foo: &'static __FooVtbl,
}
impl intercom::CoClass for Foo {
    type
    VTableList
    =
    __FooVtblList;
    fn create_vtable_list() -> Self::VTableList {
        __FooVtblList{_ISupportErrorInfo:
                          &__Foo_ISupportErrorInfoVtbl_INSTANCE,
                      Foo: &__Foo_FooVtbl_INSTANCE,}
    }
    fn query_interface(vtables: &Self::VTableList, riid: intercom::REFIID)
     -> intercom::ComResult<intercom::RawComPtr> {
        if riid.is_null() { return Err(intercom::E_NOINTERFACE) }
        Ok(match *unsafe { &*riid } {
               intercom::IID_IUnknown =>
               (&vtables._ISupportErrorInfo) as
                   *const &intercom::ISupportErrorInfoVtbl as
                   *mut &intercom::ISupportErrorInfoVtbl as
                   intercom::RawComPtr,
               intercom::IID_ISupportErrorInfo =>
               (&vtables._ISupportErrorInfo) as
                   *const &intercom::ISupportErrorInfoVtbl as
                   *mut &intercom::ISupportErrorInfoVtbl as
                   intercom::RawComPtr,
               self::IID_Foo =>
               &vtables.Foo as *const &__FooVtbl as *mut &__FooVtbl as
                   intercom::RawComPtr,
               _ => return Err(intercom::E_NOINTERFACE),
           })
    }
    fn interface_supports_error_info(riid: REFIID) -> bool {
        match *unsafe { &*riid } { self::IID_Foo => true, _ => false, }
    }
}
#[allow(non_upper_case_globals)]
const CLSID_Foo: intercom::CLSID =
    intercom::GUID{data1: 0u32,
                   data2: 0u16,
                   data3: 0u16,
                   data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],};
impl Foo {
    fn static_method(a: u16, b: i16) { }
    fn simple_method(&self) { }
    fn arg_method(&self, a: u16) { }
    fn simple_result_method(&self) -> u16 { 0 }
    fn com_result_method(&self) -> ComResult<u16> { Ok(0) }
    fn rust_result_method(&self) -> Result<u16, i32> { Ok(0) }
    fn complete_method(&mut self, a: u16, b: i16) -> ComResult<bool> {
        Ok(true)
    }
}
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn __Foo_Foo_query_interface(self_vtable:
                                                             intercom::RawComPtr,
                                                         riid:
                                                             intercom::REFIID,
                                                         out:
                                                             *mut intercom::RawComPtr)
 -> intercom::HRESULT {
    intercom::ComBox::<Foo>::query_interface(&mut *((self_vtable as usize -
                                                         __Foo_FooVtbl_offset())
                                                        as *mut _), riid, out)
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_add_ref(self_vtable:
                                                     intercom::RawComPtr)
 -> u32 {
    intercom::ComBox::<Foo>::add_ref(&mut *((self_vtable as usize -
                                                 __Foo_FooVtbl_offset()) as
                                                *mut _))
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_release(self_vtable:
                                                     intercom::RawComPtr)
 -> u32 {
    intercom::ComBox::<Foo>::release_ptr((self_vtable as usize -
                                              __Foo_FooVtbl_offset()) as
                                             *mut _)
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_simple_method(self_vtable:
                                                           intercom::RawComPtr)
 -> () {
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let __result = (*self_combox).simple_method();
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_arg_method(self_vtable:
                                                        intercom::RawComPtr,
                                                    a: u16) -> () {
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let __result = (*self_combox).arg_method(a.into());
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_simple_result_method(self_vtable:
                                                                  intercom::RawComPtr)
 -> u16 {
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let __result = (*self_combox).simple_result_method();
    __result
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_com_result_method(self_vtable:
                                                               intercom::RawComPtr,
                                                           __out: *mut u16)
 -> intercom::HRESULT {
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let __result = (*self_combox).com_result_method();
    match __result {
        Ok(v) => { *__out = v.into(); intercom::S_OK }
        Err(e) => { *__out = Default::default(); e }
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_rust_result_method(self_vtable:
                                                                intercom::RawComPtr,
                                                            __out: *mut u16)
 -> intercom::HRESULT {
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let __result = (*self_combox).rust_result_method();
    match __result {
        Ok(v) => { *__out = v.into(); intercom::S_OK }
        Err(e) => { *__out = Default::default(); intercom::return_hresult(e) }
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_complete_method(self_vtable:
                                                             intercom::RawComPtr,
                                                         a: u16, b: i16,
                                                         __out: *mut bool)
 -> intercom::HRESULT {
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let __result = (*self_combox).complete_method(a.into(), b.into());
    match __result {
        Ok(v) => { *__out = v.into(); intercom::S_OK }
        Err(e) => { *__out = Default::default(); e }
    }
}
#[allow(non_upper_case_globals)]
const __Foo_FooVtbl_INSTANCE: __FooVtbl =
    __FooVtbl{__base:
                  intercom::IUnknownVtbl{query_interface:
                                             __Foo_Foo_query_interface,
                                         add_ref: __Foo_Foo_add_ref,
                                         release: __Foo_Foo_release,},
              simple_method: __Foo_Foo_simple_method,
              arg_method: __Foo_Foo_arg_method,
              simple_result_method: __Foo_Foo_simple_result_method,
              com_result_method: __Foo_Foo_com_result_method,
              rust_result_method: __Foo_Foo_rust_result_method,
              complete_method: __Foo_Foo_complete_method,};
