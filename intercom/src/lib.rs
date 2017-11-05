#![crate_type="dylib"]
#![feature(unique, shared)]

mod classfactory; pub use classfactory::*;
mod combox; pub use combox::*;
mod comrc; pub use comrc::*;
mod bstr; pub use bstr::*;
mod guid; pub use guid::GUID;

// The crate doesn't really need the macros. However Rust will complain that
// the import does nothing if we don't define #[macro_use]. Once we define
// #[macro_use] to get rid of that warning, Rust will complain that the
// #[macro_use] does nothing. Fortunately THAT warning comes with a named
// warning option so we can allow that explicitly.
#[allow(unused_imports)]
#[macro_use]
extern crate intercom_attributes;
pub use intercom_attributes::*;

/// Raw COM pointer type.
pub type RawComPtr = *mut std::os::raw::c_void;

/// Interface ID GUID.
pub type IID = GUID;

/// A reference to an interface ID.
pub type REFIID = *const IID;

/// Class ID GUID.
pub type CLSID = GUID;

/// A reference to a class ID.
pub type REFCLSID = *const IID;

/// COM error result value.
pub type HRESULT = i32;


/// HRESULT indicating the opration completed successfully.
pub const S_OK : HRESULT = 0;



/// HRESULT for unimplemented functionality.
#[allow(overflowing_literals)]
pub const E_NOTIMPL : HRESULT = 0x80004001 as HRESULT;

/// HRESULT indicating the type does not support the requested interface.
#[allow(overflowing_literals)]
pub const E_NOINTERFACE : HRESULT = 0x80004002 as HRESULT;

/// HRESULT indicating a pointer parameter was invalid.
#[allow(overflowing_literals)]
pub const E_POINTER : HRESULT = 0x80004003 as HRESULT;

/// HRESULT for aborted operation.
#[allow(overflowing_literals)]
pub const E_ABORT : HRESULT = 0x80004004 as HRESULT;

/// HRESULT for unspecified failure.
#[allow(overflowing_literals)]
pub const E_FAIL : HRESULT = 0x80004005 as HRESULT;

/// HRESULT for invalid argument.
#[allow(overflowing_literals)]
pub const E_INVALIDARG : HRESULT = 0x80070057 as HRESULT;

/// IUnknown interface ID.
#[allow(non_upper_case_globals)]
pub const IID_IUnknown : GUID = GUID {
    data1: 0x00000000, data2: 0x0000, data3: 0x0000,
    data4: [ 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46 ]
};

/// IClassFactory interface ID.
#[allow(non_upper_case_globals)]
pub const IID_IClassFactory : GUID = GUID {
    data1: 0x00000001, data2: 0x0000, data3: 0x0000,
    data4: [ 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46 ]
};

/// IUnknown virtual table layout.
#[repr(C)]
pub struct IUnknownVtbl
{
    /// QueryInterface method pointer.
    pub query_interface : unsafe extern "stdcall" fn(
        s : RawComPtr,
        _riid : REFIID,
        out : *mut RawComPtr
    ) -> HRESULT,

    /// AddRef method pointer.
    pub add_ref: unsafe extern "stdcall" fn( s : RawComPtr ) -> u32,

    /// Release method pointer.
    pub release: unsafe extern "stdcall" fn( s : RawComPtr ) -> u32,
}

// Do we need this? Would rather not export this through an extern crate
// for another dll.
//
// com_library should have dllmain!() macro or similar that implements this
// together with the COM registration.
#[no_mangle]
#[allow(non_camel_case_types)]
pub extern "stdcall" fn DllMain(
    _dll_instance : *mut std::os::raw::c_void,
    _reason : u32,
    _reserved : *mut std::os::raw::c_void ) -> bool
{
    true
}

pub type ComResult<A> = Result<A, HRESULT>;

