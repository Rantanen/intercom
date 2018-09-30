
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
    raw_ptr: RawComPtr,
    automation_ptr: RawComPtr,
    phantom: PhantomData<T>,
}

impl<T: ?Sized> ComItf<T> {

    /// Creates a `ComItf<T>` from a raw type system COM interface pointer..
    ///
    /// # Safety
    ///
    /// The `ptr` __must__ be a valid COM interface pointer for an interface
    /// of type `T`.
    pub unsafe fn wrap( ptr : RawComPtr, ts : TypeSystem ) -> ComItf<T> {
        match ts {
            TypeSystem::Automation => ComItf {
                raw_ptr: ::std::ptr::null_mut(),
                automation_ptr: ptr,
                phantom: PhantomData,
            },
            TypeSystem::Raw => ComItf {
                raw_ptr: ptr,
                automation_ptr: ::std::ptr::null_mut(),
                phantom: PhantomData
            }
        }
    }

    /// Gets the raw COM pointer from the `ComItf<T>`.
    pub fn ptr( this : &Self, ts : TypeSystem ) -> raw::InterfacePtr<T> {
        raw::InterfacePtr::new( match ts {
            TypeSystem::Automation => this.automation_ptr,
            TypeSystem::Raw => this.raw_ptr,
        } )
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
            raw_ptr: ::std::ptr::null_mut(),
            automation_ptr: ::std::ptr::null_mut(),
            phantom: PhantomData,
        }
    }

    /// Tries to acquire a different interface from the current COM object.
    ///
    /// Returns a reference counted wrapper around the interface if successful.
    pub fn try_into< U: ComInterface + ?Sized >(
        &self
    ) -> Result< ComRc<U>, ::HRESULT >
    {
        let iunk : &ComItf<IUnknown> = self.as_ref();

        let mut err = None;
        
        // Try each type system.
        for &ts in &[ TypeSystem::Raw, TypeSystem::Automation ] {
            if let Some( iid ) = U::iid( ts ) {

                // Try to query interface using the iid.
                match iunk.query_interface( iid ) {
                    Ok( ptr ) => unsafe {
                        // QueryInterface is guaranteed to return ptr of correct
                        // interface type, which makes the ComItf::wrap safe here.
                        return Ok( ComRc::attach( ComItf::<U>::wrap(
                                ptr, TypeSystem::Automation ) ) );
                    },
                    Err( e ) => { err = Some( e ); },
                };
            }
        }

        match err {
            None => Err( E_FAIL ),
            Some( err ) => Err( err ),
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
    ) -> ::HRESULT;
}

impl<T: ?Sized> AsRef<ComItf<IUnknown>> for ComItf<T>
{
    fn as_ref( &self ) -> &ComItf<IUnknown> {
        unsafe { &*( self as *const _ as *const ComItf<IUnknown> ) }
    }
}
