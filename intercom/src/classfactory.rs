
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
    fn query_interface(
        vtables : &Self::VTableList,
        riid : REFIID,
    ) -> ComResult< RawComPtr >
    {
        if riid.is_null() { return Err( E_NOINTERFACE ) }
        unsafe { match *riid {
            super::IID_IUnknown | super::IID_IClassFactory =>
                    Ok( vtables as *const _ as RawComPtr ),
            _ => Err( E_NOINTERFACE ),
        } }
    }

    fn interface_supports_error_info( _riid : REFIID ) -> bool { false }
}

impl AsRef<IUnknownVtbl> for ClassFactoryVtbl {
    fn as_ref( &self ) -> &IUnknownVtbl {
        &self.__base
    }
}

impl< T: Fn( REFCLSID ) -> ComResult< RawComPtr > > ClassFactory<T> {

    pub fn new( clsid : REFCLSID, create_instance : T ) -> Self {
        Self { clsid, create_instance }
    }

    pub unsafe extern "stdcall" fn create_instance(
        self_vtbl : RawComPtr,
        _outer : RawComPtr,
        riid : REFIID,
        out : *mut RawComPtr,
    ) -> HRESULT
    {
        if out.is_null() {
            return E_POINTER
        }
        *out = std::ptr::null_mut();

        let cb = ComBox::< Self >::from_ptr( self_vtbl );

        let iunk_ptr = match (cb.create_instance)( cb.clsid ) {
            Ok( m ) => m,
            Err( hr ) => return hr,
        } as *const *const IUnknownVtbl;

        ((**iunk_ptr).query_interface)(
            iunk_ptr as RawComPtr,
            riid,
            out );

        S_OK
    }

    pub unsafe extern "stdcall" fn lock_server(
        self_vtbl : RawComPtr,
        lock : bool
    ) -> HRESULT
    {
        if lock {
            ComBox::<Self>::add_ref_ptr( self_vtbl );
        } else {
            ComBox::<Self>::release_ptr( self_vtbl );
        }
        S_OK
    }

    pub fn create_vtable() -> &'static ClassFactoryVtbl {
        &ClassFactoryVtbl {
            __base : IUnknownVtbl {
                query_interface : ComBox::< Self >::query_interface_ptr,
                add_ref : ComBox::< Self >::add_ref_ptr,
                release : ComBox::< Self >::release_ptr,
            },
            create_instance : Self::create_instance,
            lock_server : Self::lock_server
        }
    }
}

