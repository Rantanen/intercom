//! Tools to define Rust components compatible with the COM protocol.
//!
//! Intercom provides attributes to automatically derive `extern` compatible
//! functions for Rust methods. These functions are compatible with COM binary
//! interface standard, which allows them to be used from any language that
//! supports COM.
//!
//! # Examples
//!
//! A basic example of a calculator type exposed as a COM object.
//!
//! ```
//! #![feature(proc_macro)]
//!
//! use intercom::{com_library, com_class, com_interface, com_impl, ComResult};
//!
//! // Define COM classes to expose from this library.
//! #[com_library(AUTO_GUID, Calculator)]
//!
//! // Define the COM class and the interfaces it implements.
//! #[com_class(AUTO_GUID, Calculator)]
//! struct Calculator;
//!
//! // Define the implementation for the class. The COM interface is defined
//! // implicitly by the `impl`.
//! #[com_interface(AUTO_GUID)]
//! #[com_impl]
//! impl Calculator {
//!
//!     // Intercom requires a `new` method with no parameters for all classes.
//! #   // NOTE: This should be replaced with Default::default implementation.
//!     fn new() -> Calculator { Calculator }
//!
//!     fn add(&self, a: i32, b: i32) -> ComResult<i32> { Ok(a + b) }
//!     fn sub(&self, a: i32, b: i32) -> ComResult<i32> { Ok(a - b) }
//! }
//! # fn main() {}
//! ```
//!
//! The above library can be used for example from C# in the following manner.
//!
//! ```csharp
//! void Main()
//! {
//!     var calculator = new CalculatorLib.Calculator();
//!     Console.WriteLine( calculator.Add( 1, 2 ) );
//! }
//! ```

#![crate_type="dylib"]
#![feature(proc_macro, try_from, fundamental, specialization, non_exhaustive, integer_atomics)]

#[cfg(not(windows))]
extern crate libc;

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
/// Foo
pub use intercom_attributes::*;

#[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
#[allow(unused_imports)]
#[macro_use] extern crate failure;

mod classfactory; pub use classfactory::*;
mod combox; pub use combox::*;
mod comrc; pub use comrc::*;
mod comitf; pub use comitf::*;
mod bstr; pub use bstr::*;
mod guid; pub use guid::GUID;
mod error; pub use error::{return_hresult, get_last_error, ComError, ErrorInfo};
pub mod runtime;
pub mod alloc;

// intercom_attributes use "intercom::" to qualify things in this crate.
// Declare such module here and import everything we have in it to make those
// references valid.
mod intercom {
    pub use ::*;
}

pub trait IidOf {
    fn iid() -> &'static IID;
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

/// COM method status code.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub struct HRESULT {

    /// The numerical HRESULT code.
    pub hr : i32
}

impl HRESULT {

    /// Constructs a new `HRESULT` with the given numerical code.
    pub fn new( hr : u32 ) -> HRESULT {
        #[allow(overflowing_literals)]
        HRESULT { hr : hr as i32 }
    }
}

