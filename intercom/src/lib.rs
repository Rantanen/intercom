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
//! use intercom::{com_library, com_class, com_interface, com_impl, ComResult};
//!
//! // Define COM classes to expose from this library.
//! #[com_library(Calculator)]
//!
//! // Define the COM class and the interfaces it implements.
//! #[com_class(Calculator)]
//! struct Calculator;
//!
//! // Define the implementation for the class. The COM interface is defined
//! // implicitly by the `impl`.
//! #[com_interface]
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
#![feature(try_from, specialization, non_exhaustive, integer_atomics)]

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
#[allow(clippy::useless_attribute)]
#[allow(unused_imports)]
extern crate intercom_attributes;
/// Foo
pub use intercom_attributes::*;

#[allow(clippy::useless_attribute)]
#[allow(unused_imports)]
#[macro_use] extern crate failure;

mod classfactory; pub use classfactory::*;
mod combox; pub use combox::*;
mod comrc; pub use comrc::*;
mod comitf; pub use comitf::*;
mod strings; pub use strings::*;
mod guid; pub use guid::GUID;
mod error; pub use error::{return_hresult, get_last_error, ComError, ErrorInfo, ErrorValue};
mod interfaces;
pub mod runtime;
pub mod alloc;
mod variant;

// intercom_attributes use "intercom::" to qualify things in this crate.
// Declare such module here and import everything we have in it to make those
// references valid.
mod intercom {
    pub use ::*;
}

/// The `ComInterface` trait defines the COM interface details for a COM
/// interface trait.
pub trait ComInterface {

    /// IID of the COM interface.
    fn iid( ts : TypeSystem ) -> Option< &'static IID >;

    /// Dereferences a `ComItf<T>` into a `&T`.
    ///
    /// While in most cases the user crate will implement `T` for `ComItf<T>`,
    /// this impl exists only in the user crate and cannot be used in generic
    /// contexts. For generic `ComItf<T>` use, Intercom ipmls `Deref<Target=T>`
    /// for `ComItf<T>` which requires this method.
    fn deref( com_itf : &ComItf<Self> ) -> &Self;
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

pub mod raw {
    pub type InBSTR = *const u16;
    pub type OutBSTR = *mut u16;

    pub type InCStr = *const ::std::os::raw::c_char;
    pub type OutCStr = *mut ::std::os::raw::c_char;

    pub use variant::raw::*;
    
    #[repr(C)]
    pub struct InterfacePtr<I: ?Sized> {
        pub ptr : super::RawComPtr,
        phantom : ::std::marker::PhantomData<I>,
    }

    impl<I: ?Sized> Clone for InterfacePtr<I> {
        fn clone( &self ) -> Self {
            InterfacePtr { ptr: self.ptr, phantom: ::std::marker::PhantomData }
        }
    }

    impl<I: ?Sized> Copy for InterfacePtr<I> {}

    impl<I: ?Sized> InterfacePtr<I> {
        pub fn new( ptr : super::RawComPtr ) -> InterfacePtr<I> {
            InterfacePtr { ptr, phantom: ::std::marker::PhantomData }
        }
    }
}

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

pub use interfaces::__IUnknown_AutomationVtbl as IUnknownVtbl;
pub use interfaces::IID_IUnknown_Automation as IID_IUnknown;
pub use interfaces::IUnknown;

pub use interfaces::__ISupportErrorInfo_AutomationVtbl as ISupportErrorInfoVtbl;
pub use interfaces::IID_ISupportErrorInfo_Automation as IID_ISupportErrorInfo;
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

