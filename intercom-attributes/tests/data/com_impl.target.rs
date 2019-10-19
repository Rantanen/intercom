#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
extern crate intercom;
use intercom::*;
use std::mem::MaybeUninit;

// We need the IID and Vtbl to ensure this compiles.
//
// Normally these are provided by the [com_interface].
struct __Foo_AutomationVtbl;
const IID_Foo_Automation: intercom::IID =
    intercom::GUID{data1: 0,
                   data2: 0,
                   data3: 0,
                   data4: [0, 0, 0, 0, 0, 0, 0, 0],};

struct __Foo_RawVtbl;
const IID_Foo_Raw: intercom::IID =
    intercom::GUID{data1: 0,
                   data2: 0,
                   data3: 0,
                   data4: [0, 0, 0, 0, 0, 0, 0, 0],};

fn get_com_interface_for_Foo_Automation()
 -> intercom::serialization::ComInterfaceVariant {
    unsafe { MaybeUninit::uninit().assume_init() }
}
fn get_com_interface_for_Foo_Raw()
 -> intercom::serialization::ComInterfaceVariant {
    unsafe { MaybeUninit::uninit().assume_init() }
}
fn get_intercom_interface_info_for_Foo() -> Vec<intercom::typelib::TypeInfo> {
    unsafe { MaybeUninit::uninit().assume_init() }
}

pub struct Foo;
#[inline(always)]
#[allow(non_snake_case)]
fn __Foo_Foo_AutomationVtbl_offset() -> usize {
    unsafe {
        &intercom::ComBox::<Foo>::null_vtable().Foo_Automation as *const _ as
            usize
    }
}
#[inline(always)]
#[allow(non_snake_case)]
fn __Foo_Foo_RawVtbl_offset() -> usize {
    unsafe {
        &intercom::ComBox::<Foo>::null_vtable().Foo_Raw as *const _ as usize
    }
}
#[allow(non_upper_case_globals)]
const __Foo_ISupportErrorInfoVtbl_INSTANCE: intercom::ISupportErrorInfoVtbl =
    intercom::ISupportErrorInfoVtbl{__base:
                                        intercom::IUnknownVtbl{query_interface_Automation:
                                                                   intercom::ComBox::<Foo>::query_interface_ptr,
                                                               add_ref_Automation:
                                                                   intercom::ComBox::<Foo>::add_ref_ptr,
                                                               release_Automation:
                                                                   intercom::ComBox::<Foo>::release_ptr,},
                                    interface_supports_error_info_Automation:
                                        intercom::ComBox::<Foo>::interface_supports_error_info_ptr,};
impl intercom::HasInterface<intercom::IUnknown> for Foo { }
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct __FooVtblList {
    _ISupportErrorInfo: &'static intercom::ISupportErrorInfoVtbl,
    Foo_Automation: &'static __Foo_AutomationVtbl,
    Foo_Raw: &'static __Foo_RawVtbl,
}
impl intercom::CoClass for Foo {
    type
    VTableList
    =
    __FooVtblList;
    fn create_vtable_list() -> Self::VTableList {
        __FooVtblList{_ISupportErrorInfo:
                          &__Foo_ISupportErrorInfoVtbl_INSTANCE,
                      Foo_Automation: &__Foo_Foo_AutomationVtbl_INSTANCE,
                      Foo_Raw: &__Foo_Foo_RawVtbl_INSTANCE,}
    }
    fn query_interface(vtables: &Self::VTableList, riid: intercom::REFIID)
     -> intercom::RawComResult<intercom::RawComPtr> {
        if riid.is_null() { return Err(intercom::raw::E_NOINTERFACE) }
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
               self::IID_Foo_Automation =>
               &vtables.Foo_Automation as *const &__Foo_AutomationVtbl as
                   *mut &__Foo_AutomationVtbl as intercom::RawComPtr,
               self::IID_Foo_Raw =>
               &vtables.Foo_Raw as *const &__Foo_RawVtbl as
                   *mut &__Foo_RawVtbl as intercom::RawComPtr,
               _ => return Err(intercom::raw::E_NOINTERFACE),
           })
    }
    fn interface_supports_error_info(riid: intercom::REFIID) -> bool {
        match *unsafe { &*riid } {
            self::IID_Foo_Automation => true,
            self::IID_Foo_Raw => true,
            _ => false,
        }
    }
}
#[allow(non_upper_case_globals)]
#[doc = "`Foo` class ID."]
pub const CLSID_Foo: intercom::CLSID =
    intercom::GUID{data1: 0u32,
                   data2: 0u16,
                   data3: 0u16,
                   data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],};
