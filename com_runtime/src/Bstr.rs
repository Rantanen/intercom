
use super::*;

pub struct BStr( *mut u16 );

#[link(name = "oleaut32")]
extern "system" {
    pub fn SysAllocStringLen(
        psz: *const u16,
        len: u32
    ) -> BStr;
}

impl BStr {

    fn as_bstr_ptr( &mut self ) -> *mut u16 { self.0 }

    unsafe fn from_bstr_ptr( ptr : *mut u16 ) -> BStr { BStr( ptr ) }

    pub fn len_bytes( &self ) -> u32
    {
        unsafe {
            *(( self.0 as usize - 4 ) as *const u32 )
        }
    }

    pub fn string_to_bstr( s : &String ) -> BStr {

        let len = s.len() as u32;
        unsafe {
            SysAllocStringLen(
                s.encode_utf16().collect::<Vec<_>>().as_ptr(),
                len )
        }
    }

    pub fn bstr_to_string( &self ) -> String {

        let slice = unsafe { std::slice::from_raw_parts( 
                self.0 as *const u16,
                ( self.len_bytes() as usize ) / 2 ) };
        String::from_utf16_lossy( slice )
    }
}
