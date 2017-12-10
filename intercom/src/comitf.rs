
use super::*;
use std::marker::PhantomData;

/// An incoming COM interface pointer.
///
/// Intercom will implement the various `[com_interface]` traits for the
/// corresponding `ComItf<T>` type.
///
/// This applies only to the pure interfaces.  Implicit interfaces created
/// through `#[com_interface] impl MyStruct` constructs are not supported for
/// `ComItf<T>`.
pub struct ComItf<T> where T: ?Sized {
    ptr: RawComPtr,
    phantom: PhantomData<T>,
}

impl<T> ComItf<T> where T: ?Sized {

    /// Creates a `ComItf<T>` from a raw COM interface pointer.
    ///
    /// # Safety
    ///
    /// The `ptr` __must__ be a valid COM interface pointer for an interface
    /// of type `T`.
    pub unsafe fn wrap( ptr : RawComPtr ) -> ComItf<T> {
        ComItf {
            ptr: ptr,
            phantom: PhantomData,
        }
    }

    /// Gets the raw COM pointer from the `ComItf<T>`.
    pub fn ptr( this : &Self ) -> RawComPtr { this.ptr }
}

impl<T> AsRef<ComItf<IUnknown>> for ComItf<T> where T: ?Sized {

    fn as_ref( &self ) -> &ComItf<IUnknown> {
        unsafe { &*( self as *const _ as *const ComItf<IUnknown> ) }
    }
}
