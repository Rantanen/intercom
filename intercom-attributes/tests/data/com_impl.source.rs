extern crate intercom;
use intercom::*;

// We need the IID and Vtbl to ensure this compiles.
//
// Normally these are provided by the [com_interface].
struct __Foo_AutomationVtbl;
const IID_Foo_Automation: intercom::IID = intercom::GUID {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0, 0, 0, 0, 0, 0, 0, 0],
};

struct __Foo_RawVtbl;
const IID_Foo_Raw: intercom::IID = intercom::GUID {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0, 0, 0, 0, 0, 0, 0, 0],
};

#[com_class( clsid = "{00000000-0000-0000-0000-000000000000}", Foo)]
pub struct Foo;

#[com_impl]
impl Foo
{
    fn static_method(a: u16, b: i16) {}
    fn simple_method(&self) {}
    fn arg_method(&self, a: u16) {}

    fn simple_result_method(&self) -> u16 { 0 }
    fn com_result_method(&self) -> ComResult<u16> { Ok(0) }
    fn rust_result_method(&self) -> Result<u16, i32> { Ok(0) }
    fn tuple_result_method(&self) -> Result<(u8, u16, u32), i32> { Ok(0) }

    fn string_method(&self, input : String) -> String { input }
    fn string_result_method(&self, input : String) -> ComResult<String> { Ok(input) }

    fn complete_method(&mut self, a: u16, b: i16) -> ComResult<bool>
    {
        Ok(true)
    }
}
