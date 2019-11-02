//!
//! Intercom string representations.
//!

use std::{
    self,
    borrow::Borrow,
    ffi,
    ops::Deref,
    os::raw::c_char,
    str::{FromStr, Utf8Error},
};

use crate::intercom::{ComError, ComResult};
use crate::type_system::{
    AutomationTypeSystem, ExternType, IntercomFrom, IntercomInto, RawTypeSystem,
};

#[derive(Debug)]
pub struct FormatError;

/// Represents a borrowed BSTR string.
#[derive(PartialEq)]
pub struct BStr(
    // Invariant 1: .0.as_ptr() must be a valid BSTR pointer _or_ 0x1 if len == 0.
    //              This includes having u32 alignment.
    // Invariant 2: .0.len() must fit to u32.
    [u8],
);

impl std::fmt::Debug for BStr
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(
            f,
            "BStr(\"{}\")",
            String::from_utf16_lossy(unsafe {
                std::slice::from_raw_parts(self.as_ptr() as *const u16, self.len() as usize / 2)
            })
        )
    }
}

impl BStr
{
    /// Unsafely creates a `BStr` from a BSTR pointer.
    ///
    /// This function will cast the pointer into a `BStr`. The provied pointer
    /// **must** be a valid BSTR pointer and must be valid while the BStr is
    /// alive. The BStr must also not be moved.
    ///
    /// # Safety
    ///
    /// The parameter must be a valid BSTR pointer. This includes both the
    /// memory layout and allocation using BSTR-compatible allocation
    /// functions.
    ///
    /// In addition to this the pointer must be kept alive while the returned
    /// reference is in use.
    pub unsafe fn from_ptr<'a>(ptr: *const u16) -> &'a BStr
    {
        // The BStr invariant 1 states the ptr must be valid BSTR pointer,
        // which is u32-aligned.
        #![allow(clippy::cast_ptr_alignment)]
        let (len, final_ptr) = match ptr as usize {
            0 => (0, 1 as *const u8),
            _ => (*(ptr.offset(-2) as *const u32), ptr as *const u8),
        };

        let slice = std::slice::from_raw_parts(final_ptr, len as usize);
        Self::from_slice_unchecked(slice)
    }

    /// Unsafely creates a `BStr` from a slice.
    ///
    /// This function will cast the slice into a `BStr`. The slice **must**
    /// be a slice constructed from a valid BSTR pointer. Specifically the slice
    /// as_ptr() must result in a valid BSTR pointer.
    unsafe fn from_slice_unchecked(slice: &[u8]) -> &BStr
    {
        &*(slice as *const [u8] as *const BStr)
    }

    /// Returns the pointer as a 16-bit wide character pointer.
    pub fn as_ptr(&self) -> *const u16
    {
        // The BStr invariant 1 states the ptr must be valid BSTR pointer,
        // which is u32-aligned.
        #![allow(clippy::cast_ptr_alignment)]

        // 0x1 is a marker pointer
        let ptr = self.0.as_ptr();
        if self.0.is_empty() && ptr as usize == 0x1 {
            std::ptr::null()
        } else {
            ptr as *const u16
        }
    }

    /// Returns the string length in bytes.
    ///
    /// Does not include the length prefix or the terminating zero. However
    /// any zero bytes in the middle of the string are included.
    pub fn len_bytes(&self) -> u32
    {
        // The len() on the slice is stored separately and can be used even
        // if the buffer itself points to an invalid value as is the case with
        // some 0-length BSTRs.
        self.0.len() as u32
    }

    /// Returns the string length in characters.
    pub fn len(&self) -> u32
    {
        // As long as the BStr is valie this is safe.
        unsafe { os::SysStringLen(self.as_ptr()) }
    }

    pub fn is_empty(&self) -> bool
    {
        self.len_bytes() == 0
    }

    /// Gets the BStr as a slice of 16-bit characters.
    pub fn as_slice(&self) -> &[u8]
    {
        &self.0
    }

    pub fn to_string(&self) -> Result<String, FormatError>
    {
        match self.len_bytes() {
            x if x % 2 == 0 => String::from_utf16(unsafe {
                std::slice::from_raw_parts(self.as_ptr() as *const u16, x as usize / 2)
            })
            .map_err(|_| FormatError),
            _ => Err(FormatError),
        }
    }
}

