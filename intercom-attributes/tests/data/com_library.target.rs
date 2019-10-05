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
        pub(crate) fn get_intercom_typeinfo_for_Type()
         -> intercom::typelib::TypeInfo {



            {
                {
                    ::std::rt::begin_panic("explicit panic",
                                           &("C:\\dev\\Projects\\rust-com\\intercom-attributes\\tests/data\\com_library.source.rs",
                                             8u32, 89u32))
                }
            }
        }
    }
}
pub struct SimpleType;
pub const CLSID_SimpleType: i8 = 0i8;
pub(crate) fn get_intercom_typeinfo_for_SimpleType()
 -> intercom::typelib::TypeInfo {
    {
        {
            ::std::rt::begin_panic("explicit panic",
                                   &("C:\\dev\\Projects\\rust-com\\intercom-attributes\\tests/data\\com_library.source.rs",
                                     14u32, 87u32))
        }
    }
}
#[allow(non_upper_case_globals)]
#[doc = "Built-in Allocator class ID."]
pub const CLSID_Allocator: intercom::CLSID =
    intercom::GUID{data1: 611004625u32,
                   data2: 64989u16,
                   data3: 14555u16,
                   data4:
                       [95u8, 81u8, 222u8, 241u8, 175u8, 60u8, 148u8,
                        102u8],};
#[allow(non_upper_case_globals)]
#[doc = "Built-in ErrorStore class ID."]
pub const CLSID_ErrorStore: intercom::CLSID =
    intercom::GUID{data1: 4043109527u32,
                   data2: 48586u16,
                   data3: 13069u16,
                   data4:
                       [65u8, 93u8, 255u8, 115u8, 129u8, 121u8, 178u8,
                        133u8],};
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
                                                                         self::CLSID_Allocator
                                                                         =>
                                                                         Ok(intercom::ComBox::new(intercom::alloc::Allocator::default())
                                                                                as
                                                                                intercom::RawComPtr),
                                                                         self::CLSID_ErrorStore
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
                            [some::path::get_intercom_typeinfo_for_Type(),
                             get_intercom_typeinfo_for_SimpleType()]);
    intercom::typelib::TypeLib::__new("TestLib",
                                      intercom::GUID{data1: 0u32,
                                                     data2: 0u16,
                                                     data3: 0u16,
                                                     data4:
                                                         [0u8, 0u8, 0u8, 0u8,
                                                          0u8, 0u8, 0u8,
                                                          0u8],}, "1.0",
                                      types)
}
pub unsafe extern "C" fn IntercomTypelib(type_system:
                                                   intercom::type_system::TypeSystemName,
                                               out:
                                                   *mut intercom::RawComPtr) {
    let mut tlib = intercom::ComStruct::new(get_intercom_typelib());
    let rc = intercom::ComRc::<intercom::typelib::ITypeLib>::from(tlib);
    let itf = intercom::ComRc::detach(rc);
    *out =
        match type_system {
            intercom::type_system::TypeSystemName::Automation =>
            intercom::ComItf::ptr::<intercom::type_system::AutomationTypeSystem>(&itf).ptr,
            intercom::type_system::TypeSystemName::Raw =>
            intercom::ComItf::ptr::<intercom::type_system::RawTypeSystem>(&itf).ptr,
        }
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
        [some::path::CLSID_Type, CLSID_SimpleType, CLSID_Allocator,
         CLSID_ErrorStore];
    *pcount = 4usize;
    *pclsids = AVAILABLE_CLASSES.as_ptr();
    intercom::raw::S_OK
}
