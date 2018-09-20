
#[com_class( clsid = "00000004-0000-0000-0000-000000000000", Class1)]
pub struct Class1;

#[com_interface( com_iid = "00000006-0000-0000-0000-000000000000")]
#[com_impl]
impl Class1 {}
