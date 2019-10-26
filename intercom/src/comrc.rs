use super::*;
use crate::type_system::TypeSystem;

/// Reference counted handle to the `ComBox` data.
///
/// Provides a safe way to handle the unsafe `ComBox` values.
pub struct ComRc<T: ComInterface + ?Sized>
{
    itf: ComItf<T>,
}

impl<T: ComInterface + ?Sized> std::fmt::Debug for ComRc<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        (**self).fmt(f)
    }
}

impl<T: ComInterface + ?Sized> Clone for ComRc<T>
{
    fn clone(&self) -> Self
    {
        ComRc::from(&self.itf)
    }
}

impl<T: ComInterface + ?Sized> From<&ComItf<T>> for ComRc<T>
{
    fn from(source: &ComItf<T>) -> Self
    {
        ComItf::as_rc(source)
    }
}

impl<T: ComInterface + ?Sized> ComRc<T>
{
    /// Attaches a floating ComItf reference and brings it under managed
    /// reference counting.
    ///
    /// Does not increment the reference count.
    ///
    /// # Safety
    ///
    /// Given this does not increment the reference count, it must be used
    /// only if the `ComItf` is one that would leave the reference dangling.
    /// If something already owns the `ComItf` and will do `release` on it,
    /// the drop of the returned `ComRc` will result in a double-release.
    pub unsafe fn attach(itf: ComItf<T>) -> ComRc<T>
    {
        ComRc { itf }
    }

    /// Attaches a floating ComItf reference and brings it under managed
    /// reference counting.
    ///
    /// Does not increment the reference count.
    pub fn detach(mut rc: ComRc<T>) -> ComItf<T>
    {
        unsafe { std::mem::replace(&mut rc.itf, ComItf::null_itf()) }
    }

    /// Creates a `ComItf<T>` from a raw type system COM interface pointer..
    ///
    /// Does not increment the reference count.
    ///
    /// # Safety
    ///
    /// The `ptr` __must__ be a valid COM interface pointer for an interface
    /// of type `T`.
    pub unsafe fn wrap<TS: TypeSystem>(ptr: raw::InterfacePtr<TS, T>) -> Option<ComRc<T>>
    {
        ComItf::maybe_wrap(ptr).map(|itf| ComRc::attach(itf))
    }
}

#[cfg(windows)]
impl<T: ComInterface + ?Sized> ComRc<T>
{
    pub fn create(clsid: GUID) -> crate::ComResult<ComRc<T>>
    {
        // Only needed on Windows so have these here.
        use crate::type_system::{AutomationTypeSystem, TypeSystemName};

        // Get the IID.
        //
        // The IID we are getting here is the Automation type system ID.
        // This is the one that plays well with Windows' CoCreateInstance, etc.
        let iid = match T::iid(TypeSystemName::Automation) {
            Some(iid) => iid,
            None => return Err(ComError::E_NOINTERFACE),
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
                &mut out,
            ) {
                // On success construct the ComRc. We are using Automation type
                // system as that's the IID we used earlier.
                crate::raw::S_OK => {
                    // Wrap the pointer into ComItf. This takes care of null checks.
                    let itf =
                        ComItf::maybe_wrap::<AutomationTypeSystem>(raw::InterfacePtr::new(out))
                            .ok_or_else(|| ComError::E_POINTER)?;

                    Ok(ComRc::attach(itf))
                }
                e => Err(e.into()),
            }
        }
    }
}

impl<T: ComInterface + ?Sized> ::std::ops::Deref for ComRc<T>
{
    type Target = ComItf<T>;
    fn deref(&self) -> &Self::Target
    {
        &self.itf
    }
}

impl<T: ComInterface + ?Sized> Drop for ComRc<T>
{
    fn drop(&mut self)
    {
        if !ComItf::is_null(&self.itf) {
            self.itf.as_ref().release();
        }
    }
}

impl<T: ComInterface + ?Sized> AsRef<ComItf<T>> for ComRc<T>
{
    fn as_ref(&self) -> &ComItf<T>
    {
        &self.itf
    }
}

impl<T: ComInterface + ?Sized> std::borrow::Borrow<ComItf<T>> for ComRc<T>
{
    fn borrow(&self) -> &ComItf<T>
    {
        self.as_ref()
    }
}