macro_rules! make_hr {
    ( $(#[$attr:meta] )* $hr_name: ident = $hr_value: expr ) => {
        $(#[$attr])*
        #[allow(overflowing_literals)]
        pub const $hr_name : HRESULT = HRESULT { hr: $hr_value as i32 };
    }
}

make_hr!(
    /// `HRESULT` indicating the operation completed successfully.
    S_OK = 0 );

make_hr!(
    /// `HRESULT` indicating the operation completed successfully and returned
    /// `false`.
    S_FALSE = 1 );

make_hr!(
    /// `HRESULT` for unimplemented functionality.
    E_NOTIMPL = 0x8000_4001 );

make_hr!(
    /// `HRESULT` indicating the type does not support the requested interface.
    E_NOINTERFACE = 0x8000_4002 );

make_hr!(
    /// `HRESULT` indicating a pointer parameter was invalid.
    E_POINTER = 0x8000_4003 );

make_hr!(
    /// `HRESULT` for aborted operation.
    E_ABORT = 0x8000_4004 );

make_hr!(
    /// `HRESULT` for unspecified failure.
    E_FAIL = 0x8000_4005 );

make_hr!(
    /// `HRESULT` for invalid argument.
    E_INVALIDARG = 0x8007_0057 );

// These might be deprecated. They are a bit too specific for cross-platform
// support. We'll just need to ensure the winapi HRESULTs are compatible.
make_hr!( E_ACCESSDENIED = 0x8007_0005 );
make_hr!( STG_E_FILENOTFOUND = 0x8003_0002 );
make_hr!( RPC_E_DISCONNECTED = 0x8001_0108 );
make_hr!( RPC_E_CALL_REJECTED = 0x8001_0001 );
make_hr!( RPC_E_CALL_CANCELED = 0x8001_0002 );
make_hr!( RPC_E_TIMEOUT = 0x8001_011F );


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

    /// The `IUnknown` COM interface.
    ///
    /// All COM interfaces must inherit from `IUnknown` interface directly or
    /// indirectly. The interface provides the basis of COM reference counting
    /// and interface discovery.
    ///
    /// For Rust code, Intercom implements the interface automatically.
    #[::com_interface( "00000000-0000-0000-C000-000000000046", NO_BASE )]
    pub trait IUnknown {

        /// Tries to get a different COM interface for the current object.
        ///
        /// COM objects may (and do) implement multiple interfaces. COM defines
        /// `QueryInterface` as the mechanism for acquiring an interface pointer
        /// to a different interface the object implements.
        ///
        /// * `riid` - The `IID` of the interface to query.
        ///
        /// Returns `Ok( interface_ptr )` if the object supports the specified
        /// interface or `Err( E_NOINTERFACE )` if it doesn't.
        fn query_interface( &self, riid : ::REFIID ) -> ::ComResult< ::RawComPtr >;

        /// Increments the reference count of the object.
        ///
        /// Returns the reference count after the incrementation.
        fn add_ref( &self ) -> u32;

        /// Decreases the reference count of the object.
        ///
        /// Returns the reference count after the decrement.
        ///
        /// If the reference count reaches zero, the object will deallocate
        /// itself. As the call might deallocate the object, the caller must
        /// ensure that the released reference is not used afterwards.
        fn release( &self ) -> u32;
    }

    /// The `ISupportErrorInfo` COM interface.
    ///
    /// The `ISupportErrorInfo` is part of COM error handling concept. As the
    /// methods are traditionally limited to `HRESULT` return values, they may
    /// make more detailed `IErrorInfo` data available through the error info
    /// APIs.
    ///
    /// The `ISupportErrorInfo` interface communicates which interfaces that an
    /// object implements support detailed error info. When a COM client
    /// receives an error-HRESULT, it may query for error info support through
    /// this interface. If the interface returns an `S_OK` as opposed to
    /// `S_FALSE` return value, the client can then use separate error info
    /// APIs to retrieve a detailed `IErrorInfo` object that contains more
    /// details about the error, such as the error message.
    ///
    /// Intercom COM classes support the detailed error info for all user
    /// specified interfaces automatically. Only methods that return a
    /// two-parameter `Result<S,E>` value will store the detailed `IErrorInfo`.
    /// Other methods will set a null `IErrorInfo` value.
    #[::com_interface( "DF0B3D60-548F-101B-8E65-08002B2BD119" )]
    pub trait ISupportErrorInfo {

        /// Informs the current COM class supports `IErrorInfo` for a specific
        /// interface.
        ///
        /// * `riid` - The `IID` of the interface to query.
        ///
        /// Returns `S_OK` if the object supports `IErrorInfo` for the
        /// interface specified by the `riid` parameter. Otherwise returns
        /// `S_FALSE` - even in the case the object doesn't implement `riid`
        /// at all.
        ///
        /// # Description
        ///
        /// If the object returns `S_OK` for an interface, then any methods
        /// the object implements for that interface must store the
        /// `IErrorInfo` on failure.
        ///
        /// Intercom will implement the support for `IErrorInfo` automatically
        /// for all custom interfaces the user defines. This includes returning
        /// `S_OK` from this method.
        ///
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
#[deprecated]
#[doc(hidden)]
pub extern "stdcall" fn DllMain(
    _dll_instance : *mut std::os::raw::c_void,
    _reason : u32,
    _reserved : *mut std::os::raw::c_void ) -> bool
{
    true
}

/// Basic COM result type.
///
/// The `ComResult` maps the Rust concept of `Ok` and `Err` values to COM
/// `[out, retval]` parameter and `HRESULT` return value.
pub type ComResult<A> = Result<A, HRESULT>;