/// An owned BSTR string Rust type.
///
/// Used for passing strings with their ownership through the COM interfaces.
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
pub struct BString(
    // The pointer must be 32-bit aligned.
    *mut u16,
);

impl std::fmt::Debug for BString
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(
            f,
            "BStr(\"{}\")",
            String::from_utf16_lossy(unsafe {
                std::slice::from_raw_parts(
                    self.as_ptr() as *const u16,
                    self.len_bytes() as usize / 2,
                )
            })
        )
    }
}

impl PartialEq for BString
{
    fn eq(&self, other: &Self) -> bool
    {
        // Deref into &BStr and compare those.
        **self == **other
    }
}

impl Clone for BString
{
    fn clone(&self) -> BString
    {
        self.as_ref().to_owned()
    }
}

impl BString
{
    /// # Safety
    ///
    /// The parameter must be a valid BSTR pointer. This includes both the
    /// memory layout and allocation using BSTR-compatible allocation
    /// functions.
    ///
    /// In addition the pointer ownership moves to the BString and the pointer
    /// must not be freed outside of BString drop.
    pub unsafe fn from_ptr(ptr: *mut u16) -> BString
    {
        BString(ptr)
    }

    /// Converts a C-string into a `BString`.
    pub fn from_cstr(s: &ffi::CStr) -> Result<BString, Utf8Error>
    {
        Ok(Self::from_str(s.to_str()?).expect("Error type is never type"))
    }

    /// Returns the pointer as a 16-bit wide character pointer.
    pub fn as_mut_ptr(&mut self) -> *mut u16
    {
        self.0 as *mut u16
    }

    /// Converts the `BString` into a raw pointer.
    pub fn into_ptr(self) -> *mut u16
    {
        let ptr = self.0;
        std::mem::forget(self);
        ptr as *mut u16
    }
}

impl FromStr for BString
{
    type Err = std::string::ParseError;

    /// Converts a Rust string into a `BString`.
    fn from_str(s: &str) -> Result<BString, Self::Err>
    {
        // Avoid unnecessary allocations when the string is empty.
        // Null and empty BSTRs should be treated as equal.
        // See https://blogs.msdn.microsoft.com/ericlippert/2003/09/12/erics-complete-guide-to-bstr-semantics/
        if s.is_empty() {
            return Ok(BString(std::ptr::null_mut()));
        }

        unsafe {
            let chars = s.encode_utf16().collect::<Vec<_>>();
            let bstr = os::SysAllocStringLen(chars.as_ptr(), chars.len() as u32);

            // Memory issues are traditionally fatal in Rust and do not cause
            // Err-results.
            if bstr.0.is_null() {
                panic!("Allocating BStr failed.");
            }

            Ok(BString::from_ptr(bstr.0))
        }
    }
}

impl Deref for BString
{
    type Target = BStr;
    fn deref(&self) -> &BStr
    {
        unsafe { BStr::from_ptr(self.0) }
    }
}

// AsRef/Borrow/ToOwned implementations.

impl AsRef<BStr> for BStr
{
    fn as_ref(&self) -> &BStr
    {
        self
    }
}

impl AsRef<BStr> for BString
{
    fn as_ref(&self) -> &BStr
    {
        self
    }
}

impl Borrow<BStr> for BString
{
    fn borrow(&self) -> &BStr
    {
        self
    }
}

impl ToOwned for BStr
{
    type Owned = BString;

    fn to_owned(&self) -> Self::Owned
    {
        unsafe {
            BString::from_ptr(
                os::SysAllocStringLen(self.as_ptr(), os::SysStringLen(self.as_ptr())).0,
            )
        }
    }
}

impl<'a> From<&'a str> for BString
{
    fn from(source: &str) -> BString
    {
        BString::from_str(source).expect("Error type is never type")
    }
}

impl From<String> for BString
{
    fn from(source: String) -> BString
    {
        BString::from_str(&source).expect("Error type is never type")
    }
}

impl Default for BString
{
    fn default() -> BString
    {
        BString(std::ptr::null_mut())
    }
}

