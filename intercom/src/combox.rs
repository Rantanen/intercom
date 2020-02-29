use super::*;
use crate::attributes::{ComClass, ComInterface, HasInterface};
use crate::raw::RawComPtr;
use crate::type_system::TypeSystemName;
use std::sync::atomic::{AtomicU32, Ordering};

/// Pointer to a COM-enabled Rust struct.
///
/// Intercom requires a specific memory layout for the COM objects so that it
/// can implement reference counting and map COM method calls back to the
/// target struct instance.
///
/// This is done by requiring each COM-enabled Rust object is constructed
/// through a `ComBox<T>` type.
///
/// Technically the memory layout is specified by the [`ComBoxData`](struct.ComBoxData.html)
/// type, however that type shouldn't be needed by the user. For all intents
/// the `ComBox` type is _the_ COM-compatible object handle.
pub struct ComBox<T: ComClass>
{
    data: *mut ComBoxData<T>,
}

impl<T: ComClass> ComBox<T>
{
    /// Constructs a new `ComBox` by placing the `value` into reference
    /// counted memory.
    ///
    /// - `value` - The initial state to use for the COM object.
    pub fn new(value: T) -> ComBox<T>
    {
        // Construct a ComBoxData in memory and track the reference on it.
        let cb = ComBoxData::new(value);
        unsafe { ComBoxData::add_ref(&*cb) };

        // Return the struct.
        ComBox { data: cb }
    }

    /// Acquires a ComItf for this struct.
    ///
    /// # Safety
    ///
    /// The ComItf must not outlive the current instance without invoking
    /// `add_ref`.
    unsafe fn as_comitf<I: ComInterface + ?Sized>(&self) -> ComItf<I>
    where
        T: HasInterface<I>,
    {
        let (automation_ptr, raw_ptr) = {
            let vtbl = &self.as_ref().vtable_list;

            let automation_ptr = match I::iid(TypeSystemName::Automation) {
                Some(iid) => match <T as ComClass>::query_interface(&vtbl, iid) {
                    Ok(itf) => itf,
                    Err(_) => ::std::ptr::null_mut(),
                },
                None => ::std::ptr::null_mut(),
            };

            let raw_ptr = match I::iid(TypeSystemName::Raw) {
                Some(iid) => match <T as ComClass>::query_interface(&vtbl, iid) {
                    Ok(itf) => itf,
                    Err(_) => ::std::ptr::null_mut(),
                },
                None => ::std::ptr::null_mut(),
            };

            (automation_ptr, raw_ptr)
        };

        ComItf::maybe_new(
            raw::InterfacePtr::new(automation_ptr),
            raw::InterfacePtr::new(raw_ptr),
        )
        .expect("Intercom failed to create interface pointers")
    }
}

impl<T: ComClass + std::fmt::Debug> std::fmt::Debug for ComBox<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "ComBox(")?;
        self.as_ref().fmt(f)?;
        write!(f, ")")
    }
}

impl<T: ComClass> Drop for ComBox<T>
{
    /// Decreases the reference count by one. If this is the last reference
    /// the memory will be deallocated.
    fn drop(&mut self)
    {
        unsafe { ComBoxData::release(self.data) };
    }
}

impl<T: ComClass> AsMut<ComBoxData<T>> for ComBox<T>
{
    fn as_mut(&mut self) -> &mut ComBoxData<T>
    {
        // 'data' should always be valid pointer.
        unsafe { self.data.as_mut().expect("ComBox had null reference") }
    }
}

impl<T: ComClass> AsRef<ComBoxData<T>> for ComBox<T>
{
    fn as_ref(&self) -> &ComBoxData<T>
    {
        // 'data' should always be valid pointer.
        unsafe { self.data.as_ref().expect("ComBox had null reference") }
    }
}

impl<I: ComInterface + ?Sized, T: HasInterface<I>> From<ComBox<T>> for ComRc<I>
{
    fn from(source: ComBox<T>) -> ComRc<I>
    {
        // as_comitf does not change the reference count so attach is safe
        // as long as we forget the source so it won't release it either.
        unsafe {
            let rc = ComRc::attach(source.as_comitf());
            std::mem::forget(source);
            rc
        }
    }
}

impl<I: ComInterface + ?Sized, T: HasInterface<I>> From<&ComBox<T>> for ComRc<I>
{
    fn from(combox: &ComBox<T>) -> Self
    {
        // The ComItf temporary doesn't outlive self, making this safe.
        unsafe { ComRc::from(&combox.as_comitf()) }
    }
}

