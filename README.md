# Intercom
### Utilities for writing COM components in Rust

[![crates.io](https://img.shields.io/crates/v/intercom.svg)](https://crates.io/crates/intercom)
[![Build Status](https://dev.azure.com/intercom-rs/Intercom/_apis/build/status/Rantanen.intercom?branchName=master)](https://dev.azure.com/intercom-rs/Intercom/_build/latest?definitionId=1&branchName=master)
[![Build status](https://ci.appveyor.com/api/projects/status/q88b7xk6l72kup0y/branch/master?svg=true)](https://ci.appveyor.com/project/Rantanen/intercom/branch/master)
[![Build Status](https://travis-ci.org/Rantanen/intercom.svg?branch=master)](https://travis-ci.org/Rantanen/intercom)
[![CircleCI](https://circleci.com/gh/Rantanen/intercom/tree/master.svg?style=svg)](https://circleci.com/gh/Rantanen/intercom/tree/master)
[![codecov](https://codecov.io/gh/Rantanen/intercom/branch/master/graph/badge.svg)](https://codecov.io/gh/Rantanen/intercom)

Intercom enables the user to write reusable components in Rust, that are
binary compatible with the Component Object Model interface standard. These
components can be used in any language that supports static COM components,
including C++, C# and VB.Net.

## Example

Rust COM server:

```rust
pub use intercom::*;

#[com_library(Calculator)]

#[com_class(Calculator)]
struct Calculator {
    value: i32
}

#[com_interface]
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

## Cross platform considerations

While COM is traditionally a Windows technology, Intercom is aimed to provide
a platform independent way to perform method calls between Rust and other
languages. This gives us two guiding principles for Intercom:

1. There exists a subset of Intercom APIs/conventions, that allows users to
   write Intercom components that can be compiled and used on any platform.
2. On platforms that have their own APIs for COM, Intercom components are
   compatible with the platform APIs and expectations.

In practice this means that on Windows, for example, Intercom will allocate
strings using `SysAllocString`. This allows Intercom components to be used
with Windows technologies, such as .Net COM interop.

However the platform independent subset will require strings to be allocated
and deallocated using [`IIntercomAllocator`]

[`IIntercomAllocator`]: https://github.com/Rantanen/intercom/issues/27

## Nightly requirement

Intercom requires nightly Rust version for few unstable features. By far the
most important of these is the procedural macro attributes. Unfortunately these
are not properly feature gated.

The features with feature gates and tracking issues that Intercom depends on are
listed below:

- `specialization` - needed for handling `ComItf`, which may refer to an
  interface that might or might not be a concrete struct interface.
  Tracking issue: [#31844](https://github.com/rust-lang/rust/issues/31844)
- `non_exhaustive` - There are some types that we may want to add items to.
  Tracking issue: [#44109](https://github.com/rust-lang/rust/issues/44109)

The following features are currently in use, but more for 'nice to have'
reasons and could be worked around if we needed to get to stable quickly:

- `try_from`, tracking issue: [#33417](https://github.com/rust-lang/rust/issues/33417)
- `try_trait`, tracking issue: [#42327](https://github.com/rust-lang/rust/issues/42327)
- `integer_atomics`, tracking issue: [#32976](https://github.com/rust-lang/rust/issues/32976)

## Technical details

### Background

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

### Implementation

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

