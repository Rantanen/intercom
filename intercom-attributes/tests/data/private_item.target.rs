#![feature(prelude_import)]
#![no_std]
#![feature(proc_macro)]
#[prelude_import]
use std::prelude::v1::*;
extern crate intercom;
#[macro_use]
extern crate std as std;
use intercom::*;
trait IFoo {
    fn trait_method(&self);
}

#[doc = "`IFoo` interface ID."]
#[allow(non_upper_case_globals)]
const IID_IFoo: ::intercom::IID = ::intercom::GUID {
    data1: 0u32,
    data2: 0u16,
    data3: 0u16,
    data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
};

impl ::intercom::IidOf for IFoo {
    #[doc = "Returns `IID_IFoo`."]
    fn iid() -> &'static ::intercom::IID {
        &IID_IFoo
    }
}
#[allow(non_camel_case_types)]
#[repr(C)]
#[doc(hidden)]
struct __IFooVtbl {
    pub __base: ::intercom::IUnknownVtbl,
    pub trait_method: unsafe extern "stdcall" fn(self_vtable: ::intercom::RawComPtr) -> (),
}
impl IFoo for ::intercom::ComItf<IFoo> {
    fn trait_method(&self) -> () {
        let comptr = ::intercom::ComItf::ptr(self);
        let vtbl = comptr as *const *const __IFooVtbl;

        #[allow(unused_unsafe)]
        unsafe {
            let __result = ((**vtbl).trait_method)(comptr);
        }
    }
}

impl ::std::ops::Deref for ::intercom::ComItf<IFoo> {
    type Target = IFoo;
    fn deref(&self) -> &Self::Target {
        self
    }
}

struct Foo;
#[inline(always)]
#[allow(non_snake_case)]
fn __Foo_FooVtbl_offset() -> usize {
    unsafe { &::intercom::ComBox::<Foo>::null_vtable().Foo as *const _ as usize }
}

impl From<::intercom::ComStruct<Foo>> for ::intercom::ComRc<Foo> {
    fn from(source: ::intercom::ComStruct<Foo>) -> Self {
        let itf: ::intercom::ComItf<Foo> = source.into();
        ::intercom::ComRc::attach(itf)
    }
}

impl From<::intercom::ComStruct<Foo>> for ::intercom::ComItf<Foo> {
    fn from(source: ::intercom::ComStruct<Foo>) -> Self {
        unsafe {
            let itf = ::intercom::ComItf::wrap(
                <Foo as ::intercom::CoClass>::query_interface(
                    ::intercom::ComBox::vtable(&source),
                    &IID_Foo,
                ).expect("query_interface( IID_Foo ) failed for Foo"),
            );
            std::mem::forget(source);
            itf
        }
    }
}

impl ::std::ops::Deref for ::intercom::ComItf<Foo> {
    type Target = Foo;
    fn deref(&self) -> &Self::Target {
        unsafe {
            let self_combox = (::intercom::ComItf::ptr(self) as usize - __Foo_FooVtbl_offset())
                as *mut ::intercom::ComBox<Foo>;
            &**self_combox
        }
    }
}

#[inline(always)]
#[allow(non_snake_case)]
fn __Foo_IFooVtbl_offset() -> usize {
    unsafe { &::intercom::ComBox::<Foo>::null_vtable().IFoo as *const _ as usize }
}

impl From<::intercom::ComStruct<Foo>> for ::intercom::ComRc<IFoo> {
    fn from(source: ::intercom::ComStruct<Foo>) -> Self {
        let itf: ::intercom::ComItf<IFoo> = source.into();
        ::intercom::ComRc::attach(itf)
    }
}

impl From<::intercom::ComStruct<Foo>> for ::intercom::ComItf<IFoo> {
    fn from(source: ::intercom::ComStruct<Foo>) -> Self {
        unsafe {
            let itf = ::intercom::ComItf::wrap(
                <Foo as ::intercom::CoClass>::query_interface(
                    ::intercom::ComBox::vtable(&source),
                    &IID_IFoo,
                ).expect("query_interface( IID_IFoo ) failed for IFoo"),
            );
            std::mem::forget(source);
            itf
        }
    }
}

#[allow(non_upper_case_globals)]
const __Foo_ISupportErrorInfoVtbl_INSTANCE: ::intercom::ISupportErrorInfoVtbl =
    ::intercom::ISupportErrorInfoVtbl {
        __base: ::intercom::IUnknownVtbl {
            query_interface: ::intercom::ComBox::<Foo>::query_interface_ptr,
            add_ref: ::intercom::ComBox::<Foo>::add_ref_ptr,
            release: ::intercom::ComBox::<Foo>::release_ptr,
        },
        interface_supports_error_info: ::intercom::ComBox::<Foo>::interface_supports_error_info_ptr,
    };
#[allow(non_snake_case)]
#[doc(hidden)]
struct __FooVtblList {
    _ISupportErrorInfo: &'static ::intercom::ISupportErrorInfoVtbl,
    Foo: &'static __FooVtbl,
    IFoo: &'static __IFooVtbl,
}

