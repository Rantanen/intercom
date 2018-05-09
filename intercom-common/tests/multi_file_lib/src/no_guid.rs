use super::itfs;

use super::itfs;

#[com_class( NO_GUID, Interface1, Interface2)]
#[derive(Debug)]
pub struct NoGuid
{
    test: String
}

#[com_impl]
impl itfs::Interface1 for NoGuid {}

#[com_impl]
impl itfs::Interface2 for NoGuid {}
