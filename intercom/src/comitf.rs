use super::*;
use crate::type_system::{AutomationTypeSystem, RawTypeSystem, TypeSystem};
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
    raw_ptr: raw::InterfacePtr<RawTypeSystem, T>,
    automation_ptr: raw::InterfacePtr<AutomationTypeSystem, T>,
    phantom: PhantomData<T>,
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

/// Trait that allows constructing strong pointer types from raw
/// pointers in a type system specific way.
trait PointerOperations: TypeSystem + Sized
{
    /// Wraps a raw interface pointer into a ComItf.
    ///
    /// # Safety
    ///
    /// The returned `ComItf` must not outlast the reference held by the pointer.
    unsafe fn wrap_ptr<I: ?Sized>(ptr: crate::raw::InterfacePtr<Self, I>) -> ComItf<I>;

    /// Gets a raw interface pointer from a ComItf.
    fn get_ptr<I: ?Sized>(itf: &ComItf<I>) -> crate::raw::InterfacePtr<Self, I>;
}

/// A generic implementation that ensures _every_ type system has some
/// implementation for this.
///
/// Note that this is required to tell Rust compiler that calls to wrap_ptr
/// are okay in any case. The actual implementation here will throw a runtime
/// panic.
///
/// This trait really needs to be specialized for each type system for it to
/// work correctly.
impl<TS: TypeSystem> PointerOperations for TS
{
    default unsafe fn wrap_ptr<I: ?Sized>(_ptr: crate::raw::InterfacePtr<Self, I>) -> ComItf<I>
    {
        panic!("Not implemented");
    }

    default fn get_ptr<I: ?Sized>(_itf: &ComItf<I>) -> crate::raw::InterfacePtr<Self, I>
    {
        panic!("Not implemented");
    }
}

impl PointerOperations for AutomationTypeSystem
{
    unsafe fn wrap_ptr<I: ?Sized>(ptr: crate::raw::InterfacePtr<Self, I>) -> ComItf<I>
    {
        // Construct a ComItf from a automation pointer.
        ComItf {
            raw_ptr: raw::InterfacePtr::null(),
            automation_ptr: ptr,
            phantom: PhantomData,
        }
    }

    fn get_ptr<I: ?Sized>(itf: &ComItf<I>) -> raw::InterfacePtr<Self, I>
    {
        // Get an automation pointer from the ComItf.
        itf.automation_ptr
    }
}

impl PointerOperations for RawTypeSystem
{
    unsafe fn wrap_ptr<I: ?Sized>(ptr: raw::InterfacePtr<Self, I>) -> ComItf<I>
    {
        // Construct a ComItf from a raw pointer.
        ComItf {
            raw_ptr: ptr,
            automation_ptr: raw::InterfacePtr::null(),
            phantom: PhantomData,
        }
    }

    fn get_ptr<I: ?Sized>(itf: &ComItf<I>) -> crate::raw::InterfacePtr<Self, I>
    {
        // Get an automation pointer form the ComItf.
        itf.raw_ptr
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
