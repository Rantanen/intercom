# Contributing to Intercom

Thank you for showing interest in developing Intercom! There are many ways to
help in making Intercom better. All the usual things from filing issues to
improving documentation are appreciated. However as these tasks differ very
little between various open source projects, this document doesn't go deeply
into them.

Instead this document serves as a very high level architectural description for
developers interested in diving into the Intercom codebase.

## Sub-crates

Intercom is built from five different crates: `intercom`,
`intercom-attributes`, `intercom-common`, `intercom-cli` and `intercom-build`.

### `intercom`

Intercom is the primary crate that includes the runtime infrastructure, exports
the procedural macro attributes defined in `intercom-attributes` and includes
bits of tooling required for processing intercom libraries.

In short, Intercom is the primary crate that should be suitable as the only
dependency for projects using intercom.

Given `intercom` is a normal runtime crate, it's recommended that as much of
the runtime functionality as possible would be defined in this crate.

### `intercom-attributes`

The attributes crate is probably technically the most important crate of the
four. This crate implements the proc-macro attribute magic, which defines the
various `[com_...]` attributes which are used to implement intercom libraries.

As opposed to the `intercom` crate, the `intercom-attributes` crate is a
compile-time crate. All the attribute expansion happens during compilation.
This makes it more difficult to debug and test the functionality. Because of
this, any functionality that could be implemented in the `intercom` crate
should be there instead.

### `intercom-common`

The largest crate of all. The `intercom-common` defines the common
functionality used by both `intercom` and `intercom-attributes` crates. The
crate exists mostly because of the restriction on `proc_macro` crates which
prevents them from exporting anything but procedural macros.

The crate is considered internal and should not be depended upon by projects
using intercom.

### `intercom-cli`

The CLI crate implements the command line utility for working with intercom
libraries. The actual functionality of the crate is implemented in `intercom`
and the `intercom-cli` only implements the command line interface for this.

### `intercom-build`

Build utilities for working with crates using Intercom. As of now, this is
meaningful only on Windows, where the build crate is responsible for
implementing utility method for embedding the TypeLib and the manifest in the
final dll library file.

## Runtime models

There are three different runtime models involved in the code base. The runtime
models have an effect on what information is available and how the code can
affect the dependent code.

### Runtime

Runtime code is the simplest of all. This is the code living in `intercom`
crate which the depending code calls at runtime. Things like `ComBox`
constructors or `GUID` parsing falls under this category.

As the runtime code is compiled separately from the depending code and doesn't
change afterwards, it's easy to debug and test. The downside with the runtime
code is that there's no way to inspect the depending code in any way.

### Compile time

Compile time code is defined in `intercom-attributes` and `intercom-build`.
The attributes, such as `[com_interface]` or `[com_impl]` get expanded during
the compilation of the depending code while the build utilities are invoked
during the build process.

As the compile time code is executed during compilation, the code is able to
both inspect the current source being compiled and emit new code to be
compiled. This enables Intercom to define virtual tables and delegation methods
for the intercom calls as well as affect the way the final library is
compiled.

The major disadvantage with the compile time code is that it's complex.
Especially the attribute code. The code needs to examine the depending code
through its AST and make decisions based on this. Decisions which result in
more code being generated and compiled. As a result the code needs to consider
not only things that are but also things that might be in various different
user libraries that use the attributes.

### External

Finally there's the external code in `intercom-cli`. This code has no runtime
dependency on any depending code.

As there's no compile or runtime dependency between the depending code using
the Intercom library and the `intercom-cli` module, the code in this module
has no way to affect the dependent code.

Instead what the code is able to do is to inspect the code base similar to
compile time code, but instead of emitting new bits of code based on the
inspection the `intercom-cli` are able to emit various other artifacts such
as IDL files or manifests.

## Tests

Intercom includes two kinds of tests: Rust unit tests and cross-language
integration tests. The Rust unit tests are very similar to those of other
crates and usually placed in the source files. Some crates, such as
`intercom-attributes` define the tests in a separate tests folder.

The integration tests reside in the [Test](test) directory. These tests include
a Rust test intercom server called `test_lib` and various intercom clients
implemented in other languages.

Unfortunately as the integration tests are written in other languages, they are
not covered under `cargo test`. Instead the test projects need to be compiled
and executed separately.

Currently there is no clean way to execute the tests. AppVeyor uses a
combination of [`ci.bat`](scripts/ci.bat) and [`test.ps1`](scripts/test.ps1) to
execute these while Travis uses `cmake` and running the resulting `cpp-raw`
binary manually.

Once the cross-platform support stabilizes a bit, we should implement a simple
to run test to execute all tests supported on the current platform.
