extern crate intercom;
use intercom::*;

pub mod some {
    pub mod path {
        pub struct Type;
        pub const CLSID_Type : i8 = 0i8;
        pub(crate) fn get_intercom_typeinfo_for_Type() -> intercom::typelib::TypeInfo { panic!() }
    }
}
pub struct SimpleType;
pub const CLSID_SimpleType : i8 = 0i8;

pub(crate) fn get_intercom_typeinfo_for_SimpleType() -> intercom::typelib::TypeInfo { panic!() }


com_library!(
        libid = "00000000-0000-0000-0000-000000000000",
        some::path::Type,
        SimpleType );
