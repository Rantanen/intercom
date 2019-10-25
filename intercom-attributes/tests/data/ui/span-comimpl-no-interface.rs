
extern crate intercom;
use intercom::*;

trait NotComTrait {}

struct S;

#[com_impl]
impl NotComTrait for S {}

