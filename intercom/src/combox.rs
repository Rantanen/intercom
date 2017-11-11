
use super::*;

/// Trait required by any COM coclass type.
///
/// Used to specify the virtual table for the ComBox.
pub trait CoClass {
    type VTableList: AsRef<IUnknownVtbl>;
    fn create_vtable_list() -> Self::VTableList;
    fn query_interface(
        vtables : &Self::VTableList,
        riid : REFIID,
    ) -> ComResult< RawComPtr >;
}

/// Type factory for the concrete COM coclass types.
///
/// Includes the virtual tables required for COM method invocations, reference
/// count required for IUnknown implementation and the custom value struct
/// required for any user defined interfaces.
///
/// While this struct is available for manual handling of raw COM interface
/// pointers, it's worth realizing that it's use is inherently unsafe. Most of
/// the methods implemented for the type come with conditions that Rust isn't
/// able to enforce.
///
/// The methods that handle `RawComPtr` types must only be invoked with correct
/// pointer values. There's no type checking for the pointers and the ComBox
/// will make serious assumptions on the pointers passed in.
///
/// Furthermore the `new_ptr` constructor and the IUnknown methods `add_ref`
/// and `release` must be used correctly together. Failure to do so will result
/// either in memory leaks or access to dangling pointers.
#[repr(C)]
pub struct ComBox< T: CoClass > {
    vtable_list : T::VTableList,
    ref_count : u32,
    value: T,
}

impl<T: CoClass> ComBox<T> {

    /// Creates a new ComBox and returns a pointer to it.
    ///
    /// The box is initialized with a reference count of zero. In most cases
    /// the ComBox creation is followed by query_interface, which increments the
    /// ref_count.
    ///
    /// The value should be cleaned by calling 'release'.
    pub fn new( value : T ) -> Box<ComBox<T>> {
        Box::new( ComBox {
            vtable_list: T::create_vtable_list(),
            ref_count: 0,
            value: value,
        } )
    }

    pub fn as_comptr( &self ) -> RawComPtr
    {
        ( &self.vtable_list as &AsRef<IUnknownVtbl> ).as_ref()
                as *const IUnknownVtbl
                as *mut IUnknownVtbl
                as RawComPtr
    }

    /// Acquires a specific interface pointer.
    ///
    /// Increments the reference count to include the reference through the
    /// returned interface pointer.
    ///
    /// The acquired interface must be released explicitly when not needed
    /// anymore.
    ///
    /// The method isn't technically unsafe in regard to Rust unsafety, but
    /// it's marked as unsafe to discourage it's use due to high risks of
    /// memory leaks.
    pub unsafe fn query_interface(
        this : &mut Self,
        riid : REFIID,
        out : *mut RawComPtr,
    ) -> HRESULT {

        match T::query_interface( &this.vtable_list, riid ) {
            Ok( ptr ) => { *out = ptr; Self::add_ref( this ); S_OK },
            Err( e ) => { *out = std::ptr::null_mut(); e },
        }
    }

    /// Increments the reference count.
    ///
    /// Returns the reference count after the increment.
    ///
    /// The method isn't technically unsafe in regard to Rust unsafety, but
    /// it's marked as unsafe to discourage it's use due to high risks of
    /// memory leaks.
    pub unsafe fn add_ref( this : &mut Self ) -> u32 {
        this.ref_count += 1;
        this.ref_count
    }

    /// Gets the reference count of the object.
    pub fn get_ref_count( &self ) -> u32 {
        self.ref_count
    }

    /// Decrements the reference count. Destroys the object if the count reaches
    /// zero.
    ///
    /// Returns the reference count after the release.
    pub unsafe fn release( this : *mut Self ) -> u32 {

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
        if (*this).ref_count == 0 {
            panic!( "Attempt to release pointer with no references." );
        }

        // Decrease the ref count and store a copy of it. We'll need a local
        // copy for a return value in case we end up dropping the ComBox
        // instance. after the drop referencing *this would be undeterministic.
        (*this).ref_count -= 1;
        let rc = (*this).ref_count;

        // If that was the last reference we can drop self. Do this by giving
        // it back to a box and then dropping the box. This should reverse the
        // allocation we did by boxing the value in the first place.
        if rc == 0 { drop( Box::from_raw( this ) ); }
        rc
    }

