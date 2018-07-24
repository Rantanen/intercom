//!
//! COM library parse model.
//!
//! Defines the items constructed from the various COM attributes.
//!
//! Should unify COM attribute expansion and crate parsing for IDL/Manifest/etc.
//! purposes in the future.
//!

mod common;
pub use self::common::*;

mod com_class;
pub use self::com_class::ComClass;

mod com_interface;
pub use self::com_interface::ComInterface;

mod com_impl;
pub use self::com_impl::ComImpl;

mod com_library;
pub use self::com_library::ComLibrary;

mod com_crate;
pub use self::com_crate::ComCrate;