impl Drop for BString
{
    fn drop(&mut self)
    {
        unsafe {
            os::SysFreeString(self.as_mut_ptr());
            self.0 = std::ptr::null_mut();
        }
    }
}

impl IntercomFrom<BString> for String
{
    unsafe fn intercom_from(source: BString) -> Result<Self, ComError>
    {
        source.to_string().map_err(|_| ComError::E_INVALIDARG)
    }
}

impl IntercomFrom<CString> for String
{
    unsafe fn intercom_from(source: CString) -> Result<Self, ComError>
    {
        source.into_string().map_err(|_| ComError::E_INVALIDARG)
    }
}
pub type CStr = std::ffi::CStr;
pub type CString = std::ffi::CString;

//////////////////////////////////////////
// OS specific string allocation.

#[cfg(windows)]
mod os
{

    #[link(name = "oleaut32")]
    extern "system" {
        #[doc(hidden)]
        pub fn SysAllocStringLen(psz: *const u16, len: u32) -> crate::raw::OutBSTR;

        #[doc(hidden)]
        pub fn SysFreeString(bstr: *mut u16);

        #[doc(hidden)]
        pub fn SysStringLen(pbstr: *const u16) -> u32;
    }
}

#[cfg(not(windows))]
#[allow(non_snake_case)]
mod os
{
    use libc;

    #[doc(hidden)]
    pub unsafe fn SysAllocStringLen(psz: *const u16, len: u32) -> crate::raw::OutBSTR
    {
        // Match the SysAllocStringLen implementation on Windows when
        // psz is null.
        if psz.is_null() {
            return crate::raw::OutBSTR(std::ptr::null_mut());
        }

        // Length prefix + data length + null-terminator.
        // The length of BSTR is expressed as bytes in the prefix.
        let data_length = (len * 2) as usize;
        let buffer_length: usize = 4 + data_length + 2;
        let buffer = libc::malloc(buffer_length);
        if buffer.is_null() {
            return crate::raw::OutBSTR(std::ptr::null_mut());
        }

        // Set the length prefix.
        let length_u32 = data_length as u32;
        let length_prefix = &length_u32 as *const _ as *const libc::c_void;
        libc::memcpy(buffer, length_prefix, 4);

        // The actual data.
        let src_buffer = psz as *const u8 as *mut libc::c_void;
        libc::memcpy(buffer.offset(4), src_buffer, data_length as usize);

        let null_terminator: u16 = 0;
        let null_terminator = &null_terminator as *const _ as *const libc::c_void;
        libc::memcpy(buffer.offset(4 + data_length as isize), null_terminator, 2);

        let buffer = buffer.offset(4) as *mut u16;
        crate::raw::OutBSTR(buffer)
    }

    #[doc(hidden)]
    pub unsafe fn SysFreeString(pbstr: *mut u16)
    {
        if !pbstr.is_null() {
            let ptr = pbstr.offset(-2) as *mut libc::c_void;
            libc::free(ptr);
        }
    }

    #[doc(hidden)]
    pub unsafe fn SysStringLen(pbstr: *const u16) -> u32
    {
        // The BSTR pointers should be u32-aligned.
        #![allow(clippy::cast_ptr_alignment)]
        if pbstr.is_null() {
            0
        } else {
            *(pbstr.offset(-2) as *const u32) / 2
        }
    }
}

#[derive(Debug, Clone)]
pub enum IntercomString
{
    BString(BString),
    CString(CString),
    String(String),
}

impl From<BString> for IntercomString
{
    fn from(source: BString) -> Self
    {
        IntercomString::BString(source)
    }
}

impl From<String> for IntercomString
{
    fn from(source: String) -> Self
    {
        IntercomString::String(source)
    }
}

impl From<CString> for IntercomString
{
    fn from(source: CString) -> Self
    {
        IntercomString::CString(source)
    }
}

impl IntercomFrom<IntercomString> for BString
{
    unsafe fn intercom_from(source: IntercomString) -> Result<Self, ComError>
    {
        match source {
            IntercomString::BString(bstring) => bstring.intercom_into(),
            IntercomString::CString(cstring) => cstring.intercom_into(),
            IntercomString::String(string) => string.intercom_into(),
        }
    }
}

