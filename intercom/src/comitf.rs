
use super::*;
use std::marker::PhantomData;

/// Intercom type system.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeSystem {

    /// Type system compatible with COM automation types.
    Automation,

    /// Type system using raw C types.
    Raw,
}

/// An incoming COM interface pointer.
///
/// Intercom will implement the various `[com_interface]` traits for the
/// corresponding `ComItf<T>` type.
///
/// This applies only to the pure interfaces.  Implicit interfaces created
/// through `#[com_interface] impl MyStruct` constructs are not supported for
/// `ComItf<T>`.
pub struct ComItf<T> where T: ?Sized {
    raw_ptr: raw::InterfacePtr<T>,
    automation_ptr: raw::InterfacePtr<T>,
    phantom: PhantomData<T>,
}

impl<T: ?Sized> std::fmt::Debug for ComItf<T> {
    fn fmt( &self, f : &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( f, "ComItf(automation = {:?}, raw = {:?})",
                self.automation_ptr, self.raw_ptr )
    }
}

impl<T: ?Sized> Clone for ComItf<T> {
    fn clone( &self ) -> Self {
        ComItf {
            raw_ptr: self.raw_ptr,
            automation_ptr: self.automation_ptr,
            phantom: PhantomData
        }
    }
}

impl<T: ?Sized> Copy for ComItf<T> { }

impl<T: ?Sized> ComItf<T> {

    /// Creates a `ComItf<T>` from a raw type system COM interface pointer..
    ///
    /// # Safety
    ///
    /// The `ptr` __must__ be a valid COM interface pointer for an interface
    /// of type `T`.
    pub unsafe fn new(
        automation : raw::InterfacePtr<T>,
        raw : raw::InterfacePtr<T>
    ) -> ComItf<T> {
        ComItf {
            raw_ptr: raw,
            automation_ptr: automation,
            phantom: PhantomData,
        }
    }

    /// Creates a `ComItf<T>` from a raw type system COM interface pointer..
    ///
    /// # Safety
    ///
    /// The `ptr` __must__ be a valid COM interface pointer for an interface
    /// of type `T`.
    pub unsafe fn wrap( ptr : raw::InterfacePtr<T>, ts : TypeSystem ) -> ComItf<T> {
        match ts {
            TypeSystem::Automation => ComItf {
                raw_ptr: raw::InterfacePtr::null(),
                automation_ptr: ptr,
                phantom: PhantomData,
            },
            TypeSystem::Raw => ComItf {
                raw_ptr: ptr,
                automation_ptr: raw::InterfacePtr::null(),
                phantom: PhantomData
            }
        }
    }

    /// Gets the raw COM pointer from the `ComItf<T>`.
    pub fn ptr( this : &Self, ts : TypeSystem ) -> raw::InterfacePtr<T> {
        match ts {
            TypeSystem::Automation => this.automation_ptr,
            TypeSystem::Raw => this.raw_ptr,
        }
    }

    pub fn maybe_ptr(
        this : &Self,
        ts : TypeSystem
    ) -> Option<raw::InterfacePtr<T>> {

        // Acquire the pointer.
        let ptr = match ts {
            TypeSystem::Automation => this.automation_ptr,
            TypeSystem::Raw => this.raw_ptr,
        };

        // Check for null.
        if ptr.is_null() {
            None
        } else {
            Some( ptr )
        }
    }

    /// Returns a `ComItf<T>` value that references a null pointer.
    ///
    /// # Safety
    ///
    /// The `ComItf<T>` returned by the function will be invalid for any
    /// method calls. Its purpose is to act as a return value from COM
    /// methods in the case of an error result.
    pub unsafe fn null_itf() -> ComItf<T> {
        ComItf {
            raw_ptr: raw::InterfacePtr::null(),
            automation_ptr: raw::InterfacePtr::null(),
            phantom: PhantomData,
        }
    }

    /// Checks whether the interface represents a null pointer.
    ///
    /// This should not be a case normally but may occur after certain unsafe
    /// operations.
    pub fn is_null( itf : &Self ) -> bool {
        itf.raw_ptr.is_null() && itf.automation_ptr.is_null()
    }
}

impl<T: ComInterface + ?Sized> ComItf<T> {

    // ComItf is a smart pointer and shouldn't introduce methods on 'self'.
    #[allow(clippy::wrong_self_convention)]
    pub fn as_unknown( this : &Self ) -> ComItf<dyn IUnknown> {
        ComItf {
            raw_ptr: this.raw_ptr.as_unknown(),
            automation_ptr: this.automation_ptr.as_unknown(),
            phantom: PhantomData,
        }
    }
}

impl<T: ComInterface + ?Sized, S: ComInterface + ?Sized>
        std::convert::TryFrom<&ComRc<S>> for ComRc<T> {

    type Error = ::ComError;

    fn try_from( source : &ComRc<S> ) -> Result< ComRc<T>, ::ComError >
    {
        ComRc::<T>::try_from( &**source )
    }
}

impl<T: ComInterface + ?Sized, S: ComInterface + ?Sized> std::convert::TryFrom<&ComItf<S>> for ComRc<T> {

    type Error = ::ComError;

    fn try_from( source : &ComItf<S> ) -> Result< ComRc<T>, ::ComError >
    {
        let iunk : &ComItf<dyn IUnknown> = source.as_ref();

        let mut err = None;

        // Try each type system.
        for &ts in &[ TypeSystem::Raw, TypeSystem::Automation ] {
            if let Some( iid ) = T::iid( ts ) {

                // Try to query interface using the iid.
                match iunk.query_interface( iid ) {
                    Ok( ptr ) => unsafe {

                        // QueryInterface is guaranteed to return ptr of correct
                        // interface type, which makes the ComItf::wrap safe here.
                        return Ok( ComRc::attach( ComItf::<T>::wrap(
                                raw::InterfacePtr::new( ptr ),
                                TypeSystem::Automation ) ) );
                    },
                    Err( e ) => { err = Some( e ); },
                };
            }
        }

        // If we got here, none of the query interfaces we invoked returned
        // anything.
        //
        // If 'err' is None, we didn't even get to invoke any of the query
        // interfaces. This is a case when the interface doesn't have IID
        // for any of the type systems.
        match err {
            None => Err( ::ComError::E_FAIL ),
            Some( err ) => Err( err.into() ),
        }
    }
}

impl<T: ComInterface + ?Sized> std::ops::Deref for ComItf<T> {
    type Target = T;

    fn deref( &self ) -> &T {
        ComInterface::deref( self )
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
    ) -> ::raw::HRESULT;
}

impl<T: ComInterface + ?Sized> AsRef<ComItf<dyn IUnknown>> for ComItf<T>
{
    fn as_ref( &self ) -> &ComItf<dyn IUnknown> {
        unsafe { &*( self as *const _ as *const ComItf<dyn IUnknown> ) }
    }
}
