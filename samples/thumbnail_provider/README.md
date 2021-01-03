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

### Note on CLSID

The CLSID (`61a6080e-0e9f-3cef-50a7-622d16145b78`) is intended to be unique. A
new GUID should be generated for the CLSID if this sample is used as the basis
for a new thumbnail provider.
