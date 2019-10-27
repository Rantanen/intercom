
extern crate intercom;
use intercom::*;

#[com_interface]
trait NotComInterface {}

struct S;

#[com_impl]
impl NotComInterface for S {}
