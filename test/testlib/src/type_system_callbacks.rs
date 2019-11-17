use intercom::*;
use strings::{IStringTests, STRING_DATA};

#[com_class(TypeSystemCaller)]
#[derive(Default)]
pub struct TypeSystemCaller;

#[com_interface]
impl TypeSystemCaller
{
    pub fn new() -> Self
    {
        TypeSystemCaller
    }

    pub fn call_string(&self, i: u32, callback: &ComItf<dyn IStringTests>) -> ComResult<()>
    {
        let actual = callback.string_to_index(STRING_DATA[i as usize])?;
        if actual == i {
            Ok(())
        } else {
            Err(ComError::E_FAIL)
        }
    }

    fn receive_string(&self, i: u32, callback: &ComItf<dyn IStringTests>) -> ComResult<()>
    {
        let actual = callback.index_to_string(i)?;
        let expected = STRING_DATA[i as usize];

        if actual == expected {
            Ok(())
        } else {
            Err(ComError::E_FAIL)
        }
    }

    fn pass_bstr(&self, callback: &ComItf<dyn IStringTests>) -> ComResult<()>
    {
        let bstr = BString::from("\u{1F4A9}");
        callback.bstr_parameter(&bstr, bstr.as_ptr() as usize)
    }

    fn pass_bstring(&self, callback: &ComItf<dyn IStringTests>) -> ComResult<()>
    {
        let bstr = BString::from("\u{1F4A9}");
        callback.bstring_parameter(bstr)
    }

    fn receive_bstring(&self, callback: &ComItf<dyn IStringTests>) -> ComResult<()>
    {
        let (bstr, ptr) = callback.bstring_return_value()?;

        if bstr.to_string().unwrap() != "\u{1F4A9}" {
            return Err(ComError::E_FAIL);
        }

        if bstr.as_ptr() as usize != ptr {
            return Err(ComError::E_POINTER);
        }

        Ok(())
    }

    fn pass_cstr(&self, callback: &ComItf<dyn IStringTests>) -> ComResult<()>
    {
        let cstr = CString::new("\u{1F4A9}").unwrap();
        callback.cstr_parameter(&cstr, cstr.as_ptr() as usize)
    }

    fn pass_cstring(&self, callback: &ComItf<dyn IStringTests>) -> ComResult<()>
    {
        let cstr = CString::new("\u{1F4A9}").unwrap();
        callback.cstring_parameter(cstr)
    }

    fn receive_cstring(&self, callback: &ComItf<dyn IStringTests>) -> ComResult<()>
    {
        let (cstr, ptr) = callback.cstring_return_value()?;

        if cstr.to_string_lossy() != "\u{1F4A9}" {
            return Err(ComError::E_FAIL);
        }

        if cstr.as_ptr() as usize != ptr {
            return Err(ComError::E_POINTER);
        }

        Ok(())
    }
}