/// Type factory for the concrete COM coclass types.
///
/// Includes the virtual tables required for COM method invocations, reference
/// count required for `IUnknown` implementation and the custom value struct
/// required for any user defined interfaces.
///
/// While this struct is available for manual handling of raw COM interface
/// pointers, it's worth realizing that it's use is inherently unsafe. Most of
/// the methods implemented for the type come with conditions that Rust isn't
/// able to enforce.
///
/// The methods that handle `RawComPtr` types must only be invoked with correct
/// pointer values. There's no type checking for the pointers and the `ComBoxData`
/// will make serious assumptions on the pointers passed in.
///
/// Furthermore the `new_ptr` constructor and the `IUnknown` methods `add_ref`
/// and `release` must be used correctly together. Failure to do so will result
/// either in memory leaks or access to dangling pointers.
#[repr(C)]
pub struct ComBoxData<T: ComClass>
{
    vtable_list: T::VTableList,
    ref_count: AtomicU32,
    value: T,
}

impl<T: ComClass> ComBoxData<T>
{
    /// Creates a new ComBoxData and returns a pointer to it.
    ///
    /// The box is initialized with a reference count of zero. In most cases
    /// the ComBoxData creation is followed by query_interface, which increments the
    /// ref_count.
    ///
    /// The value should be cleaned by calling 'release'.
    pub fn new(value: T) -> *mut ComBoxData<T>
    {
        // TODO: Fix this to use raw heap allocation at some point. There's
        // no need to construct and immediately detach a Box.
        Box::into_raw(Box::new(ComBoxData {
            vtable_list: T::VTABLE,
            ref_count: AtomicU32::new(0),
            value,
        }))
    }

    /// Acquires a specific interface pointer.
    ///
    /// Increments the reference count to include the reference through the
    /// returned interface pointer.
    ///
    /// The acquired interface must be released explicitly when not needed
    /// anymore.
    ///
    /// # Safety
    ///
    /// The `out` pointer must be valid for writing the interface pointer to.
    pub unsafe fn query_interface(this: &Self, riid: REFIID, out: *mut RawComPtr) -> raw::HRESULT
    {
        match T::query_interface(&this.vtable_list, riid) {
            Ok(ptr) => {
                *out = ptr;
                Self::add_ref(this);
                raw::S_OK
            }
            Err(e) => {
                *out = std::ptr::null_mut();
                e
            }
        }
    }

    /// Increments the reference count.
    ///
    /// Returns the reference count after the increment.
    ///
    /// # Safety
    ///
    /// The method isn't technically unsafe in regard to Rust unsafety, but
    /// it's marked as unsafe to discourage it's use due to high risks of
    /// memory leaks.
    pub unsafe fn add_ref(this: &Self) -> u32
    {
        let previous_value = this.ref_count.fetch_add(1, Ordering::Relaxed);
        previous_value + 1
    }

    /// Gets the reference count of the object.
    pub fn get_ref_count(&self) -> u32
    {
        self.ref_count.load(Ordering::Relaxed)
    }

    /// Decrements the reference count. Destroys the object if the count reaches
    /// zero.
    ///
    /// Returns the reference count after the release.
    ///
    /// # Safety
    ///
    /// The pointer must be valid and not previously released. After the call
    /// completes, the struct may have been deallocated and the pointer should
    /// be considered dangling.
    pub unsafe fn release(this: *mut Self) -> u32
    {
        // Ensure we're not releasing an interface that has no references.
        //
        // Note: If the interface has no references, it has already been
        // dropped. As a result we can't guarantee that it's ref_count stays
        // as zero as the memory could have been reallocated for something else.
        //
        // However this is still an effective check in the case where the client
        // attempts to release a com pointer twice and the memory hasn't been
        // reused.
        //
        // It might not be deterministic, but in the cases where it triggers
        // it's way better than the access violation error we'd otherwise get.
        if (*this).ref_count.load(Ordering::Relaxed) == 0 {
            panic!("Attempt to release pointer with no references.");
        }

        // Decrease the ref count and store a copy of it. We'll need a local
        // copy for a return value in case we end up dropping the ComBoxData
        // instance. after the drop referencing *this would be undeterministic.
        let previous_value = (*this).ref_count.fetch_sub(1, Ordering::Relaxed);
        let rc = previous_value - 1;

        // If that was the last reference we can drop self. Do this by giving
        // it back to a box and then dropping the box. This should reverse the
        // allocation we did by boxing the value in the first place.
        if rc == 0 {
            drop(Box::from_raw(this));
        }
        rc
    }

