
//! The class factory is an infrastructure type used by the COM clients to
//! create instances of the `#[com_class(..)]`es provided by the intercom
//! library.
//!
//! Intercom implements the class factory infrastructure automatically when the
//! user specifies the `#[com_library(..)]` -attribute.

use super::*;

#[allow(non_camel_case_types)]
#[doc(hidden)]
#[cfg(windows)]
pub struct ClassFactoryVtbl {
    pub __base: IUnknownVtbl,
    pub create_instance: unsafe extern "stdcall" fn( RawComPtr, RawComPtr, REFIID, *mut RawComPtr ) -> raw::HRESULT,
    pub lock_server: unsafe extern "stdcall" fn( RawComPtr, bool ) -> raw::HRESULT
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
#[cfg(not(windows))]
pub struct ClassFactoryVtbl {
    pub __base: IUnknownVtbl,
    pub create_instance: unsafe extern "C" fn( RawComPtr, RawComPtr, REFIID, *mut RawComPtr ) -> raw::HRESULT,
    pub lock_server: unsafe extern "C" fn( RawComPtr, bool ) -> raw::HRESULT
}

#[doc(hidden)]
pub struct ClassFactory<T> {
    pub clsid : REFCLSID,
    pub create_instance : T,
}

impl< T: Fn( REFCLSID ) -> RawComResult< RawComPtr > > CoClass
        for ClassFactory<T> {

    type VTableList = &'static ClassFactoryVtbl;
    fn create_vtable_list() -> Self::VTableList {
        ClassFactory::<T>::create_vtable()
    }
    fn query_interface(
        vtables : &Self::VTableList,
        riid : REFIID,
    ) -> RawComResult< RawComPtr >
    {
        if riid.is_null() { return Err( raw::E_NOINTERFACE ) }
        unsafe { match *riid {
            super::IID_IUnknown | super::IID_IClassFactory =>
                    Ok( vtables as *const _ as RawComPtr ),
            _ => Err( raw::E_NOINTERFACE ),
        } }
    }

    fn interface_supports_error_info( _riid : REFIID ) -> bool { false }
}

impl AsRef<IUnknownVtbl> for ClassFactoryVtbl {
    fn as_ref( &self ) -> &IUnknownVtbl {
        &self.__base
    }
}

impl< T: Fn( REFCLSID ) -> RawComResult< RawComPtr > > ClassFactory<T> {

    pub fn new( clsid : REFCLSID, create_instance : T ) -> Self {
        Self { clsid, create_instance }
    }

    /// # Safety
    ///
    /// The pointers _must_ be valid.
    #[cfg(windows)]
    pub unsafe extern "stdcall" fn create_instance(
        self_vtbl : RawComPtr,
        _outer : RawComPtr,
        riid : REFIID,
        out : *mut RawComPtr,
    ) -> raw::HRESULT
    {
        Self::create_instance_agnostic(self_vtbl, _outer, riid, out)
    }

    /// # Safety
    ///
    /// The pointers _must_ be valid.
    #[cfg(not(windows))]
    pub unsafe extern "C" fn create_instance(
        self_vtbl : RawComPtr,
        _outer : RawComPtr,
        riid : REFIID,
        out : *mut RawComPtr,
    ) -> raw::HRESULT
    {
        Self::create_instance_agnostic(self_vtbl, _outer, riid, out)
    }

    /// # Safety
    ///
    /// The pointers _must_ be valid.
    #[cfg(windows)]
    pub unsafe extern "stdcall" fn lock_server(
        self_vtbl : RawComPtr,
        lock : bool
    ) -> raw::HRESULT
    {
        Self::lock_server_agnostic(self_vtbl, lock)
    }

    /// # Safety
    ///
    /// The pointers _must_ be valid.
    #[cfg(not(windows))]
    pub unsafe extern "C" fn lock_server(
        self_vtbl : RawComPtr,
        lock : bool
    ) -> raw::HRESULT
    {
        Self::lock_server_agnostic(self_vtbl, lock)
    }

    pub fn create_vtable() -> &'static ClassFactoryVtbl {
        &ClassFactoryVtbl {
            __base : IUnknownVtbl {
                query_interface_Automation : ComBox::< Self >::query_interface_ptr,
                add_ref_Automation : ComBox::< Self >::add_ref_ptr,
                release_Automation : ComBox::< Self >::release_ptr,
            },
            create_instance : Self::create_instance,
            lock_server : Self::lock_server
        }
    }

    /// # Safety
    ///
    /// The pointers _must_ be valid.
    unsafe fn create_instance_agnostic(
        self_vtbl : RawComPtr,
        _outer : RawComPtr,
        riid : REFIID,
        out : *mut RawComPtr,
    ) -> raw::HRESULT
    {
        if out.is_null() {
            return raw::E_POINTER
        }
        *out = std::ptr::null_mut();

        let cb = ComBox::< Self >::from_ptr( self_vtbl );

        let iunk_ptr = match (cb.create_instance)( cb.clsid ) {
            Ok( m ) => m,
            Err( hr ) => return hr,
        } as *const *const IUnknownVtbl;

        let query_result = ((**iunk_ptr).query_interface_Automation)(
            iunk_ptr as RawComPtr,
            riid,
            out );

        // Avoid leaking memory in case query_interface fails.
        if query_result != raw::S_OK {
            drop( Box::from_raw( iunk_ptr as RawComPtr ) );
        }
        query_result
    }

    /// # Safety
    ///
    /// The pointers _must_ be valid.
    unsafe fn lock_server_agnostic(
        self_vtbl : RawComPtr,
        lock : bool
    ) -> raw::HRESULT
    {
        if lock {
            ComBox::<Self>::add_ref_ptr( self_vtbl );
        } else {
            ComBox::<Self>::release_ptr( self_vtbl );
        }
        raw::S_OK
    }
}

