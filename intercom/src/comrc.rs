
use super::*;

/// Reference counted handle to the `ComBox` data.
///
/// Provides a safe way to handle the unsafe `ComBox` values.
pub struct ComRc<T : ?Sized> {
    itf : ComItf<T>
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
}

#[cfg(windows)]
impl<T: ComInterface + ?Sized> ComRc<T>
{
    pub fn create( clsid : GUID ) -> ::ComResult< ComRc<T> > {

        let iid = match T::iid( TypeSystem::Automation ) {
            Some( iid ) => iid,
            None => return Err( E_NOINTERFACE ),
        };

        unsafe {
            let mut out = ::std::ptr::null_mut();
            match CoCreateInstance(
                    clsid,
                    std::ptr::null_mut(),
                    1, // in-proc server.
                    iid,
                    &mut out ) {

                ::S_OK => Ok( ComRc::attach( ComItf::wrap(
                                out, TypeSystem::Automation ) ) ),
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
