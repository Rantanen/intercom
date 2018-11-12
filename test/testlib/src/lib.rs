#![crate_type="dylib"]
#![feature(type_ascription, try_from)]


extern crate intercom;
use intercom::*;
extern crate winapi;
extern crate chrono;

// #[com_library] does not allow path references so all of the types must be
// in scope here.
pub mod primitive; use primitive::*;
pub mod return_interfaces; use return_interfaces::*;
pub mod stateful; use stateful::*;
pub mod result; use result::*;
pub mod interface_params; use interface_params::*;
pub mod error_info; use error_info::*;
pub mod alloc; use alloc::*;
pub mod strings; use strings::*;
pub mod type_system_callbacks; use type_system_callbacks::*;
pub mod variant; use variant::*;
pub mod unicode; use unicode::*;

// Declare available COM classes.
#[com_library(
    RefCountOperations,
    PrimitiveOperations,
    StatefulOperations,
    ResultOperations,
    ClassCreator,
    CreatedClass,
    SharedImplementation,
    ErrorSource,
    AllocTests,
    StringTests,
    TypeSystemCaller,
    VariantTests,
    UnicodeConversion,
)]
#[allow(dead_code)]  // #[com_library] requires an item so we need one here for now.
struct S;

