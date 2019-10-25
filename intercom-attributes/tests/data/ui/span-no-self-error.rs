
extern crate intercom;
use intercom::*;

// Traits that have methods with no receiver cannot be turned into trait objects,
// which is needed for Intercom.

#[com_interface]
trait IFoo {
    fn arg_type();
}

