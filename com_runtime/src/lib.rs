#![crate_type="dylib"]
#![feature(unique, shared)]

use std::ptr;

// <3 winapi
// (Re-defining these here as not to pull the whole winapi as dev dependency)
#[repr(C)]
#[derive(Eq, PartialEq, Debug)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [ u8; 8 ],
}

pub type IID = GUID;
pub type REFIID = *const IID;
pub type REFCLSID = *const IID;
pub type HRESULT = i32;


pub type RawComPtr = *mut std::os::raw::c_void;

pub const S_OK : HRESULT = 0;

#[allow(overflowing_literals)]
pub const E_NOINTERFACE : HRESULT = 0x80004002 as HRESULT;

#[allow(non_upper_case_globals)]
pub const IID_IUnknown : GUID = GUID {
    data1: 0, data2: 0, data3: 0,
    data4: [ 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46 ]
};

#[no_mangle]
#[allow(non_camel_case_types)]
pub extern "stdcall" fn DllMain(
    _dll_instance : *mut std::os::raw::c_void,
    _reason : u32,
    _reserved : *mut std::os::raw::c_void ) -> bool
{
    true
}

pub trait CoClass {
    type VTableList: std::any::Any;
    fn create_vtable_list() -> Self::VTableList;
}

#[repr(C)]
pub struct ComBox< T: CoClass > {
    vtable_list : T::VTableList,
    ref_count : u32,
    value: T,
}

impl<T: CoClass> ComBox<T> {

    pub fn allocate( value : T ) -> ptr::Unique<ComBox<T>> {
        Box::into_unique( Box::new( ComBox {
            vtable_list: T::create_vtable_list(),
            ref_count: 1,
            value: value,
        } ) )
    }

    pub fn add_ref( this : &mut Self ) -> u32 {
        this.ref_count += 1;
        this.ref_count
    }

    pub unsafe fn release( this : *mut Self ) -> u32 {
        (*this).ref_count -= 1;
        let rc = (*this).ref_count;

        // If that was the last reference we can drop self. Do this by giving
        // it back to a box and then dropping the box. This should reverse the
        // allocation we did by boxing the value in the first place.
        if rc == 0 {
            drop( Box::from_raw( this ) );
        }
        rc
    }

    pub unsafe extern "stdcall" fn add_ref_ptr(
        self_iunk : RawComPtr
    ) -> u32
    {
        ComBox::add_ref( &mut *( self_iunk as *mut ComBox< T > ) )
    }

    pub unsafe extern "stdcall" fn release_ptr(
        self_iunk : RawComPtr
    ) -> u32
    {
        ComBox::release( self_iunk as *mut ComBox< T > )
    }

    pub unsafe fn vtable( this : &Self ) -> &T::VTableList {
        &this.vtable_list
    }

    #[inline(always)]
    pub unsafe fn null_vtable() -> &'static T::VTableList {
        let null_combox =
                std::ptr::null() as *const ComBox< T >;
        &(*null_combox).vtable_list
    }
}

impl<T> std::ops::Deref for ComBox<T> where T: CoClass {
    type Target = T;
    fn deref( &self ) -> &T { &self.value }
}

impl<T> std::ops::DerefMut for ComBox<T> where T: CoClass {
    fn deref_mut( &mut self ) -> &mut T { &mut self.value }
}

impl<T : CoClass> std::convert::Into< RawComPtr > for ComRc<T> {
    fn into(self) -> *mut std::os::raw::c_void {
        self.ptr.as_ptr() as RawComPtr
    }
}

pub struct ComRc< T: CoClass > {
    ptr : ptr::Shared<ComBox<T>>
}

impl<T> ComRc<T> where T : CoClass {
    pub fn new( value : T ) -> ComRc<T> {
        ComRc {
            ptr: ptr::Shared::from( ComBox::allocate( value ) )
        }
    }

    pub fn as_ptr( this : &Self ) -> RawComPtr {
        this.ptr.as_ptr() as RawComPtr
    }

    pub unsafe fn query_interface(
        this : &Self,
        iid : &GUID,
        out : *mut RawComPtr
    ) -> HRESULT
    {
        // The iunknown vtable is at the start of the data.
        let vtables = ComBox::vtable( this.ptr.as_ref() );
        let iunk = vtables as *const _ as *const *const IUnknownVtbl;
        ((**iunk).query_interface)(
                this.ptr.as_ptr() as RawComPtr, iid, out )
    }
}

