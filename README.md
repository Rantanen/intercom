# Intercom
### Utilities for writing COM components in Rust

[![crates.io](https://img.shields.io/crates/v/intercom.svg)](https://crates.io/crates/intercom)
[![Build status](https://ci.appveyor.com/api/projects/status/q88b7xk6l72kup0y?svg=true)](https://ci.appveyor.com/project/Rantanen/intercom)
[![Build Status](https://travis-ci.org/Rantanen/intercom.svg?branch=master)](https://travis-ci.org/Rantanen/intercom)
[![codecov](https://codecov.io/gh/Rantanen/intercom/branch/master/graph/badge.svg)](https://codecov.io/gh/Rantanen/intercom)

Intercom enables the user to write reusable components in Rust, that are
binary compatible with the Component Object Model interface standard. These
components can be used in any language that supports static COM components,
including C++, C# and VB.Net.

## Example

Rust COM server:

```rust
#![feature(proc_macro)]

pub use intercom::*;

#[com_library(AUTO_GUID, Calculator)]

#[com_class(AUTO_GUID)]
struct Calculator {
    value: i32
}

#[com_interface(AUTO_GUID)]
#[com_impl]
impl Calculator {
    pub fn new() -> Calculator { Calculator { value: 0 } }

    pb fn add(&mut self, value: i32) -> ComResult<i32> {
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

## Other crates

Intercom isn't the first time Rust is playing with COM interfaces. There are at
least two other crates that are related to COM support.

- [`winapi-rs`](https://github.com/retep998/winapi-rs) contains various COM
  interface definitions for Microsoft's Windows APIs.
- [`winrt-rust`](https://github.com/contextfree/winrt-rust) provides support for
  Windows Runtime (WinRT) APIs for Rust.

Ideally these crates would play well together. If you encounter usability
issues in using these crates together with Intercom, feel free to create an
issue describing the problem.

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

The Intercom libraries are built over Rust [proc macro attributes]. Currently
there are four attributes available:

- `[com_library(LIBID, Classes...)]` - A required attribute that implements the
  exported `DllGetClassObject` entry point as well as the `CoClass` for the
  `ClassFactory`.
- `[com_interface(IID)]` - An attribute that specifies a `trait` or an `impl`
  as a COM interface. The attribute results in the virtual table struct to be
  defined for the interface.
- `[com_class(CLSID, Itfs...)]` - An attribute defined on a `struct`. This
  attribute implements the necessary `CoClass` for the struct, which allows
  constructing the reference counted `ComBox<T>` instances on the object.
- `[com_impl]` - Finally the `[com_impl]` attribute specifies the `impl`s that
  implement the `[com_interface]`s for the `[com_class]` types. While the
  attribute doesn't provide any extra information for the implementation, it
  has technical reasons to exist. Its expansion is responsible for defining the
  delegating methods that know how to translate the COM call coming from the
  client into a Rust call to the user defined functions or the primary
  `IUnknown` methods implemented by the `[com_class]` expansion.

[proc macro attributes]: https://github.com/rust-lang/rfcs/blob/master/text/1566-proc-macros.md

