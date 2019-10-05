#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
extern crate intercom;
use intercom::*;

trait IFoo {
    fn trait_method(&self);
}
#[doc = "`IFoo` interface ID."]
#[allow(non_upper_case_globals)]
const IID_IFoo_Automation: intercom::IID =
    intercom::GUID{data1: 0u32,
                   data2: 0u16,
                   data3: 0u16,
                   data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],};
#[doc = r" Gets type description of the #interface_variant COM class."]
fn get_com_interface_for_IFoo_Automation()
 -> intercom::serialization::ComInterfaceVariant {
    intercom::serialization::ComInterfaceVariant::new("IFoo_Automation".to_string(),
                                                      <intercom::type_system::AutomationTypeSystem
                                                          as
                                                          intercom::type_system::TypeSystem>::key())
}
#[allow(non_camel_case_types)]
#[repr(C)]
#[doc(hidden)]
struct __IFoo_AutomationVtbl {
    pub __base: intercom::IUnknownVtbl,
    pub trait_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                intercom::RawComPtr)
                                     ->
                                         <() as
                                         intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
}
#[doc = "`IFoo` interface ID."]
#[allow(non_upper_case_globals)]
const IID_IFoo_Raw: intercom::IID =
    intercom::GUID{data1: 0u32,
                   data2: 0u16,
                   data3: 0u16,
                   data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8],};
#[doc = r" Gets type description of the #interface_variant COM class."]
fn get_com_interface_for_IFoo_Raw()
 -> intercom::serialization::ComInterfaceVariant {
    intercom::serialization::ComInterfaceVariant::new("IFoo_Raw".to_string(),
                                                      <intercom::type_system::RawTypeSystem
                                                          as
                                                          intercom::type_system::TypeSystem>::key())
}
#[allow(non_camel_case_types)]
#[repr(C)]
#[doc(hidden)]
struct __IFoo_RawVtbl {
    pub __base: intercom::IUnknownVtbl,
    pub trait_method_Raw: unsafe extern "C" fn(self_vtable:
                                                         intercom::RawComPtr)
                              ->
                                  <() as
                                  intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
}
impl IFoo for intercom::ComItf<IFoo> {
    fn trait_method(&self) -> () {
        #[allow(unused_imports)]
        use intercom::ComInto;
        #[allow(unused_imports)]
        use intercom::ErrorValue;
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::AutomationTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __IFoo_AutomationVtbl;
            #[allow(unused_unsafe)]
            let result: Result<(), intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).trait_method_Automation)(comptr.ptr);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 0u8],};
                         Ok({ })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <() as intercom::ErrorValue>::from_com_error(err),
                   };
        }
        if let Some(comptr) =
               ComItf::maybe_ptr::<intercom::type_system::RawTypeSystem>(self)
           {
            use intercom::type_system::{IntercomFrom, IntercomInto};
            let vtbl = comptr.ptr as *const *const __IFoo_RawVtbl;
            #[allow(unused_unsafe)]
            let result: Result<(), intercom::ComError> =
                (||
                     unsafe {
                         let __result =
                             ((**vtbl).trait_method_Raw)(comptr.ptr);
                         let INTERCOM_iid =
                             intercom::GUID{data1: 0u32,
                                            data2: 0u16,
                                            data3: 0u16,
                                            data4:
                                                [0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                                 0u8, 1u8],};
                         Ok({ })
                     })();
            return match result {
                       Ok(v) => v,
                       Err(err) =>
                       <() as intercom::ErrorValue>::from_com_error(err),
                   };
        }
        <() as
            intercom::ErrorValue>::from_com_error(intercom::ComError::E_POINTER.into())
    }
}
impl intercom::ComInterface for IFoo {
    #[doc = "Returns the IID of the requested interface."]
    fn iid(ts: intercom::type_system::TypeSystemName)
     -> Option<&'static intercom::IID> {
        match ts {
            intercom::type_system::TypeSystemName::Automation =>
            Some(&IID_IFoo_Automation),
            intercom::type_system::TypeSystemName::Raw => Some(&IID_IFoo_Raw),
        }
    }
    fn deref(com_itf: &intercom::ComItf<IFoo>) -> &(IFoo + 'static) {
        com_itf
    }
}
impl intercom::type_system::BidirectionalTypeInfo for IFoo {
    #[doc = r" The name of the type."]
    fn type_name() -> &'static str { "IFoo" }
}

