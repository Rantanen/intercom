extern crate difference;
extern crate intercom_common;

#[cfg(feature = "generators")]
mod generator_tests {

    use difference::Changeset;

    use intercom_common::model;
    use itnercom_common::generators;
    use itnercom_common::generators::GeneratorError;

    use std;
    use std::fs;
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_idl() {
        generator_test("idl", |c, out| {
            generators::idl::IdlModel::from_crate(c).unwrap().write(out)
        })
    }

    #[test]
    fn test_manifest() {
        generator_test("manifest", |c, out| {
            generators::manifest::ManifestModel::from_crate(c)
                .unwrap()
                .write(out)
        })
    }

    #[test]
    fn test_cpp_header() {
        generator_test("cpp.h", |c, out| {
            generators::cpp::CppModel::from_crate(c)
                .unwrap()
                .write_header(out)
        })
    }

    #[test]
    fn test_cpp_source() {
        generator_test("cpp.cpp", |c, out| {
            generators::cpp::CppModel::from_crate(c)
                .unwrap()
                .write_source(out)
        })
    }

    /// Runs the actual generator tests.
    fn generator_test<F>(ext: &str, f: F)
    where
        F: Fn(&model::ComCrate, &mut Write) -> Result<(), GeneratorError>,
    {
        // Read the environment variable that controls whether we should
        // update the tests or not. By default this is false and we'll
        // panic when the target doesn't match the current output.
        //
        // The user can then check the output and see if the changes are
        // intended. If so, they can specify UPDATE_XYZ_TESTS=1 to overwrite
        // the current target files with the new ones.
        let update_tests_var = format!("UPDATE_{}_TESTS", ext.replace(".", "_").to_uppercase());
        let update_tests_val = std::env::var(&update_tests_var).unwrap_or_else(|_| "0".to_string());
        let update_tests = update_tests_val == "1";

        // Get the source rust files.
        let gen_path = test_path().join("generators");
        let source_paths = fs::read_dir(gen_path)
            .unwrap()
            .into_iter()
            .map(|e| e.expect("Failed to read entry").path())
            .map(|p| p.to_str().unwrap().to_owned())
            .filter(|p| p.ends_with(".rs"))
            .map(PathBuf::from);

        // Run tests for all the source files.
        let mut failed = 0;
        for source_path in source_paths {
            // Parse the crate.
            let source_name = source_path.file_stem().unwrap().to_str().unwrap();
            let krate = model::ComCrate::parse_file(source_name, &source_path).unwrap();

            // Get the output from the generator function.
            let mut out = vec![];
            f(&krate, &mut out).unwrap();
            let actual_string = String::from_utf8(out).unwrap();

            // Read the target file into a string.
            let mut target_string = String::new();
            let target = source_path.with_extension(ext);
            {
                let _ =
                    fs::File::open(&target).and_then(|mut f| f.read_to_string(&mut target_string));
            }

            // Compare.
            let changeset =
                Changeset::new(&normalize(&actual_string), &normalize(&target_string), "\n");

            // Check for differences and decide what to do if we find any.
            if changeset.diffs.len() > 1 {
                // Differences found. Check if we need to update targets or
                // report error.
                if update_tests {
                    // User wants to update targets. Overwrite the old target
                    // file with the actual string we received from the
                    // generators.
                    let mut target_file = fs::File::create(&target)
                        .expect(&format!("Couldn't open target file {}", target.display()));
                    write!(target_file, "{}", actual_string).unwrap();
                    println!("Updated {}", target.display());
                } else {
                    // No test case update. Print the differences and report
                    // an error.
                    println!("+-------------------------------------------");
                    println!("| Output differs: {}", source_name);
                    println!("{}", changeset);
                    failed += 1;
                }
            }
        }

        if failed > 0 {
            panic!(
                "{} tests failed. \
                 Set {}=1 and run the tests again to overwrite \
                 the target files.",
                failed, update_tests_var
            );
        }
    }

    fn normalize(text: &str) -> String {
        text.replace("\r", "")
    }

    /// Get the path to 'tests' directory.
    fn test_path() -> PathBuf {
        let mut current_path = std::env::current_dir().unwrap();
        let relative_path = Path::new(file!()).parent().unwrap();

        while !current_path.join(relative_path).exists() {
            current_path = current_path.parent().unwrap().to_owned();
        }

        current_path.join(relative_path)
    }
}
