# Contributing to Intercom

Thank you for showing interest in developing Intercom! There are many ways to
help in making Intercom better. All the usual things from filing issues to
improving documentation are appreciated. However as these tasks differ very
little between various open source projects, this document mostly skips them.

Instead this document serves as a very high level architectural description for
developers interested in diving into the Intercom codebase.

## Sub-crates

Intercom is built from four different crates: `intercom`,
`intercom-attributes`, `intercom-utils` and `intercom-common`.

### `intercom`

Intercom is the primary crate that includes the runtime infrastructure as well
as exports the attributes defined in the `intercom-attributes` crate.

Given `intercom` is a normal runtime crate, it's recommended that as much of
the Intercom functionality as possible would be included in this crate.

### `intercom-attributes`

The attributes crate is probably the most importat crate of the four. This
crate implements the proc-macro attribute magic that is the various `[com_...]`
attributes which define the COM interfaces.

As opposed to the `intercom` crate, the `intercom-attributes` crate is a
compile-time crate. All the attribute expansion happens during compilation.
This makes it more difficult to debug and test the functionality. Because of
this, any functionality that could be implemented in the `intercom` crate
should be there instead.

### `intercom-utils`

The utils crate implements the command line utility for working with
Intercom-libraries. Mostly this involves things like generating cross-language
headers or bindings for the COM libraries, etc.

### `intercom-common`

The `intercom-common` crate defines common functionality used for processing
the various attributes. This functionality is needed for both
`intercom-attributes` and `intercom-utils` crates.

In theory the `intercom-utils` crate could depend on `intercom-attributes` to
get the functionality, but `intercom-attributes` depends on Rust compiler
crates which would then prevent `intercom-utils` from being compiled into an
executable.

## Runtime models

There are three different runtime models involved in the code base. The runtime
models have an effect on what information is available and how the code can
affect the dependent code.

### Runtime

Runtime code is the simplest of all. This is the code living in `interop` crate
which the depending code calls at runtime. Things like `ComBox` constructors or
`GUID` parsing falls under this category.

As the runtime code is compiled separately from the depending code and doesn't
change afterwards, it's easy to debug and test. The downside with the runtime
code is that there's no way to inspect the depending code in any way.

### Compile time

Compile time code is defined in `interop-attributes`. These attributes, such as
`[com_interface]` or `[com_impl]` get expanded during the compilation of the
depending code.

As the compile time code is expanded during compilation, the code is able to
both inspect the current source being compiled and emit new code to be
compiled. This enables Intercom to define virtual tables and delegation methods
for the COM calls.

The major disadvantage with the compile time code is that it's complex. The
code needs to examine the depending code through its AST and make decisions
based on this. Decisions which result in more code being generated and
compiled. As a result the code needs to consider not only things that are but
also things that might be in various different user libraries that use the
attributes.

### External

Finally there's the external code in `interop-utils`. This code has no runtime
dependency on any depending code.

As there's no compile or runtime dependency between the depending code using
the Intercom library and the `intercom-utils` module, the code in this module
has no way to affect the dependent code.

Instead what the code is able to do is to inspect the code base similar to
compile time code, but instead of emitting new bits of code based on the
inspection the `intercom-utils` are able to emit various other artifacts such
as IDL files or manifests.

## Tests

Currently our test harness is quite lacking. The most important tests are the
integration tests under the [test](test) directory. These tests include a Rust
test COM server using Intercom and various COM clients implemented in other
languages.

Unfortunately as the tests are written in other languages, they are not covered
under `cargo test`. Instead the test projects need to be compiled and executed
separately.

There is a [`test.ps1`](scripts/test.ps1) script for AppVeyor currently. At
some point we should extend cross-platform testing over to Travis which should
result in a test.sh script to run the tests. However both of these scripts are
designed to work on their respective CI servers.

