use intercom::*;

pub static STRING_DATA: &[&str] = &["", "Test", "öäå", "\u{1F980}"];

#[com_interface]
pub trait IStringTests
{
    fn string_to_index(&self, s: &str) -> ComResult<u32>;

    fn index_to_string(&self, i: u32) -> ComResult<String>;

    fn bstr_parameter(&self, s: &BStr, ptr: usize) -> ComResult<()>;

    fn bstring_parameter(&self, s: BString) -> ComResult<()>;

    fn bstring_return_value(&self) -> ComResult<(BString, usize)>;

    fn cstr_parameter(&self, s: &CStr, ptr: usize) -> ComResult<()>;

    fn cstring_parameter(&self, s: CString) -> ComResult<()>;

    fn cstring_return_value(&self) -> ComResult<(CString, usize)>;

    fn invalid_string(&self, s: &str) -> ComResult<()>;
}

#[com_class(IStringTests)]
#[derive(Default)]
pub struct StringTests;

impl StringTests
{
    pub fn new() -> StringTests
    {
        StringTests
    }
}

impl IStringTests for StringTests
{
    fn string_to_index(&self, s: &str) -> ComResult<u32>
    {
        for (idx, &candidate) in STRING_DATA.iter().enumerate() {
            if s == candidate {
                return Ok(idx as u32);
            }
        }

        Err(ComError::E_FAIL)
    }

    fn index_to_string(&self, i: u32) -> ComResult<String>
    {
        STRING_DATA
            .get(i as usize)
            .map(|s| s.to_string())
            .ok_or_else(|| ComError::E_FAIL)
    }

    fn bstr_parameter(&self, s: &BStr, ptr: usize) -> ComResult<()>
    {
        let string = s.to_string().map_err(|_| ComError::E_INVALIDARG)?;

        if string != "\u{1F600}" {
            return Err(ComError::E_FAIL);
        }

        if s.as_ptr() as usize == ptr {
            Ok(())
        } else {
            Err(ComError::E_POINTER)
        }
    }

    fn bstring_parameter(&self, s: BString) -> ComResult<()>
    {
        let string = s.to_string().map_err(|_| ComError::E_INVALIDARG)?;

        if string != "\u{1F600}" {
            Err(ComError::E_FAIL)
        } else {
            Ok(())
        }
    }

    fn bstring_return_value(&self) -> ComResult<(BString, usize)>
    {
        let bs: BString = BString::from("\u{1F600}");
        let ptr = bs.as_ptr() as usize;

        Ok((bs, ptr))
    }

    fn cstr_parameter(&self, s: &CStr, ptr: usize) -> ComResult<()>
    {
        if s.to_string_lossy() != "\u{1F600}" {
            return Err(ComError::E_FAIL);
        }

        if s.as_ptr() as usize == ptr {
            Ok(())
        } else {
            Err(ComError::E_POINTER)
        }
    }

    fn cstring_parameter(&self, s: CString) -> ComResult<()>
    {
        if s.to_string_lossy() != "\u{1F600}" {
            Err(ComError::E_FAIL)
        } else {
            Ok(())
        }
    }

    fn cstring_return_value(&self) -> ComResult<(CString, usize)>
    {
        let bs: CString = CString::new("\u{1F600}").unwrap();
        let ptr = bs.as_ptr() as usize;

        Ok((bs, ptr))
    }

    fn invalid_string(&self, s: &str) -> ComResult<()>
    {
        // Don't do any validation here.
        // Intercom should do validation automatically.
        println!("String parameter was not invalid: {}", s);

        // Caller expects E_INVALIDARG, use E_FAIL to indicate something
        // went wrong.
        Err(ComError::E_FAIL)
    }
}
