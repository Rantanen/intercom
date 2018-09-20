
use super::itfs;

#[com_class( clsid = "00000005-0000-0000-0000-000000000000", Interface1, Interface2)]
struct Class2;

#[com_impl]
impl itfs::Interface1 for Class2 {}

#[com_impl]
impl itfs::Interface2 for Class2 {}
