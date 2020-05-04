extern crate intercom;
use intercom::prelude::*;

#[com_class]
struct Struct;

#[com_interface]
impl Struct {}

com_library! {
    interface Struct,
}
