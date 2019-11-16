//! The class factory is an infrastructure type used by the COM clients to
//! create instances of the `#[com_class(..)]`es provided by the intercom
//! library.
//!
//! Intercom implements the class factory infrastructure automatically when the
//! user specifies the `#[com_library(..)]` -attribute.

use super::*;
use crate::attributes;
use crate::raw::RawComPtr;
use crate::type_system::AutomationTypeSystem;

#[com_interface]
pub trait IClassFactory
{
    fn create_instance(&self, outer: RawComPtr, riid: REFIID) -> ComResult<RawComPtr>;

    fn lock_server(&self, lock: bool) -> ComResult<()>;
}

#[doc(hidden)]
#[com_class(IClassFactory)]
pub struct ClassFactory<T: Default + intercom::CoClass>
{
    phantom: std::marker::PhantomData<T>,
}

impl<T: Default + CoClass> IClassFactory for ClassFactory<T>
{
    fn create_instance(&self, outer: RawComPtr, riid: REFIID) -> ComResult<RawComPtr>
    {
        unsafe {
            let mut out = std::ptr::null_mut();
            let hr = ComBoxData::query_interface(ComBoxData::of(self), riid, &mut out);
            if hr == raw::S_OK {
                Ok(out)
            } else {
                Err(ComError::from(hr))
            }
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

impl<T: Default + CoClass> ClassFactory<T>
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
