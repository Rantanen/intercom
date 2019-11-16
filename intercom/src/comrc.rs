use super::*;
use crate::type_system::{ExternInput, ExternOutput, TypeSystem};

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

impl<TS: TypeSystem, T: ComInterface + ?Sized> From<crate::raw::InterfacePtr<TS, T>> for ComRc<T>
{
    fn from(source: crate::raw::InterfacePtr<TS, T>) -> Self
    {
        // Lone ComItf doesn't outlast the pointer.
        unsafe { ComItf::as_rc(&ComItf::wrap(source)) }
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
    pub fn detach(rc: ComRc<T>) -> ComItf<T>
    {
        let itf = ComItf { ..rc.itf };
        std::mem::forget(rc);
        itf
    }

    /// Creates a `ComItf<T>` from a raw type system COM interface pointer..
    ///
    /// Does not increment the reference count.
    ///
    /// # Safety
    ///
    /// The `ptr` must be owned by us. `wrap` will not call `add_ref` on the `ptr`.
    pub unsafe fn wrap<TS: TypeSystem>(ptr: raw::InterfacePtr<TS, T>) -> ComRc<T>
    {
        ComRc::attach(ComItf::wrap(ptr))
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
                    let ptr = raw::InterfacePtr::new(out).ok_or_else(|| ComError::E_POINTER)?;
                    let comitf = ComItf::wrap::<AutomationTypeSystem>(ptr);
                    Ok(ComRc::attach(comitf))
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
        self.itf.as_raw_iunknown().release();
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

unsafe impl<TS: TypeSystem, I: crate::ComInterface + ?Sized> ExternInput<TS> for crate::ComRc<I>
where
    I: ForeignType,
{
    // `ComRc` as extern input should be non-nullable, but `NonNull` is not safe
    // to pass in from C as there are no guarantees the external code would not
    // pass in a null pointer.
    type ForeignType = Option<crate::raw::InterfacePtr<TS, I>>;

    type Lease = Self;
    unsafe fn into_foreign_parameter(self) -> ComResult<(Self::ForeignType, Self::Lease)>
    {
        match ComItf::ptr::<TS>(&self) {
            Some(ptr) => Ok((Some(ptr), self)),
            None => Err(ComError::E_POINTER),
        }
    }

    type Owned = Self;
    unsafe fn from_foreign_parameter(source: Self::ForeignType) -> ComResult<Self>
    {
        match source {
            Some(ptr) => Ok(ComRc::from(ptr)),
            None => Err(ComError::E_POINTER),
        }
    }
}

unsafe impl<TS: TypeSystem, I: crate::ComInterface + ?Sized> ExternOutput<TS> for crate::ComRc<I>
where
    I: ForeignType,
{
    type ForeignType = Option<crate::raw::InterfacePtr<TS, I>>;

    fn into_foreign_output(self) -> ComResult<Self::ForeignType>
    {
        Ok(ComItf::ptr(&ComRc::detach(self)))
    }

    unsafe fn from_foreign_output(source: Self::ForeignType) -> ComResult<Self>
    {
        match source {
            Some(ptr) => Ok(ComRc::wrap(ptr)),
            None => Err(ComError::E_POINTER),
        }
    }
}

unsafe impl<TS: TypeSystem, I: crate::ComInterface + ?Sized> ExternInput<TS>
    for Option<crate::ComRc<I>>
where
    I: ForeignType,
{
    type ForeignType = Option<crate::raw::InterfacePtr<TS, I>>;

    type Lease = Self;
    unsafe fn into_foreign_parameter(self) -> ComResult<(Self::ForeignType, Self::Lease)>
    {
        match &self {
            None => Ok((None, self)),
            Some(rc) => match ComItf::ptr::<TS>(rc) {
                None => Err(ComError::E_POINTER),
                Some(ptr) => Ok((Some(ptr), self)),
            },
        }
    }

    type Owned = Self;
    unsafe fn from_foreign_parameter(source: Self::ForeignType) -> ComResult<Self>
    {
        Ok(match source {
            Some(ptr) => Some(ComRc::from(ptr)),
            None => None,
        })
    }
}

unsafe impl<TS: TypeSystem, I: crate::ComInterface + ?Sized> ExternOutput<TS>
    for Option<crate::ComRc<I>>
where
    I: ForeignType,
{
    type ForeignType = Option<crate::raw::InterfacePtr<TS, I>>;

    fn into_foreign_output(self) -> ComResult<Self::ForeignType>
    {
        match self {
            None => Ok(None),
            Some(rc) => match ComItf::ptr::<TS>(&ComRc::detach(rc)) {
                None => Err(ComError::E_POINTER),
                Some(ptr) => Ok(Some(ptr)),
            },
        }
    }

    unsafe fn from_foreign_output(source: Self::ForeignType) -> ComResult<Self>
    {
        Ok(match source {
            Some(ptr) => Some(ComRc::wrap(ptr)),
            None => None,
        })
    }
}
