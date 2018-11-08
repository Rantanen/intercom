
use super::*;

/// Reference counted handle to the `ComBox` data.
///
/// Provides a safe way to handle the unsafe `ComBox` values.
pub struct ComRc<T : ?Sized> {
    itf : ComItf<T>
}

impl<T: ?Sized> std::fmt::Debug for ComRc<T> {
    fn fmt( &self, f : &mut std::fmt::Formatter ) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: ?Sized> Clone for ComRc<T> {
    fn clone( &self ) -> Self {
        let rc = ComRc { itf : self.itf.clone() };
        rc.itf.as_ref().release();
        rc
    }
}

// ComRc is a smart pointer and shouldn't introduce methods on 'self'.
//
// Various as_ and into_ methods here are properly implemented static methods
// which is the recommended alternative - compare this to std::Box.
#[allow(clippy::wrong_self_convention)]
impl<T : ?Sized> ComRc<T> {

    /// Attaches a floating ComItf reference and brings it under managed
    /// reference counting.
    ///
    /// Does not increment the reference count.
    pub fn attach( itf : ComItf<T> ) -> ComRc<T> {
        ComRc { itf }
    }

    /// Creates a `ComItf<T>` from a raw type system COM interface pointer..
    ///
    /// Does not increment the reference count.
    ///
    /// # Safety
    ///
    /// The `ptr` __must__ be a valid COM interface pointer for an interface
    /// of type `T`.
    pub unsafe fn wrap(
        ptr : raw::InterfacePtr<T>,
        ts : TypeSystem
    ) -> ComRc<T> {
        ComRc::attach( ComItf::wrap( ptr, ts ) )
    }
}

#[cfg(windows)]
impl<T: ComInterface + ?Sized> ComRc<T>
{
    pub fn create( clsid : GUID ) -> ::ComResult< ComRc<T> > {

        // Get the IID.
        //
        // The IID we are getting here is the Automation type system ID.
        // This is the one that plays well with Windows' CoCreateInstance, etc.
        let iid = match T::iid( TypeSystem::Automation ) {
            Some( iid ) => iid,
            None => return Err( E_NOINTERFACE ),
        };

        unsafe {

            // Invoke CoCreateInstance and return a result based on the return
            // value.  
            let mut out = ::std::ptr::null_mut();
            match CoCreateInstance(
                    clsid,
                    std::ptr::null_mut(),
                    1, // in-proc server.
                    iid,
                    &mut out ) {

                // On success construct the ComRc. We are using Automation type
                // system as that's the IID we used earlier.
                ::S_OK => Ok( ComRc::attach( ComItf::wrap(
                                raw::InterfacePtr::new( out ),
                                TypeSystem::Automation ) ) ),
                e => Err( e ),
            }
        }
    }
}

impl<T : ?Sized > ::std::ops::Deref for ::intercom::ComRc< T > {
    type Target = ComItf< T >;
    fn deref( &self ) -> &Self::Target {
        &self.itf
    }
}

impl<T : ?Sized> Drop for ComRc<T> {
    fn drop( &mut self ) {
        self.itf.as_ref().release();
    }
}

impl<T: ?Sized> AsRef<ComItf<T>> for ComRc<T> {
    fn as_ref( &self ) -> &ComItf<T> {
        &self.itf
    }
}
