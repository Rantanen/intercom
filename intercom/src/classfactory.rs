//! The class factory is an infrastructure type used by the COM clients to
//! create instances of the `#[com_class(..)]`es provided by the intercom
//! library.
//!
//! Intercom implements the class factory infrastructure automatically when the
//! user specifies the `#[com_library(..)]` -attribute.

use super::*;
use crate::attributes;
use crate::type_system::AutomationTypeSystem;

type IUnknownVtbl = <dyn crate::IUnknown as attributes::ComInterface<
    intercom::type_system::AutomationTypeSystem,
>>::VTable;

#[allow(non_camel_case_types)]
#[doc(hidden)]
pub struct ClassFactoryVtbl
{
    pub __base: IUnknownVtbl,
    pub create_instance:
        unsafe extern "system" fn(RawComPtr, RawComPtr, REFIID, *mut RawComPtr) -> raw::HRESULT,
    pub lock_server: unsafe extern "system" fn(RawComPtr, bool) -> raw::HRESULT,
}

#[doc(hidden)]
pub struct ClassFactory<T: Default + CoClass>
{
    phantom: std::marker::PhantomData<T>,
}

impl<T: Default + CoClass> CoClass for ClassFactory<T>
{
    type VTableList = &'static ClassFactoryVtbl;
    fn create_vtable_list() -> Self::VTableList
    {
        ClassFactory::<T>::create_vtable()
    }

    fn query_interface(vtables: &Self::VTableList, riid: REFIID) -> RawComResult<RawComPtr>
    {
        if riid.is_null() {
            return Err(raw::E_NOINTERFACE);
        }
        unsafe {
            let riid = &*riid;
            if riid == IUnknown::iid_ts::<AutomationTypeSystem>() || *riid == IID_IClassFactory {
                Ok(vtables as *const _ as RawComPtr)
            } else {
                Err(raw::E_NOINTERFACE)
            }
        }
    }

    fn interface_supports_error_info(_riid: REFIID) -> bool
    {
        false
    }
}

impl AsRef<IUnknownVtbl> for ClassFactoryVtbl
{
    fn as_ref(&self) -> &IUnknownVtbl
    {
        &self.__base
    }
}

impl<T: Default + CoClass> ClassFactory<T>
{
    /// # Safety
    ///
    /// The `out` pointer must be valid for receiving the requested interface.
    pub unsafe fn create(
        riid: intercom::REFIID,
        out: *mut intercom::RawComPtr,
    ) -> crate::error::raw::HRESULT
    {
        let factory = Self {
            phantom: std::marker::PhantomData,
        };

        intercom::ComBoxData::query_interface(intercom::ComBox::new(factory).as_mut(), riid, out)
    }

    /// # Safety
    ///
    /// The pointers _must_ be valid.
    pub unsafe extern "system" fn create_instance(
        _self_vtbl: RawComPtr,
        _outer: RawComPtr,
        riid: REFIID,
        out: *mut RawComPtr,
    ) -> raw::HRESULT
    {
        let mut combox = ComBox::new(T::default());
        ComBoxData::query_interface(combox.as_mut(), riid, out)
    }

    /// # Safety
    ///
    /// The pointers _must_ be valid.
    pub unsafe extern "system" fn lock_server(self_vtbl: RawComPtr, lock: bool) -> raw::HRESULT
    {
        if lock {
            ComBoxData::<Self>::add_ref_ptr(self_vtbl);
        } else {
            ComBoxData::<Self>::release_ptr(self_vtbl);
        }
        raw::S_OK
    }

    pub fn create_vtable() -> &'static ClassFactoryVtbl
    {
        &ClassFactoryVtbl {
            __base: IUnknownVtbl {
                query_interface: ComBoxData::<Self>::query_interface_ptr,
                add_ref: ComBoxData::<Self>::add_ref_ptr,
                release: ComBoxData::<Self>::release_ptr,
            },
            create_instance: Self::create_instance,
            lock_server: Self::lock_server,
        }
    }
}
