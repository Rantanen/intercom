extern crate intercom;
use intercom::*;

#[com_interface(
    com_iid = "00000000-0000-0000-0000-000000000000",
    raw_iid = "00000000-0000-0000-0000-000000000001")]
trait IFoo {
    fn trait_method(&self);
}

#[com_class(
    clsid = "00000000-0000-0000-0000-000000000000", Foo, IFoo)]
struct Foo;

#[com_interface(
    com_iid = "00000000-0000-0000-0000-000000000002",
    raw_iid = "00000000-0000-0000-0000-000000000003")]
#[com_impl]
impl Foo {
    pub fn struct_method(&self) {}
}

#[com_impl]
impl IFoo for Foo {
    fn trait_method(&self) {}
}

