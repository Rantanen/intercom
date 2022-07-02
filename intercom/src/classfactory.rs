//! The class factory is an infrastructure type used by the COM clients to
//! create instances of the `#[com_class(..)]`es provided by the intercom
//! library.
//!
//! Intercom implements the class factory infrastructure automatically when the
//! user specifies the `#[com_library(..)]` -attribute.

use super::*;
use crate::attributes;
use crate::raw::RawComPtr;

#[com_interface(
    com_iid = "00000001-0000-0000-C000-000000000046",
    raw_iid = "11111112-0000-0000-C000-000000000046"
)]
pub trait IClassFactory
{
    /// # Safety
    ///
    /// The REFIID must be a valid IID pointer.
    unsafe fn create_instance(&self, outer: RawComPtr, riid: REFIID) -> ComResult<RawComPtr>;

    fn lock_server(&self, lock: bool) -> ComResult<()>;
}

#[doc(hidden)]
#[com_class(IClassFactory)]
pub struct ClassFactory<T: Default + intercom::attributes::ComClass>
{
    phantom: std::marker::PhantomData<T>,
}

impl<T: Default + attributes::ComClass> IClassFactory for ClassFactory<T>
{
    unsafe fn create_instance(&self, _outer: RawComPtr, riid: REFIID) -> ComResult<RawComPtr>
    {
        let instance = ComBox::new(T::default());
        let mut out = std::ptr::null_mut();
        let hr = ComBoxData::query_interface(instance.as_ref(), riid, &mut out);
        if hr == raw::S_OK {
            Ok(out)
        } else {
            Err(ComError::from(hr))
        }
    }

    fn lock_server(&self, lock: bool) -> ComResult<()>
    {
        unsafe {
            if lock {
                ComBoxData::add_ref(ComBoxData::of(self));
            } else {
                ComBoxData::release(
                    ComBoxData::of(self) as *const ComBoxData<Self> as *mut ComBoxData<Self>
                );
            }
        }
        Ok(())
    }
}

impl<T: Default + attributes::ComClass> ClassFactory<T>
{
    /// # Safety
    ///
    /// The `out` pointer must be valid for receiving the requested interface.
    pub unsafe fn create(riid: intercom::REFIID, out: *mut RawComPtr)
        -> crate::error::raw::HRESULT
    {
        let factory = Self {
            phantom: std::marker::PhantomData,
        };

        intercom::ComBoxData::query_interface(intercom::ComBox::new(factory).as_mut(), riid, out)
    }
}
