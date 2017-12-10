
use super::*;

/// A Rust wrapper for the `BSTR` string type.
///
/// Used for passing Rust `String` types through the COM interfaces. Intercom
/// should take care of the conversion in most cases, allowing the user to
/// stick with `String` types in their own code.
///
/// # `BSTR` details
///
/// The `BSTR` is both a length prefixed and zero terminated string with UTF-16
/// encoding. It is the string type widely used with Microsoft COM for
/// interoperability purposes.
///
/// What makes the `BSTR` exotic is that the `*mut u16` pointer references the
/// start of the string data. The length prefix is located _before_ the pointed
/// value.
///
/// It is important to note that when COM servers return `BSTR` strings, they
/// pass ownership of the string to the COM client. After this the COM client
/// is responsible for de-allocating the memory. Because of this it is
/// important that the memory allocation for `BSTR` values is well defined.
///
/// On Windows this means allocating the strings using `SysAllocString` or
/// `SysAllocStringLen` methods and freeing them with `SysFreeString` by
/// default.
#[repr(C)]
pub struct BStr( *mut u16 );

#[cfg(windows)]
#[link(name = "oleaut32")]
extern "system" {
    #[doc(hidden)]
    pub fn SysAllocStringLen(
        psz: *const u16,
        len: u32
    ) -> BStr;
}

#[cfg(not(windows))]
#[allow(non_snake_case)]
#[doc(hidden)]
pub unsafe fn SysAllocStringLen(
    _psz: *const u16,
    _len: u32
) -> BStr
{
    panic!( "Not implemented" );
}

impl BStr {

    /// Returns the text length in bytes.
    ///
    /// Does not include the length prefix or the terminating zero. However
    /// any zero bytes in the middle of the string are included.
    pub fn len_bytes( &self ) -> u32
    {
        unsafe {
            *(( self.0 as usize - 4 ) as *const u32 )
        }
    }

    /// Converts a Rust string into a `BStr`.
    pub fn string_to_bstr( s : &str ) -> BStr {

        let len = s.len() as u32;
        unsafe {
            SysAllocStringLen(
                s.encode_utf16().collect::<Vec<_>>().as_ptr(),
                len )
        }
    }

    /// Converts a `BStr` into a Rust `String`.
    pub fn bstr_to_string( &self ) -> String {

        let slice = unsafe { std::slice::from_raw_parts( 
                self.0 as *const u16,
                ( self.len_bytes() as usize ) / 2 ) };
        String::from_utf16_lossy( slice )
    }
}

impl Default for BStr {

    /// Default value representing an empty string.
    fn default() -> Self { BStr( std::ptr::null_mut() ) }
}

impl From<BStr> for String {
    fn from( source : BStr ) -> String {
        source.bstr_to_string()
    }
}

impl From<String> for BStr {
    fn from( source : String ) -> BStr {
        BStr::string_to_bstr( &source )
    }
}
