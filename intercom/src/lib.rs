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
//! com_library!(Calculator);
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
//! #   // TODO: This should be replaced with Default::default implementation.
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
#![feature(specialization, non_exhaustive, integer_atomics, associated_type_defaults)]
#![allow(clippy::match_bool)]

extern crate self as intercom;

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
pub use intercom_attributes::*;

#[allow(clippy::useless_attribute)]
#[allow(unused_imports)]
#[macro_use] extern crate failure;

pub mod prelude;

mod classfactory; pub use crate::classfactory::*;
mod combox; pub use crate::combox::*;
mod comrc; pub use crate::comrc::*;
mod comitf; pub use crate::comitf::*;
mod strings; pub use crate::strings::*;
mod guid; pub use crate::guid::GUID;
pub mod error; pub use crate::error::{ComError, store_error, load_error, ErrorValue};
mod interfaces;
pub mod runtime;
pub mod alloc;
mod variant; pub use crate::variant::{Variant, VariantError};
pub mod typelib;
pub mod type_system; pub use crate::type_system::{ ComItemCategory, ItemInfo, BidirectionalTypeInfo, InputTypeInfo, OutputTypeInfo };
pub mod serialization;

/// The `ComInterface` trait defines the COM interface details for a COM
/// interface trait.
pub trait ComInterface {

    /// IID of the COM interface.
    fn iid( ts : type_system::TypeSystemName ) -> Option< &'static IID >;

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

    pub use crate::variant::raw::*;
    pub use crate::error::raw::*;
    pub use crate::type_system::TypeSystem;

    #[repr(C)]
    #[derive(PartialEq, Eq)]
    pub struct InterfacePtr<TS: TypeSystem, I: ?Sized> {
        pub ptr : super::RawComPtr,
        phantom_itf : ::std::marker::PhantomData<I>,
        phantom_ts : ::std::marker::PhantomData<TS>,
    }

    impl<TS: TypeSystem, I: ?Sized> Clone for InterfacePtr<TS, I>
    {
        fn clone( &self ) -> Self {
            InterfacePtr::new( self.ptr )
        }
    }

    impl<TS: TypeSystem, I: ?Sized> Copy for InterfacePtr<TS, I> {}

    impl<TS: TypeSystem, I: ?Sized> std::fmt::Debug for InterfacePtr<TS, I> {
        fn fmt( &self, f : &mut std::fmt::Formatter ) -> std::fmt::Result {
            write!( f, "InterfacePtr({:?})", self.ptr )
        }
    }

    impl<TS: TypeSystem, I: ?Sized> InterfacePtr<TS, I> {
        pub fn new( ptr : super::RawComPtr ) -> InterfacePtr<TS, I> {
            InterfacePtr {
                ptr,
                phantom_itf: ::std::marker::PhantomData,
                phantom_ts: ::std::marker::PhantomData,
            }
        }

        pub fn null() -> Self {
            Self::new( std::ptr::null_mut() )
        }

        pub fn is_null( self ) -> bool {
            self.ptr.is_null()
        }
    }

    impl<TS: TypeSystem, I: crate::ComInterface + ?Sized> InterfacePtr<TS, I> {
        pub fn as_unknown( self ) -> InterfacePtr<TS, dyn crate::IUnknown> {
            InterfacePtr::new( self.ptr )
        }
    }
}


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

pub use crate::interfaces::__IUnknown_AutomationVtbl as IUnknownVtbl;
pub use crate::interfaces::IID_IUnknown_Automation as IID_IUnknown;
pub use crate::interfaces::IUnknown;

pub use crate::interfaces::__ISupportErrorInfo_AutomationVtbl as ISupportErrorInfoVtbl;
pub use crate::interfaces::IID_ISupportErrorInfo_Automation as IID_ISupportErrorInfo;
pub use crate::interfaces::ISupportErrorInfo;

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
pub type ComResult<A> = Result<A, ComError>;

/// Basic COM result type.
///
/// The `ComResult` maps the Rust concept of `Ok` and `Err` values to COM
/// `[out, retval]` parameter and `HRESULT` return value.
pub type RawComResult<A> = Result<A, raw::HRESULT>;

