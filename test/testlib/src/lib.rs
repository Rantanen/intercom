#![crate_type="dylib"]
#![feature(type_ascription, try_from)]


extern crate intercom;
use intercom::*;
extern crate winapi;
extern crate chrono;

pub mod primitive;
pub mod return_interfaces;
pub mod stateful;
pub mod result;
pub mod interface_params;
pub mod error_info;
pub mod alloc;
pub mod strings;
pub mod type_system_callbacks;
pub mod variant;
pub mod unicode;

// Declare available COM classes.
com_library!(
    primitive::PrimitiveOperations,
    return_interfaces::RefCountOperations,
    return_interfaces::ClassCreator,
    return_interfaces::CreatedClass,
    stateful::StatefulOperations,
    result::ResultOperations,
    interface_params::SharedImplementation,
    error_info::ErrorTests,
    alloc::AllocTests,
    strings::StringTests,
    type_system_callbacks::TypeSystemCaller,
    variant::VariantTests,
    unicode::UnicodeConversion,
);
