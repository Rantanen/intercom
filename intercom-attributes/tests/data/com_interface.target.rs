#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;

extern crate intercom;
use intercom::*;

pub trait Foo {
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

    fn string_method(&self, msg: String)
    -> String;
    fn comitf_method(&self, itf: ComItf<Foo>)
    -> ComResult<ComItf<IUnknown>>;
}
#[doc = "`Foo` interface ID."]
#[allow(non_upper_case_globals)]
pub const IID_Foo_Automation: ::intercom::IID =
    ::intercom::GUID{data1: 0u32,
                     data2: 0u16,
                     data3: 0u16,
                     data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],};
impl ::intercom::IidOf for Foo {
    #[doc = "Returns `IID_Foo_Automation`."]
    fn iid() -> &'static ::intercom::IID { &IID_Foo_Automation }
}
#[allow(non_camel_case_types)]
#[repr(C)]
#[doc(hidden)]
pub struct __Foo_AutomationVtbl {
    pub __base: ::intercom::IUnknownVtbl,
    pub simple_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                 ::intercom::RawComPtr)
                                      -> (),
    pub arg_method_Automation: unsafe extern "C" fn(self_vtable:
                                                              ::intercom::RawComPtr,
                                                          a: u16) -> (),
    pub simple_result_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                        ::intercom::RawComPtr)
                                             -> u16,
    pub com_result_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                     ::intercom::RawComPtr,
                                                                 __out:
                                                                     *mut u16)
                                          -> ::intercom::HRESULT,
    pub rust_result_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                      ::intercom::RawComPtr,
                                                                  __out:
                                                                      *mut u16)
                                           -> ::intercom::HRESULT,
    pub complete_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                   ::intercom::RawComPtr,
                                                               a: u16, b: i16,
                                                               __out:
                                                                   *mut bool)
                                        -> ::intercom::HRESULT,
    pub string_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                 ::intercom::RawComPtr,
                                                             msg:
                                                                 ::intercom::raw::InBSTR)
                                      -> ::intercom::raw::OutBSTR,
    pub comitf_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                 ::intercom::RawComPtr,
                                                             itf: ComItf<Foo>,
                                                             __out:
                                                                 *mut ComItf<IUnknown>)
                                      -> ::intercom::HRESULT,
}
#[doc = "`Foo` interface ID."]
#[allow(non_upper_case_globals)]
pub const IID_Foo_Raw: ::intercom::IID =
    ::intercom::GUID{data1: 0u32,
                     data2: 0u16,
                     data3: 0u16,
                     data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8],};
