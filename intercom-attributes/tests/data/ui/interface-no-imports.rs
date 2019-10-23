extern crate intercom;

#[intercom::com_interface]
trait IFoo {

    fn arg_type(&self, input: u32);

    fn ret_type(&self) -> intercom::ComResult<u32>;

    fn all_type(&self, input: u32) -> intercom::ComResult<u32>;
}

