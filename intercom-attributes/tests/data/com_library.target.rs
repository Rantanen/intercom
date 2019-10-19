#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
extern crate intercom;
use intercom::*;

pub mod some {
    pub mod path {
        pub struct Type;
        pub const CLSID_Type: i8 = 0i8;
        pub(crate) fn get_intercom_coclass_info_for_Type()
         -> intercom::typelib::TypeInfo {



            {
                {
                    ::std::rt::begin_panic("explicit panic",
                                           &("C:\\Dev\\Projects\\rust-com\\intercom-attributes\\tests/data\\com_library.source.rs",
                                             8u32, 93u32))
                }
            }
        }
    }
}
pub struct SimpleType;
pub const CLSID_SimpleType: i8 = 0i8;
pub(crate) fn get_intercom_coclass_info_for_SimpleType()
 -> intercom::typelib::TypeInfo {
    {
        {
            ::std::rt::begin_panic("explicit panic",
                                   &("C:\\Dev\\Projects\\rust-com\\intercom-attributes\\tests/data\\com_library.source.rs",
                                     14u32, 91u32))
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
pub unsafe extern "C" fn DllGetClassObject(rclsid: intercom::REFCLSID,
                                                 riid: intercom::REFIID,
                                                 pout:
                                                     *mut intercom::RawComPtr)
 -> intercom::raw::HRESULT {
    let mut com_struct =
        intercom::ComStruct::new(intercom::ClassFactory::new(rclsid,
                                                             |clsid|
                                                                 {
                                                                     match *clsid
                                                                         {
                                                                         self::some::path::CLSID_Type
                                                                         =>
                                                                         Ok(intercom::ComBox::new(some::path::Type::new())
                                                                                as
                                                                                intercom::RawComPtr),
                                                                         self::CLSID_SimpleType
                                                                         =>
                                                                         Ok(intercom::ComBox::new(SimpleType::new())
                                                                                as
                                                                                intercom::RawComPtr),
                                                                         intercom::alloc::CLSID_Allocator
                                                                         =>
                                                                         Ok(intercom::ComBox::new(intercom::alloc::Allocator::default())
                                                                                as
                                                                                intercom::RawComPtr),
                                                                         intercom::error::CLSID_ErrorStore
                                                                         =>
                                                                         Ok(intercom::ComBox::new(intercom::error::ErrorStore::default())
                                                                                as
                                                                                intercom::RawComPtr),
                                                                         _ =>
                                                                         Err(intercom::raw::E_NOINTERFACE),
                                                                     }
                                                                 }));
    intercom::ComBox::query_interface(com_struct.as_mut(), riid, pout);
    intercom::raw::S_OK
}
pub(crate) fn get_intercom_typelib() -> intercom::typelib::TypeLib {
    let types =
        <[_]>::into_vec(box
                            [intercom::alloc::get_intercom_coclass_info_for_Allocator(),
                             intercom::error::get_intercom_coclass_info_for_ErrorStore(),
                             some::path::get_intercom_coclass_info_for_Type(),
                             get_intercom_coclass_info_for_SimpleType()]).into_iter().flatten().collect::<Vec<_>>();
    intercom::typelib::TypeLib::__new("TestLib".into(),
                                      intercom::GUID{data1: 0u32,
                                                     data2: 0u16,
                                                     data3: 0u16,
                                                     data4:
                                                         [0u8, 0u8, 0u8, 0u8,
                                                          0u8, 0u8, 0u8,
                                                          0u8],},
                                      "1.0".into(), types)
}
#[no_mangle]
pub unsafe extern "C" fn IntercomTypeLib(type_system:
                                                   intercom::type_system::TypeSystemName,
                                               out: *mut intercom::RawComPtr)
 -> intercom::raw::HRESULT {
    let mut tlib = intercom::ComStruct::new(get_intercom_typelib());
    let rc =
        intercom::ComRc::<intercom::typelib::IIntercomTypeLib>::from(&tlib);
    let itf = intercom::ComRc::detach(rc);
    *out =
        match type_system {
            intercom::type_system::TypeSystemName::Automation =>
            intercom::ComItf::ptr::<intercom::type_system::AutomationTypeSystem>(&itf).ptr,
            intercom::type_system::TypeSystemName::Raw =>
            intercom::ComItf::ptr::<intercom::type_system::RawTypeSystem>(&itf).ptr,
        };
    intercom::raw::S_OK
}
#[no_mangle]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
pub unsafe extern "C" fn IntercomListClassObjects(pcount: *mut usize,
                                                        pclsids:
                                                            *mut *const intercom::CLSID)
 -> intercom::raw::HRESULT {
    if pcount.is_null() { return intercom::raw::E_POINTER; }
    if pclsids.is_null() { return intercom::raw::E_POINTER; }
    static AVAILABLE_CLASSES: [::intercom::CLSID; 4usize] =
        [some::path::CLSID_Type, CLSID_SimpleType,
         intercom::alloc::CLSID_Allocator, intercom::error::CLSID_ErrorStore];
    *pcount = 4usize;
    *pclsids = AVAILABLE_CLASSES.as_ptr();
    intercom::raw::S_OK
}
