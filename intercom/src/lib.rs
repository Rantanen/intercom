#![crate_type="dylib"]
#![feature(unique, shared)]
#![feature(proc_macro, try_from)]

mod classfactory; pub use classfactory::*;
mod combox; pub use combox::*;
mod comrc; pub use comrc::*;
mod comitf; pub use comitf::*;
mod bstr; pub use bstr::*;
mod guid; pub use guid::GUID;
mod error; pub use error::{return_hresult, get_last_error, ComError, ErrorInfo};

// The crate doesn't really need the macros. However Rust will complain that
// the import does nothing if we don't define #[macro_use]. Once we define
// #[macro_use] to get rid of that warning, Rust will complain that the
// #[macro_use] does nothing. Fortunately THAT warning comes with a named
// warning option so we can allow that explicitly.
//
// Unfortunately clippy disagrees on the macro_use being unused and claims that
// the unused_imports attribute is useless. So now we also need to tell clippy
// to ignore useless attributes in this scenario! \:D/
#[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
#[allow(unused_imports)]
extern crate intercom_attributes;
pub use intercom_attributes::*;

// intercom_attributes use "intercom::" to qualify things in this crate.
// Declare such module here and import everything we have in it to make those
// references valid.
mod intercom {
    pub use ::*;
}

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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub struct HRESULT { pub hr : i32 }

impl HRESULT {
    pub fn new( hr : u32 ) -> HRESULT {
        #[allow(overflowing_literals)]
        HRESULT { hr : hr as i32 }
    }
}

macro_rules! make_hr {
    ( $hr_name: ident, $hr_value: expr ) => {
        #[allow(overflowing_literals)]
        pub const $hr_name : HRESULT = HRESULT { hr: $hr_value as i32 };
    }
}

/// `HRESULT` indicating the operation completed successfully.
make_hr!( S_OK, 0 );

/// `HRESULT` indicating the operation completed successfully and returned FALSE.
make_hr!( S_FALSE, 1 );

/// `HRESULT` for unimplemented functionality.
make_hr!( E_NOTIMPL, 0x8000_4001 );

/// `HRESULT` indicating the type does not support the requested interface.
make_hr!( E_NOINTERFACE, 0x8000_4002 );

/// `HRESULT` indicating a pointer parameter was invalid.
make_hr!( E_POINTER, 0x8000_4003 );

/// `HRESULT` for aborted operation.
make_hr!( E_ABORT, 0x8000_4004 );

/// `HRESULT` for unspecified failure.
make_hr!( E_FAIL, 0x8000_4005 );

/// `HRESULT` for invalid argument.
make_hr!( E_INVALIDARG, 0x8007_0057 );

make_hr!( E_ACCESSDENIED, 0x8007_0005 );

make_hr!( STG_E_FILENOTFOUND, 0x8003_0002 );

make_hr!( RPC_E_DISCONNECTED, 0x8001_0108 );

make_hr!( RPC_E_CALL_REJECTED, 0x8001_0001 );

make_hr!( RPC_E_CALL_CANCELED, 0x8001_0002 );

make_hr!( RPC_E_TIMEOUT, 0x8001_011F );


/// `IClassFactory` interface ID.
#[allow(non_upper_case_globals)]
pub const IID_IClassFactory : GUID = GUID {
    data1: 0x0000_0001, data2: 0x0000, data3: 0x0000,
    data4: [ 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46 ]
};

/// `IErrorInfo` interface ID.
#[allow(non_upper_case_globals)]
pub const IID_IErrorInfo : GUID = GUID {
    data1: 0x1CF2_B120, data2: 0x547D, data3: 0x101B,
    data4: [ 0x8E, 0x65, 0x08, 0x00, 0x2B, 0x2B, 0xD1, 0x19 ]
};

mod interfaces {

    #[::com_interface( "00000000-0000-0000-C000-000000000046", NO_BASE )]
    pub trait IUnknown {
        fn query_interface( &self, riid : ::REFIID ) -> ::ComResult< ::RawComPtr >;
        fn add_ref( &self ) -> u32;
        fn release( &self ) -> u32;
    }

    #[::com_interface( "DF0B3D60-548F-101B-8E65-08002B2BD119" )]
    pub trait ISupportErrorInfo {
        fn interface_supports_error_info( &self, riid : ::REFIID ) -> ::HRESULT;
    }
}

pub use interfaces::__IUnknownVtbl as IUnknownVtbl;
pub use interfaces::IID_IUnknown;
pub use interfaces::IUnknown;

pub use interfaces::__ISupportErrorInfoVtbl as ISupportErrorInfoVtbl;
pub use interfaces::IID_ISupportErrorInfo;
pub use interfaces::ISupportErrorInfo;

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