    /// Converts a RawComPtr to a ComBoxData reference.
    ///
    /// # Safety
    ///
    /// The method is unsafe in two different ways:
    ///
    /// - There is no way for the method to ensure the RawComPtr points to
    ///   a valid ComBoxData<T> instance. It's the caller's responsibility to
    ///   ensure the method is not called with invalid pointers.
    ///
    /// - As the pointers have no lifetime tied to them, the borrow checker
    ///   is unable to enforce the lifetime of the ComBoxData reference. If the
    ///   ComBoxData is free'd by calling release on the pointer, the ComBoxData
    ///   reference will still reference the old, now free'd value. The caller
    ///   must ensure that the returned reference won't be used in case the
    ///   ComBoxData is released.
    pub unsafe fn from_ptr<'a>(ptr: RawComPtr) -> &'a mut ComBoxData<T>
    {
        &mut *(ptr as *mut ComBoxData<T>)
    }

    /// Returns a reference to the virtual table on the ComBoxData.
    pub fn vtable(ct: &ComBox<T>) -> &T::VTableList
    {
        unsafe { &(*ct.data).vtable_list }
    }

    /// Gets the ComBoxData holding the value.
    ///
    /// # Safety
    ///
    /// This is unsafe for two reasons:
    ///
    /// - There is no way for the method to check that the value is actually
    ///   contained in a `ComBoxData`. It is up to the caller to ensure this method
    ///   is only called with values that exist within a `ComBoxData`.
    ///
    /// - The method returns a mutable reference to the ComBoxData containing the
    ///   value. As demonstrated by the parameter type, the caller already has
    ///   a mutable reference to the value itself. As a result the caller will
    ///   end up with two different mutable references to the value - the direct
    ///   one given as a parameter and an indirect one available through the
    ///   return value. The caller should not attempt to access the value data
    ///   through the returned `ComBoxData` reference.
    pub unsafe fn of(value: &T) -> &ComBoxData<T>
    {
        // Resolve the offset of the 'value' field.
        let null_combox = std::ptr::null() as *const ComBoxData<T>;
        let value_offset = &((*null_combox).value) as *const _ as usize;

        let combox_loc = value as *const T as usize - value_offset;
        &mut *(combox_loc as *mut ComBoxData<T>)
    }

    /// Gets the ComBoxData holding the value.
    ///
    /// # Safety
    ///
    /// This is unsafe for two reasons:
    ///
    /// - There is no way for the method to check that the value is actually
    ///   contained in a `ComBoxData`. It is up to the caller to ensure this method
    ///   is only called with values that exist within a `ComBoxData`.
    ///
    /// - The method returns a mutable reference to the ComBoxData containing the
    ///   value. As demonstrated by the parameter type, the caller already has
    ///   a mutable reference to the value itself. As a result the caller will
    ///   end up with two different mutable references to the value - the direct
    ///   one given as a parameter and an indirect one available through the
    ///   return value. The caller should not attempt to access the value data
    ///   through the returned `ComBoxData` reference.
    pub unsafe fn of_mut(value: &mut T) -> &mut ComBoxData<T>
    {
        // Resolve the offset of the 'value' field.
        let null_combox = std::ptr::null() as *const ComBoxData<T>;
        let value_offset = &((*null_combox).value) as *const _ as usize;

        let combox_loc = value as *mut T as usize - value_offset;
        &mut *(combox_loc as *mut ComBoxData<T>)
    }

    /// Returns a reference to a null-ComBoxData vtable pointer list.
    ///
    /// # Safety
    ///
    /// **The reference itself is invalid and must not be dereferenced.**
    ///
    /// The reference may be used to further get references to the various
    /// VTableList fields to resolve offset values between the various VTable
    /// pointers and the actual `ComBoxData` containing these pointers.
    #[inline]
    pub unsafe fn null_vtable() -> &'static T::VTableList
    {
        let null_combox = std::ptr::null() as *const ComBoxData<T>;
        &(*null_combox).vtable_list
    }
}

impl<T> std::ops::Deref for ComBoxData<T>
where
    T: ComClass,
{
    type Target = T;
    fn deref(&self) -> &T
    {
        &self.value
    }
}

impl<T> std::ops::DerefMut for ComBoxData<T>
where
    T: ComClass,
{
    fn deref_mut(&mut self) -> &mut T
    {
        &mut self.value
    }
}

impl<T> std::ops::Deref for ComBox<T>
where
    T: ComClass,
{
    type Target = T;
    fn deref(&self) -> &T
    {
        unsafe { &(*self.data).value }
    }
}

impl<T> std::ops::DerefMut for ComBox<T>
where
    T: ComClass,
{
    fn deref_mut(&mut self) -> &mut T
    {
        unsafe { &mut (*self.data).value }
    }
}

impl<T: Default + ComClass> Default for ComBox<T>
{
    fn default() -> Self
    {
        ComBox::new(T::default())
    }
}
