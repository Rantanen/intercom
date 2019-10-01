//!
//! Intercom string representations.
//!

use std::{
    self, ffi,
    ops::Deref,
    borrow::Borrow,
    str::{ FromStr, Utf8Error },
    os::raw::c_char,
};

use crate::intercom::{ComError, ComResult};
use type_system::{ExternType, AutomationTypeSystem, RawTypeSystem, IntercomFrom, IntercomInto};

#[derive(Debug)]
pub struct FormatError;

/// Represents a borrowed BSTR string.
#[repr(C)]
#[derive(PartialEq)]
pub struct BStr(
    // Invariant 1: .0.as_ptr() must be a valid BSTR pointer _or_ 0x1 if len == 0.
    //              This includes having u32 alignment.
    // Invariant 2: .0.len() must fit to u32.
    [u8]
);

impl std::fmt::Debug for BStr {
    fn fmt( &self, f : &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( f, "BStr(\"{}\")",
                String::from_utf16_lossy( unsafe { std::slice::from_raw_parts(
                        self.as_ptr() as *const u16,
                        self.len() as usize / 2 ) } ) )
    }
}

impl BStr {

    /// Unsafely creates a `BStr` from a BSTR pointer.
    ///
    /// This function will cast the pointer into a `BStr`. The provied pointer
    /// **must** be a valid BSTR pointer and must be valid while the BStr is
    /// alive. The BStr must also not be moved.
    pub unsafe fn from_ptr<'a>( ptr : *const u16 ) -> &'a BStr {

        // The BStr invariant 1 states the ptr must be valid BSTR pointer,
        // which is u32-aligned.
        #![allow(clippy::cast_ptr_alignment)]
        let ( len, final_ptr ) = match ptr as usize {
            0 => ( 0, 1 as *const u8 ),
            _ => ( *( ptr.offset( -2 ) as *const u32 ), ptr as *const u8 ),
        };

        let slice = std::slice::from_raw_parts( final_ptr, len as usize );
        Self::from_slice_unchecked( slice )
    }

    /// Unsafely creates a `BStr` from a slice.
    ///
    /// This function will cast the slice into a `BStr`. The slice **must**
    /// be a slice constructed from a valid BSTR pointer. Specifically the slice
    /// as_ptr() must result in a valid BSTR pointer.
    unsafe fn from_slice_unchecked( slice : &[u8] ) -> &BStr {
        &*( slice as *const [u8] as *const BStr )
    }

    /// Returns the pointer as a 16-bit wide character pointer.
    pub fn as_ptr( &self ) -> *const u16 {

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
    pub fn len_bytes( &self ) -> u32 {

        // The len() on the slice is stored separately and can be used even
        // if the buffer itself points to an invalid value as is the case with
        // some 0-length BSTRs.
        self.0.len() as u32
    }

    /// Returns the string length in characters.
    pub fn len( &self ) -> u32 {

        // As long as the BStr is valie this is safe.
        unsafe {
            os::SysStringLen( self.as_ptr() )
        }
    }

    pub fn is_empty( &self ) -> bool {
        self.len_bytes() == 0
    }

    /// Gets the BStr as a slice of 16-bit characters.
    pub fn as_slice( &self ) -> &[u8] {
        &self.0
    }

    pub fn to_string( &self ) -> Result<String, FormatError> {
        match self.len_bytes() {
            x if x % 2 == 0 =>
                String::from_utf16( unsafe { std::slice::from_raw_parts(
                        self.as_ptr() as *const u16,
                        x as usize / 2 ) } )
                    .map_err( |_| FormatError ),
            _ => Err( FormatError ),
        }
    }
}

#[repr(C)]
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
    *mut u16
);

impl std::fmt::Debug for BString {
    fn fmt( &self, f : &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( f, "BStr(\"{}\")",
                String::from_utf16_lossy( unsafe { std::slice::from_raw_parts(
                        self.as_ptr() as *const u16,
                        self.len_bytes() as usize / 2 ) } ) )
    }
}

impl PartialEq for BString {
    fn eq( &self, other : &Self ) -> bool {

        // Deref into &BStr and compare those.
        **self == **other
    }
}