impl ::intercom::CoClass for Foo {
    type VTableList = __FooVtblList;
    fn create_vtable_list() -> Self::VTableList {
        __FooVtblList {
            _ISupportErrorInfo: &__Foo_ISupportErrorInfoVtbl_INSTANCE,
            Foo: &__Foo_FooVtbl_INSTANCE,
            IFoo: &__Foo_IFooVtbl_INSTANCE,
        }
    }
    fn query_interface(
        vtables: &Self::VTableList,
        riid: ::intercom::REFIID,
    ) -> ::intercom::ComResult<::intercom::RawComPtr> {
        if riid.is_null() {
            return Err(::intercom::E_NOINTERFACE);
        }
        Ok(match *unsafe { &*riid } {
            ::intercom::IID_IUnknown => {
                (&vtables._ISupportErrorInfo) as *const &::intercom::ISupportErrorInfoVtbl
                    as *mut &::intercom::ISupportErrorInfoVtbl
                    as ::intercom::RawComPtr
            }
            ::intercom::IID_ISupportErrorInfo => {
                (&vtables._ISupportErrorInfo) as *const &::intercom::ISupportErrorInfoVtbl
                    as *mut &::intercom::ISupportErrorInfoVtbl
                    as ::intercom::RawComPtr
            }
            self::IID_Foo => {
                &vtables.Foo as *const &__FooVtbl as *mut &__FooVtbl as ::intercom::RawComPtr
            }
            self::IID_IFoo => {
                &vtables.IFoo as *const &__IFooVtbl as *mut &__IFooVtbl as ::intercom::RawComPtr
            }
            _ => return Err(::intercom::E_NOINTERFACE),
        })
    }
    fn interface_supports_error_info(riid: ::intercom::REFIID) -> bool {
        match *unsafe { &*riid } {
            self::IID_Foo => true,
            self::IID_IFoo => true,
            _ => false,
        }
    }
}

#[allow(non_upper_case_globals)]
#[doc = "`Foo` class ID."]
pub const CLSID_Foo: ::intercom::CLSID = ::intercom::GUID {
    data1: 0u32,
    data2: 0u16,
    data3: 0u16,
    data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
};

impl Foo {
    pub fn struct_method(&self) {}
}

#[allow(non_snake_case)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_query_interface(
    self_vtable: ::intercom::RawComPtr,
    riid: ::intercom::REFIID,
    out: *mut ::intercom::RawComPtr,
) -> ::intercom::HRESULT {
    ::intercom::ComBox::<Foo>::query_interface(
        &mut *((self_vtable as usize - __Foo_FooVtbl_offset()) as *mut _),
        riid,
        out,
    )
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_add_ref(self_vtable: ::intercom::RawComPtr) -> u32 {
    ::intercom::ComBox::<Foo>::add_ref(
        &mut *((self_vtable as usize - __Foo_FooVtbl_offset()) as *mut _),
    )
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_release(self_vtable: ::intercom::RawComPtr) -> u32 {
    ::intercom::ComBox::<Foo>::release_ptr(
        (self_vtable as usize - __Foo_FooVtbl_offset()) as *mut _,
    )
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_struct_method(self_vtable: ::intercom::RawComPtr) -> () {
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as *mut ::intercom::ComBox<Foo>;
    let self_struct: &Foo = &**self_combox;
    let __result = self_struct.struct_method();
}

#[allow(non_upper_case_globals)]
const __Foo_FooVtbl_INSTANCE: __FooVtbl = __FooVtbl {
    __base: ::intercom::IUnknownVtbl {
        query_interface: __Foo_Foo_query_interface,
        add_ref: __Foo_Foo_add_ref,
        release: __Foo_Foo_release,
    },
    struct_method: __Foo_Foo_struct_method,
};

impl IFoo for Foo {
    fn trait_method(&self) {}
}

#[allow(non_snake_case)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_IFoo_query_interface(
    self_vtable: ::intercom::RawComPtr,
    riid: ::intercom::REFIID,
    out: *mut ::intercom::RawComPtr,
) -> ::intercom::HRESULT {
    ::intercom::ComBox::<Foo>::query_interface(
        &mut *((self_vtable as usize - __Foo_IFooVtbl_offset()) as *mut _),
        riid,
        out,
    )
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_IFoo_add_ref(self_vtable: ::intercom::RawComPtr) -> u32 {
    ::intercom::ComBox::<Foo>::add_ref(
        &mut *((self_vtable as usize - __Foo_IFooVtbl_offset()) as *mut _),
    )
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_IFoo_release(self_vtable: ::intercom::RawComPtr) -> u32 {
    ::intercom::ComBox::<Foo>::release_ptr(
        (self_vtable as usize - __Foo_IFooVtbl_offset()) as *mut _,
    )
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_IFoo_trait_method(self_vtable: ::intercom::RawComPtr) -> () {
    let self_combox =
        (self_vtable as usize - __Foo_IFooVtbl_offset()) as *mut ::intercom::ComBox<Foo>;
    let self_struct: &IFoo = &**self_combox;
    let __result = self_struct.trait_method();
}

#[allow(non_upper_case_globals)]
const __Foo_IFooVtbl_INSTANCE: __IFooVtbl = __IFooVtbl {
    __base: ::intercom::IUnknownVtbl {
        query_interface: __Foo_IFoo_query_interface,
        add_ref: __Foo_IFoo_add_ref,
        release: __Foo_IFoo_release,
    },
    trait_method: __Foo_IFoo_trait_method,
};

#[doc = "`Foo` interface ID."]
#[allow(non_upper_case_globals)]
pub const IID_Foo: ::intercom::IID = ::intercom::GUID {
    data1: 0u32,
    data2: 0u16,
    data3: 0u16,
    data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
};

impl ::intercom::IidOf for Foo {
    #[doc = "Returns `IID_Foo`."]
    fn iid() -> &'static ::intercom::IID {
        &IID_Foo
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[doc(hidden)]
pub struct __FooVtbl {
    pub __base: ::intercom::IUnknownVtbl,
    pub trait_method: unsafe extern "stdcall" fn(self_vtable: ::intercom::RawComPtr) -> (),
}
