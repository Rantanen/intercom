#![feature(proc_macro)]
extern crate intercom;
use intercom::*;

#[com_interface("00000000-0000-0000-0000-000000000000")]
trait IFoo {
    fn trait_method(&self);
}

#[com_class("00000000-0000-0000-0000-000000000000", Foo, IFoo)]
struct Foo;

#[com_impl]
impl Foo {
    pub fn struct_method(&self) {}
}

#[com_interface("00000000-0000-0000-0000-000000000000")]
#[com_impl]
impl IFoo for Foo {
    fn trait_method(&self) {}
}

