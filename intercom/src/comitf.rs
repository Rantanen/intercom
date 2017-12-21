
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

impl<T: ?Sized> ComItf<T> {

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

    /// Tries to acquire a different interface from the current COM object.
    ///
    /// Returns a reference counted wrapper around the interface if successful.
    pub fn try_into< U: IidOf + ?Sized >(
        &self
    ) -> Result< ComRc<U>, ::HRESULT >
    {
        let iunk : &ComItf<IUnknown> = self.as_ref();

        match iunk.query_interface( U::iid() ) {
            Ok( ptr ) => unsafe {
                // QueryInterface is guaranteed to return ptr of correct
                // interface type, which makes the ComItf::wrap safe here.
                Ok( ComRc::attach( ComItf::<U>::wrap( ptr ) ) )
            },
            Err( e ) => Err( e )
        }
    }
}

#[cfg(windows)]
#[link(name = "ole32")]
extern "system" {

    #[doc(hidden)]
    pub fn CoCreateInstance(
        clsid : ::guid::GUID,
        outer : RawComPtr,
        cls_context: u32,
        riid : ::REFIID,
        out : &mut RawComPtr,
    ) -> ::HRESULT;
}

impl<T: ?Sized> AsRef<ComItf<IUnknown>> for ComItf<T>
{
    fn as_ref( &self ) -> &ComItf<IUnknown> {
        unsafe { &*( self as *const _ as *const ComItf<IUnknown> ) }
    }
}
