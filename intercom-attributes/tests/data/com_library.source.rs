extern crate intercom;
use intercom::*;
use std::mem::MaybeUninit;

pub mod some {
    pub mod path {
        use std::mem::MaybeUninit;
        pub struct Type;
        pub const CLSID_Type : i8 = 0i8;
        pub(crate) fn get_intercom_coclass_info_for_Type() -> intercom::typelib::TypeInfo {
            unsafe { MaybeUninit::uninit().assume_init() }
        }
    }
}
pub struct SimpleType;
pub const CLSID_SimpleType : i8 = 0i8;

pub(crate) fn get_intercom_coclass_info_for_SimpleType() -> intercom::typelib::TypeInfo {
    unsafe { MaybeUninit::uninit().assume_init() }
}


com_library!(
        libid = "00000000-0000-0000-0000-000000000000",
        some::path::Type,
        SimpleType );
