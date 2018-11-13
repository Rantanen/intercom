
//! Enables the generation of manifest file for registration free COM on
//! Windows.

use std::io::Write;
use std::path::Path;

use super::GeneratorError;

use model;
use utils;
use ast_converters::GetIdent;

use handlebars::Handlebars;

#[derive(PartialEq, Debug, Serialize)]
pub struct ManifestModel {
    pub lib_name : String,
    pub lib_id : String,
    pub file_name : String,
    pub coclasses : Vec<ManifestCoClass>,
}

#[derive(PartialEq, Debug, Serialize)]
pub struct ManifestCoClass {
    pub name : String,
    pub clsid : String,
}

impl ManifestModel {

    /// Generates the model from files in the path.
    ///
    /// - `path` - The path must point to a crate root containing Cargo.toml or
    ///            to the Cargo.toml itself.
    pub fn from_path( path : &Path,) -> Result<ManifestModel, GeneratorError>
    {
        let krate = model::ComCrate::parse_package( path )
                .map_err( GeneratorError::CrateParseError )?;
        ManifestModel::from_crate( &krate )
    }

    /// Generates the model from an existing Intercom crate model.
    pub fn from_crate(
        c : &model::ComCrate
    ) -> Result<ManifestModel, GeneratorError>
    {
        let lib = c.lib().as_ref().ok_or( GeneratorError::MissingLibrary )?;

        // Gather all the com classes. These need to be declared in the manifest.
        let classes = lib.coclasses().iter().map(|class_path| {

            let class_name = class_path
                    .get_ident()
                    .expect( "coclass had no name" )
                    .to_string();
            let coclass = &c.structs()[ &class_name.to_string() ];
            let clsid = coclass.clsid().as_ref()
                    .ok_or_else( || GeneratorError::MissingClsid(
                                        coclass.name().to_string() ) )?;
            Ok( ManifestCoClass {
                name : coclass.name().to_string(),
                clsid : format!( "{}", clsid ),
            } )
        } ).collect::<Result<Vec<_>, GeneratorError>>()?;

        Ok( ManifestModel {
            lib_name : utils::pascal_case( lib.name() ),
            file_name : lib.name().to_owned(),
            lib_id : format!( "{}", lib.libid() ),
            coclasses : classes,
        } )
    }

    /// Generates the manifest content.
    ///
    /// - `out` - The writer to use for output.
    pub fn write(
        &self,
        out : &mut Write,
    ) -> Result<(), GeneratorError>
    {
        let mut reg = Handlebars::new();
        reg.register_template_string( "manifest", include_str!( "manifest.hbs" ) )
                .expect( "Error in the built-in Manifest template." );

        let rendered = reg
                .render( "manifest", self )
                .expect( "Rendering a valid ComCrate to Manifest failed" );
        write!( out, "{}", rendered )?;

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn crate_to_manifest() {

        let krate = model::ComCrate::parse( "com_library", &[ r#"
            #[com_library( libid = "11112222-3333-4444-5555-666677778888", CoClass )]
            struct S;

            #[com_interface( com_iid = "22223333-4444-5555-6666-777788889999", base = NO_BASE )]
            trait IInterface {
                fn method( &self, a : u32 ) -> ComResult<bool>;
            }

            #[com_class( clsid = "33334444-5555-6666-7777-888899990000", CoClass, IInterface )]
            struct CoClass;

            #[com_interface( com_iid = "44445555-6666-7777-8888-99990000AAAA" )]
            #[com_impl]
            impl CoClass {
                pub fn new() -> CoClass { CoClass }
                pub fn com_method( &self, b : u32 ) {}
            }

            #[com_impl]
            impl IInterface for CoClass {
                fn method( &self, a : u32 ) -> ComResult<bool> { unreachable!() }
            }
        "# ] ).expect( "Could not parse test crate" );

        let expected_manifest = ManifestModel {
            lib_name : "ComLibrary".to_owned(),
            lib_id : "{11112222-3333-4444-5555-666677778888}".to_owned(),
            file_name : "com_library".to_owned(),
            coclasses : vec![
                ManifestCoClass {
                    name : "CoClass".to_owned(),
                    clsid : "{33334444-5555-6666-7777-888899990000}".to_owned(),
                },
                ManifestCoClass {
                    name : "Allocator".to_owned(),
                    clsid : "{EC444090-9CDC-31A4-4023-D0458C5CD45C}".to_owned(),
                },
                ManifestCoClass {
                    name : "ErrorStore".to_owned(),
                    clsid : "{1467B819-62DF-3720-4EE6-6E76FD4E1120}".to_owned(),
                },
            ],
        };

        let actual_manifest = ManifestModel::from_crate( &krate ).unwrap();

        assert_eq!( expected_manifest, actual_manifest );
    }
}
