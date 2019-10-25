
extern crate intercom;
use intercom::*;

// The NotExternType does not implement ExternType trait so it cannot be used
// as input/output type.
struct NotExternType;

#[com_interface]
trait IFoo {

    fn arg_type(&self, bad_type: NotExternType);

    fn ret_type(&self) -> ComResult<NotExternType>;

    fn all_type(&self, bad_type: NotExternType) -> ComResult<NotExternType>;
}

