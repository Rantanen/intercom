use intercom::*;

#[com_class(AllocTests)]
pub struct AllocTests;

#[com_interface]
#[com_impl]
impl AllocTests
{
    pub fn new() -> AllocTests
    {
        AllocTests
    }

    pub fn get_bstr(&self, value: u32) -> String
    {
        format!("{}", value)
    }

    pub fn get_bstr_result(&self, value: u32) -> ComResult<String>
    {
        Ok(format!("{}", value))
    }
}