    /// Converts a RawComPtr to a ComBox reference.
    ///
    /// The method is unsafe in two different ways
    ///
    /// - There is no way for the method to ensure the RawComPtr points to
    ///   a valid ComBox<T> instance. It's the caller's responsibility to
    ///   ensure the method is not called with invalid pointers.
    ///   
    /// - As the pointers have no lifetime tied to them, the borrow checker
    ///   is unable to enforce the lifetime of the ComBox reference. If the
    ///   ComBox is free'd by calling release on the pointer, the ComBox
    ///   reference will still reference the old, now free'd value. The caller
    ///   must ensure that the returned reference won't be used in case the
    ///   ComBox is released.
    pub unsafe fn from_ptr<'a>( ptr : RawComPtr ) -> &'a mut ComBox< T >
    {
        &mut *( ptr as *mut ComBox< T > )
    }

    /// Pointer variant of the `query_interface` function.
    pub unsafe extern "stdcall" fn query_interface_ptr(
        self_iunk : RawComPtr,
        riid : REFIID,
        out : *mut RawComPtr,
    ) -> HRESULT
    {
        ComBox::query_interface( ComBox::<T>::from_ptr( self_iunk ), riid, out )
    }

    /// Pointer variant of the `add_ref` function.
    pub unsafe extern "stdcall" fn add_ref_ptr(
        self_iunk : RawComPtr
    ) -> u32
    {
        ComBox::add_ref( ComBox::<T>::from_ptr( self_iunk ) )
    }

    /// Pointer variant of the `release` function.
    pub unsafe extern "stdcall" fn release_ptr(
        self_iunk : RawComPtr
    ) -> u32
    {
        ComBox::release( self_iunk as *mut ComBox< T > )
    }

    /// Returns a reference to the virtual on the ComBox.
    pub unsafe fn vtable( this : &Self ) -> &T::VTableList {
        &this.vtable_list
    }

    pub fn iunknown( this : &Self ) -> &IUnknownVtbl {
        this.vtable_list.as_ref()
    }

    /// Gets the ComBox holding the value.
    ///
    /// This is unsafe for two reasons:
    /// - There is no way for the method to check that the value is actually
    ///   contained in a `ComBox`. It is up to the caller to ensure this method
    ///   is only called with values that exist within a `ComBox`.
    /// - The method returns a mutable reference to the ComBox containing the
    ///   value. As demonstrated by the parameter type, the caller already has
    ///   a mutable reference to the value itself. As a result the caller will
    ///   end up with two different mutable references to the value - the direct
    ///   one given as a parameter and an indirect one available through the
    ///   return value. The caller should not attempt to access the value data
    ///   through the returned `ComBox` reference.
    pub unsafe fn of( value : &T ) -> &ComBox< T > {

        // Resolve the offset of the 'value' field.
        let null_combox = std::ptr::null() as *const ComBox<T>;
        let value_offset = 
            &( (*null_combox).value ) as *const _ as usize;

        let combox_loc = value as *const T as usize - value_offset;
        &mut *( combox_loc as *mut ComBox< T > )
    }

    /// Gets the ComBox holding the value.
    ///
    /// This is unsafe for two reasons:
    /// - There is no way for the method to check that the value is actually
    ///   contained in a `ComBox`. It is up to the caller to ensure this method
    ///   is only called with values that exist within a `ComBox`.
    /// - The method returns a mutable reference to the ComBox containing the
    ///   value. As demonstrated by the parameter type, the caller already has
    ///   a mutable reference to the value itself. As a result the caller will
    ///   end up with two different mutable references to the value - the direct
    ///   one given as a parameter and an indirect one available through the
    ///   return value. The caller should not attempt to access the value data
    ///   through the returned `ComBox` reference.
    pub unsafe fn of_mut( value : &mut T ) -> &mut ComBox< T > {

        // Resolve the offset of the 'value' field.
        let null_combox = std::ptr::null() as *const ComBox<T>;
        let value_offset = 
            &( (*null_combox).value ) as *const _ as usize;

        let combox_loc = value as *mut T as usize - value_offset;
        &mut *( combox_loc as *mut ComBox< T > )
    }

    /// Returns a reference to a null-ComBox vtable pointer list.
    ///
    /// **The reference itself is invalid and must not be dereferenced.**
    ///
    /// The reference may be used to further get references to the various
    /// VTableList fields to resolve offset values between the various VTable
    /// pointers and the actual `ComBox` containing these pointers.
    #[inline(always)]
    pub unsafe fn null_vtable() -> &'static T::VTableList {
        let null_combox =
                std::ptr::null() as *const ComBox< T >;
        &(*null_combox).vtable_list
    }
}

impl<T> std::ops::Deref for ComBox<T> where T: CoClass {
    type Target = T;
    fn deref( &self ) -> &T { &self.value }
}

impl<T> std::ops::DerefMut for ComBox<T> where T: CoClass {
    fn deref_mut( &mut self ) -> &mut T { &mut self.value }
}

