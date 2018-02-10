
//! Procedural macro attributes for defining intercom libraries.
//!
//! These macros implement various low level items that enable the associated
//! types to be instantiated and invoked over the COM interface protocol.
//!
//! **Instead of depending on this crate directly, the users should depend on
//! the main `intercom` crate isntead.**
//!
//! The split into two crates is an artificial limitation of Rust. The crates
//! defining procedural macros cannot export anything else than procedural
//! macros.
#![feature(proc_macro)]
#![allow(unused_imports)]
#![feature(catch_expr)]

extern crate intercom_common;
use intercom_common::attributes::*;

extern crate proc_macro;
use proc_macro::{TokenStream, LexError};

// Note the rustdoc comments on the [proc_macro_attribute] functions document
// "attributes", not "functions".
//
// While at "com_interface" function creates virtual tables, etc. when it is
// invoked, the attribute doesn't "creates" these. Instead the attribute just
// "defines" the trait/impl as a COM interface.
//
// The runtime documentation for developers is present in the expand_...
// methods below.

/// Defines an intercom interface.
///
/// ```rust,ignore
/// #[com_interface(IID, base?)]
/// trait Foo { /* ... */ }
/// ```
///
/// - `IID` - A unique ID of the interface used to query for it. Must be either
///           a valid GUID or `AUTO_GUID` specifier.
/// - `base` - Base interface. Defaults to `IUnknown` if not specified.
///
/// Associated types: `trait`, `impl Struct`
///
/// Intercom interfaces form the basis of the cross language API provided by
/// the user library. The interfaces define the available methods that can be
/// called through the interface pointers given to the clients.
///
/// Each interface automatically inherits from the base `IUnknown` interface,
/// which provides the clients a way to perform reference counting and the
/// ability to query for other interfaces the object might implement.
#[proc_macro_attribute]
pub fn com_interface(
    attr: TokenStream,
    tokens: TokenStream,
) -> TokenStream
{
    match expand_com_interface( &attr, tokens ) {
        Ok(t) => t,
        Err(e) => panic!( "{}", e ),
    }
}

/// Defines an implementation of an intercom interface.
///
/// ```rust,ignore
/// #[com_impl]
/// impl Foo for Struct { /* ... */ }
/// ```
///
/// Associated types: `impl Trait for Struct`, `impl Struct`
///
/// The attribute allows Intercom to implement raw FFI functions for the
/// interface's methods. Intercom allows the use of non-FFI compatible types
/// as arguments and return values for the interface methods. The automatic
/// FFI layer handles conversion between these types and FFI compatible types.
#[proc_macro_attribute]
pub fn com_impl(
    attr: TokenStream,
    tokens: TokenStream,
) -> TokenStream
{
    match expand_com_impl( &attr, tokens ) {
        Ok(t) => t,
        Err(e) => panic!( "{}", e ),
    }
}

/// Defines a COM class that implements one or more COM interfaces.
///
/// ```rust,ignore
/// #[com_class(CLSID, interfaces...)]
/// struct S { /* ... */ }
/// ```
///
/// - `CLSID` - A unique ID of the exposed class. The clients use the class ID
///             to specify the class when they want to construct an object.
///             The value must be a valid GUID, `AUTO_GUID` or `NO_GUID`.
/// - `interfaces` - Any number of interfaces that the class implements.
///
/// Associated types: `struct`, `enum`
///
/// If the `CLSID` is specified as `NO_GUID`, the class cannot be constructed
/// by the clients. It can still be returned as a return value from other
/// intercom methods.
#[proc_macro_attribute]
pub fn com_class(
    attr: TokenStream,
    tokens: TokenStream,
) -> TokenStream
{
    match expand_com_class( &attr, tokens ) {
        Ok(t) => t,
        Err(e) => panic!( "{}", e ),
    }
}

/// Defines the COM library.
///
/// ```rust,ignore
/// #[com_library(LIBID, classes...)]
/// # struct S;  // Unfortunately the attribute needs to be on SOMETHING.
/// ```
///
/// - `LIBID` - A unique ID that specifies the current intercom library. Must
///             be a valid GUID or `AUTO_GUID`.
/// - `classes` - List of intercom classes that can be constructed by the
///               clients.
///
/// Associated types: None
///
/// The attribute results in the implementation of the object creation
/// infrastructure that allows external clients to load the library and
/// instantiate the specified types.
#[proc_macro_attribute]
pub fn com_library(
    attr: TokenStream,
    tokens: TokenStream,
) -> TokenStream
{
    match expand_com_library( &attr, tokens ) {
        Ok(t) => t,
        Err(e) => panic!( "{}", e ),
    }
}

