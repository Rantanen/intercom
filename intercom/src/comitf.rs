
use super::*;
use std::marker::PhantomData;

pub struct ComItf<T> where T: ?Sized {
    ptr: RawComPtr,
    phantom: PhantomData<T>,
}

impl<T> ComItf<T> where T: ?Sized {
    pub fn wrap( ptr : RawComPtr ) -> ComItf<T> {
        ComItf {
            ptr: ptr,
            phantom: PhantomData,
        }
    }

    pub fn ptr( this : &Self ) -> RawComPtr { this.ptr }
}

impl<T> AsRef<ComItf<IUnknown>> for ComItf<T> where T: ?Sized {

    fn as_ref( &self ) -> &ComItf<IUnknown> {
        unsafe { &*( self as *const _ as *const ComItf<IUnknown> ) }
    }
}
