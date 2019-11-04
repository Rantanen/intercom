#![crate_type = "dylib"]
#![feature(type_ascription, specialization)]

extern crate intercom;
use intercom::*;
extern crate chrono;
extern crate winapi;

pub mod alloc;
pub mod error_info;
pub mod interface_params;
pub mod primitive;
pub mod result;
pub mod return_interfaces;
pub mod stateful;
pub mod strings;
pub mod type_system_callbacks;
pub mod unicode;
pub mod variant;

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
    variant::VariantImpl,
    unicode::UnicodeConversion,
);