impl<T: CoClass> Drop for ComRc<T> {
    fn drop( &mut self ) {
        unsafe { ComBox::release( self.ptr.as_ptr() ) };
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct IUnknownVtbl
{
    pub query_interface : unsafe extern "stdcall" fn(
        s : RawComPtr,
        _riid : REFIID,
        out : *mut RawComPtr
    ) -> HRESULT,
    pub add_ref: unsafe extern "stdcall" fn( s : RawComPtr ) -> u32,
    pub release: unsafe extern "stdcall" fn( s : RawComPtr ) -> u32,
}

#[allow(non_camel_case_types)]
pub struct __ClassFactory_vtable {
    pub __base: IUnknownVtbl,
    pub create_instance: unsafe extern "stdcall" fn( RawComPtr, RawComPtr, REFIID, *mut RawComPtr ) -> HRESULT,
    pub lock_server: unsafe extern "stdcall" fn( RawComPtr, bool ) -> HRESULT
}

pub struct ClassFactory {
    pub __vtable : &'static __ClassFactory_vtable,
    pub clsid : REFCLSID,
    pub rc : u32
}

impl ClassFactory {

    pub unsafe extern "stdcall" fn query_interface(
        self_void : RawComPtr,
        _riid : REFIID,
        out : *mut RawComPtr
    ) -> HRESULT {
        // Query interface needs to increment RC.
        let self_ptr : *mut ClassFactory = std::mem::transmute( self_void );
        (*self_ptr).rc += 1;
        *out = self_void;
        S_OK
    }

    pub unsafe extern "stdcall" fn add_ref(
        self_void : RawComPtr
    ) -> u32 {
        let self_ptr : *mut ClassFactory = std::mem::transmute( self_void );
        (*self_ptr).rc += 1;
        (*self_ptr).rc
    }

    pub unsafe extern "stdcall" fn release(
        self_void : RawComPtr
    ) -> u32 {
        let self_ptr : *mut ClassFactory = std::mem::transmute( self_void );

        // We need a copy of the rc value in case we end up
        // dropping the ptr. We can't reference it during
        // return at that point.
        (*self_ptr).rc -= 1;
        let rc = (*self_ptr).rc;
        if rc == 0 {
            // Take ownership of the ptr and let it go out
            // of scope to destroy it.
            Box::from_raw( self_ptr );
        }
        rc
    }

    pub unsafe extern "stdcall" fn lock_server(
        self_void : RawComPtr,
        lock : bool
    ) -> HRESULT {
        if lock {
            ClassFactory::add_ref( self_void );
        } else {
            ClassFactory::release( self_void );
        }
        S_OK
    }
}

pub type ComResult<A> = Result<A, HRESULT>;

enum GuidFormat { Braces, Hyphens, Raw }

const GUID_ERR : &str = "The GUID must be in the {00000000-0000-0000-0000-000000000000} format";

impl GUID {

    pub fn parse( guid : &str ) -> Result< GUID, &'static str >
    {
        // We support the following formats:
        // {00000000-0000-0000-0000-000000000000} (38 chars)
        // 00000000-0000-0000-0000-000000000000 (36 chars)
        // 00000000000000000000000000000000 (32 chars)
        let guid_format = match guid.len() {
            38 => GuidFormat::Braces,
            36 => GuidFormat::Hyphens,
            32 => GuidFormat::Raw,
            _ => return Err( GUID_ERR )
        };

        let format = match guid_format {
            GuidFormat::Braces => vec![
                Some( b'{' ), None, None, None, None, None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None, None, None, None, None, None, None, None, None,
                Some( b'}' )
            ],
            GuidFormat::Hyphens => vec![
                None, None, None, None, None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None, None, None, None, None, None, None, None, None
            ],
            GuidFormat::Raw => vec![
                None, None, None, None, None, None, None, None,
                None, None, None, None,
                None, None, None, None,
                None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None
            ]
        };

        let mut buffer = [ 0u8; 16 ];
        let mut digit = 0;
        for ( i_char, chr ) in guid.bytes().enumerate() {

            // If this is a fixed character, ensure we have the correct one.
            if let Some( b ) = format[ i_char ] {
                if chr == b { continue } else { return Err( GUID_ERR ) }
            }

            let value : u8 = match chr {
                b'0'...b'9' => chr - b'0',
                b'a'...b'f' => chr - b'a' + 10,
                b'A'...b'F' => chr - b'A' + 10,
                _ => return Err( GUID_ERR )
            };

            let half = digit % 2;
            let byte = ( digit - half ) / 2;

            if half == 0 {
                buffer[ byte ] += value * 16;
            } else {
                buffer[ byte ] += value;
            }

            digit += 1;
        }

        Ok( GUID {
            data1:
                ( ( buffer[ 0 ] as u32 ) << 24 ) +
                ( ( buffer[ 1 ] as u32 ) << 16 ) +
                ( ( buffer[ 2 ] as u32 ) << 8 ) +
                ( ( buffer[ 3 ] as u32 ) << 0 ),
            data2:
                ( ( buffer[ 4 ] as u16 ) << 8 ) +
                ( ( buffer[ 5 ] as u16 ) << 0 ),
            data3:
                ( ( buffer[ 6 ] as u16 ) << 8 ) +
                ( ( buffer[ 7 ] as u16 ) << 0 ),
            data4: [
                buffer[ 8 ], buffer[ 9 ], buffer[ 10 ], buffer[ 11 ],
                buffer[ 12 ], buffer[ 13 ], buffer[ 14 ], buffer[ 15 ],
            ]
        } )
    }
}

impl std::fmt::Display for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!( f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7] )
    }
}
