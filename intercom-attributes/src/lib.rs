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

#![allow(unused_imports)]

extern crate intercom_common;
use intercom_common::attributes::*;

extern crate proc_macro;
use proc_macro::{LexError, TokenStream};

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
pub fn com_interface(attr: TokenStream, tokens: TokenStream) -> TokenStream
{
    match expand_com_interface(attr, tokens) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
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
pub fn com_class(attr: TokenStream, tokens: TokenStream) -> TokenStream
{
    match expand_com_class(attr, tokens) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    }
}

/// Defines a COM library sub-module.
///
/// ```rust,ignore
/// com_module!(items...)]
/// ```
///
/// - `items` - List of items contained in this module.
///
/// The macro results in the implementation of the object creation
/// infrastructure that allows external clients to load the library and
/// instantiate the specified types.
#[proc_macro]
pub fn com_module(args: TokenStream) -> TokenStream
{
    match expand_com_module(args, false) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    }
}

/// Defines the COM library.
///
/// ```rust,ignore
/// com_library!( libid = "...", items...)]
/// ```
///
/// - `libid` - A unique ID that specifies the current intercom library.
///             Optional, the libid is generated randomly if omitted.
/// - `items` - List of items contained in this library.
///
/// The macro results in the implementation of the object creation
/// infrastructure that allows external clients to load the library and
/// instantiate the specified types.
#[proc_macro]
pub fn com_library(args: TokenStream) -> TokenStream
{
    match expand_com_module(args, true) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    }
}

/// Metadata for method signature.
///
/// ```rust,ignore
/// #[com_interface]
/// pub trait IFoo {
///     #[signature(OUT, a, b, c)]
///     fn foo(&self, a: u32, b: u32, c: u32) -> ComResult<u32>;
/// }
/// ```
#[proc_macro_attribute]
pub fn com_signature(_attr: TokenStream, tokens: TokenStream) -> TokenStream
{
    tokens
}

/// Derives the implementation of the trait ForeignType for a type.
#[proc_macro_derive(ForeignType)]
pub fn named_type_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    match expand_bidirectional_type_info(input) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    }
}

/// Derives the implementation of the trait ExternInput for a type.
#[proc_macro_derive(ExternInput)]
pub fn derive_extern_parameter(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    match expand_derive_extern_parameter(input) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    }
}

/// Derives the implementation of the trait ExternOutput for a type.
#[proc_macro_derive(ExternOutput)]
pub fn derive_extern_output(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    match expand_derive_extern_output(input) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    }
}
