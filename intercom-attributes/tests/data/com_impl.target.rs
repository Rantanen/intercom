#![feature(prelude_import)]
#![no_std]
#![feature(use_extern_macros, attr_literals)]

#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
extern crate intercom;
use intercom::*;

// We need the IID and Vtbl to ensure this compiles.
//
// Normally these are provided by the [com_interface].
struct __FooVtbl;
const IID_Foo: intercom::IID = intercom::GUID {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0, 0, 0, 0, 0, 0, 0, 0],
};

pub struct Foo;

// Virtual table offset.
//
// Needed to convert the FooVtbl pointer back to ComBox pointer.
#[inline(always)]
#[allow(non_snake_case)]
fn __Foo_FooVtbl_offset() -> usize {

    unsafe {

        // We are using null reference here - but as the null pointer should
        // never be referenced this is okay.
        &::intercom::ComBox::<Foo>::null_vtable().Foo as *const _ as usize
    }
}

impl From<::intercom::ComStruct<Foo>> for ::intercom::ComRc<Foo> {
    fn from( source: ::intercom::ComStruct<Foo> ) -> Self {
        let itf: ::intercom::ComItf<Foo> = source.into();
        ::intercom::ComRc::attach( itf )
    }
}

impl From<::intercom::ComStruct<Foo>> for ::intercom::ComItf<Foo> {
    fn from( source: ::intercom::ComStruct<Foo> ) -> Self {
        unsafe {

            // We are using the CoClass::query_interface here. This one
            // doesn't increment ref count, which means well need to be careful
            // not to decrease it when 'source' goes out of scope.
            let itf = ::intercom::ComItf::wrap(
                < Foo as ::intercom::CoClass >::query_interface(
                        ::intercom::ComBox::vtable( &source ), &IID_Foo )
                    .expect( "query_interface( IID_Foo ) failed for Foo" ) );

            // Don't decrease the ref count.
            std::mem::forget( source );

            itf
        }
    }
}

impl ::std::ops::Deref for ::intercom::ComItf<Foo> {
    type Target = Foo;
    fn deref(&self) -> &Self::Target {
        unsafe {
            let self_combox = (
                    ::intercom::ComItf::ptr(self) as usize
                        - __Foo_FooVtbl_offset()
                ) as *mut ::intercom::ComBox<Foo>;
            &**self_combox
        }
    }
}

// ISupportErrorInfo virtual table instance for the Foo COM class.
//
// Each COM class needs IUnknown and ISupportErrorInfo virtual tables - as the
// ISupportErrorInfo inherits from IUnknown we can use it as the IUnknownVtbl
// as well.
#[allow(non_upper_case_globals)]
const __Foo_ISupportErrorInfoVtbl_INSTANCE: ::intercom::ISupportErrorInfoVtbl =
    ::intercom::ISupportErrorInfoVtbl {
        __base: ::intercom::IUnknownVtbl {
            query_interface: ::intercom::ComBox::<Foo>::query_interface_ptr,
            add_ref: ::intercom::ComBox::<Foo>::add_ref_ptr,
            release: ::intercom::ComBox::<Foo>::release_ptr,
        },
        interface_supports_error_info:
                ::intercom::ComBox::<Foo>::interface_supports_error_info_ptr,
};

// The Foo COM class virtual table list.
//
// This struct lists the virtual tables of all interfaces that the Foo COM
// class supports.
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct __FooVtblList {

    // ISupportErrorInfo virtual table. It's important this is the first
    // one as this is what we use for IUnknown as well.
    _ISupportErrorInfo: &'static ::intercom::ISupportErrorInfoVtbl,

    // The implicit "IFoo" interface.
    Foo: &'static __FooVtbl,
}

// Implement CoClass for Foo. This allows using ComBox to wrap Foo.
impl ::intercom::CoClass for Foo {

    // Virtual table list.
    type VTableList = __FooVtblList;

    // The virtual table list constructor.
    //
    // Each ComBox instance will have a virtual table list embedded in it.
    // Note that each virtual table WITHIN the list is a pointer to a static
    // virtual table - so the size of the list itself doesn't depend on the
    // amount of methods - only on the amount of interfaces implemented.
    fn create_vtable_list() -> Self::VTableList {
        __FooVtblList{
            _ISupportErrorInfo: &__Foo_ISupportErrorInfoVtbl_INSTANCE,
            Foo: &__Foo_FooVtbl_INSTANCE,
        }
    }

