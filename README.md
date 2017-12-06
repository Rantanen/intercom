# Intercom
### Utilities for writing COM components in Rust

[![crates.io](https://img.shields.io/crates/v/intercom.svg)](https://crates.io/crates/intercom)
[![Build status](https://ci.appveyor.com/api/projects/status/q88b7xk6l72kup0y?svg=true)](https://ci.appveyor.com/project/Rantanen/intercom)
[![Build Status](https://travis-ci.org/Rantanen/intercom.svg?branch=master)](https://travis-ci.org/Rantanen/intercom)

The COM export allows one to write reusable components in Rust, which can then
be consumed in any COM-compatible language. This includes C++, C#, VB.Net, etc.

## Example

Rust COM server:

```rust
#![feature(plugin, custom_attribute)]
#![plugin(com_library)]

#[com_library(
    Calculator
)]

#[com_class("{12341234-1234-1234-1234-123412340001}")]
struct Calculator {
    value: i32
}

#[com_interface("{12341234-1234-1234-1234-123412340002}")]
#[com_impl]
impl Calculator {
    pub fn new() -> Calculator { Calculator { value: 0 } }

    pb fn add(&mut self, value: i32) -> com_runtime::ComResult<i32> {
        self.value += value;
        Ok(self.value)
    }
}
```

C# COM client:

```c#
class Program
{
    [STAThread]
    static void Main( string[] args )
    {
        var calc = new Calculator.Interop.Calculator();
        calc.Add( 10 );
        var result = calc.Add( 100 );
        Console.WriteLine( result );
    }
}
```

## Status

- [x] Basic vtable and delegate construction.
- [x] Proper QueryInterface implementation with proper IID checking.
- [x] Multiple interface implementation.
- [ ] Automatic IDL generation for MIDL. This is needed for the toolchain.
      being able to define an interface in Rust and then call that from C++/C#
      would be awesome if the user didn't need to specify the IDL by hand.
- [ ] Ability to accept `Rc<Itf>` as input value and call its methods.
- [ ] More complex parameter values, such as the dreaded structs.
  - [x] Primitive values (Anything that is binary compatible between Rust
        and COM)
  - [x] BSTRs
  - [ ] SAFEARRAY
  - [ ] Structs (?) - This might already be okay **for `__out` values**,
        actual return values need work but might be low priority.
- [ ] Test harness...
- [ ] IErrorInfo
- [ ] `IStringAllocator` or ` IAllocator` to allow sharing allocated memory.
      This is needed especially if we strive for cross platform compatibiliity.
      The problem comes mainly through strings, especially BSTRs and those are
      usually handled by the `SysAllocString`, etc. in Windows.

### Maybe one day
- [ ] Automatic IDispatch derive.

### Refactoring changes

- [ ] Get rid of the field offset calculations. Instead implement
      `struct IInterfacePtr( RawComPtr )` types for the COM interfaces. These
      would have `AsRef` impls for the various virtual tables. The compiler
      should be able to resolve these statically. The use of these raw structs
      would also make the interface definitions a bit saner. For example the
      IUnknown implementation would accept `IUnknownPtr` parameter instead of
      a `ComPtr`.
- [ ] `ComRc` needs a `Cow` enum (or similar) within it to track whether the
      `ComRc` borrows or owns the value. I want to make the `ComRc` _the_ type
      too use when handling various COM pointers - no matter whether these are
      self created, received as function call parameters are result of return
      values from COM invocations.

# Technical details

## Background

COM is based on the use of virtual tables (also known as dispatch tables).
There is one exported symbol, `DllGetClassObject`, which the clients (pieces of
code using the COM module) use to acquire the initial [IClassFactory]
interface. Once the client has the IClassFactory interface available, the rest
of the method calls can be done through the COM interfaces.

Each COM interface is represented by an object containing a virtual table and
any data that the server (the library implementing the interfaces) requires.
When the client tries to invoke a method on the interface, it will use the
virtual table to resolve the correct method address and performs the call
specifically using pascal calling convention (`__stdcall` in most C++ compilers
and `"stdcall"` in Rust).

[IClassFactory]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms694364(v=vs.85).asp

## Implementation

The `COM-export` libraries are built over Rust [syntax extensions]. Currently
there are four attributes available:

- `[com_library(Classes...)]` - A required attribute that implements the
  exported `DllGetClassObject` entry point as well as the CoClass implementing
  the `IClassFactory` interface.
- `[com_interface(IID)]` - An attribute that specifies a `trait` or an `impl`
  as a COM interface. The attribute results in the virtual table struct to be
  defined for the interface.
- `[com_class(CLSID, Itfs...)]` - An attribute defined on a `struct`. This
  attribute creates the reference counted `CoClass` struct for the type and
  implements the basic `IUnknown` interface for it. 
- `[com_impl]` - Finally the `[com_impl]` attribute specifies the `impl`s that
  implement the `[com_interface]`s for the `[com_class]` types. While the
  attribute doesn't provide any extra information for the implementation, it
  has technical reasons to exist. Its expansion is responsible for defining the
  delegating methods that know how to translate the COM call coming from the
  client into a Rust call to the user defined functions or the primary
  `IUnknown` methods implemented by the `[com_class]` expansion.

[syntax extensions]: https://doc.rust-lang.org/1.12.0/book/compiler-plugins.html

## Expansions

Rough approximation of the attribute expansions is displayed below. This is
meant to give some kind of an idea of how the COM objects are implemented. For
accurate representation, have a look at the `com_library` source code.

The attributes are described in somewhat of a dependnecy order.

### `#[com_interface(...)]`

Defines the IID constant and virtual table struct for the trait.

```rust
#[com_interface("{...}")]
trait IFoo { fn method(&self, parameter: i32) -> com_runtime::Result<i8>; }
```

#### Expands

```rust
const IID_IFoo : GUID = Guid::parse("{...}");

struct __IFooVtbl {
    __base: com_runtime::IUnknownVtbl,
    method: unsafe extern "stdcall" fn(
        self_void: *mut c_void,
        prameter: i32,
        __out: *mut i8
    ) -> HRESULT
}
```

### `#[com_class(...)]`

Defines the CoClass struct and the primary IUnknown implementation for the
struct.

```rust
#[com_class("{...}", IFoo, IBar)]
struct Foo { ... }
```

#### Expands

```rust
/// List of vtables for all the interfaces implemented by Foo.
///
/// The specific interface vtables are expanded during [com_impl].
struct __FooVtblList {
    IUnknown: com_runtime::IUnknownVtbl,
    IFoo: __IFooVtbl,
    IBar: __IBarVtbl
}

/// Foo::IUnknown vtable
const __Foo_IUnknownVtbl_INSTANCE : com_runtime::IUnknownVtbl
    = com_runtime::IUnknownVtbl {
        query_interface: com_runtime::ComBox::< Foo >::query_interface_ptr,
        add_ref: com_runtime::ComBox::< Foo >::add_ref,
        release: com_runtime::ComBox::< Foo >::release,
    };

/// CoClass implementation for Foo.
/// This is a requirement for using Foo with the ComBox<T> type, which
/// implements IUnknown and the virtual tables for COM clients.
impl com_runtime::CoClass for Foo {
    type VTableList = __FooVtblList;

    /// Constructs the list of vtables for the object.
    ///
    /// Since these are the interface pointers passed to the COM clients, these
    /// need a known offset from the ComBox<..> &self reference. We achieve
    /// this by constructing the list as a copy for each ComBox<..> instance.
    fn create_vtable_list() -> self::VTableList {
        __FooVtblList {
            IUnknown: &__Foo_IUnknownVtbl_INSTANCE,
            IFoo: &__Foo_IFooVtbl_INSTANCE,
            IBar: &__Foo_IBarVtbl_INSTANCE,
        }
    }

    /// Query interface implementation.
    ///
    /// Returns a virtual table reference for the given interface ID.
    fn query_interface(
        vtables : &Self::VTableList,
        riid : com_runtime::REFIID
    ) -> ComResult< com_runtime::RawComPtr > {

        match *riid {
            IID_IUnknown => Ok( &vtables.IUnknown ),
            IID_Foo => Ok( &vtables.Foo ),
            IID_Bar => Ok( &vtables.Bar ),
            _ => Err( E_NOINTERFACE )
        }
    }
}

/// The vtbl list must be resorvable to an IUnknown virtual table.
impl AsRef<com_runtime::IUnknownVtbl> for __FooVtblList {
    fn as_ref< &self ) -> &com_runtime::IUnknownVtbl { &self.IUnknown }
}

/// vtable offsets.
///
/// These are needed when we receive a call through a vtable and we need to
/// translate the vtable pointer back to the __FooCoClass instance.
const __Foo_IFooVtbl_offset : usize = offset_of!(ComBox<Foo>, vtable_list.IFoo);
const __Foo_IBarVtbl_offset : usize = offset_of!(ComBox<Foo>, vtable_list.IBar);
```

### `#[com_impl]`

Creates the delegating method implementation and the static virtual table
instance.

```rust
#[com_impl]
impl IFoo for Foo {
    fn method(&self, parameter: i32) -> com_runtime::Result<i8> { ... }
}
```

#### Expands

```rust
/// Foo::IFoo::IUnknown::QueryInterface
pub unsafe extern "stdcall" fn __Foo_IFoo_query_interface(
    self_vtable: *mut c_void,
    riid: *mut GUID,
    out: *mut *mut c_void
) -> HRESULT
{
    // Convert the interface pointer to ComBox pointer and delegate to the
    // ComBox query_interface.
    com_runtime::ComBox::< Foo >::query_interface(
            transmute( self_vtable - __Foo_IFooVtbl_offset ), riid, out )
}

// These are implemented the same way as QueryInterface above.
pub unsafe extern "stdcall" fn __Foo_IFoo_add_ref(...) { ... }
pub unsafe extern "stdcall" fn __Foo_IFoo_release(...) { ... }

/// Delegating method implementation for the `<Foo as IFoo>.method`.
pub unsafe extern "stdcall" fn __Foo_IFoo_method(
    self_vtable : RawComPtr, parameter: i32, __out: *mut i8
) -> HRESULT
{
    // Acquires the ComBox reference from the virtual table pointer.
    let self_comptr = (self_vtable - __Foo_IFooVtbl_offset)
        as *mut com_runtime::ComBox< Foo >;
    let result = (*self_combox).method(parameter);
    match result {
        Ok(out) => { *__out = out; S_OK },
        Err(e) => { *__out = ptr::null_mut(); e },
    }
}

/// The static vtable instance for the interface.
///
/// The VTableList instances contained in each ComBox<..> contain refernces
/// to this instance.
const __Foo_IFooVtbl_INSTANCE : __IFooVtbl = __IFooVtbl {
    __base: com_runtime::IUnknownVtbl {
        query_interface: __Foo_IFoo_query_interface,
        add_ref: __Foo_IFoo_add_ref,
        release: __Foo_IFoo_release
    },
    method: __Foo_IFoo_method
};
```

### `#[com_library]`

Defines the entry point and the `IClassFactory` implementation for creating the
CoClasses.

```rust
#[com_library(Foo)]
```

#### Expands

```rust
/// Class factory create_instance implementation.
pub unsafe extern "stdcall" fn __ClassFactory_create_instance(
    self_void: *mut c_void,
    outer: *mut c_void,
    riid: *mut GUID,
    out: *mut *mut c_void
) -> HRESULT
{
    let self_ptr = self_void as *mut com_runtime::ClassFactory;
    let coclass = match self_ptr.clsid {
        CLSID_Foo => Box::into_raw(Box::new(__FooCoClass::new())),
        _ => return CLASS_E_CLASSNOTAVAILABLE,
    }

    // The IUnknown vtable is at the very beginning of the CoClass struct.
    // This allows us to acquire pointer to it without knowing which CoClass
    // this is.
    (*coclass as com_runtime::IUnknownVtbl).query_interface(coclass, riid, out)
}

/// Class factory vtable required due to custom create_instance.
const __ClassFactoryVtbl_INSTANCE : com_runtime::__ClassFactory_vtbl =
        com_runtime::__ClassFactory_vtbl {
            __base : com_runtime::IUnknownVtbl {
                query_interface: com_runtime::ClassFactory::query_interface,
                add_ref: com_runtime::ClassFactory::add_ref,
                release: com_runtime::ClassFactory::release,
            },
            create_instance: __ClassFactory_create_instance,
            locK_server: com_runtime::ClassFactory::lock_server,
        };

/// The symbolic DllGetClassObject entry point.
#[no_mangle]
pub unsafe extern "stdcall" fn DllGetClassObject(
    rclsid: *mut c_void,
    riid: *mut GUID,
    out: *mut *mut c_void
) -> HRESULT
{
    *out = com_runtime::ComBox::new_ptr(
            com_runtime::ClassFactory::new( rclsid, | clsid | {
                match *clsid {
                    CLSID_Foo => Ok( com_runtime::ComBox::new_ptr(
                        Foo::new() ).as_ptr() as com_runtime::RawComPtr ),
                    _ => Err( E_NOINTERFACE ),
                }
            } ) ).as_ptr() as com_runtime::RawComPtr;
    com_runtime::S_OK
}
```
