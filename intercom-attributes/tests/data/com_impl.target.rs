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
const __Foo_IUnknownVtbl_INSTANCE: intercom::IUnknownVtbl =
    intercom::IUnknownVtbl{query_interface:
                               intercom::ComBox::<Foo>::query_interface_ptr,
                           add_ref: intercom::ComBox::<Foo>::add_ref_ptr,
                           release: intercom::ComBox::<Foo>::release_ptr,};
#[allow(non_snake_case)]
pub struct __FooVtblList {
    _IUnknown: &'static intercom::IUnknownVtbl,
    Foo: &'static __FooVtbl,
}
#[allow(non_snake_case)]
impl AsRef<intercom::IUnknownVtbl> for __FooVtblList {
    fn as_ref(&self) -> &intercom::IUnknownVtbl { &self._IUnknown }
}
impl intercom::CoClass for Foo {
    type
    VTableList
    =
    __FooVtblList;
    fn create_vtable_list() -> Self::VTableList {
        __FooVtblList{_IUnknown: &__Foo_IUnknownVtbl_INSTANCE,
                      Foo: &__Foo_FooVtbl_INSTANCE,}
    }
    fn query_interface(vtables: &Self::VTableList, riid: intercom::REFIID)
     -> intercom::ComResult<intercom::RawComPtr> {
        if riid.is_null() { return Err(intercom::E_NOINTERFACE) }
        Ok(match *unsafe { &*riid } {
               intercom::IID_IUnknown =>
               (&vtables._IUnknown) as *const &intercom::IUnknownVtbl as
                   *mut &intercom::IUnknownVtbl as intercom::RawComPtr,
               self::IID_Foo =>
               &vtables.Foo as *const &__FooVtbl as *mut &__FooVtbl as
                   intercom::RawComPtr,
               _ => return Err(intercom::E_NOINTERFACE),
           })
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
    let self_comptr =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let result = (*self_comptr).simple_method();
    return result
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_arg_method(self_vtable:
                                                        intercom::RawComPtr,
                                                    a: u16) -> () {
    let self_comptr =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let result = (*self_comptr).arg_method(a.into());
    return result
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_simple_result_method(self_vtable:
                                                                  intercom::RawComPtr)
 -> u16 {
    let self_comptr =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let result = (*self_comptr).simple_result_method();
    return result
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_com_result_method(self_vtable:
                                                               intercom::RawComPtr,
                                                           __out: *mut u16)
 -> ::intercom::HRESULT {
    let self_comptr =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let result = (*self_comptr).com_result_method();
    match result {
        Ok(r) => { *__out = r.into(); intercom::S_OK }
        Err(e) => { *__out = Default::default(); e }
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_rust_result_method(self_vtable:
                                                                intercom::RawComPtr,
                                                            __out: *mut u16)
 -> ::intercom::HRESULT {
    let self_comptr =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let result = (*self_comptr).rust_result_method();
    match result {
        Ok(r) => { *__out = r.into(); intercom::S_OK }
        Err(e) => { *__out = Default::default(); e }
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub unsafe extern "stdcall" fn __Foo_Foo_complete_method(self_vtable:
                                                             intercom::RawComPtr,
                                                         a: u16, b: i16,
                                                         __out: *mut bool)
 -> ::intercom::HRESULT {
    let self_comptr =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut intercom::ComBox<Foo>;
    let result = (*self_comptr).complete_method(a.into(), b.into());
    match result {
        Ok(r) => { *__out = r.into(); intercom::S_OK }
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