    // The query interface implementation.
    fn query_interface(
        vtables: &Self::VTableList,
        riid: ::intercom::REFIID
    ) -> ::intercom::ComResult<::intercom::RawComPtr> {

        // E_POINTER is returned only if the receiving object pointer is null.
        // E_NOINTERFACE is returned in all other error scenarios.
        if riid.is_null() {
            return Err(::intercom::E_NOINTERFACE)
        }

        Ok( match *unsafe { &*riid } {

            // Use the ISupportErrorInfof or the IUnknown implementation.
            ::intercom::IID_IUnknown =>
                    (&vtables._ISupportErrorInfo) as
                        *const &::intercom::ISupportErrorInfoVtbl as
                        *mut &::intercom::ISupportErrorInfoVtbl as
                        ::intercom::RawComPtr,

            ::intercom::IID_ISupportErrorInfo =>
                    (&vtables._ISupportErrorInfo) as
                        *const &::intercom::ISupportErrorInfoVtbl as
                        *mut &::intercom::ISupportErrorInfoVtbl as
                        ::intercom::RawComPtr,

            // We are using "qualified" version of the IID. This ensures
            // that missing IID declarations are reported as compilation
            // errors.
            //
            // Using just "IID_Foo" would consider bad IID name as a pattern
            // binding that matches anything.
            self::IID_Foo =>
                    &vtables.Foo as
                        *const &__FooVtbl as
                        *mut &__FooVtbl as
                        ::intercom::RawComPtr,

            _ => return Err(::intercom::E_NOINTERFACE),
        })
    }

    fn interface_supports_error_info(riid: ::intercom::REFIID) -> bool {
        match *unsafe { &*riid } {

            // Only the custom interfaces support error info.
            self::IID_Foo => true,

            _ => false,
        }
    }
}

// Class ID.
#[allow(non_upper_case_globals)]
#[doc = "`Foo` class ID."]
pub const CLSID_Foo: ::intercom::CLSID =
    ::intercom::GUID {
        data1: 0u32, data2: 0u16, data3: 0u16,
        data4: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]
    };


impl Foo
{
    fn static_method(a: u16, b: i16) { }
    fn simple_method(&self) { }
    fn arg_method(&self, a: u16) { }

    fn simple_result_method(&self) -> u16 { 0 }
    fn com_result_method(&self) -> ComResult<u16> { Ok(0) }
    fn rust_result_method(&self) -> Result<u16, i32> { Ok(0) }
    fn tuple_result_method(&self) -> Result<(u8, u16, u32), i32> { Ok(0) }

    fn string_method(&self, input : String) -> String { input }

    fn complete_method(&mut self, a: u16, b: i16) -> ComResult<bool>
    {
        Ok(true)
    }
}

// Method implementations for the implicit "IFoo" interface.