impl IntercomFrom<IntercomString> for CString
{
    unsafe fn intercom_from(source: IntercomString) -> Result<Self, ComError>
    {
        match source {
            IntercomString::BString(bstring) => bstring.intercom_into(),
            IntercomString::CString(cstring) => cstring.intercom_into(),
            IntercomString::String(string) => string.intercom_into(),
        }
    }
}

impl IntercomFrom<IntercomString> for String
{
    unsafe fn intercom_from(source: IntercomString) -> Result<Self, ComError>
    {
        match source {
            IntercomString::BString(bstring) => bstring.intercom_into(),
            IntercomString::CString(cstring) => cstring.intercom_into(),
            IntercomString::String(string) => string.intercom_into(),
        }
    }
}

// Automation type system.

impl ExternType<AutomationTypeSystem> for &str
{
    type ExternInputType = crate::raw::InBSTR;
    type ExternOutputType = crate::raw::OutBSTR;
    type OwnedExternType = BString;
    type OwnedNativeType = String;
}

impl ExternType<AutomationTypeSystem> for String
{
    type ExternInputType = crate::raw::InBSTR;
    type ExternOutputType = crate::raw::OutBSTR;
    type OwnedExternType = BString;
    type OwnedNativeType = String;
}

impl ExternType<AutomationTypeSystem> for &BStr
{
    type ExternInputType = crate::raw::InBSTR;
    type ExternOutputType = crate::raw::OutBSTR;
    type OwnedExternType = crate::raw::InBSTR;
    type OwnedNativeType = crate::raw::InBSTR;
}

impl ExternType<AutomationTypeSystem> for BString
{
    type ExternInputType = crate::raw::InBSTR;
    type ExternOutputType = crate::raw::OutBSTR;
    type OwnedExternType = BString;
    type OwnedNativeType = BString;
}

impl ExternType<AutomationTypeSystem> for &CStr
{
    type ExternInputType = crate::raw::InBSTR;
    type ExternOutputType = crate::raw::OutBSTR;
    type OwnedExternType = BString;
    type OwnedNativeType = CString;
}

impl ExternType<AutomationTypeSystem> for CString
{
    type ExternInputType = crate::raw::InBSTR;
    type ExternOutputType = crate::raw::OutBSTR;
    type OwnedExternType = BString;
    type OwnedNativeType = CString;
}

// Raw type system.

impl ExternType<RawTypeSystem> for &str
{
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = CString;
    type OwnedNativeType = String;
}

impl ExternType<RawTypeSystem> for String
{
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = CString;
    type OwnedNativeType = String;
}

impl ExternType<RawTypeSystem> for &BStr
{
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = CString;
    type OwnedNativeType = BString;
}

impl ExternType<RawTypeSystem> for BString
{
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = CString;
    type OwnedNativeType = BString;
}

impl ExternType<RawTypeSystem> for &CStr
{
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = *const c_char;
    type OwnedNativeType = *const c_char;
}

impl ExternType<RawTypeSystem> for CString
{
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = CString;
    type OwnedNativeType = CString;
}

// InBSTR -> X

impl IntercomFrom<crate::raw::InBSTR> for String
{
    unsafe fn intercom_from(source: crate::raw::InBSTR) -> ComResult<Self>
    {
        Ok(BStr::from_ptr(source.0)
            .to_string()
            .map_err(|_| ComError::E_INVALIDARG)?)
    }
}

impl IntercomFrom<crate::raw::InBSTR> for BString
{
    unsafe fn intercom_from(source: crate::raw::InBSTR) -> ComResult<Self>
    {
        Ok(BStr::from_ptr(source.0).to_owned())
    }
}

