use super::*;
use crate::type_system::{AutomationTypeSystem, ExternInput, RawTypeSystem, TypeSystem};
use std::marker::PhantomData;

/// An incoming COM interface pointer.
///
/// Intercom will implement the various `[com_interface]` traits for the
/// corresponding `ComItf<T>` type.
///
/// This applies only to the pure interfaces.  Implicit interfaces created
/// through `#[com_interface] impl MyStruct` constructs are not supported for
/// `ComItf<T>`.
pub struct ComItf<T>
where
    T: ?Sized,
{
    pub(crate) raw_ptr: raw::InterfacePtr<RawTypeSystem, T>,
    pub(crate) automation_ptr: raw::InterfacePtr<AutomationTypeSystem, T>,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: ?Sized> std::fmt::Debug for ComItf<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(
            f,
            "ComItf(automation = {:?}, raw = {:?})",
            self.automation_ptr, self.raw_ptr
        )
    }
}

impl<T: ?Sized> ComItf<T>
{
    /// Creates a `ComItf<T>` from a raw type system COM interface pointer..
    ///
    /// # Safety
    ///
    /// The `ptr` __must__ be a valid COM interface pointer for an interface
    /// of type `T`.
    pub unsafe fn new(
        automation: raw::InterfacePtr<AutomationTypeSystem, T>,
        raw: raw::InterfacePtr<RawTypeSystem, T>,
    ) -> ComItf<T>
    {
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
    /// of type `T`. The `ComItf` must not outlast the reference owned by the
    /// provided pointer.
    pub unsafe fn maybe_wrap<TS: TypeSystem>(ptr: raw::InterfacePtr<TS, T>) -> Option<ComItf<T>>
    {
        if ptr.is_null() {
            None
        } else {
            Some(TS::wrap_ptr(ptr))
        }
    }

    /// Gets the raw COM pointer from the `ComItf<T>`.
    pub fn ptr<TS: TypeSystem>(this: &Self) -> raw::InterfacePtr<TS, T>
    {
        TS::get_ptr(this)
    }

    pub fn maybe_ptr<TS: TypeSystem>(this: &Self) -> Option<raw::InterfacePtr<TS, T>>
    {
        // Acquire the pointer.
        let ptr = Self::ptr(this);

        // Check for null.
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }

    /// Returns a `ComItf<T>` value that references a null pointer.
    ///
    /// # Safety
    ///
    /// The `ComItf<T>` returned by the function will be invalid for any
    /// method calls. Its purpose is to act as a return value from COM
    /// methods in the case of an error result.
    pub unsafe fn null_itf() -> ComItf<T>
    {
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
    pub fn is_null(itf: &Self) -> bool
    {
        itf.raw_ptr.is_null() && itf.automation_ptr.is_null()
    }
}

impl ComItf<dyn IUnknown>
{
    /// Tries to convert the ComRc into a different interface within a single
    /// type system. Used to implement the generic conversion method.
    fn query_interface_ts<TS: TypeSystem, TTarget: ComInterface + ?Sized>(
        &self,
    ) -> ComResult<ComRc<TTarget>>
    {
        // Try to get the IID.
        let iid = match TTarget::iid(TS::key()) {
            None => return Err(ComError::E_NOINTERFACE),
            Some(iid) => iid,
        };

        // Try to query interface using the iid.
        let iunk: &dyn IUnknown = &*self;
        match iunk.query_interface(iid) {
            Ok(ptr) => {
                // Interface was available. Convert the raw pointer into
                // a strong type-system specific InterfacePtr.
                //
                // The pointer has already been addref'd by query_interface
                // so it's safe to attach it here.
                unsafe {
                    let target_itf = raw::InterfacePtr::<TS, TTarget>::new(ptr);
                    let itf = ComItf::maybe_wrap(target_itf).ok_or_else(|| ComError::E_POINTER)?;
                    Ok(ComRc::attach(itf))
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}

impl<T: ComInterface + ?Sized> ComItf<T>
{
    /// Query interface on the ComItf.
    pub fn query_interface<TTarget: ComInterface + ?Sized>(this: &Self)
        -> ComResult<ComRc<TTarget>>
    {
        // Get the IUnknown interface.
        let iunk: &ComItf<dyn IUnknown> = this.as_ref();

        // Try every type system.
        //
        // From Rust side we don't really care which type system we end up with.
        // Both of these work for Rust calls.
        //
        // We'll try RawTypeSystem first because that has a better chance of
        // providing lower overhead calls.
        if let Ok(itf) = iunk.query_interface_ts::<RawTypeSystem, TTarget>() {
            return Ok(itf);
        }
        if let Ok(itf) = iunk.query_interface_ts::<AutomationTypeSystem, TTarget>() {
            return Ok(itf);
        }

        // If we got here, none of the query interfaces we invoked returned
        // anything.
        Err(ComError::E_NOINTERFACE)
    }

    pub fn as_rc(this: &Self) -> ComRc<T>
    {
        let iunk: &ComItf<dyn IUnknown> = this.as_ref();

        // Calling `add_ref` makes the pointer safe for attach.
        //
        // ComRc::copy, etc. depend on this function so we can't just delegate
        // this there. :)
        unsafe {
            iunk.add_ref();
            ComRc::attach(ComItf {
                automation_ptr: this.automation_ptr,
                raw_ptr: this.raw_ptr,
                phantom: PhantomData,
            })
        }
    }
}

impl<T: ComInterface + ?Sized> ToOwned for ComItf<T>
{
    type Owned = ComRc<T>;

    fn to_owned(&self) -> Self::Owned
    {
        Self::Owned::from(self)
    }
}

impl<T: ComInterface + ?Sized> std::ops::Deref for ComItf<T>
{
    type Target = T;

    fn deref(&self) -> &T
    {
        ComInterface::deref(self)
    }
}

unsafe impl<'a, TS: TypeSystem, I: crate::ComInterface + ?Sized> ExternInput<TS>
    for &'a crate::ComItf<I>
where
    I: ForeignType,
{
    type ForeignType = crate::raw::InterfacePtr<TS, I>;

    type Lease = ();
    unsafe fn into_foreign_parameter(self) -> ComResult<(Self::ForeignType, Self::Lease)>
    {
        Ok((ComItf::ptr(self), ()))
    }

    type Owned = ComItf<I>;
    unsafe fn from_foreign_parameter(source: Self::ForeignType) -> ComResult<Self::Owned>
    {
        crate::ComItf::maybe_wrap(source).ok_or_else(|| crate::ComError::E_INVALIDARG)
    }
}

#[cfg(windows)]
#[link(name = "ole32")]
extern "system" {

    #[doc(hidden)]
    pub fn CoCreateInstance(
        clsid: crate::guid::GUID,
        outer: RawComPtr,
        cls_context: u32,
        riid: crate::REFIID,
        out: &mut RawComPtr,
    ) -> crate::raw::HRESULT;
}

impl<T: ComInterface + ?Sized> AsRef<ComItf<dyn IUnknown>> for ComItf<T>
{
    fn as_ref(&self) -> &ComItf<dyn IUnknown>
    {
        unsafe { &*(self as *const _ as *const ComItf<dyn IUnknown>) }
    }
}
