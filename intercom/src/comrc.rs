
use super::*;

/// Reference counted handle to the `ComBox` data.
///
/// Provides a safe way to handle the unsafe `ComBox` values.
pub struct ComRc< T: CoClass > {
    ptr : *mut ComBox<T>
}

// ComRc is a smart pointer and shouldn't introduce methods on 'self'.
//
// Various as_ and into_ methods here are properly implemented static methods
// which is the recommended alternative - compare this to std::Box.
#[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
impl<T> ComRc<T> where T : CoClass {

    /// Creates a new reference counted COM object.
    pub fn new( value : T ) -> ComRc<T> {

        // Construct the ComBox and register the reference held by ComRc.
        let mut cb = ComBox::new( value );
        unsafe { ComBox::add_ref( &mut cb ) };
        ComRc { ptr: Box::into_raw( cb ) }
    }

    /// Acquires a raw COM pointer to the object.
    pub fn as_ptr( this : &Self ) -> RawComPtr {
        this.ptr as RawComPtr
    }

    /// Converts the ComRc into a raw COM pointer. Prevents the ref count
    /// being decremented as the ComRc goes out of scope.
    pub fn into_raw( this: Self ) -> RawComPtr {
        let ptr = this.ptr as RawComPtr;
        std::mem::forget( this );
        ptr
    }

    /// Performs a query interface operation.
    ///
    /// The operation assumes the COM object has the IUnknown virtual table
    /// pointer at the start of the data. It also assumes the output pointer
    /// points to a valid data that the interface can be written into.
    ///
    /// As the query interface results in a new reference to the COM object in
    /// the out parameter, this operation ends up incrementing the reference
    /// count. The receiver of the interface must call release on the COM object
    /// when the interface is not needed anymore.
    pub unsafe fn query_interface(
        this : &Self,
        iid : &GUID,
    ) -> ComResult<RawComPtr>
    {
        // The iunknown vtable is at the start of the data.
        let vtables = ComBox::vtable( &*this.ptr );
        let iunk = vtables as *const _ as *const *const IUnknownVtbl;
        let mut out_ptr = std::ptr::null_mut();
        let hr = ((**iunk).query_interface)(
                this.ptr as RawComPtr, iid, &mut out_ptr );

        if hr == S_OK {
            Ok( out_ptr )
        } else {
            Err( hr )
        }
    }
}

impl<T: CoClass> Drop for ComRc<T> {

    /// Decrements the reference count on the ComBox.
    fn drop( &mut self ) {
        unsafe { ComBox::release( self.ptr ) };
    }
}

impl<T : CoClass> std::convert::Into< RawComPtr > for ComRc<T> {
    fn into(self) -> RawComPtr {
        self.ptr as RawComPtr
    }
}

impl<T : CoClass> std::ops::Deref for ComRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &**self.ptr }
    }
}

