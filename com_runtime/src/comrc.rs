
use super::*;

/// Reference counted handle to the `ComBox` data.
///
/// Provides a safe way to handle the unsafe `ComBox` values.
pub struct ComRc< T: CoClass > {
    ptr : ptr::Shared<ComBox<T>>
}

impl<T> ComRc<T> where T : CoClass {

    /// Creates a new reference counted COM object.
    pub fn new( value : T ) -> ComRc<T> {
        ComRc {
            ptr: ptr::Shared::from( unsafe { ComBox::new_ptr( value ) } )
        }
    }

    /// Acquires a raw COM pointer to the object.
    pub fn as_ptr( this : &Self ) -> RawComPtr {
        this.ptr.as_ptr() as RawComPtr
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
        out : *mut RawComPtr
    ) -> HRESULT
    {
        // The iunknown vtable is at the start of the data.
        let vtables = ComBox::vtable( this.ptr.as_ref() );
        let iunk = vtables as *const _ as *const *const IUnknownVtbl;
        ((**iunk).query_interface)(
                this.ptr.as_ptr() as RawComPtr, iid, out )
    }
}

impl<T: CoClass> Drop for ComRc<T> {

    /// Decrements the reference count on the ComBox.
    fn drop( &mut self ) {
        unsafe { ComBox::release( self.ptr.as_ptr() ) };
    }
}

impl<T : CoClass> std::convert::Into< RawComPtr > for ComRc<T> {
    fn into(self) -> RawComPtr {
        self.ptr.as_ptr() as RawComPtr
    }
}


