
use super::*;

#[allow(non_camel_case_types)]
pub struct ClassFactoryVtbl {
    pub __base: IUnknownVtbl,
    pub create_instance: unsafe extern "stdcall" fn( RawComPtr, RawComPtr, REFIID, *mut RawComPtr ) -> HRESULT,
    pub lock_server: unsafe extern "stdcall" fn( RawComPtr, bool ) -> HRESULT
}

pub struct ClassFactory<T> {
    pub clsid : REFCLSID,
    pub create_instance : T,
}

impl< T: Fn( REFCLSID ) -> ComResult< RawComPtr > > CoClass for ClassFactory<T> {
    type VTableList = &'static ClassFactoryVtbl;
    fn create_vtable_list() -> Self::VTableList {
        ClassFactory::<T>::create_vtable()
    }
}

impl< T: Fn( REFCLSID ) -> ComResult< RawComPtr > > ClassFactory<T> {

    pub fn new( clsid : REFCLSID, create_instance : T ) -> Self {
        Self { clsid, create_instance }
    }

    pub unsafe extern "stdcall" fn query_interface(
        self_vtbl : RawComPtr,
        riid : REFIID,
        out : *mut RawComPtr,
    ) -> HRESULT
    {
        *out = match *riid {
            IID_IUnknown | IID_IClassFactory => self_vtbl,
            _ => return E_NOINTERFACE,
        };

        S_OK
    }

    pub unsafe extern "stdcall" fn create_instance(
        self_vtbl : RawComPtr,
        outer : RawComPtr,
        iid : REFIID,
        out : *mut RawComPtr,
    ) -> HRESULT
    {
        let cb = ComBox::< Self >::from_ptr( self_vtbl );

        *out = match (cb.create_instance)( cb.clsid ) {
            Ok( m ) => m,
            Err( hr ) => return hr,
        };

        S_OK
    }

    pub unsafe extern "stdcall" fn lock_server(
        self_vtbl : RawComPtr,
        lock : bool
    ) -> HRESULT
    {
        // match lock {
            // true => ComBox::add_ref_ptr( self_vtbl ),
            // false => ComBox::release_ptr( self_vtbl ),
        // };
        S_OK
    }

    pub fn create_vtable() -> &'static ClassFactoryVtbl {
        &ClassFactoryVtbl {
            __base : IUnknownVtbl {
                query_interface : Self::query_interface,
                add_ref : ComBox::< Self >::add_ref_ptr,
                release : ComBox::< Self >::release_ptr,
            },
            create_instance : Self::create_instance,
            lock_server : Self::lock_server
        }
    }
}

