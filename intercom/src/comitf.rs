
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
#[fundamental]
#[repr(C)]
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

    /// Returns a `ComItf<T>` value that references a null pointer.
    ///
    /// # Safety
    ///
    /// The `ComItf<T>` returned by the function will be invalid for any
    /// method calls. Its purpose is to act as a return value from COM
    /// methods in the case of an error result.
    pub unsafe fn null_itf() -> ComItf<T> {
        ComItf {
            ptr: ::std::ptr::null_mut(),
            phantom: PhantomData,
        }
    }

    pub fn try_into< U: IidOf + ?Sized >(
        &self
    ) -> Result< ComItf<U>, ::HRESULT >
    {
        let iunk : &ComItf<IUnknown> = self.as_ref();

        match iunk.query_interface( U::iid() ) {
            Ok( ptr ) => Ok( ComItf::<U> {
                ptr: ptr,
                phantom: PhantomData
            } ),
            Err( e ) => Err( e )
        }
    }
}

impl<T> AsRef<ComItf<IUnknown>> for ComItf<T> where T: ?Sized {

    fn as_ref( &self ) -> &ComItf<IUnknown> {
        unsafe { &*( self as *const _ as *const ComItf<IUnknown> ) }
    }
}
