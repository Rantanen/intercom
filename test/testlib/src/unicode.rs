use intercom::*;
use std::os::raw::c_char;

#[com_class(UnicodeConversion)]
#[derive(Default)]
pub struct UnicodeConversion;

#[com_interface]
impl UnicodeConversion
{
    fn utf8_to_utf16(&self, input: *const c_char) -> ComResult<*mut u16>
    {
        if input.is_null() {
            return Ok(std::ptr::null_mut());
        }

        let cstr = unsafe { CStr::from_ptr(input) };
        let rust_str = cstr.to_str().map_err(|_| ComError::E_INVALIDARG)?;

        let utf16: Vec<_> = rust_str.encode_utf16().collect();
        let buffer_len = (utf16.len() + 1) * 2;

        unsafe {
            let buffer = intercom::alloc::allocate((utf16.len() + 1) * 2) as *mut u16;
            std::ptr::copy_nonoverlapping(
                utf16.as_ptr() as *const c_char,
                buffer as *mut c_char,
                buffer_len,
            );
            Ok(buffer)
        }
    }

    fn utf16_to_utf8(&self, input: *const u16) -> ComResult<*mut c_char>
    {
        if input.is_null() {
            return Ok(std::ptr::null_mut());
        }

        let slice = unsafe {
            // Find the first zero byte.
            let mut len = 0usize;
            while *input.add(len) != 0 {
                len += 1;
            }

            std::slice::from_raw_parts(input, len)
        };

        let s = String::from_utf16(slice).map_err(|_| ComError::E_INVALIDARG)?;
        let cstring = CString::new(s).map_err(|_| ComError::E_INVALIDARG)?;
        let utf8bytes = cstring.to_bytes();

        unsafe {
            let buffer = intercom::alloc::allocate(utf8bytes.len() + 1) as *mut c_char;
            std::ptr::copy_nonoverlapping(
                utf8bytes.as_ptr() as *const c_char,
                buffer as *mut c_char,
                utf8bytes.len() + 1,
            );
            Ok(buffer)
        }
    }
}