impl<'a> IntercomFrom<&'a crate::raw::InBSTR> for &'a BStr
{
    unsafe fn intercom_from(source: &'a crate::raw::InBSTR) -> ComResult<Self>
    {
        Ok(BStr::from_ptr(source.0))
    }
}

impl<'a> IntercomFrom<crate::raw::InBSTR> for &'a BStr
{
    unsafe fn intercom_from(source: crate::raw::InBSTR) -> ComResult<Self>
    {
        Ok(BStr::from_ptr(source.0))
    }
}

impl IntercomFrom<crate::raw::InBSTR> for CString
{
    unsafe fn intercom_from(source: crate::raw::InBSTR) -> ComResult<Self>
    {
        CString::new(
            BStr::from_ptr(source.0)
                .to_string()
                .map_err(|_| ComError::E_INVALIDARG)?,
        )
        .map_err(|_| ComError::E_INVALIDARG)
    }
}

impl<TPtr, TTarget> IntercomFrom<*mut TPtr> for TTarget
where
    TTarget: IntercomFrom<*const TPtr>,
{
    default unsafe fn intercom_from(source: *mut TPtr) -> ComResult<Self>
    {
        let bstring: ComResult<TTarget> = (source as *const TPtr).intercom_into();

        bstring
    }
}

// The *mut u16 strings should be BSTRs and must not be freed using the
// normal `alloc::free`.

impl IntercomFrom<intercom::raw::OutBSTR> for BString
{
    unsafe fn intercom_from(source: intercom::raw::OutBSTR) -> ComResult<Self>
    {
        Ok(BString::from_ptr(source.0))
    }
}

impl IntercomFrom<intercom::raw::OutBSTR> for String
{
    unsafe fn intercom_from(source: intercom::raw::OutBSTR) -> ComResult<Self>
    {
        BString::from_ptr(source.0)
            .to_string()
            .map_err(|_| ComError::E_INVALIDARG)
    }
}

impl IntercomFrom<intercom::raw::OutBSTR> for CString
{
    unsafe fn intercom_from(source: intercom::raw::OutBSTR) -> ComResult<Self>
    {
        CString::new(String::intercom_from(source)?).map_err(|_| ComError::E_INVALIDARG)
    }
}

// *c_char -> X

impl IntercomFrom<*const c_char> for String
{
    unsafe fn intercom_from(source: *const c_char) -> ComResult<Self>
    {
        Ok(CStr::from_ptr(source)
            .to_str()
            .map_err(|_| ComError::E_INVALIDARG)?
            .to_string())
    }
}

/*
impl IntercomFrom<*mut c_char> for String {
    unsafe fn intercom_from( source: *mut c_char ) -> ComResult<String> {

        // TODO:
        // We really shouldn't blanket unsafe here.
        // The intercom_from should turn into an unsafe function instead.
        unsafe {

            // Convert the string. Maintain the result for now.
            let result = CStr::from_ptr( source )
                .to_str().map( |s| s.to_string() )
                .map_err( |_| ComError::E_INVALIDARG );

            // Free the buffer.
            ::alloc::free( source as *mut _ );

            result
        }
    }
}
*/

impl IntercomFrom<*const c_char> for BString
{
    unsafe fn intercom_from(source: *const c_char) -> ComResult<Self>
    {
        Ok(BString::from(
            CStr::from_ptr(source)
                .to_str()
                .map_err(|_| ComError::E_INVALIDARG)?,
        ))
    }
}

/*
impl IntercomFrom<*mut c_char> for BString {
    unsafe fn intercom_from( source: *mut c_char ) -> ComResult<Self> {
        unsafe {
            let bstring : ComResult<BString> =
                    ( source as *const c_char ).intercom_into();

            // Free the buffer.
            ::alloc::free( source as *mut _ );

            bstring
        }
    }
}
*/

impl<'a> IntercomFrom<*const c_char> for CString
{
    unsafe fn intercom_from(source: *const c_char) -> ComResult<Self>
    {
        Ok(CStr::from_ptr(source).into())
    }
}

/* We don't know where the *mut c_char is coming from so we shouldn't really
 * give it to CString given CString assumes it has reserved the memory on
 * its own.
 * */
impl IntercomFrom<*mut c_char> for CString
{
    unsafe fn intercom_from(source: *mut c_char) -> ComResult<CString>
    {
        Ok(CString::from_raw(source))
    }
}

impl<'a> IntercomFrom<*const c_char> for &'a CStr
{
    unsafe fn intercom_from(source: *const c_char) -> ComResult<Self>
    {
        Ok(CStr::from_ptr(source))
    }
}

impl<'a> IntercomFrom<&*const c_char> for &'a CStr
{
    unsafe fn intercom_from(source: &*const c_char) -> ComResult<Self>
    {
        Ok(CStr::from_ptr(*source))
    }
}

// X -> BSTR

impl IntercomFrom<&BStr> for crate::raw::InBSTR
{
    unsafe fn intercom_from(source: &BStr) -> ComResult<Self>
    {
        Ok(crate::raw::InBSTR(source.as_ptr()))
    }
}

impl IntercomFrom<&BString> for crate::raw::InBSTR
{
    unsafe fn intercom_from(source: &BString) -> ComResult<Self>
    {
        Ok(crate::raw::InBSTR(source.as_ptr()))
    }
}

impl IntercomFrom<BString> for crate::raw::OutBSTR
{
    unsafe fn intercom_from(source: BString) -> ComResult<Self>
    {
        Ok(crate::raw::OutBSTR(source.into_ptr()))
    }
}

impl IntercomFrom<BString> for crate::raw::InBSTR
{
    unsafe fn intercom_from(source: BString) -> ComResult<Self>
    {
        Ok(crate::raw::InBSTR(source.into_ptr()))
    }
}

impl IntercomFrom<CString> for crate::raw::OutBSTR
{
    unsafe fn intercom_from(source: CString) -> ComResult<Self>
    {
        let bstr: BString = source.intercom_into()?;
        Ok(crate::raw::OutBSTR(bstr.into_ptr()))
    }
}

// X -> *c_char

impl IntercomFrom<&CStr> for *const c_char
{
    unsafe fn intercom_from(source: &CStr) -> ComResult<Self>
    {
        Ok(source.as_ptr())
    }
}

impl IntercomFrom<&CString> for *const c_char
{
    unsafe fn intercom_from(source: &CString) -> ComResult<Self>
    {
        Ok(source.as_ptr())
    }
}

impl IntercomFrom<CString> for *const c_char
{
    unsafe fn intercom_from(source: CString) -> ComResult<Self>
    {
        // FIXME: These should be allocated with the intercom allocator.
        let ptr = source.as_ptr();
        std::mem::forget(source);
        Ok(ptr as _)
    }
}

impl IntercomFrom<CString> for *mut c_char
{
    unsafe fn intercom_from(source: CString) -> ComResult<Self>
    {
        // FIXME: These should be allocated with the intercom allocator.
        let ptr = source.as_ptr();
        std::mem::forget(source);
        Ok(ptr as _)
    }
}

impl IntercomFrom<BString> for *mut c_char
{
    unsafe fn intercom_from(source: BString) -> ComResult<Self>
    {
        let cstring: CString = source.intercom_into()?;
        cstring.intercom_into()
    }
}

// String -> X

impl IntercomFrom<String> for CString
{
    unsafe fn intercom_from(source: String) -> ComResult<Self>
    {
        CString::new(source).map_err(|_| ComError::E_INVALIDARG)
    }
}

impl IntercomFrom<&str> for CString
{
    unsafe fn intercom_from(source: &str) -> ComResult<Self>
    {
        CString::new(source).map_err(|_| ComError::E_INVALIDARG)
    }
}

impl IntercomFrom<String> for BString
{
    unsafe fn intercom_from(source: String) -> ComResult<Self>
    {
        Ok(BString::from(source.as_ref()))
    }
}

impl<'a> IntercomFrom<&'a str> for BString
{
    unsafe fn intercom_from(source: &str) -> ComResult<Self>
    {
        Ok(BString::from(source))
    }
}

// CString -> X

impl IntercomFrom<CString> for BString
{
    unsafe fn intercom_from(source: CString) -> ComResult<Self>
    {
        Ok(BString::from(
            source
                .to_str()
                .map_err(|_| ComError::E_INVALIDARG)?
                .to_string(),
        ))
    }
}

impl<'a> IntercomFrom<&'a CString> for &'a CStr
{
    unsafe fn intercom_from(source: &'a CString) -> ComResult<Self>
    {
        Ok(source.as_ref())
    }
}

// BString -> X

impl IntercomFrom<BString> for CString
{
    unsafe fn intercom_from(source: BString) -> ComResult<Self>
    {
        CString::new(source.to_string().map_err(|_| ComError::E_INVALIDARG)?)
            .map_err(|_| ComError::E_INVALIDARG)
    }
}

impl<'a> IntercomFrom<&'a BString> for &'a BStr
{
    unsafe fn intercom_from(source: &'a BString) -> ComResult<Self>
    {
        Ok(source.as_ref())
    }
}

impl IntercomFrom<&BStr> for CString
{
    unsafe fn intercom_from(source: &BStr) -> ComResult<Self>
    {
        CString::new(source.to_string().map_err(|_| ComError::E_INVALIDARG)?)
            .map_err(|_| ComError::E_INVALIDARG)
    }
}

impl IntercomFrom<&CStr> for BString
{
    unsafe fn intercom_from(source: &CStr) -> ComResult<Self>
    {
        Ok(BString::from(
            source
                .to_str()
                .map_err(|_| ComError::E_INVALIDARG)?
                .to_string(),
        ))
    }
}

impl<'a> IntercomFrom<&'a String> for &'a str
{
    unsafe fn intercom_from(source: &'a String) -> ComResult<Self>
    {
        Ok(source.as_ref())
    }
}

impl IntercomFrom<String> for *mut c_char
{
    unsafe fn intercom_from(source: String) -> ComResult<Self>
    {
        let bytes = source.as_bytes();

        // We just allocated the memory. This is safe.
        let buffer = crate::alloc::allocate(bytes.len() + 1) as *mut u8;
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer, bytes.len());
        *buffer.add(bytes.len()) = 0;

