extern crate difference;
extern crate regex;
extern crate term;

use difference::{Changeset, Difference};

use std::fs;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::process::{Command, Stdio};

// Given the default Rust test runner doesn't expose programmatic test cases
// we are using single "check_expansions" test to process all the data files.
//
// This is similar approach to what rustfmt does.

#[test]
fn check_expansions() {
    let root_path = find_root().unwrap();

    let crate_path = root_path.join("intercom-attributes");
    let data_path = crate_path.join("tests/data");

    // Get the source test data files.
    let test_data = fs::read_dir(data_path).unwrap();
    let source_paths = test_data
        .into_iter()
        .map(|e| e.expect("Failed to read entry").path())
        .map(|p| p.to_str().unwrap().to_owned())
        .filter(|p| p.ends_with(".source.rs"))
        .collect::<Vec<_>>();

    // Running "cargo test" in a clean build directory does not
    // finalize the compilation of all the crates.
    // The final binaries are unavailable in the target directory.
    // Force the building here to ensure they are available
    // for the tests.
    build_crate("intercom");
    build_crate("intercom-fmt");

    // Process each source file. Track the failures.
    let mut failed = 0;
    for source_path in source_paths {
        // Get the source and target code.

        // Construct the target file path by replacing the ".source.rs" with a
        // ".target.rs". There's a small discrepancy here as the .source.rs had
        // to be at the end for the file to count as source file, but here
        // we are replacing the .target. everywhere in the file name.
        //
        // This shouldn't matter in practice as these are test files and we can
        // decide on their naming as we write them.
        let mut target_code = String::new();
        let target_path = source_path.replace(".source.rs", ".target.rs");
        {
            // Scope the lifetime of the open file handle.

            // Ignore the possible result. If we can't read the file we'll assume it was empty.
            let _ = fs::File::open(&target_path)
                .map(|mut target_file| target_file.read_to_string(&mut target_code));
        }

        // The source is compiled using rustc
        let mut source_code = build(crate_path.to_str().unwrap(), &source_path);

        // Generate diffs for both sources
        // Ensure the linebreaks are the same for both. This seems to be
        // somewhat of an issue on AppVeyor.
        target_code = target_code.replace("\r", "");
        source_code = source_code.replace("\r", "");

        // Normalize the calling conventions.
        // The expected results use "stdcall". "C" is more likely
        // to appear in the tests on its own.
        // See https://github.com/Rantanen/intercom/pull/31#issuecomment-353516541
        target_code = target_code.replace("stdcall", "C");
        source_code = source_code.replace("stdcall", "C");

        // Use rustfmt to format both pieces of code so that we have a
        // canonical format for them. Without rustfmt we'd need to match the
        // compiler pretty print format in the reference target files - which,
        // despite its name, isn't very pretty.
        let changeset = Changeset::new(
            &format(source_code.trim()),
            &format(target_code.trim()),
            "\n",
        );

        // If these were equal, there's only one "Same" diff segment.
        // If there is more than one, they differed.
        if changeset.diffs.len() > 1 {
            if std::env::var("UPDATE_TARGETS").is_ok() {
                // The user wants to update the targets.
                use std::io::Write;
                let mut target_file = fs::File::create(&target_path).unwrap();
                target_file
                    .write_all(source_code.as_bytes())
                    .expect(&format!("Writing target file {} failed", &target_path));
            } else {
                // Print the changeset for debugging purposes and increment the
                // amount of failed items. By default this prints nice colored diff.
                if let Some(mut t) = term::stdout() {
                    for i in 0..changeset.diffs.len() {
                        match changeset.diffs[i] {
                            Difference::Same(ref x) => {
                                t.reset().unwrap();
                                for line in x.lines() {
                                    writeln!(t, "  {}", line).expect("Write failed");
                                }
                            }
                            Difference::Add(ref x) => {
                                t.fg(term::color::GREEN).unwrap();
                                for line in x.lines() {
                                    writeln!(t, "+ {}", line).expect("Write failed");
                                }
                            }
                            Difference::Rem(ref x) => {
                                t.fg(term::color::RED).unwrap();
                                for line in x.lines() {
                                    writeln!(t, "- {}", line).expect("Write failed");
                                }
                            }
                        }
                    }
                } else {
                    println!("{}", changeset);
                }
                failed += 1;
            }
        }
    }

    // Ensure there were no failures.
    //
    // If we fail here, cargo will display our printlns to the user.
    assert_eq!(failed, 0, "{} tests failed", failed);
}

