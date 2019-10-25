
extern crate intercom;
use intercom::*;

#[com_interface]
trait IFoo {

    fn arg_type(&self, input: u32);

    fn ret_type(&self) -> ComResult<u32>;

    fn all_type(&self, input: u32) -> ComResult<u32>;
}