        Ok(buffer as *mut c_char)
    }
}

impl IntercomFrom<String> for intercom::raw::OutBSTR
{
    unsafe fn intercom_from(source: String) -> ComResult<Self>
    {
        Ok(intercom::raw::OutBSTR(BString::from(source).into_ptr()))
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn can_construct_bstring()
    {
        let bstrs: Vec<BString> = vec!["foo".into(), "foo".to_string().into()];

        for bstr in bstrs {
            assert_eq!(bstr.len_bytes(), 6);
            assert_eq!(bstr.len(), 3);

            let ptr = bstr.as_ptr();
            unsafe {
                assert_eq!(*(ptr.offset(-2) as *const u32), 6);
                assert_eq!(*(ptr.offset(0)), 102u16);
                assert_eq!(*(ptr.offset(1)), 111u16);
                assert_eq!(*(ptr.offset(2)), 111u16);
                assert_eq!(*(ptr.offset(3)), 0);
            }
        }
    }

    #[test]
    fn can_construct_bstr()
    {
        let bstring: BString = "foo".into();
        let bstr_data = [6u16, 0u16, 102u16, 111u16, 111u16, 0u16];

        let bstrs: Vec<&BStr> = vec![bstring.as_ref(), unsafe {
            BStr::from_ptr(bstr_data.as_ptr().offset(2))
        }];

        for bstr in bstrs {
            assert_eq!(bstr.len_bytes(), 6);
            assert_eq!(bstr.len(), 3);

            let ptr = bstr.as_ptr();
            unsafe {
                assert_eq!(*(ptr.offset(-2) as *const u32), 6);
                assert_eq!(*(ptr.offset(0)), 102u16);
                assert_eq!(*(ptr.offset(1)), 111u16);
                assert_eq!(*(ptr.offset(2)), 111u16);
                assert_eq!(*(ptr.offset(3)), 0);
            }
        }
    }

    #[test]
    fn bstr_eq()
    {
        let bstr_data = [6u16, 0u16, 102u16, 111u16, 111u16, 0u16];
        let bstr = unsafe { BStr::from_ptr(bstr_data.as_ptr().offset(2)) };

        let bstring_foo: BString = "foo".into();
        assert_eq!(bstr, &*bstring_foo);

        let bstring_bar: BString = "bar".into();
        assert_ne!(bstr, &*bstring_bar);
    }

    #[test]
    fn bstring_eq()
    {
        let bstring_foo1: BString = "foo".into();
        let bstring_foo2: BString = "foo".into();
        assert_eq!(bstring_foo1, bstring_foo2);

        let bstring_bar: BString = "bar".into();
        assert_ne!(bstring_foo1, bstring_bar);
    }
}