#[allow(non_camel_case_types)]
#[repr(C)]
#[doc(hidden)]
pub struct __Foo_RawVtbl {
    pub __base: ::intercom::IUnknownVtbl,
    pub simple_method_Raw: unsafe extern "C" fn(self_vtable:
                                                          ::intercom::RawComPtr)
                               -> (),
    pub arg_method_Raw: unsafe extern "C" fn(self_vtable:
                                                       ::intercom::RawComPtr,
                                                   a: u16) -> (),
    pub simple_result_method_Raw: unsafe extern "C" fn(self_vtable:
                                                                 ::intercom::RawComPtr)
                                      -> u16,
    pub com_result_method_Raw: unsafe extern "C" fn(self_vtable:
                                                              ::intercom::RawComPtr,
                                                          __out: *mut u16)
                                   -> ::intercom::HRESULT,
    pub rust_result_method_Raw: unsafe extern "C" fn(self_vtable:
                                                               ::intercom::RawComPtr,
                                                           __out: *mut u16)
                                    -> ::intercom::HRESULT,
    pub complete_method_Raw: unsafe extern "C" fn(self_vtable:
                                                            ::intercom::RawComPtr,
                                                        a: u16, b: i16,
                                                        __out: *mut bool)
                                 -> ::intercom::HRESULT,
    pub string_method_Raw: unsafe extern "C" fn(self_vtable:
                                                          ::intercom::RawComPtr,
                                                      msg:
                                                          ::intercom::raw::InBSTR)
                               -> ::intercom::raw::OutBSTR,
    pub comitf_method_Raw: unsafe extern "C" fn(self_vtable:
                                                          ::intercom::RawComPtr,
                                                      itf: ComItf<Foo>,
                                                      __out:
                                                          *mut ComItf<IUnknown>)
                               -> ::intercom::HRESULT,
}
impl Foo for ::intercom::ComItf<Foo> {
    fn simple_method(&self) -> () {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __Foo_AutomationVtbl;
        #[allow(unused_unsafe)]
        let result: Result<(), ::intercom::ComError> =
            (||
                 unsafe {
                     let __result =
                         ((**vtbl).simple_method_Automation)(comptr);
                     Ok({ })
                 })();
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        match result {
            Ok(v) => v,
            Err(err) =>
            <() as ErrorValue>::from_error(::intercom::return_hresult(err)),
        }
    }
    fn arg_method(&self, a: u16) -> () {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __Foo_AutomationVtbl;
        #[allow(unused_unsafe)]
        let result: Result<(), ::intercom::ComError> =
            (||
                 unsafe {
                     let __result =
                         ((**vtbl).arg_method_Automation)(comptr, a.into());
                     Ok({ })
                 })();
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        match result {
            Ok(v) => v,
            Err(err) =>
            <() as ErrorValue>::from_error(::intercom::return_hresult(err)),
        }
    }
    fn simple_result_method(&self) -> u16 {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __Foo_AutomationVtbl;
        #[allow(unused_unsafe)]
        let result: Result<u16, ::intercom::ComError> =
            (||
                 unsafe {
                     let __result =
                         ((**vtbl).simple_result_method_Automation)(comptr);
                     Ok({ __result.into() })
                 })();
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        match result {
            Ok(v) => v,
            Err(err) =>
            <u16 as ErrorValue>::from_error(::intercom::return_hresult(err)),
        }
    }
    fn com_result_method(&self) -> ComResult<u16> {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __Foo_AutomationVtbl;
        #[allow(unused_unsafe)]
        let result: Result<ComResult<u16>, ::intercom::ComError> =
            (||
                 unsafe {
                     let mut __out: u16 = Default::default();
                     let __result =
                         ((**vtbl).com_result_method_Automation)(comptr,
                                                                 &mut __out);
                     Ok({
                            if __result == ::intercom::S_OK {
                                Ok(__out.into())
                            } else {
                                Err(::intercom::get_last_error(__result))
                            }
                        })
                 })();
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        match result {
            Ok(v) => v,
            Err(err) =>
            <ComResult<u16> as
                ErrorValue>::from_error(::intercom::return_hresult(err)),
        }
    }
    fn rust_result_method(&self) -> Result<u16, i32> {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __Foo_AutomationVtbl;
        #[allow(unused_unsafe)]
        let result: Result<Result<u16, i32>, ::intercom::ComError> =
            (||
                 unsafe {
                     let mut __out: u16 = Default::default();
                     let __result =
                         ((**vtbl).rust_result_method_Automation)(comptr,
                                                                  &mut __out);
                     Ok({
                            if __result == ::intercom::S_OK {
                                Ok(__out.into())
                            } else {
                                Err(::intercom::get_last_error(__result))
                            }
                        })
                 })();
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        match result {
            Ok(v) => v,
            Err(err) =>
            <Result<u16, i32> as
                ErrorValue>::from_error(::intercom::return_hresult(err)),
        }
    }
    fn complete_method(&mut self, a: u16, b: i16) -> ComResult<bool> {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __Foo_AutomationVtbl;
        #[allow(unused_unsafe)]
        let result: Result<ComResult<bool>, ::intercom::ComError> =
            (||
                 unsafe {
                     let mut __out: bool = Default::default();
                     let __result =
                         ((**vtbl).complete_method_Automation)(comptr,
                                                               a.into(),
                                                               b.into(),
                                                               &mut __out);
                     Ok({
                            if __result == ::intercom::S_OK {
                                Ok(__out.into())
                            } else {
                                Err(::intercom::get_last_error(__result))
                            }
                        })
                 })();
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        match result {
            Ok(v) => v,
            Err(err) =>
            <ComResult<bool> as
                ErrorValue>::from_error(::intercom::return_hresult(err)),
        }
    }
    fn string_method(&self, msg: String) -> String {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __Foo_AutomationVtbl;
        let mut __msg_temporary =
            <&::intercom::BStr as
                ::intercom::FromWithTemporary<String>>::to_temporary(msg)?;
        #[allow(unused_unsafe)]
        let result: Result<String, ::intercom::ComError> =
            (||
                 unsafe {
                     let __result =
                         ((**vtbl).string_method_Automation)(comptr,
                                                             <&::intercom::BStr
                                                                 as
                                                                 ::intercom::FromWithTemporary<String>>::from_temporary(&mut __msg_temporary)?.as_ptr());
                     Ok({
                            ::intercom::BString::from_ptr(__result).com_into()?
                        })
                 })();
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        match result {
            Ok(v) => v,
            Err(err) =>
            <String as
                ErrorValue>::from_error(::intercom::return_hresult(err)),
        }
    }
    fn comitf_method(&self, itf: ComItf<Foo>) -> ComResult<ComItf<IUnknown>> {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __Foo_AutomationVtbl;
        #[allow(unused_unsafe)]
        let result:
                Result<ComResult<ComItf<IUnknown>>, ::intercom::ComError> =
            (||
                 unsafe {
                     let mut __out: ComItf<IUnknown> = ComItf::null_itf();
                     let __result =
                         ((**vtbl).comitf_method_Automation)(comptr,
                                                             itf.into(),
                                                             &mut __out);
                     Ok({
                            if __result == ::intercom::S_OK {
                                Ok(__out.into())
                            } else {
                                Err(::intercom::get_last_error(__result))
                            }
                        })
                 })();
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        match result {
            Ok(v) => v,
            Err(err) =>
            <ComResult<ComItf<IUnknown>> as
                ErrorValue>::from_error(::intercom::return_hresult(err)),
        }
    }
}
impl ::std::ops::Deref for ::intercom::ComItf<Foo> {
    type
    Target
    =
    Foo;
    fn deref(&self) -> &Self::Target { self }
}