/// Compiles a single file using rustc using similar options than what
/// cargo would have used.
fn build(cwd: &str, path: &str) -> String {
    #[cfg(debug_assertions)]
    let conf = "debug";

    #[cfg(not(debug_assertions))]
    let conf = "release";

    // Launch rustc.
    let output = std::process::Command::new("rustc")
        .current_dir(cwd)
        .env("CARGO_PKG_NAME", "TestLib")
        .args(&[
            "--crate-name",
            "source",
            "--crate-type",
            "lib",
            path,
            "--out-dir",
            "tests/out",
            "-L",
            &format!("dependency=../target/{}/deps", conf),
            "--extern",
            &format!("intercom=../target/{}/libintercom.rlib", conf),
            "--pretty=expanded",
            "-Z",
            "unstable-options",
        ])
        .output()
        .expect("Failed to execute");

    // Ensure the compilation was successful.
    if !output.status.success() {
        println!("FAILED TO COMPILE SOURCE {}", path);
        panic!(format!(
            "{}\n\n{}",
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        ));
    }

    // stdout is utf8 byte stream. Parse it into a string.
    String::from_utf8(output.stdout).expect("Bad output")
}

fn format(code: &str) -> String {
    let intercom_fmt = find_intercom_fmt().unwrap();
    let mut formatter = Command::new(&intercom_fmt)
        .arg("-p")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to launch formatter {:?}", intercom_fmt));

    // Send the code to the formatter.
    // "stdin" of the formatter will be closed automatically when the scope terminates.
    {
        let mut stdin = formatter.stdin.as_mut().unwrap();
        std::io::copy(&mut Cursor::new(code.as_bytes()), &mut stdin)
            .expect("Failed to send data for formatting.");
    }

    let output = formatter
        .wait_with_output()
        .expect("Failed to read formatter results.");
    let status = output.status;
    assert!(
        status.success() && status.code() == Some(0),
        "Formatting failed with status \"{:?}\".",
        status.code()
    );

    // Convert the UTF-8 output into a string.
    String::from_utf8(output.stdout).expect("Bad output")
}

/// Builds the "intercom" library
fn build_crate(module: &str) {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    #[cfg(not(debug_assertions))]
    {
        cmd.arg("--release");
    }

    let status = cmd
        .current_dir(find_root().unwrap().join(module))
        .status()
        .expect(&format!("Failed to build crate \"{0}\"", module));
    assert!(status.success());
}

fn find_root() -> std::io::Result<PathBuf> {
    let mut root_path = std::env::current_exe()?;
    loop {
        if root_path.join("Cargo.toml").exists() {
            break;
        }
        assert!(root_path.pop());
    }

    Ok(root_path)
}

fn find_intercom_fmt() -> std::io::Result<PathBuf> {
    // Avoid the need to determine the current build target by basing the search on the current
    // executable.
    let mut intercom_fmt_dir = std::env::current_exe()?;
    assert!(intercom_fmt_dir.pop()); // The name of the executable.
    let original_path = format!("{:?}", intercom_fmt_dir);
    loop {
        // Stop search when intercom-fmt has been found.
        if let Some(intercom_fmt) = has_intercom_fmt(&intercom_fmt_dir) {
            return Ok(intercom_fmt);
        }

        // Move towards root
        assert!(
            intercom_fmt_dir.pop(),
            format!(
                "Could not locate intercom-fmt. Search started from \"{0}\".",
                original_path
            )
        );
    }
}

/// Determines if the given directory has intercom-fmt
fn has_intercom_fmt(dir: &PathBuf) -> Option<PathBuf> {
    #[cfg(windows)]
    let intercom_fmt = dir.join("intercom-fmt.exe");

    #[cfg(not(windows))]
    let intercom_fmt = dir.join("intercom-fmt");

    if intercom_fmt.exists() && intercom_fmt.metadata().unwrap().is_file() {
        Some(intercom_fmt)
    } else {
        None
    }
}
