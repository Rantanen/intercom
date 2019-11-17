use intercom::*;

#[com_class(AllocTests)]
#[derive(Default)]
pub struct AllocTests;

#[com_interface]
impl AllocTests
{
    pub fn get_bstr_result(&self, value: u32) -> ComResult<String>
    {
        Ok(format!("{}", value))
    }
}
