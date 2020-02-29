use super::*;
use intercom::raw::OutBSTR;
use std::os::raw;

/// A memory allocator to be used for allocating/deallocating memory shared
/// with intercom libraries.
#[crate::com_class(IAllocator)]
#[derive(Default)]
pub struct Allocator;

#[crate::com_interface(
    com_iid = "18EE22B3-B0C6-44A5-A94A-7A417676FB66",
    raw_iid = "7A6F6564-04B5-4455-A223-EA0512B8CC63"
)]
pub trait IAllocator: crate::IUnknown
{
    /// Allocates a new BSTR based on an existing string value.
    ///
    /// # Arguments
    ///
    /// - `psz` - A pointer to an existing wide character (16-bit) string.
    /// - `len` - String length.
    ///
    /// # Safety
    ///
    /// The function is safe to call as long as the `psz` is valid in relation
    /// to the given `len`. The returned value must be freed using BSTR aware
    /// free function, such as the `free_bstr` in this interface or the
    /// `SysFreeString` function on Windows.
    unsafe fn alloc_bstr(&self, text: *const u16, len: u32) -> OutBSTR;

    /// Frees a BSTR value.
    ///
    /// # Arguments
    ///
    /// - `bstr` - Previously allocated BSTR value.
    ///
    /// # Safety
    ///
    /// The function is safe as long as the `bstr` is a valid BSTR value.
    unsafe fn free_bstr(&self, bstr: OutBSTR);

    /// Allocates a segment of memory that is safe to pass through intercom
    /// interfaces.
    ///
    /// # Arguments
    ///
    /// - `len` - Size of data to allocate.
    ///
    /// # Safety
    ///
    /// The returned value must be freed using the `free` method in this
    /// interface or the intercom `alloc::free` function.
    unsafe fn alloc(&self, len: usize) -> *mut raw::c_void;

    /// Frees a segment of memory received through intercom interfaces.
    ///
    /// # Arguments
    ///
    /// - `ptr` - Memory to free.
    ///
    /// # Safety
    ///
    /// The memory must have been allocated using the `alloc` method in this
    /// interface or the intercom `alloc::allocate` function.
    unsafe fn free(&self, ptr: *mut raw::c_void);
}

impl IAllocator for Allocator
{
    unsafe fn alloc_bstr(&self, text: *const u16, len: u32) -> OutBSTR
    {
        OutBSTR(os::alloc_bstr(text, len))
    }

    unsafe fn free_bstr(&self, bstr: OutBSTR)
    {
        os::free_bstr(bstr.0)
    }

    unsafe fn alloc(&self, len: usize) -> *mut raw::c_void
    {
        os::alloc(len)
    }

    unsafe fn free(&self, ptr: *mut raw::c_void)
    {
        os::free(ptr)
    }
}

/// Allocates a segment of memory that is safe to pass through intercom
/// interfaces.
///
/// # Arguments
///
/// - `len` - Size of data to allocate.
///
/// # Safety
///
/// The returned value must be freed using the `free` method in this
/// module or the `IAllocator::free` method.
pub unsafe fn allocate(len: usize) -> *mut raw::c_void
{
    os::alloc(len)
}

/// Frees a segment of memory received through intercom interfaces.
///
/// # Arguments
///
/// - `ptr` - Memory to free.
///
/// # Safety
///
/// The memory must have been allocated using the `allocate` method in this
/// module or the `IAllocator::alloc` method.
pub unsafe fn free(ptr: *mut raw::c_void)
{
    os::free(ptr)
}

#[cfg(windows)]
mod os
{
    use std::os::raw;

    pub unsafe fn alloc_bstr(psz: *const u16, len: u32) -> *mut u16
    {
        SysAllocStringLen(psz, len)
    }

    pub unsafe fn free_bstr(bstr: *mut u16)
    {
        SysFreeString(bstr)
    }

    pub unsafe fn alloc(len: usize) -> *mut raw::c_void
    {
        CoTaskMemAlloc(len)
    }

    pub unsafe fn free(ptr: *mut raw::c_void)
    {
        CoTaskMemFree(ptr)
    }

    #[doc(hidden)]
    #[link(name = "oleaut32")]
    extern "system" {
        pub fn SysAllocStringLen(psz: *const u16, len: u32) -> *mut u16;
        pub fn SysFreeString(bstr: *mut u16);
    }

    #[doc(hidden)]
    #[link(name = "ole32")]
    extern "system" {
        pub fn CoTaskMemAlloc(len: usize) -> *mut raw::c_void;
        pub fn CoTaskMemFree(ptr: *mut raw::c_void);
    }
}

#[cfg(not(windows))]
mod os
{
    use std::os::raw;

    /// # Safety
    ///
    /// See IAllocator above.
    pub unsafe fn alloc_bstr(psz: *const u16, len: u32) -> *mut u16
    {
        // text size in bytes.
        let text_size: usize = (len * 2) as usize;

        // BSTR layout:
        //
        // | Length:u32 | Text data...:[u16] | Zero termiantion:u16 |
        //
        // As bytes this is 4 + len * 2 + 2, or:
        let ptr = libc::malloc(text_size + 6);
        let text_data = (ptr as usize + 4) as *mut u16;

        // Store the length.
        *(ptr as *mut u32) = text_size as u32;

        // Copy text data to the buffer. Size is indicates as bytes, so
        // double the amount of u16-chars.
        libc::memcpy(text_data as *mut _, psz as *mut _, text_size);

        // Zero termination.
        *((text_data as usize + text_size) as *mut _) = 0u16;

        // Return a pointer to the text data as per BSTR spec.
        text_data
    }

    /// # Safety
    ///
    /// See IAllocator above.
    pub unsafe fn free_bstr(bstr: *mut u16)
    {
        // Ignore null pointers. The offset would make these non-null and crash
        // the application.
        if bstr.is_null() {
            return;
        }

        // Offset the ptr back to the start of the reserved memory and free it.
        let ptr = (bstr as usize - 4) as *mut _;
        libc::free(ptr)
    }

    /// # Safety
    ///
    /// See IAllocator above.
    pub unsafe fn alloc(len: usize) -> *mut raw::c_void
    {
        libc::malloc(len) as *mut _
    }

    /// # Safety
    ///
    /// See IAllocator above.
    pub unsafe fn free(ptr: *mut raw::c_void)
    {
        libc::free(ptr as *mut _)
    }
}
