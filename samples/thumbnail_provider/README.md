# Thumbnail Provider Example

## Running the example

- `cargo build` the project.
- `regsvr32 target\debug\thumbnail_provider.dll` from an __elevated__ command
  prompt to register the thumbnail provider.
- Associate the thumbnail provider with a file type. `association.reg` has
  example configuration for associating the provider with `*.rust` files.

The provider can be unregistered with `regsvr32 /u
target\debug\thumbnail_provider.dll`. Any file associations must be cleaned
manually.
