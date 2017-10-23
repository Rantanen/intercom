#![crate_type="dylib"]

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

pub type ComPtr = *mut std::os::raw::c_void;

pub const S_OK : u32 = 0;
pub const E_NOINTERFACE : u32 = 0x80004002;

#[no_mangle]
#[allow(non_camel_case_types)]
pub extern "stdcall" fn DllMain(
    _dll_instance : *mut std::os::raw::c_void,
    _reason : u32,
    _reserved : *mut std::os::raw::c_void ) -> bool
{
    true
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct __IUnknown_vtable
{
    pub query_interface : unsafe extern "stdcall" fn(
        s : ComPtr,
        _riid : REFIID,
        out : *mut ComPtr
    ) -> u32,
    pub add_ref: unsafe extern "stdcall" fn( s : ComPtr ) -> u32,
    pub release: unsafe extern "stdcall" fn( s : ComPtr ) -> u32,
}

#[allow(non_camel_case_types)]
pub struct __ClassFactory_vtable {
    pub __base: __IUnknown_vtable,
    pub create_instance: unsafe extern "stdcall" fn( ComPtr, ComPtr, REFIID, *mut ComPtr ) -> u32,
    pub lock_server: unsafe extern "stdcall" fn( ComPtr, bool ) -> u32
}

pub struct ClassFactory {
    pub __vtable : &'static __ClassFactory_vtable,
    pub clsid : REFCLSID,
    pub rc : u32
}

impl ClassFactory {

    pub unsafe extern "stdcall" fn query_interface(
        self_void : ComPtr,
        _riid : REFIID,
        out : *mut ComPtr
    ) -> u32 {
        // Query interface needs to increment RC.
        let self_ptr : *mut ClassFactory = std::mem::transmute( self_void );
        (*self_ptr).rc += 1;
        *out = self_void;
        S_OK
    }

    pub unsafe extern "stdcall" fn add_ref(
        self_void : ComPtr
    ) -> u32 {
        let self_ptr : *mut ClassFactory = std::mem::transmute( self_void );
        (*self_ptr).rc += 1;
        (*self_ptr).rc
    }

    pub unsafe extern "stdcall" fn release(
        self_void : ComPtr
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
        self_void : ComPtr,
        lock : bool
    ) -> u32 {
        if lock {
            ClassFactory::add_ref( self_void );
        } else {
            ClassFactory::release( self_void );
        }
        S_OK
    }
}

pub type ComResult<A> = Result<A, u32>;

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