pub fn get_intercom_coclass_info_for_Foo()
 -> Vec<intercom::typelib::TypeInfo> {





    // Should be VARIANT_BOOL in Automation interface.

    <[_]>::into_vec(box
                        [intercom::typelib::TypeInfo::Class(intercom::ComStruct::new(intercom::typelib::CoClass::__new("Foo".into(),
                                                                                                                       intercom::GUID{data1:
                                                                                                                                          0u32,
                                                                                                                                      data2:
                                                                                                                                          0u16,
                                                                                                                                      data3:
                                                                                                                                          0u16,
                                                                                                                                      data4:
                                                                                                                                          [0u8,
                                                                                                                                           0u8,
                                                                                                                                           0u8,
                                                                                                                                           0u8,
                                                                                                                                           0u8,
                                                                                                                                           0u8,
                                                                                                                                           0u8,
                                                                                                                                           0u8],},
                                                                                                                       <[_]>::into_vec(box
                                                                                                                                           [intercom::typelib::InterfaceRef{name:
                                                                                                                                                                                "Foo".into(),
                                                                                                                                                                            iid_automation:
                                                                                                                                                                                IID_Foo_Automation,
                                                                                                                                                                            iid_raw:
                                                                                                                                                                                IID_Foo_Raw,}])))),
                         get_intercom_interface_info_for_Foo()])
}
#[doc = r" Gets type description of the #class_ident COM class."]
fn get_com_class_for_Foo() -> intercom::serialization::ComClass {
    intercom::serialization::ComClass::new("Foo".to_string(),
                                           get_interfaces_for_Foo())
}
#[doc = r" Gets the interfaces of the #class_ident COM class."]
fn get_interfaces_for_Foo()
 -> Vec<intercom::serialization::ComInterfaceVariant> {
    <[_]>::into_vec(box
                        [get_com_interface_for_Foo_Automation(),
                         get_com_interface_for_Foo_Raw()])
}
impl Foo {
    fn static_method(a: u16, b: i16) { }
    fn simple_method(&self) { }
    fn arg_method(&self, a: u16) { }
    fn simple_result_method(&self) -> u16 { 0 }
    fn com_result_method(&self) -> ComResult<u16> { Ok(0) }
    fn rust_result_method(&self) -> Result<u16, i32> { Ok(0) }
    fn tuple_result_method(&self) -> Result<(u8, u16, u32), i32> { Ok(0) }
    fn string_method(&self, input: String) -> String { input }
    fn string_result_method(&self, input: String) -> ComResult<String> {
        Ok(input)
    }
    fn complete_method(&mut self, a: u16, b: i16) -> ComResult<bool> {
        Ok(true)
    }
    fn bool_method(&self, input: bool) -> ComResult<bool> { Ok(input) }
    fn variant_method(&self, input: Variant) -> ComResult<Variant> {
        Ok(input)
    }
}
#[allow(non_snake_case)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_query_interface(self_vtable:
                                                                    intercom::RawComPtr,
                                                                riid:
                                                                    <intercom::REFIID
                                                                    as
                                                                    intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                                out:
                                                                    *mut <intercom::RawComPtr
                                                                         as
                                                                         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    intercom::ComBox::<Foo>::query_interface(&mut *((self_vtable as usize -
                                                         __Foo_Foo_AutomationVtbl_offset())
                                                        as *mut _), riid, out)
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_add_ref(self_vtable:
                                                            intercom::RawComPtr)
 ->
     <u32 as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    intercom::ComBox::<Foo>::add_ref(&mut *((self_vtable as usize -
                                                 __Foo_Foo_AutomationVtbl_offset())
                                                as *mut _))
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_release(self_vtable:
                                                            intercom::RawComPtr)
 ->
     <u32 as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    intercom::ComBox::<Foo>::release_ptr((self_vtable as usize -
                                              __Foo_Foo_AutomationVtbl_offset())
                                             as *mut _)
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_simple_method_Automation(self_vtable:
                                                                             intercom::RawComPtr)
 ->
     <() as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<() as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize -
                          __Foo_Foo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result = self_struct.simple_method();
                 Ok({ })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<() as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_arg_method_Automation(self_vtable:
                                                                          intercom::RawComPtr,
                                                                      a:
                                                                          <u16
                                                                          as
                                                                          intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType)
 ->
     <() as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<() as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize -
                          __Foo_Foo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result =
                     self_struct.arg_method((&<<u16 as
                                               intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::OwnedNativeType>::intercom_from(a)?).intercom_into()?);
                 Ok({ })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<() as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_simple_result_method_Automation(self_vtable:
                                                                                    intercom::RawComPtr)
 ->
     <u16 as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<u16 as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize -
                          __Foo_Foo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result = self_struct.simple_result_method();
                 Ok({ __result.intercom_into()? })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<u16 as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_com_result_method_Automation(self_vtable:
                                                                                 intercom::RawComPtr,
                                                                             __out:
                                                                                 *mut <u16
                                                                                      as
                                                                                      intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize -
                          __Foo_Foo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result = self_struct.com_result_method();
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_rust_result_method_Automation(self_vtable:
                                                                                  intercom::RawComPtr,
                                                                              __out:
                                                                                  *mut <u16
                                                                                       as
                                                                                       intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize -
                          __Foo_Foo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result = self_struct.rust_result_method();
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_tuple_result_method_Automation(self_vtable:
                                                                                   intercom::RawComPtr,
                                                                               __out1:
                                                                                   *mut <u8
                                                                                        as
                                                                                        intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                                                                               __out2:
                                                                                   *mut <u16
                                                                                        as
                                                                                        intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                                                                               __out3:
                                                                                   *mut <u32
                                                                                        as
                                                                                        intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize -
                          __Foo_Foo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result = self_struct.tuple_result_method();
                 Ok({
                        match __result {
                            Ok((v1, v2, v3)) => {
                                *__out1 = v1.intercom_into()?;
                                *__out2 = v2.intercom_into()?;
                                *__out3 = v3.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out1 =
                                    intercom::type_system::ExternDefault::extern_default();
                                *__out2 =
                                    intercom::type_system::ExternDefault::extern_default();
                                *__out3 =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_string_method_Automation(self_vtable:
                                                                             intercom::RawComPtr,
                                                                         input:
                                                                             <String
                                                                             as
                                                                             intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType)
 ->
     <String as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<String as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize -
                          __Foo_Foo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result =
                     self_struct.string_method((&<<String as
                                                  intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::OwnedNativeType>::intercom_from(input)?).intercom_into()?);
                 Ok({ __result.intercom_into()? })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<String as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_string_result_method_Automation(self_vtable:
                                                                                    intercom::RawComPtr,
                                                                                input:
                                                                                    <String
                                                                                    as
                                                                                    intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                                                __out:
                                                                                    *mut <String
                                                                                         as
                                                                                         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize -
                          __Foo_Foo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result =
                     self_struct.string_result_method((&<<String as
                                                         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::OwnedNativeType>::intercom_from(input)?).intercom_into()?);
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_complete_method_Automation(self_vtable:
                                                                               intercom::RawComPtr,
                                                                           a:
                                                                               <u16
                                                                               as
                                                                               intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                                           b:
                                                                               <i16
                                                                               as
                                                                               intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                                           __out:
                                                                               *mut <bool
                                                                                    as
                                                                                    intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize -
                          __Foo_Foo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &mut Foo = &mut **self_combox;
                 let __result =
                     self_struct.complete_method((&<<u16 as
                                                    intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::OwnedNativeType>::intercom_from(a)?).intercom_into()?,
                                                 (&<<i16 as
                                                    intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::OwnedNativeType>::intercom_from(b)?).intercom_into()?);
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_bool_method_Automation(self_vtable:
                                                                           intercom::RawComPtr,
                                                                       input:
                                                                           <bool
                                                                           as
                                                                           intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                                       __out:
                                                                           *mut <bool
                                                                                as
                                                                                intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize -
                          __Foo_Foo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result =
                     self_struct.bool_method((&<<bool as
                                                intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::OwnedNativeType>::intercom_from(input)?).intercom_into()?);
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Automation_variant_method_Automation(self_vtable:
                                                                              intercom::RawComPtr,
                                                                          input:
                                                                              <Variant
                                                                              as
                                                                              intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                                          __out:
                                                                              *mut <Variant
                                                                                   as
                                                                                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize -
                          __Foo_Foo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result =
                     self_struct.variant_method((&<<Variant as
                                                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::OwnedNativeType>::intercom_from(input)?).intercom_into()?);
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_upper_case_globals)]
const __Foo_Foo_AutomationVtbl_INSTANCE: __Foo_AutomationVtbl =
    __Foo_AutomationVtbl{__base:
                             intercom::IUnknownVtbl{query_interface_Automation:
                                                        __Foo_Foo_Automation_query_interface,
                                                    add_ref_Automation:
                                                        __Foo_Foo_Automation_add_ref,
                                                    release_Automation:
                                                        __Foo_Foo_Automation_release,},
                         simple_method_Automation:
                             __Foo_Foo_Automation_simple_method_Automation,
                         arg_method_Automation:
                             __Foo_Foo_Automation_arg_method_Automation,
                         simple_result_method_Automation:
                             __Foo_Foo_Automation_simple_result_method_Automation,
                         com_result_method_Automation:
                             __Foo_Foo_Automation_com_result_method_Automation,
                         rust_result_method_Automation:
                             __Foo_Foo_Automation_rust_result_method_Automation,
                         tuple_result_method_Automation:
                             __Foo_Foo_Automation_tuple_result_method_Automation,
                         string_method_Automation:
                             __Foo_Foo_Automation_string_method_Automation,
                         string_result_method_Automation:
                             __Foo_Foo_Automation_string_result_method_Automation,
                         complete_method_Automation:
                             __Foo_Foo_Automation_complete_method_Automation,
                         bool_method_Automation:
                             __Foo_Foo_Automation_bool_method_Automation,
                         variant_method_Automation:
                             __Foo_Foo_Automation_variant_method_Automation,};
#[allow(non_snake_case)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_query_interface(self_vtable:
                                                             intercom::RawComPtr,
                                                         riid:
                                                             <intercom::REFIID
                                                             as
                                                             intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternInputType,
                                                         out:
                                                             *mut <intercom::RawComPtr
                                                                  as
                                                                  intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    intercom::ComBox::<Foo>::query_interface(&mut *((self_vtable as usize -
                                                         __Foo_Foo_RawVtbl_offset())
                                                        as *mut _), riid, out)
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_add_ref(self_vtable:
                                                     intercom::RawComPtr)
 ->
     <u32 as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    intercom::ComBox::<Foo>::add_ref(&mut *((self_vtable as usize -
                                                 __Foo_Foo_RawVtbl_offset())
                                                as *mut _))
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_release(self_vtable:
                                                     intercom::RawComPtr)
 ->
     <u32 as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    intercom::ComBox::<Foo>::release_ptr((self_vtable as usize -
                                              __Foo_Foo_RawVtbl_offset()) as
                                             *mut _)
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_simple_method_Raw(self_vtable:
                                                               intercom::RawComPtr)
 ->
     <() as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<() as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize - __Foo_Foo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result = self_struct.simple_method();
                 Ok({ })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<() as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_arg_method_Raw(self_vtable:
                                                            intercom::RawComPtr,
                                                        a:
                                                            <u16 as
                                                            intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType)
 ->
     <() as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<() as
                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize - __Foo_Foo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result =
                     self_struct.arg_method((&<<u16 as
                                               intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::OwnedNativeType>::intercom_from(a)?).intercom_into()?);
                 Ok({ })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<() as
         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_simple_result_method_Raw(self_vtable:
                                                                      intercom::RawComPtr)
 ->
     <u16 as
     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<u16 as
                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize - __Foo_Foo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result = self_struct.simple_result_method();
                 Ok({ __result.intercom_into()? })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<u16 as
         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_com_result_method_Raw(self_vtable:
                                                                   intercom::RawComPtr,
                                                               __out:
                                                                   *mut <u16
                                                                        as
                                                                        intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize - __Foo_Foo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result = self_struct.com_result_method();
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_rust_result_method_Raw(self_vtable:
                                                                    intercom::RawComPtr,
                                                                __out:
                                                                    *mut <u16
                                                                         as
                                                                         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize - __Foo_Foo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result = self_struct.rust_result_method();
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_tuple_result_method_Raw(self_vtable:
                                                                     intercom::RawComPtr,
                                                                 __out1:
                                                                     *mut <u8
                                                                          as
                                                                          intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
                                                                 __out2:
                                                                     *mut <u16
                                                                          as
                                                                          intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
                                                                 __out3:
                                                                     *mut <u32
                                                                          as
                                                                          intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize - __Foo_Foo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result = self_struct.tuple_result_method();
                 Ok({
                        match __result {
                            Ok((v1, v2, v3)) => {
                                *__out1 = v1.intercom_into()?;
                                *__out2 = v2.intercom_into()?;
                                *__out3 = v3.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out1 =
                                    intercom::type_system::ExternDefault::extern_default();
                                *__out2 =
                                    intercom::type_system::ExternDefault::extern_default();
                                *__out3 =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_string_method_Raw(self_vtable:
                                                               intercom::RawComPtr,
                                                           input:
                                                               <String as
                                                               intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType)
 ->
     <String as
     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<String as
                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize - __Foo_Foo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result =
                     self_struct.string_method((&<<String as
                                                  intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::OwnedNativeType>::intercom_from(input)?).intercom_into()?);
                 Ok({ __result.intercom_into()? })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<String as
         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_string_result_method_Raw(self_vtable:
                                                                      intercom::RawComPtr,
                                                                  input:
                                                                      <String
                                                                      as
                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType,
                                                                  __out:
                                                                      *mut <String
                                                                           as
                                                                           intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize - __Foo_Foo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result =
                     self_struct.string_result_method((&<<String as
                                                         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::OwnedNativeType>::intercom_from(input)?).intercom_into()?);
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_complete_method_Raw(self_vtable:
                                                                 intercom::RawComPtr,
                                                             a:
                                                                 <u16 as
                                                                 intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType,
                                                             b:
                                                                 <i16 as
                                                                 intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType,
                                                             __out:
                                                                 *mut <bool as
                                                                      intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize - __Foo_Foo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &mut Foo = &mut **self_combox;
                 let __result =
                     self_struct.complete_method((&<<u16 as
                                                    intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::OwnedNativeType>::intercom_from(a)?).intercom_into()?,
                                                 (&<<i16 as
                                                    intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::OwnedNativeType>::intercom_from(b)?).intercom_into()?);
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_bool_method_Raw(self_vtable:
                                                             intercom::RawComPtr,
                                                         input:
                                                             <bool as
                                                             intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType,
                                                         __out:
                                                             *mut <bool as
                                                                  intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize - __Foo_Foo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result =
                     self_struct.bool_method((&<<bool as
                                                intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::OwnedNativeType>::intercom_from(input)?).intercom_into()?);
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_Foo_Raw_variant_method_Raw(self_vtable:
                                                                intercom::RawComPtr,
                                                            input:
                                                                <Variant as
                                                                intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternInputType,
                                                            __out:
                                                                *mut <Variant
                                                                     as
                                                                     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType)
 ->
     <intercom::raw::HRESULT as
     intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType {
    use intercom::type_system::{IntercomFrom, IntercomInto};
    let result:
            Result<<intercom::raw::HRESULT as
                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType,
                   intercom::ComError> =
        (||
             {
                 let self_combox =
                     (self_vtable as usize - __Foo_Foo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &Foo = &**self_combox;
                 let __result =
                     self_struct.variant_method((&<<Variant as
                                                   intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::OwnedNativeType>::intercom_from(input)?).intercom_into()?);
                 Ok({
                        match __result {
                            Ok(v1) => {
                                *__out = v1.intercom_into()?;
                                intercom::raw::S_OK
                            }
                            Err(e) => {
                                *__out =
                                    intercom::type_system::ExternDefault::extern_default();
                                intercom::store_error(e).hresult
                            }
                        }
                    })
             })();
    use intercom::ErrorValue;
    match result {
        Ok(v) => v,
        Err(err) =>
        <<intercom::raw::HRESULT as
         intercom::type_system::ExternType<intercom::type_system::RawTypeSystem>>::ExternOutputType
            as ErrorValue>::from_error(intercom::store_error(err)),
    }
}
#[allow(non_upper_case_globals)]
const __Foo_Foo_RawVtbl_INSTANCE: __Foo_RawVtbl =
    __Foo_RawVtbl{__base:
                      intercom::IUnknownVtbl{query_interface_Automation:
                                                 __Foo_Foo_Raw_query_interface,
                                             add_ref_Automation:
                                                 __Foo_Foo_Raw_add_ref,
                                             release_Automation:
                                                 __Foo_Foo_Raw_release,},
                  simple_method_Raw: __Foo_Foo_Raw_simple_method_Raw,
                  arg_method_Raw: __Foo_Foo_Raw_arg_method_Raw,
                  simple_result_method_Raw:
                      __Foo_Foo_Raw_simple_result_method_Raw,
                  com_result_method_Raw: __Foo_Foo_Raw_com_result_method_Raw,
                  rust_result_method_Raw:
                      __Foo_Foo_Raw_rust_result_method_Raw,
                  tuple_result_method_Raw:
                      __Foo_Foo_Raw_tuple_result_method_Raw,
                  string_method_Raw: __Foo_Foo_Raw_string_method_Raw,
                  string_result_method_Raw:
                      __Foo_Foo_Raw_string_result_method_Raw,
                  complete_method_Raw: __Foo_Foo_Raw_complete_method_Raw,
                  bool_method_Raw: __Foo_Foo_Raw_bool_method_Raw,
                  variant_method_Raw: __Foo_Foo_Raw_variant_method_Raw,};
impl intercom::HasInterface<Foo> for Foo { }