impl Clone for BString {
    fn clone( &self ) -> BString {
        self.as_ref().to_owned()
    }
}

impl BString {

    pub unsafe fn from_ptr( ptr : *mut u16 ) -> BString {
        BString( ptr )
    }

    /// Converts a C-string into a `BString`.
    pub fn from_cstr( s : &ffi::CStr ) -> Result<BString, Utf8Error> {
        Ok( Self::from_str( s.to_str()? ).expect( "Error type is never type" ) )
    }

    /// Returns the pointer as a 16-bit wide character pointer.
    pub fn as_mut_ptr( &mut self ) -> *mut u16 {
        self.0 as *mut u16
    }

    /// Converts the `BString` into a raw pointer.
    pub fn into_ptr( self ) -> *mut u16 {
        let ptr = self.0;
        std::mem::forget( self );
        ptr as *mut u16
    }
}

impl FromStr for BString {
    type Err = std::string::ParseError;

    /// Converts a Rust string into a `BString`.
    fn from_str( s : &str ) -> Result<BString, Self::Err> {

        // Avoid unnecessary allocations when the string is empty.
        // Null and empty BSTRs should be treated as equal.
        // See https://blogs.msdn.microsoft.com/ericlippert/2003/09/12/erics-complete-guide-to-bstr-semantics/
        if s.is_empty() { return Ok( BString( std::ptr::null_mut() ) ); }

        unsafe {

            let chars = s.encode_utf16().collect::<Vec<_>>();
            let bstr = os::SysAllocStringLen( chars.as_ptr(), chars.len() as u32 );

            // Memory issues are traditionally fatal in Rust and do not cause
            // Err-results.
            if bstr.0.is_null() { panic!( "Allocating BStr failed." ); }

            Ok( bstr )
        }
    }
}

impl Deref for BString {
    type Target = BStr;
    fn deref( &self ) -> &BStr {
        unsafe { BStr::from_ptr( self.0 ) }
    }
}

// AsRef/Borrow/ToOwned implementations.

impl AsRef<BStr> for BStr {
    fn as_ref( &self ) -> &BStr {
        self
    }
}

impl AsRef<BStr> for BString {
    fn as_ref( &self ) -> &BStr {
        self
    }
}

impl Borrow<BStr> for BString {
    fn borrow( &self ) -> &BStr {
        self
    }
}

impl ToOwned for BStr {
    type Owned = BString;

    fn to_owned( &self ) -> Self::Owned {
        unsafe {
            os::SysAllocStringLen(
                    self.as_ptr(),
                    os::SysStringLen( self.as_ptr() ) )
        }
    }
}

impl<'a> From<&'a str> for BString {
    fn from( source : &str ) -> BString {
        BString::from_str( source ).expect( "Error type is never type" )
    }
}

impl From<String> for BString {
    fn from( source : String ) -> BString {
        BString::from_str( &source ).expect( "Error type is never type" )
    }
}

impl Default for BString {
    fn default() -> BString { BString( std::ptr::null_mut() ) }
}

impl Drop for BString {
    fn drop( &mut self ) {
        unsafe {
            os::SysFreeString( self.as_mut_ptr() );
            self.0 = std::ptr::null_mut();
        }
    }
}

pub trait FromWithTemporary<'a, TSource>
    where Self: Sized
{
    type Temporary;
    fn to_temporary( source : TSource ) -> Result<Self::Temporary, ComError>;
    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError>;
}

impl<'a, T: Copy> FromWithTemporary<'a, T> for T {

    type Temporary = T;

    fn to_temporary( source : T ) -> Result<Self::Temporary, ComError> { Ok(source) }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( *temp )
    }
}

impl<'a> FromWithTemporary<'a, &'a BStr >
        for BString {

    type Temporary = &'a BStr;

    fn to_temporary( bstr : &'a BStr ) -> Result<Self::Temporary, ComError> { Ok(bstr) }
    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( (*temp).to_owned() )
    }
}

