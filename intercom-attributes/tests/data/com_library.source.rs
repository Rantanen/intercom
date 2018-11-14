extern crate intercom;
use intercom::*;

pub mod some {
    pub mod path {
        pub struct Type;
        pub const CLSID_Type : i8 = 0i8;
    }
}
pub struct SimpleType;
pub const CLSID_SimpleType : i8 = 0i8;

com_library!(
        libid = "00000000-0000-0000-0000-000000000000",
        some::path::Type,
        SimpleType );
