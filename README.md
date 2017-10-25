# COM-export-rs
### Utilities for writing COM components in Rust

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
struct Calculator{
    value: i32
}

#[com_interface("{12341234-1234-1234-1234-123412340002}")]
#[com_impl]
impl Calculator {
    pub fn new() -> Calculator { Calculator { value: 0 } }

    pb fn add( &mut self, value: i32 ) -> com_runtime::ComResult<i32>
    {
        self.value += value;
        Ok( self.value )
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
- [ ] More complex parameter values, such as the dreaded structs.
- [ ] Test harness...
- [ ] IErrorInfo

### Maybe one day
- [ ] Automatic IDispatch derive.
- [ ] Automatic IDL generation for MIDL.

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

### Expansions

Rough approximation of the attribute expansions is displayed below. This is
meant to give some kind of an idea of how the COM objects are implemented. For
accurate representation, have a look at the `com_library` source code.

The attributes are described in somewhat of a dependnecy order.

#### `[com_interface(...)]`

Defines the IID constant and virtual table struct for the trait.

```rust
#[com_interface("{...}")]
trait IFoo { fn method(&self, parameter: i32) -> com_runtime::Result<i8>; }

// Expands the following items:

const IID_IFoo: GUID = Guid::parse( "{...}" );

struct __IFooVtbl {
    __base: com_runtime::IUnknownVtbl,
    method: unsafe extern "stdcall" fn(
        self_void: *mut c_void,
        prameter: i32,
        __out: *mut i8
    ) -> HRESULT
}
```

#### `[com_class(...)]`

Defines the CoClass struct and the primary IUnknown implementation for the
struct.

```rust
#[com_class("{...}", IFoo, IBar)]
struct Foo { ... }

// Expands the following items:

/// List of vtables for all the interfaces implemented by Foo.
struct __FooVtblList {
    IUnknown: com_runtime::IUnknownVtbl,
    IFoo: __IFooVtbl,
    IBar: __IBarVtbl
}

/// CoClass instance for Foo.
/// This is the type given out when Foo is created by the COM client.
struct __FooCoClass {
    vtables: &__FooVtblList,
    rc: u32,
    data: Foo
}

/// vtable offsets.
///
/// These are needed when we receive a call through a vtable and we need to
/// translate the vtable pointer back to the __FooCoClass instance.
const __Foo_IFooVtbl_offset: usize = offset_of!( __FooCoClass, vtables.IFoo );
const __Foo_IBarVtbl_offset: usize = offset_of!( __FooCoClass, vtables.IBar );

/// Foo::IUnknown::QueryInterface
pub unsafe extern "stdcall" fn __Foo_IUnknown_query_interface(
    self_void: *mut c_void,
    riid: *const GUID,
    out: *mut *mut c_void
) -> HRESULT
{
    *out = match *riid {
        IID_IUnknown => self_void,  // self_void is the IUnknown pointer here.
        IID_IFoo => self_void + __Foo_IFooVtbl_offset,
        IID_IBar => self_void + __Foo_IBarVtbl_offset,
        _ => return E_NOINTERFACE
    }

    __Foo_IUnknown_add_ref( self_void );
    S_OK
}

/// Foo::IUnknown::AddRef
pub unsafe extern "stdcall" fn __Foo_IUnknown_add_ref(
    self_void: *mut c_void
) -> u32
{
    let self_ptr = self_void as *mut __FooCoClass;
    (*self_ptr).rc += 1;
    (*self_ptr).rc
}

/// Foo::IUnknown::Release
pub unsafe extern "stdcall" fn __Foo_IUnknown_release(
    self_void: *mut c_void
) -> u32
{
    let self_ptr = self_void as *mut __FooCoClass;
    (*self_ptr).rc -= 1;
    rc = (*self_ptr).rc;

    if rc == 0 {
        // Drop self_ptr
    }
    rc
}

/// Foo::IUnknown vtable
const __Foo_IUnknownVtbl_INSTANCE: com_runtime::IUnknownVtbl
    = com_runtime::IUnknownVtbl {
        query_interface: __Foo_IUnknown_query_interface,
        add_ref: __Foo_IUnknown_add_ref,
        release: __Foo_IUnknown_release,
    };

/// CoClass impl
impl __FooCoClass {
    fn new() -> __FooCoClass {
        __FooCoClass {
            vtables: {
                IUnknown: &__Foo_IUnknownVtbl_INSTANCE;
                IFoo: &__Foo_IFooVtbl_INSTANCE,
                IBar: &__Foo_IBarVtbl_INSTANCE,
            },
            rc: 0,
            data: Foo::new()
        }
    }
}
```

#### `#[com_impl]`

Creates the delegating method implementation and the static virtual table
instance.

```rust
#[com_impl]
impl IFoo for Foo {
    fn method(&self, parameter: i32) -> com_runtime::Result<i8> { ... }
}

// Expands the following items:

/// Foo::IFoo::IUnknown::QueryInterface
pub unsafe extern "stdcall" fn __Foo_IFoo_query_interface(
    self_void: *mut c_void,
    riid: *mut GUID,
    out: *mut *mut c_void
) -> HRESULT
{
    let iunk_void = (self_void - __Foo_IFooVtbl_offset);
    __Foo_IUnknown_query_interface(iunk_void, riid, out)
}

// These are implemented the same way as QueryInterface above.
pub unsafe extern "stdcall" fn __Foo_IFoo_add_ref(...) { ... }
pub unsafe extern "stdcall" fn __Foo_IFoo_release(...) { ... }

pub unsafe extern "stdcall" fn __Foo_IFoo_method(
    &self, parameter: i32, __out: *mut i8
) -> HRESULT
{
    self_coclass = ( self_void - __Foo_IFooVtbl_offset ) as *mut __FooCoClass;
    let result = (*self_coclass).data.method( parameter );
    match result {
        Ok( out ) => { *__out = out; S_OK },
        Err( e ) => e
    }
}

const __Foo_IFooVtbl_INSTANCE : __IFooVtbl = __IFooVtbl {
    __base: com_runtime::IUnknownVtbl {
        query_interface: __Foo_IFoo_query_interface,
        add_ref: __Foo_IFoo_add_ref,
        release: __Foo_IFoo_release
    },
    method: __Foo_IFoo_method
};
```

#### `#[com_library]`

Defines the entry point and the `IClassFactory` implementation for creating the
CoClasses.

```rust
#[com_library(Foo)]

// Expands the following items:

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
        CLSID_Foo => Box::into_raw( Box::new( __FooCoClass::new() ) ),
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
    *out = Box::into_raw( Box::new( com_runtime::ClassFactory {
        vtable: &__ClassFactoryVtbl_INSTANCE,
        rc: 1,
        clsid: *rclsid,
    } ) );
    S_OK
}
```
