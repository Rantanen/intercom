#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
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

    // Should be VARIANT_BOOL in Automation interface.
    fn bool_method(&self, input: bool)
    -> ComResult<bool>;

    fn variant_method(&self, input: Variant)
    -> ComResult<Variant>;
}
#[doc = "`Foo` interface ID."]
#[allow(non_upper_case_globals)]
pub const IID_Foo_Automation: ::intercom::IID =
    ::intercom::GUID{data1: 0u32,
                     data2: 0u16,
                     data3: 0u16,
                     data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],};
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
                                          -> ::intercom::raw::HRESULT,
    pub rust_result_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                      ::intercom::RawComPtr,
                                                                  __out:
                                                                      *mut u16)
                                           -> ::intercom::raw::HRESULT,
    pub complete_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                   ::intercom::RawComPtr,
                                                               a: u16, b: i16,
                                                               __out:
                                                                   *mut ::intercom::raw::VariantBool)
                                        -> ::intercom::raw::HRESULT,
    pub string_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                 ::intercom::RawComPtr,
                                                             msg:
                                                                 ::intercom::raw::InBSTR)
                                      -> ::intercom::raw::OutBSTR,
    pub comitf_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                 ::intercom::RawComPtr,
                                                             itf:
                                                                 ::intercom::raw::InterfacePtr<Foo>,
                                                             __out:
                                                                 *mut ::intercom::raw::InterfacePtr<IUnknown>)
                                      -> ::intercom::raw::HRESULT,
    pub bool_method_Automation: unsafe extern "C" fn(self_vtable:
                                                               ::intercom::RawComPtr,
                                                           input:
                                                               ::intercom::raw::VariantBool,
                                                           __out:
                                                               *mut ::intercom::raw::VariantBool)
                                    -> ::intercom::raw::HRESULT,
    pub variant_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                  ::intercom::RawComPtr,
                                                              input:
                                                                  ::intercom::raw::Variant,
                                                              __out:
                                                                  *mut ::intercom::raw::Variant)
                                       -> ::intercom::raw::HRESULT,
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
                                   -> ::intercom::raw::HRESULT,
    pub rust_result_method_Raw: unsafe extern "C" fn(self_vtable:
                                                               ::intercom::RawComPtr,
                                                           __out: *mut u16)
                                    -> ::intercom::raw::HRESULT,
    pub complete_method_Raw: unsafe extern "C" fn(self_vtable:
                                                            ::intercom::RawComPtr,
                                                        a: u16, b: i16,
                                                        __out: *mut bool)
                                 -> ::intercom::raw::HRESULT,
    pub string_method_Raw: unsafe extern "C" fn(self_vtable:
                                                          ::intercom::RawComPtr,
                                                      msg:
                                                          ::intercom::raw::InCStr)
                               -> ::intercom::raw::OutCStr,
    pub comitf_method_Raw: unsafe extern "C" fn(self_vtable:
                                                          ::intercom::RawComPtr,
                                                      itf:
                                                          ::intercom::raw::InterfacePtr<Foo>,
                                                      __out:
                                                          *mut ::intercom::raw::InterfacePtr<IUnknown>)
                               -> ::intercom::raw::HRESULT,
    pub bool_method_Raw: unsafe extern "C" fn(self_vtable:
                                                        ::intercom::RawComPtr,
                                                    input: bool,
                                                    __out: *mut bool)
                             -> ::intercom::raw::HRESULT,
    pub variant_method_Raw: unsafe extern "C" fn(self_vtable:
                                                           ::intercom::RawComPtr,
                                                       input:
                                                           ::intercom::raw::Variant,
                                                       __out:
                                                           *mut ::intercom::raw::Variant)
                                -> ::intercom::raw::HRESULT,
}
impl Foo for ::intercom::ComItf<Foo> {
    fn arg_method(&self, a: u16) -> () {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Automation) {
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<(), ::intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).arg_method_Automation)(comptr.ptr,
                                                              a.into());
                         Ok({ })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <() as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Raw) {
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<(), ::intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).arg_method_Raw)(comptr.ptr, a.into());
                         Ok({ })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <() as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        <() as
            ErrorValue>::from_error(::intercom::store_error(::intercom::ComError::E_POINTER))
    }
    fn bool_method(&self, input: bool) -> ComResult<bool> {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Automation) {
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<bool>, ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out: ::intercom::raw::VariantBool =
                             false.into();
                         let __result =
                             ((**vtbl).bool_method_Automation)(comptr.ptr,
                                                               input.into(),
                                                               &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(__out.into())
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<bool> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Raw) {
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<bool>, ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out: bool = false;
                         let __result =
                             ((**vtbl).bool_method_Raw)(comptr.ptr,
                                                        input.into(),
                                                        &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(__out.into())
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<bool> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        <ComResult<bool> as
            ErrorValue>::from_error(::intercom::store_error(::intercom::ComError::E_POINTER))
    }
    fn com_result_method(&self) -> ComResult<u16> {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Automation) {
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<u16>, ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out: u16 = Default::default();
                         let __result =
                             ((**vtbl).com_result_method_Automation)(comptr.ptr,
                                                                     &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(__out.into())
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<u16> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Raw) {
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<u16>, ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out: u16 = Default::default();
                         let __result =
                             ((**vtbl).com_result_method_Raw)(comptr.ptr,
                                                              &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(__out.into())
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<u16> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        <ComResult<u16> as
            ErrorValue>::from_error(::intercom::store_error(::intercom::ComError::E_POINTER))
    }
    fn comitf_method(&self, itf: ComItf<Foo>) -> ComResult<ComItf<IUnknown>> {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Automation) {
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result:
                    Result<ComResult<ComItf<IUnknown>>,
                           ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 ::intercom::raw::InterfacePtr<IUnknown> =
                             ::intercom::raw::InterfacePtr::new(::std::ptr::null_mut());
                         let __result =
                             ((**vtbl).comitf_method_Automation)(comptr.ptr,
                                                                 ::intercom::ComItf::ptr(&itf.into(),
                                                                                         ::intercom::TypeSystem::Automation),
                                                                 &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(::intercom::ComItf::wrap(__out,
                                                                ::intercom::TypeSystem::Automation))
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<ComItf<IUnknown>> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Raw) {
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result:
                    Result<ComResult<ComItf<IUnknown>>,
                           ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out:
                                 ::intercom::raw::InterfacePtr<IUnknown> =
                             ::intercom::raw::InterfacePtr::new(::std::ptr::null_mut());
                         let __result =
                             ((**vtbl).comitf_method_Raw)(comptr.ptr,
                                                          ::intercom::ComItf::ptr(&itf.into(),
                                                                                  ::intercom::TypeSystem::Raw),
                                                          &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(::intercom::ComItf::wrap(__out,
                                                                ::intercom::TypeSystem::Raw))
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<ComItf<IUnknown>> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        <ComResult<ComItf<IUnknown>> as
            ErrorValue>::from_error(::intercom::store_error(::intercom::ComError::E_POINTER))
    }
    fn complete_method(&mut self, a: u16, b: i16) -> ComResult<bool> {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Automation) {
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<bool>, ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out: ::intercom::raw::VariantBool =
                             false.into();
                         let __result =
                             ((**vtbl).complete_method_Automation)(comptr.ptr,
                                                                   a.into(),
                                                                   b.into(),
                                                                   &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(__out.into())
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<bool> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Raw) {
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<bool>, ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out: bool = false;
                         let __result =
                             ((**vtbl).complete_method_Raw)(comptr.ptr,
                                                            a.into(),
                                                            b.into(),
                                                            &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(__out.into())
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<bool> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        <ComResult<bool> as
            ErrorValue>::from_error(::intercom::store_error(::intercom::ComError::E_POINTER))
    }
    fn rust_result_method(&self) -> Result<u16, i32> {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Automation) {
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<Result<u16, i32>, ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out: u16 = Default::default();
                         let __result =
                             ((**vtbl).rust_result_method_Automation)(comptr.ptr,
                                                                      &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(__out.into())
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <Result<u16, i32> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Raw) {
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<Result<u16, i32>, ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out: u16 = Default::default();
                         let __result =
                             ((**vtbl).rust_result_method_Raw)(comptr.ptr,
                                                               &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(__out.into())
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <Result<u16, i32> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        <Result<u16, i32> as
            ErrorValue>::from_error(::intercom::store_error(::intercom::ComError::E_POINTER))
    }
    fn simple_method(&self) -> () {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Automation) {
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<(), ::intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).simple_method_Automation)(comptr.ptr);
                         Ok({ })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <() as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Raw) {
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<(), ::intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).simple_method_Raw)(comptr.ptr);
                         Ok({ })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <() as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        <() as
            ErrorValue>::from_error(::intercom::store_error(::intercom::ComError::E_POINTER))
    }
    fn simple_result_method(&self) -> u16 {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Automation) {
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<u16, ::intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).simple_result_method_Automation)(comptr.ptr);
                         Ok({ __result.into() })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <u16 as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Raw) {
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<u16, ::intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).simple_result_method_Raw)(comptr.ptr);
                         Ok({ __result.into() })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <u16 as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        <u16 as
            ErrorValue>::from_error(::intercom::store_error(::intercom::ComError::E_POINTER))
    }
    fn string_method(&self, msg: String) -> String {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Automation) {
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            let mut __msg_temporary =
                <&::intercom::BStr as
                    ::intercom::FromWithTemporary<String>>::to_temporary(msg)?;
            #[allow(unused_unsafe)]
            let result: Result<String, ::intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).string_method_Automation)(comptr.ptr,
                                                                 <&::intercom::BStr
                                                                     as
                                                                     ::intercom::FromWithTemporary<String>>::from_temporary(&mut __msg_temporary)?.as_ptr());
                         Ok({
                                ::intercom::BString::from_ptr(__result).com_into()?
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <String as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Raw) {
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            let mut __msg_temporary =
                <&::intercom::CStr as
                    ::intercom::FromWithTemporary<String>>::to_temporary(msg)?;
            #[allow(unused_unsafe)]
            let result: Result<String, ::intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).string_method_Raw)(comptr.ptr,
                                                          <&::intercom::CStr
                                                              as
                                                              ::intercom::FromWithTemporary<String>>::from_temporary(&mut __msg_temporary)?.as_ptr());
                         Ok({
                                ::intercom::CString::from_raw(__result).com_into()?
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <String as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        <String as
            ErrorValue>::from_error(::intercom::store_error(::intercom::ComError::E_POINTER))
    }
    fn variant_method(&self, input: Variant) -> ComResult<Variant> {
        #[allow(unused_imports)]
        use ::intercom::ComInto;
        #[allow(unused_imports)]
        use ::intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Automation) {
            let vtbl = comptr.ptr as *const *const __Foo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<Variant>, ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out: ::intercom::raw::Variant =
                             Default::default();
                         let __result =
                             ((**vtbl).variant_method_Automation)(comptr.ptr,
                                                                  input.com_into()?,
                                                                  &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(__out.into())
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<Variant> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr(self, ::intercom::TypeSystem::Raw) {
            let vtbl = comptr.ptr as *const *const __Foo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<ComResult<Variant>, ::intercom::ComError> =
                (||
                     unsafe {
                         let mut __out: ::intercom::raw::Variant =
                             Default::default();
                         let __result =
                             ((**vtbl).variant_method_Raw)(comptr.ptr,
                                                           input.com_into()?,
                                                           &mut __out);
                         Ok({
                                if __result == ::intercom::raw::S_OK {
                                    Ok(__out.into())
                                } else {
                                    Err(::intercom::get_last_error(__result))
                                }
                            })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <ComResult<Variant> as
                           ErrorValue>::from_error(::intercom::store_error(err)),
                   };
        }
        <ComResult<Variant> as
            ErrorValue>::from_error(::intercom::store_error(::intercom::ComError::E_POINTER))
    }
}
impl ::intercom::ComInterface for Foo {
    #[doc = "Returns the IID of the requested interface."]
    fn iid(ts: ::intercom::TypeSystem) -> Option<&'static ::intercom::IID> {
        match ts {
            ::intercom::TypeSystem::Automation => Some(&IID_Foo_Automation),
            ::intercom::TypeSystem::Raw => Some(&IID_Foo_Raw),
        }
    }
    fn deref(com_itf: &::intercom::ComItf<Foo>) -> &(Foo + 'static) {
        com_itf
    }
}