struct Foo;
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
#[inline(always)]
#[allow(non_snake_case)]
fn __Foo_IFoo_AutomationVtbl_offset() -> usize {
    unsafe {
        &intercom::ComBox::<Foo>::null_vtable().IFoo_Automation as *const _ as
            usize
    }
}
#[inline(always)]
#[allow(non_snake_case)]
fn __Foo_IFoo_RawVtbl_offset() -> usize {
    unsafe {
        &intercom::ComBox::<Foo>::null_vtable().IFoo_Raw as *const _ as usize
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
struct __FooVtblList {
    _ISupportErrorInfo: &'static intercom::ISupportErrorInfoVtbl,
    Foo_Automation: &'static __Foo_AutomationVtbl,
    Foo_Raw: &'static __Foo_RawVtbl,
    IFoo_Automation: &'static __IFoo_AutomationVtbl,
    IFoo_Raw: &'static __IFoo_RawVtbl,
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
                      Foo_Raw: &__Foo_Foo_RawVtbl_INSTANCE,
                      IFoo_Automation: &__Foo_IFoo_AutomationVtbl_INSTANCE,
                      IFoo_Raw: &__Foo_IFoo_RawVtbl_INSTANCE,}
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
               self::IID_IFoo_Automation =>
               &vtables.IFoo_Automation as *const &__IFoo_AutomationVtbl as
                   *mut &__IFoo_AutomationVtbl as intercom::RawComPtr,
               self::IID_IFoo_Raw =>
               &vtables.IFoo_Raw as *const &__IFoo_RawVtbl as
                   *mut &__IFoo_RawVtbl as intercom::RawComPtr,
               _ => return Err(intercom::raw::E_NOINTERFACE),
           })
    }
    fn interface_supports_error_info(riid: intercom::REFIID) -> bool {
        match *unsafe { &*riid } {
            self::IID_Foo_Automation => true,
            self::IID_Foo_Raw => true,
            self::IID_IFoo_Automation => true,
            self::IID_IFoo_Raw => true,
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
pub(crate) fn get_intercom_typeinfo_for_Foo() -> intercom::typelib::TypeInfo {
    intercom::typelib::TypeInfo::Class(intercom::typelib::CoClass{})
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
                         get_com_interface_for_IFoo_Automation(),
                         get_com_interface_for_Foo_Raw(),
                         get_com_interface_for_IFoo_Raw()])
}
impl Foo {
    pub fn struct_method(&self) { }
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
unsafe extern "C" fn __Foo_Foo_Automation_struct_method_Automation(self_vtable:
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
                 let __result = self_struct.struct_method();
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
#[allow(non_upper_case_globals)]
const __Foo_Foo_AutomationVtbl_INSTANCE: __Foo_AutomationVtbl =
    __Foo_AutomationVtbl{__base:
                             intercom::IUnknownVtbl{query_interface_Automation:
                                                        __Foo_Foo_Automation_query_interface,
                                                    add_ref_Automation:
                                                        __Foo_Foo_Automation_add_ref,
                                                    release_Automation:
                                                        __Foo_Foo_Automation_release,},
                         struct_method_Automation:
                             __Foo_Foo_Automation_struct_method_Automation,};
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
unsafe extern "C" fn __Foo_Foo_Raw_struct_method_Raw(self_vtable:
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
                 let __result = self_struct.struct_method();
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
#[allow(non_upper_case_globals)]
const __Foo_Foo_RawVtbl_INSTANCE: __Foo_RawVtbl =
    __Foo_RawVtbl{__base:
                      intercom::IUnknownVtbl{query_interface_Automation:
                                                 __Foo_Foo_Raw_query_interface,
                                             add_ref_Automation:
                                                 __Foo_Foo_Raw_add_ref,
                                             release_Automation:
                                                 __Foo_Foo_Raw_release,},
                  struct_method_Raw: __Foo_Foo_Raw_struct_method_Raw,};
impl intercom::HasInterface<Foo> for Foo { }
#[doc = "`Foo` interface ID."]
#[allow(non_upper_case_globals)]
pub const IID_Foo_Automation: intercom::IID =
    intercom::GUID{data1: 0u32,
                   data2: 0u16,
                   data3: 0u16,
                   data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8],};
#[doc = r" Gets type description of the #interface_variant COM class."]
fn get_com_interface_for_Foo_Automation()
 -> intercom::serialization::ComInterfaceVariant {
    intercom::serialization::ComInterfaceVariant::new("Foo_Automation".to_string(),
                                                      <intercom::type_system::AutomationTypeSystem
                                                          as
                                                          intercom::type_system::TypeSystem>::key())
}
#[allow(non_camel_case_types)]
#[repr(C)]
#[doc(hidden)]
pub struct __Foo_AutomationVtbl {
    pub __base: intercom::IUnknownVtbl,
    pub struct_method_Automation: unsafe extern "C" fn(self_vtable:
                                                                 intercom::RawComPtr)
                                      ->
                                          <() as
                                          intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
}
#[doc = "`Foo` interface ID."]
#[allow(non_upper_case_globals)]
pub const IID_Foo_Raw: intercom::IID =
    intercom::GUID{data1: 0u32,
                   data2: 0u16,
                   data3: 0u16,
                   data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8],};
#[doc = r" Gets type description of the #interface_variant COM class."]
fn get_com_interface_for_Foo_Raw()
 -> intercom::serialization::ComInterfaceVariant {
    intercom::serialization::ComInterfaceVariant::new("Foo_Raw".to_string(),
                                                      <intercom::type_system::RawTypeSystem
                                                          as
                                                          intercom::type_system::TypeSystem>::key())
}
#[allow(non_camel_case_types)]
#[repr(C)]
#[doc(hidden)]
pub struct __Foo_RawVtbl {
    pub __base: intercom::IUnknownVtbl,
    pub struct_method_Raw: unsafe extern "C" fn(self_vtable:
                                                          intercom::RawComPtr)
                               ->
                                   <() as
                                   intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType,
}
impl intercom::ComInterface for Foo {
    #[doc = "Returns the IID of the requested interface."]
    fn iid(ts: intercom::type_system::TypeSystemName)
     -> Option<&'static intercom::IID> {
        match ts {
            intercom::type_system::TypeSystemName::Automation =>
            Some(&IID_Foo_Automation),
            intercom::type_system::TypeSystemName::Raw => Some(&IID_Foo_Raw),
        }
    }
    fn deref(com_itf: &intercom::ComItf<Foo>) -> &Foo {
        let some_iunk: &intercom::ComItf<intercom::IUnknown> =
            com_itf.as_ref();
        let iunknown_iid =
            intercom::IUnknown::iid(intercom::type_system::TypeSystemName::Automation).expect("IUnknown must have Automation IID");
        let primary_iunk =
            some_iunk.query_interface(iunknown_iid).expect("All types must implement IUnknown");
        let combox: *mut intercom::ComBox<Foo> =
            primary_iunk as *mut intercom::ComBox<Foo>;
        unsafe {
            intercom::ComBox::release(combox);
            use std::ops::Deref;
            (*combox).deref()
        }
    }
}
impl intercom::type_system::BidirectionalTypeInfo for Foo {
    #[doc = r" The name of the type."]
    fn type_name() -> &'static str { "Foo" }
}
impl IFoo for Foo {
    fn trait_method(&self) { }
}
#[allow(non_snake_case)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_IFoo_Automation_query_interface(self_vtable:
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
                                                         __Foo_IFoo_AutomationVtbl_offset())
                                                        as *mut _), riid, out)
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_IFoo_Automation_add_ref(self_vtable:
                                                             intercom::RawComPtr)
 ->
     <u32 as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    intercom::ComBox::<Foo>::add_ref(&mut *((self_vtable as usize -
                                                 __Foo_IFoo_AutomationVtbl_offset())
                                                as *mut _))
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_IFoo_Automation_release(self_vtable:
                                                             intercom::RawComPtr)
 ->
     <u32 as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    intercom::ComBox::<Foo>::release_ptr((self_vtable as usize -
                                              __Foo_IFoo_AutomationVtbl_offset())
                                             as *mut _)
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_IFoo_Automation_trait_method_Automation(self_vtable:
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
                          __Foo_IFoo_AutomationVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &IFoo = &**self_combox;
                 let __result = self_struct.trait_method();
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
#[allow(non_upper_case_globals)]
const __Foo_IFoo_AutomationVtbl_INSTANCE: __IFoo_AutomationVtbl =
    __IFoo_AutomationVtbl{__base:
                              intercom::IUnknownVtbl{query_interface_Automation:
                                                         __Foo_IFoo_Automation_query_interface,
                                                     add_ref_Automation:
                                                         __Foo_IFoo_Automation_add_ref,
                                                     release_Automation:
                                                         __Foo_IFoo_Automation_release,},
                          trait_method_Automation:
                              __Foo_IFoo_Automation_trait_method_Automation,};
#[allow(non_snake_case)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_IFoo_Raw_query_interface(self_vtable:
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
                                                         __Foo_IFoo_RawVtbl_offset())
                                                        as *mut _), riid, out)
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_IFoo_Raw_add_ref(self_vtable:
                                                      intercom::RawComPtr)
 ->
     <u32 as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    intercom::ComBox::<Foo>::add_ref(&mut *((self_vtable as usize -
                                                 __Foo_IFoo_RawVtbl_offset())
                                                as *mut _))
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_IFoo_Raw_release(self_vtable:
                                                      intercom::RawComPtr)
 ->
     <u32 as
     intercom::type_system::ExternType<intercom::type_system::AutomationTypeSystem>>::ExternOutputType {
    intercom::ComBox::<Foo>::release_ptr((self_vtable as usize -
                                              __Foo_IFoo_RawVtbl_offset()) as
                                             *mut _)
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "C" fn __Foo_IFoo_Raw_trait_method_Raw(self_vtable:
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
                     (self_vtable as usize - __Foo_IFoo_RawVtbl_offset()) as
                         *mut intercom::ComBox<Foo>;
                 let self_struct: &IFoo = &**self_combox;
                 let __result = self_struct.trait_method();
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
#[allow(non_upper_case_globals)]
const __Foo_IFoo_RawVtbl_INSTANCE: __IFoo_RawVtbl =
    __IFoo_RawVtbl{__base:
                       intercom::IUnknownVtbl{query_interface_Automation:
                                                  __Foo_IFoo_Raw_query_interface,
                                              add_ref_Automation:
                                                  __Foo_IFoo_Raw_add_ref,
                                              release_Automation:
                                                  __Foo_IFoo_Raw_release,},
                   trait_method_Raw: __Foo_IFoo_Raw_trait_method_Raw,};
impl intercom::HasInterface<IFoo> for Foo { }
