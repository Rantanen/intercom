
extern crate intercom_common;

use std::path::{Path, PathBuf};
use intercom_common::model;

#[test]
fn parsing_crate_with_single_file_by_package_path() {

    let crate_path = test_path().join( "single_file_lib" );
    let model = model::ComCrate::parse_package( crate_path.as_ref() ).unwrap();
    assert_eq!( model, expected_crate( "single_file_lib" ) );
}

#[test]
fn parsing_crate_with_single_file_by_toml_path() {

    let crate_path = test_path().join( "single_file_lib/Cargo.toml" );
    let model = model::ComCrate::parse_package( crate_path.as_ref() ).unwrap();
    assert_eq!( model, expected_crate( "single_file_lib" ) );
}

#[test]
fn parsing_crate_with_multi_file_by_package_path() {

    let crate_path = test_path().join( "multi_file_lib" );
    let model = model::ComCrate::parse_package( crate_path.as_ref() ).unwrap();
    assert_eq!( model, expected_crate( "multi_file_lib" ) );
}

#[test]
fn parsing_crate_with_multi_file_by_toml_path() {

    let crate_path = test_path().join( "multi_file_lib/Cargo.toml" );
    let model = model::ComCrate::parse_package( crate_path.as_ref() ).unwrap();
    assert_eq!( model, expected_crate( "multi_file_lib" ) );
}

fn expected_crate( lib_name : &str ) -> model::ComCrate {

    model::ComCrate::parse( lib_name, &[
        r#"
            use cls1::Class1;
            use cls2::Class2;

            #[com_library( "00000001-0000-0000-0000-000000000000", Class1, Class2 )]

            mod itfs {

                #[com_interface( "00000002-0000-0000-0000-000000000000")]
                trait Interface1 {}

                #[com_interface( "00000003-0000-0000-0000-000000000000")]
                trait Interface2 {}
            }

            mod cls1 {

                #[com_class( "00000004-0000-0000-0000-000000000000", Class1)]
                pub struct Class1;

                #[com_interface( "00000006-0000-0000-0000-000000000000")]
                #[com_impl]
                impl Class1 {}
            }

            mod cls2 {

                use super::itfs;

                #[com_class( "00000005-0000-0000-0000-000000000000", Interface1, Interface2)]
                struct Class2;

                #[com_impl]
                impl itfs::Interface1 for Class2 {}

                #[com_impl]
                impl itfs::Interface2 for Class2 {}
            }

            mod no_guid {

                use super::itfs;

                #[com_class( NO_GUID, Interface1, Interface2)]
                #[derive(Debug)]
                pub struct NoGuid
                {
                    test: String
                }

                #[com_impl]
                impl itfs::Interface1 for NoGuid {}

                #[com_impl]
                impl itfs::Interface2 for NoGuid {}
            }

        "#
    ] ).unwrap()
}

fn test_path() -> PathBuf {
    let mut current_path = std::env::current_dir().unwrap();
    let relative_path = Path::new( file!() ).parent().unwrap();

    while ! current_path.join( relative_path ).exists() {
        current_path = current_path.parent().unwrap().to_owned();
    }

    current_path.join( relative_path )
}
