
#[com_library( libid = "00000001-0000-0000-0000-000000000000", Class1, Class2 )]

mod itfs;
mod cls1;
mod cls2;
mod no_guid;

use cls1::Class1;
use cls2::Class2;
use no_guid::NoGuid;
