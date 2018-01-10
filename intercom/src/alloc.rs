
use super::*;
use std::os::raw;

/// A memory allocator to be used for allocating/deallocating memory shared
/// with intercom libraries.
#[com_class( AUTO_GUID, Allocator )]
pub struct Allocator;

#[com_interface( "18EE22B3-B0C6-44A5-A94A-7A417676FB66" )]
#[com_impl]
impl Allocator {
    unsafe fn alloc_bstr( &self, text : *const u16, len : u32 ) -> *mut u16 {
        os::alloc_bstr( text, len )
    }

    unsafe fn free_bstr( &self, bstr : *mut u16 ) {
        os::free_bstr( bstr )
    }

    unsafe fn alloc( &self, len : usize ) -> *mut raw::c_void {
        os::alloc( len )
    }

    unsafe fn free( &self, ptr : *mut raw::c_void ) {
        os::free( ptr )
    }
}

impl Default for Allocator {
    fn default() -> Allocator { Allocator }
}


#[cfg(windows)]
pub mod os {
    use std::os::raw;

    pub unsafe fn alloc_bstr(
        psz: *const u16,
        len: u32
    ) -> *mut u16 {
        SysAllocStringLen( psz, len )
    }

    pub unsafe fn free_bstr(
        bstr : *mut u16
    ) {
        SysFreeString( bstr )
    }

    pub unsafe fn alloc(
        len: usize
    ) -> *mut raw::c_void {
        CoTaskMemAlloc( len )
    }

    pub unsafe fn free(
        ptr : *mut raw::c_void
    ) {
        CoTaskMemFree( ptr )
    }

    #[doc(hidden)]
    #[link(name = "oleaut32")]
    extern "system" {
        pub fn SysAllocStringLen( psz: *const u16, len: u32) -> *mut u16;
        pub fn SysFreeString( bstr: *mut u16 );
    }

    #[doc(hidden)]
    #[link(name = "ole32")]
    extern "system" {
        pub fn CoTaskMemAlloc( len : usize ) -> *mut raw::c_void;
        pub fn CoTaskMemFree( ptr : *mut raw::c_void );
    }
}

#[cfg(not(windows))]
pub mod os {
    use std::libc;

    pub unsafe fn alloc_bstr(
        psz: *const u16,
        len: u32
    ) -> *mut u16
    {
        // BSTR layout:
        //
        // | Length:u32 | Text data...:[u16] | Zero termiantion:u16 |
        //
        // As bytes this is 4 + len * 2 + 2, or:
        let ptr = libc::malloc( len * 2 + 6 );
        let text_data = ( ptr + 4 ) as *mut u16;

        // Store the length.
        *( ptr as *mut u32 ) = len;

        // Copy text data to the buffer. Size is indicates as bytes, so
        // double the amount of u16-chars.
        libc::memcpy(
                text_data as *mut c_void,
                psz as *mut c_void,
                len * 2 );

        // Zero termination.
        *( text_data + len ) = 0;

        // Return a pointer to the text data as per BSTR spec.
        text_data
    }

    pub unsafe fn free_bstr(
        bstr : *mut u16
    ) -> *mut u16 {

        // Offset the ptr back to the start of the reserved memory and free it.
        let ptr = bstr - 2;
        libc::free( ptr as *mut c_void );
    }

    pub unsafe fn alloc(
        len: usize
    ) -> *mut libc::c_void {
        libc::malloc( len )
    }

    pub unsafe fn free(
        ptr : *mut libc::c_void
    ) {
        libc::free( ptr as *mut c_void )
    }
}