impl<'a> FromWithTemporary<'a, &'a BStr>
        for &'a str {

    type Temporary = String;

    fn to_temporary( bstr : &'a BStr ) -> Result<Self::Temporary, ComError> {
        bstr.to_string().map_err( |_| ComError::E_INVALIDARG )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, &'a BStr>
        for String {

    type Temporary = &'a BStr;

    fn to_temporary( bstr : &'a BStr ) -> Result<Self::Temporary, ComError> {
        Ok( bstr )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        temp.to_string().map_err( |_| ComError::E_INVALIDARG )
    }
}

impl<'a> FromWithTemporary<'a, BString >
        for &'a BStr {

    type Temporary = BString;

    fn to_temporary( source : BString ) -> Result<Self::Temporary, ComError> {
        Ok( source )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, CString >
        for &'a BStr {

    type Temporary = BString;

    fn to_temporary( source : CString ) -> Result<Self::Temporary, ComError> {
        source.com_into()
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, &'a str>
        for &'a BStr {

    type Temporary = BString;

    fn to_temporary( source : &'a str ) -> Result<Self::Temporary, ComError> {
        BString::from_str( source ).map_err( |_| ComError::E_INVALIDARG )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, String>
        for &'a BStr {

    type Temporary = BString;

    fn to_temporary( source : String ) -> Result<Self::Temporary, ComError> {
        BString::from_str( &source ).map_err( |_| ComError::E_INVALIDARG )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, &'a CStr>
        for &'a BStr {

    type Temporary = BString;

    fn to_temporary( source : &'a CStr ) -> Result<Self::Temporary, ComError> {
        source.to_str()
            .map( |s| s.into() )
            .map_err( |_| ComError::E_INVALIDARG )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, &'a BStr>
        for &'a CStr {

    type Temporary = CString;

    fn to_temporary( source : &'a BStr ) -> Result<Self::Temporary, ComError> {
        let string = source.to_string()
                .map_err( |_| ComError::E_INVALIDARG )?;

        CString::new( string )
                .map_err( |_| ComError::E_INVALIDARG )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, &'a BStr>
        for CString {

    type Temporary = &'a BStr;

    fn to_temporary( source : &'a BStr ) -> Result<Self::Temporary, ComError> {
        Ok( source )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        let string = temp.to_string()
                .map_err( |_| ComError::E_INVALIDARG )?;

        CString::new( string )
                .map_err( |_| ComError::E_INVALIDARG )
    }
}

pub trait ComFrom<TSource> : Sized {
    fn com_from( source : TSource ) -> Result<Self, ComError>;
}

pub trait ComInto<TTarget> {
    fn com_into( self ) -> Result<TTarget, ComError>;
}

impl<TTarget, TSource> ComInto<TTarget> for TSource
    where TTarget : ComFrom< TSource > {

    fn com_into( self ) -> Result<TTarget, ComError> {
        TTarget::com_from( self )
    }
}

impl<T> ComFrom<T> for T {
    fn com_from( source : T ) -> Result<T, ComError> {
        Ok( source )
    }
}

impl ComFrom<BString> for String {
    fn com_from( source : BString ) -> Result<Self, ComError> {
        let mut bstr : &BStr = &source;
        < String as FromWithTemporary<&BStr> >::from_temporary( &mut bstr )
    }
}

impl ComFrom<String> for BString {
    fn com_from( source : String ) -> Result<Self, ComError> {
        Ok( BString::from( source ) )
    }
}

impl ComFrom<CString> for String {
    fn com_from( source : CString ) -> Result<Self, ComError> {
        source.into_string()
                .map_err( |_| ComError::E_INVALIDARG )
    }
}

impl ComFrom<String> for CString {
    fn com_from( source: String ) -> Result<Self, ComError> {
        CString::new( source )
                .map_err( |_| ComError::E_INVALIDARG )
    }
}

impl ComFrom<CString> for BString {
    fn com_from( source : CString ) -> Result<Self, ComError> {
        source.to_str()
                .map( BString::from )
                .map_err( |_| ComError::E_INVALIDARG )
    }
}

impl ComFrom<BString> for CString {
    fn com_from( source : BString ) -> Result<Self, ComError> {
        let string = source.to_string()
                .map_err( |_| ComError::E_INVALIDARG )?;

        CString::new( string )
                .map_err( |_| ComError::E_INVALIDARG )
    }
}

pub type CStr = std::ffi::CStr;
pub type CString = std::ffi::CString;

impl<'a> FromWithTemporary<'a, &'a CStr >
        for CString {

    type Temporary = &'a CStr;

    fn to_temporary( cstr : &'a CStr ) -> Result<Self::Temporary, ComError> { Ok(cstr) }
    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( (*temp).to_owned() )
    }
}

impl<'a> FromWithTemporary<'a, &'a CStr>
        for &'a str {

    type Temporary = String;

    fn to_temporary( cstr : &'a CStr ) -> Result<Self::Temporary, ComError> {
        cstr.to_str()
            .map( ToString::to_string )
            .map_err( |_| ComError::E_INVALIDARG )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, &'a CStr>
        for String {

    type Temporary = &'a CStr;

    fn to_temporary( cstr : &'a CStr ) -> Result<Self::Temporary, ComError> {
        Ok( cstr )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        temp.to_str()
            .map( ToString::to_string )
            .map_err( |_| ComError::E_INVALIDARG )
    }
}

impl<'a> FromWithTemporary<'a, CString >
        for &'a CStr {

    type Temporary = CString;

    fn to_temporary( source : CString ) -> Result<Self::Temporary, ComError> {
        Ok( source )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, BString >
        for &'a CStr {

    type Temporary = CString;

    fn to_temporary( source : BString ) -> Result<Self::Temporary, ComError> {
        source.com_into()
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, &'a str>
        for &'a CStr {

    type Temporary = CString;

    fn to_temporary( source : &'a str ) -> Result<Self::Temporary, ComError> {
        CString::new( source ).map_err( |_| ComError::E_INVALIDARG )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, String>
        for &'a CStr {

    type Temporary = CString;

    fn to_temporary( source : String ) -> Result<Self::Temporary, ComError> {
        CString::new( source ).map_err( |_| ComError::E_INVALIDARG )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        Ok( &**temp )
    }
}

impl<'a> FromWithTemporary<'a, &'a CStr>
        for BString {

    type Temporary = &'a CStr;

    fn to_temporary( source : &'a CStr ) -> Result<Self::Temporary, ComError> {
        Ok( source )
    }

    fn from_temporary( temp : &'a mut Self::Temporary ) -> Result<Self, ComError> {
        temp.to_str()
            .map( |s| s.into() )
            .map_err( |_| ComError::E_INVALIDARG )
    }
}

//////////////////////////////////////////
// OS specific string allocation.

#[cfg(windows)]
mod os {
    use super::*;

    #[link(name = "oleaut32")]
    extern "system" {
        #[doc(hidden)]
        pub fn SysAllocStringLen(
            psz: *const u16,
            len: u32
        ) -> BString;

        #[doc(hidden)]
        pub fn SysFreeString(
            bstr: *mut u16
        );

        #[doc(hidden)]
        pub fn SysStringLen(
            pbstr: *const u16,
        ) -> u32;
    }
}

#[cfg(not(windows))]
#[allow(non_snake_case)]
mod os {
    use super::*;
    use libc;

    #[doc(hidden)]
    pub unsafe fn SysAllocStringLen(
        psz: *const u16,
        len: u32
    ) -> BString
    {
        // Match the SysAllocStringLen implementation on Windows when
        // psz is null.
        if psz.is_null() {
            return BString( std::ptr::null_mut() );
        }

        // Length prefix + data length + null-terminator.
        // The length of BSTR is expressed as bytes in the prefix.
        let data_length = ( len * 2 ) as usize;
        let buffer_length: usize = 4 + data_length + 2;
        let buffer = libc::malloc( buffer_length );
        if buffer.is_null() {
            return BString( std::ptr::null_mut() );
        }

        // Set the length prefix.
        let length_u32 = data_length as u32;
        let length_prefix = &length_u32 as *const _ as *const libc::c_void;
        libc::memcpy( buffer, length_prefix, 4 );

        // The actual data.
        let src_buffer = psz as *const u8 as *mut libc::c_void;
        libc::memcpy( buffer.offset( 4 ), src_buffer, data_length as usize );

        let null_terminator: u16 = 0;
        let null_terminator = &null_terminator as *const _ as *const libc::c_void;
        libc::memcpy( buffer.offset( 4 + data_length as isize ), null_terminator, 2 );

        let buffer = buffer.offset( 4 ) as *mut u16;
        BString( buffer )
    }

    #[doc(hidden)]
    pub unsafe fn SysFreeString(
        pbstr: *mut u16
    ) {
        if ! pbstr.is_null() {
            libc::free( pbstr.offset( -2 ) as *mut libc::c_void );
        }
    }

    #[doc(hidden)]
    pub unsafe fn SysStringLen(
        pbstr: *const u16,
    ) -> u32 {

        // The BSTR pointers should be u32-aligned.
        #![allow(clippy::cast_ptr_alignment)]
        if pbstr.is_null() {
            0
        } else {
            *( pbstr.offset( -2 ) as *const u32 ) / 2
        }
    }
}

#[derive(Debug, Clone)]
pub enum IntercomString {
    BString( BString ),
    CString( CString ),
    String( String ),
}

impl From<BString> for IntercomString {
    fn from( source : BString ) -> Self {
        IntercomString::BString( source )
    }
}

impl From<String> for IntercomString {
    fn from( source : String ) -> Self {
        IntercomString::String( source )
    }
}

impl From<CString> for IntercomString {
    fn from( source : CString ) -> Self {
        IntercomString::CString( source )
    }
}

impl ComFrom<IntercomString> for BString {
    fn com_from( source : IntercomString ) -> Result<Self, ComError> {
        match source {
            IntercomString::BString( bstring ) => bstring.com_into(),
            IntercomString::CString( cstring ) => cstring.com_into(),
            IntercomString::String( string ) => string.com_into()
        }
    }
}

impl ComFrom<IntercomString> for CString {
    fn com_from( source : IntercomString ) -> Result<Self, ComError> {
        match source {
            IntercomString::BString( bstring ) => bstring.com_into(),
            IntercomString::CString( cstring ) => cstring.com_into(),
            IntercomString::String( string ) => string.com_into()
        }
    }
}

impl ComFrom<IntercomString> for String {
    fn com_from( source : IntercomString ) -> Result<Self, ComError> {
        match source {
            IntercomString::BString( bstring ) => bstring.com_into(),
            IntercomString::CString( cstring ) => cstring.com_into(),
            IntercomString::String( string ) => string.com_into()
        }
    }
}

// Automation type system.

impl ExternType<AutomationTypeSystem> for &str {
    type ExternInputType = ::raw::InBSTR;
    type ExternOutputType = ::raw::OutBSTR;
    type OwnedExternType = BString;
    type OwnedNativeType = String;
}

impl ExternType<AutomationTypeSystem> for String {
    type ExternInputType = ::raw::InBSTR;
    type ExternOutputType = ::raw::OutBSTR;
    type OwnedExternType = BString;
    type OwnedNativeType = String;
}

impl ExternType<AutomationTypeSystem> for &BStr {
    type ExternInputType = ::raw::InBSTR;
    type ExternOutputType = ::raw::OutBSTR;
    type OwnedExternType = ::raw::InBSTR;
    type OwnedNativeType = BString;
}

impl ExternType<AutomationTypeSystem> for BString {
    type ExternInputType = ::raw::InBSTR;
    type ExternOutputType = ::raw::OutBSTR;
    type OwnedExternType = BString;
    type OwnedNativeType = BString;
}

impl ExternType<AutomationTypeSystem> for &CStr {
    type ExternInputType = ::raw::InBSTR;
    type ExternOutputType = ::raw::OutBSTR;
    type OwnedExternType = BString;
    type OwnedNativeType = CString;
}

impl ExternType<AutomationTypeSystem> for CString {
    type ExternInputType = ::raw::InBSTR;
    type ExternOutputType = ::raw::OutBSTR;
    type OwnedExternType = BString;
    type OwnedNativeType = CString;
}

// Raw type system.

impl ExternType<RawTypeSystem> for &str {
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = CString;
    type OwnedNativeType = String;
}

impl ExternType<RawTypeSystem> for String {
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = CString;
    type OwnedNativeType = String;
}

impl ExternType<RawTypeSystem> for &BStr {
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = CString;
    type OwnedNativeType = BString;
}

impl ExternType<RawTypeSystem> for BString {
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = CString;
    type OwnedNativeType = BString;
}

impl ExternType<RawTypeSystem> for &CStr {
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = *const c_char;
    type OwnedNativeType = CString;
}

impl ExternType<RawTypeSystem> for CString {
    type ExternInputType = *const c_char;
    type ExternOutputType = *mut c_char;
    type OwnedExternType = CString;
    type OwnedNativeType = CString;
}

// InBSTR -> X

impl IntercomFrom<::raw::InBSTR> for String {
    fn intercom_from( source: ::raw::InBSTR ) -> ComResult<Self> {
        unsafe {
            Ok( BStr::from_ptr( source )
                    .to_string()
                    .map_err( |_| ComError::E_INVALIDARG )? )
        }
    }
}

impl IntercomFrom<::raw::InBSTR> for BString {
    fn intercom_from( source: ::raw::InBSTR ) -> ComResult<Self> {
        unsafe { Ok( BStr::from_ptr( source ).to_owned() ) }
    }
}

impl<'a> IntercomFrom<::raw::InBSTR> for &'a BStr {
    fn intercom_from( source: ::raw::InBSTR ) -> ComResult<Self> {
        unsafe { Ok( BStr::from_ptr( source ) ) }
    }
}

impl IntercomFrom<::raw::InBSTR> for CString {
    fn intercom_from( source: ::raw::InBSTR ) -> ComResult<Self> {
        unsafe {
            CString::new(
                    BStr::from_ptr( source ).to_string()
                        .map_err( |_| ComError::E_INVALIDARG )?
                ).map_err( |_| ComError::E_INVALIDARG )
        }
    }
}

impl<TPtr, TTarget> IntercomFrom<*mut TPtr> for TTarget
        where TTarget: IntercomFrom<*const TPtr>
{
    fn intercom_from( source: *mut TPtr ) -> ComResult<Self> {
        let bstring : ComResult<TTarget> =
                ( source as *const TPtr ).intercom_into();

        // Free the buffer.
        unsafe { ::alloc::free( source as *mut _ ); }

        bstring
    }
}

// *c_char -> X

impl IntercomFrom<*const c_char> for String {
    fn intercom_from( source: *const c_char ) -> ComResult<Self> {
        unsafe {
            Ok( CStr::from_ptr( source )
                    .to_str()
                    .map_err( |_| ComError::E_INVALIDARG )?
                    .to_string() )
        }
    }
}

/*
impl IntercomFrom<*mut c_char> for String {
    fn intercom_from( source: *mut c_char ) -> ComResult<String> {

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

impl IntercomFrom<*const c_char> for BString {
    fn intercom_from( source: *const c_char ) -> ComResult<Self> {
        unsafe {
            Ok( BString::from(
                CStr::from_ptr( source )
                    .to_str()
                    .map_err( |_| ComError::E_INVALIDARG )?
            ) )
        }
    }
}

/*
impl IntercomFrom<*mut c_char> for BString {
    fn intercom_from( source: *mut c_char ) -> ComResult<Self> {
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

impl<'a> IntercomFrom<*const c_char> for CString {
    fn intercom_from( source: *const c_char ) -> ComResult<Self> {
        unsafe { Ok( CStr::from_ptr( source ).into() ) }
    }
}

/*
impl IntercomFrom<*mut c_char> for CString {
    fn intercom_from( source: *mut c_char ) -> ComResult<CString> {

        // TODO:
        // We really shouldn't blanket unsafe here.
        // The intercom_from should turn into an unsafe function instead.
        unsafe {

            // Convert the string. Maintain the result for now.
            let cstring : ComResult<CString> =
                    ( source as *const c_char ).intercom_into();

            // Free the buffer.
            ::alloc::free( source as *mut _ );

            cstring
        }
    }
}
*/

impl<'a> IntercomFrom<*const c_char> for &'a CStr {
    fn intercom_from( source: *const c_char ) -> ComResult<Self> {
        unsafe { Ok( CStr::from_ptr( source ) ) }
    }
}

// X -> BSTR

impl IntercomFrom<&BStr> for ::raw::InBSTR {
    fn intercom_from( source: &BStr ) -> ComResult<Self> {
        Ok( source.as_ptr() )
    }
}

impl IntercomFrom<&BString> for ::raw::InBSTR {
    fn intercom_from( source: &BString ) -> ComResult<Self> {
        Ok( source.as_ptr() )
    }
}

impl IntercomFrom<BString> for ::raw::OutBSTR {
    fn intercom_from( source: BString ) -> ComResult<Self> {
        Ok( source.into_ptr() )
    }
}

impl IntercomFrom<CString> for ::raw::OutBSTR {
    fn intercom_from( source: CString ) -> ComResult<Self> {
        let bstr : BString = source.intercom_into()?;
        Ok( bstr.into_ptr() )
    }
}

// X -> *c_char

impl IntercomFrom<&CStr> for *const c_char {
    fn intercom_from( source: &CStr ) -> ComResult<Self> {
        Ok( source.as_ptr() )
    }
}

impl IntercomFrom<&CString> for *const c_char {
    fn intercom_from( source: &CString ) -> ComResult<Self> {
        Ok( source.as_ptr() )
    }
}

impl IntercomFrom<CString> for *mut c_char {
    fn intercom_from( source: CString ) -> ComResult<Self> {
        let bytes = source.as_bytes();

        // We just allocated the memory. This is safe.
        unsafe {
            let buffer = crate::alloc::allocate( bytes.len() + 1 ) as *mut u8;
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                buffer,
                bytes.len() );
            *buffer.offset( ( bytes.len() + 1 ) as isize ) = 0;

            Ok( buffer as *mut c_char )
        }
    }
}

impl IntercomFrom<BString> for *mut c_char {
    fn intercom_from( source: BString ) -> ComResult<Self> {
        let cstring : CString = source.intercom_into()?;
        cstring.intercom_into()
    }
}

// String -> X

impl IntercomFrom<String> for CString {
    fn intercom_from( source: String ) -> ComResult<Self> {
        CString::new( source )
                .map_err( |_| ComError::E_INVALIDARG )
    }
}

impl IntercomFrom<&str> for CString {
    fn intercom_from( source: &str ) -> ComResult<Self> {
        CString::new( source )
                .map_err( |_| ComError::E_INVALIDARG )
    }
}

impl IntercomFrom<String> for BString {
    fn intercom_from( source : String ) -> ComResult<Self> {
        Ok( BString::from( source.as_ref() ) )
    }
}

impl<'a> IntercomFrom<&'a str> for BString {
    fn intercom_from( source : &str ) -> ComResult<Self> {
        Ok( BString::from( source ) )
    }
}

// CString -> X

impl IntercomFrom<CString> for BString {
    fn intercom_from( source: CString ) -> ComResult<Self> {
        Ok( BString::from(
            source.to_str()
                .map_err( |_| ComError::E_INVALIDARG )?
                .to_string() ) )
    }
}

impl<'a> IntercomFrom<&'a CString> for &'a CStr {
    fn intercom_from( source: &'a CString ) -> ComResult<Self> {
        Ok( source.as_ref() )
    }
}

// BString -> X

impl IntercomFrom<BString> for CString {
    fn intercom_from( source: BString ) -> ComResult<Self> {
        CString::new( source.to_string().map_err( |_| ComError::E_INVALIDARG )? )
                .map_err( |_| ComError::E_INVALIDARG )
    }
}

impl<'a> IntercomFrom<&'a BString> for &'a BStr {
    fn intercom_from( source: &'a BString ) -> ComResult<Self> {
        Ok( source.as_ref() )
    }
}

impl IntercomFrom<&BStr> for CString {
    fn intercom_from( source: &BStr ) -> ComResult<Self> {
        CString::new( source.to_string().map_err( |_| ComError::E_INVALIDARG )? )
                .map_err( |_| ComError::E_INVALIDARG )
    }
}

impl IntercomFrom<&CStr> for BString {
    fn intercom_from( source: &CStr ) -> ComResult<Self> {
        Ok( BString::from(
            source.to_str()
                .map_err( |_| ComError::E_INVALIDARG )?
                .to_string() ) )
    }
}

impl<'a> IntercomFrom<&'a String> for &'a str {
    fn intercom_from( source: &'a String ) -> ComResult<Self> {
        Ok( source.as_ref() )
    }
}

impl<'a> IntercomInto<&'a str> for &'a String {
    fn intercom_into( self ) -> ComResult<&'a str> {
        Ok( self.as_ref() )
    }
}

impl IntercomFrom<String> for *mut c_char {
    fn intercom_from( source: String ) -> ComResult<Self> {
        let bytes = source.as_bytes();

        // We just allocated the memory. This is safe.
        unsafe {
            let buffer = crate::alloc::allocate( bytes.len() + 1 ) as *mut u8;
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                buffer,
                bytes.len() );
            *buffer.offset( ( bytes.len() + 1 ) as isize ) = 0;

            Ok( buffer as *mut c_char )
        }
    }
}

impl IntercomFrom<String> for *mut u16 {
    fn intercom_from( source: String ) -> ComResult<Self> {
        Ok( BString::from( source ).into_ptr() )
    }
}

/*
impl IntercomFrom<::raw::OutBSTR> for String {
    fn intercom_from( source: ::raw::OutBSTR ) -> ComResult<Self> {

        // TODO:
        // We really shouldn't blanket unsafe here.
        // The intercom_from should turn into an unsafe function instead.
        unsafe {
            BString::from_ptr( source )
                .to_string()
                .map_err( |_| ComError::E_INVALIDARG )
        }
    }
}
*/


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_construct_bstring() {

        let bstrs : Vec<BString> = vec![
            "foo".into(),
            "foo".to_string().into()
        ];

        for bstr in bstrs {

            assert_eq!( bstr.len_bytes(), 6 );
            assert_eq!( bstr.len(), 3 );

            let ptr = bstr.as_ptr();
            unsafe {
                assert_eq!( *( ptr.offset( -2 ) as *const u32 ), 6 );
                assert_eq!( *( ptr.offset( 0 ) ), 102u16 );
                assert_eq!( *( ptr.offset( 1 ) ), 111u16 );
                assert_eq!( *( ptr.offset( 2 ) ), 111u16 );
                assert_eq!( *( ptr.offset( 3 ) ), 0 );
            }
        }
    }

    #[test]
    fn can_construct_bstr() {

        let bstring : BString = "foo".into();
        let bstr_data = [ 6u16, 0u16, 102u16, 111u16, 111u16, 0u16 ];

        let bstrs : Vec<&BStr> = vec![
            bstring.as_ref(),
            unsafe {
                BStr::from_ptr( bstr_data.as_ptr().offset( 2 ) )
            },
        ];

        for bstr in bstrs {

            assert_eq!( bstr.len_bytes(), 6 );
            assert_eq!( bstr.len(), 3 );

            let ptr = bstr.as_ptr();
            unsafe {
                assert_eq!( *( ptr.offset( -2 ) as *const u32 ), 6 );
                assert_eq!( *( ptr.offset( 0 ) ), 102u16 );
                assert_eq!( *( ptr.offset( 1 ) ), 111u16 );
                assert_eq!( *( ptr.offset( 2 ) ), 111u16 );
                assert_eq!( *( ptr.offset( 3 ) ), 0 );
            }
        }
    }

    #[test]
    fn bstr_eq() {

        let bstr_data = [ 6u16, 0u16, 102u16, 111u16, 111u16, 0u16 ];
        let bstr = unsafe { BStr::from_ptr( bstr_data.as_ptr().offset( 2 ) ) };

        let bstring_foo : BString = "foo".into();
        assert_eq!( bstr, &*bstring_foo );

        let bstring_bar : BString = "bar".into();
        assert_ne!( bstr, &*bstring_bar );
    }

    #[test]
    fn bstring_eq() {

        let bstring_foo1 : BString = "foo".into();
        let bstring_foo2 : BString = "foo".into();
        assert_eq!( bstring_foo1, bstring_foo2 );

        let bstring_bar : BString = "bar".into();
        assert_ne!( bstring_foo1, bstring_bar );
    }
}