#[allow(non_snake_case)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_query_interface(
    self_vtable: ::intercom::RawComPtr,
    riid: ::intercom::REFIID,
    out: *mut ::intercom::RawComPtr
) -> ::intercom::HRESULT
{
    // The self_vtable points to the "IFoo" vtable pointer.
    //
    // We need to use the FooVtbl offset to translate this back into the
    // ComBox pointer.
    //
    // This same pattern applies to all the delegating methods.
    ::intercom::ComBox::<Foo>::query_interface(
        &mut *((self_vtable as usize - __Foo_FooVtbl_offset()) as *mut _),
        riid,
        out
    )
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_add_ref(
    self_vtable: ::intercom::RawComPtr
) -> u32
{
    ::intercom::ComBox::<Foo>::add_ref(
        &mut *((self_vtable as usize - __Foo_FooVtbl_offset()) as *mut _)
    )
}
#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_release(
    self_vtable: ::intercom::RawComPtr
) -> u32
{
    // We are using the release_ptr delegate here as the release might end
    // up freeing the ComBox if this is the last reference we are releasing.
    //
    // Not sure if Rust would make some assumptions on the lifetime of the
    // combox if we had a &-borrow on it instead of a raw pointer.
    ::intercom::ComBox::<Foo>::release_ptr(
        (self_vtable as usize - __Foo_FooVtbl_offset()) as *mut _
    )
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_simple_method(
    self_vtable: ::intercom::RawComPtr
) -> ()
{
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut ::intercom::ComBox<Foo>;

    let self_struct : &Foo = &**self_combox;
    let __result = self_struct.simple_method();
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_arg_method(
    self_vtable: ::intercom::RawComPtr,
    a: u16
) -> ()
{
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut ::intercom::ComBox<Foo>;

    let self_struct : &Foo = &**self_combox;
    let __result = self_struct.arg_method(a.into());
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_simple_result_method(
    self_vtable: ::intercom::RawComPtr
) -> u16
{
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut ::intercom::ComBox<Foo>;

    let self_struct : &Foo = &**self_combox;
    let __result = self_struct.simple_result_method();
    __result.into()
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_com_result_method(
    self_vtable: ::intercom::RawComPtr,
    __out: *mut u16
) -> ::intercom::HRESULT
{
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut ::intercom::ComBox<Foo>;

    let self_struct : &Foo = &**self_combox;
    let __result = self_struct.com_result_method();

    // Convert the Rust result into [retval] and HRESULT.
    // On error we need to reset the [retval] into a "known" value.
    match __result {
        Ok(v1) => { *__out = v1.into(); ::intercom::S_OK }
        Err(e) => {
            *__out = Default::default();
            ::intercom::return_hresult( e )
        }
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_rust_result_method(
    self_vtable: ::intercom::RawComPtr,
    __out: *mut u16
) -> ::intercom::HRESULT
{
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut ::intercom::ComBox<Foo>;

    let self_struct : &Foo = &**self_combox;
    let __result = self_struct.rust_result_method();
    match __result {
        Ok(v1) => { *__out = v1.into(); ::intercom::S_OK }
        Err(e) => {
            *__out = Default::default();

            // This is Result<_,_> method instead of ComResult<_>. In this case
            // the Err value needs to be converted to HRESULT for the COM
            // return value. The return_hresult also stores the detailed error
            // description to support IErrorInfo.
            ::intercom::return_hresult(e)
        }
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_tuple_result_method(
    self_vtable: ::intercom::RawComPtr,
    __out1: *mut u8,
    __out2: *mut u16,
    __out3: *mut u32
) -> ::intercom::HRESULT
{
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut ::intercom::ComBox<Foo>;

    let self_struct : &Foo = &**self_combox;
    let __result = self_struct.tuple_result_method();
    match __result {
        Ok((v1, v2, v3)) => {
            *__out1 = v1.into();
            *__out2 = v2.into();
            *__out3 = v3.into();
            ::intercom::S_OK
        },
        Err(e) => {
            *__out1 = Default::default();
            *__out2 = Default::default();
            *__out3 = Default::default();

            // This is Result<_,_> method instead of ComResult<_>. In this case
            // the Err value needs to be converted to HRESULT for the COM
            // return value. The return_hresult also stores the detailed error
            // description to support IErrorInfo.
            ::intercom::return_hresult(e)
        }
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_string_method(
    self_vtable: ::intercom::RawComPtr,
    input: ::intercom::BStr,
) -> ::intercom::BStr {
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut ::intercom::ComBox<Foo>;

    let self_struct : &Foo = &**self_combox;
    let __result = self_struct.string_method(input.into());
    __result.into()
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[doc(hidden)]
unsafe extern "stdcall" fn __Foo_Foo_complete_method(
    self_vtable: ::intercom::RawComPtr,
    a: u16,
    b: i16,
    __out: *mut bool
) -> ::intercom::HRESULT
{
    let self_combox =
        (self_vtable as usize - __Foo_FooVtbl_offset()) as
            *mut ::intercom::ComBox<Foo>;

    let self_struct : &mut Foo = &mut **self_combox;
    let __result = self_struct.complete_method(a.into(), b.into());
    match __result {
        Ok(v1) => { *__out = v1.into(); ::intercom::S_OK }
        Err(e) => {
            *__out = Default::default();
            ::intercom::return_hresult( e )
        }
    }
}

// The implicit "IFoo" interface virtual table instance for the "Foo" COM class.
#[allow(non_upper_case_globals)]
const __Foo_FooVtbl_INSTANCE: __FooVtbl =
    __FooVtbl {
        __base: ::intercom::IUnknownVtbl {
            query_interface: __Foo_Foo_query_interface,
            add_ref: __Foo_Foo_add_ref,
            release: __Foo_Foo_release,
        },
        simple_method: __Foo_Foo_simple_method,
        arg_method: __Foo_Foo_arg_method,
        simple_result_method: __Foo_Foo_simple_result_method,
        com_result_method: __Foo_Foo_com_result_method,
        rust_result_method: __Foo_Foo_rust_result_method,
        tuple_result_method: __Foo_Foo_tuple_result_method,
        string_method: __Foo_Foo_string_method,
        complete_method: __Foo_Foo_complete_method,
    };
