extern crate difference;
extern crate regex;
extern crate term;

// Compile the loggers so they are available for the tests.
extern crate log;
extern crate simple_logger;

use difference::{Changeset, Difference};

use std::fs;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use term::Terminal;

// Given the default Rust test runner doesn't expose programmatic test cases
// we are using single "check_expansions" test to process all the data files.
//
// This is similar approach to what rustfmt does.

struct TestConfig
{
    crate_path: PathBuf,
    data_path: PathBuf,
}

#[test]
fn check_expansions()
{
    let root_path = find_root().unwrap();

    let crate_path = root_path.join("intercom-attributes");
    let config = TestConfig {
        data_path: crate_path.join("tests/data"),
        crate_path,
    };
    // Running "cargo test" in a clean build directory does not
    // finalize the compilation of all the crates.
    // The final binaries are unavailable in the target directory.
    // Force the building here to ensure they are available
    // for the tests.
    build_crate("intercom");
    build_crate("intercom-fmt");

    let failed = test_path(&config, "macro", TestMode::Macro)
        + test_path(&config, "ui", TestMode::UI)
        + test_path(&config, "run", TestMode::Run);

    // Ensure there were no failures.
    //
    // If we fail here, cargo will display our printlns to the user.
    assert_eq!(failed, 0, "{} tests failed", failed);
}

struct OutputResult
{
    message: String,
    actual: String,
    expected_path: Option<String>,
    changeset: Changeset,
}

impl OutputResult
{
    /// Prints the diff.
    pub fn show_diff(&self, ctx_size: usize, t: &mut dyn Terminal<Output = std::io::Stdout>)
    {
        enum Lines
        {
            Expected(String),
            Actual(String),
            Same(String),
        }

        // Resolve the changed lines so we can print only the context.
        let mut all_lines = vec![];
        for cs in &self.changeset.diffs {
            match cs {
                Difference::Same(ref x) => all_lines.push(
                    x.lines()
                        .map(|l| Lines::Same(format!("  {}", l)))
                        .collect::<Vec<_>>(),
                ),
                Difference::Add(ref x) => all_lines.push(
                    x.lines()
                        .map(|l| Lines::Expected(format!("E {}", l)))
                        .collect::<Vec<_>>(),
                ),
                Difference::Rem(ref x) => all_lines.push(
                    x.lines()
                        .map(|l| Lines::Actual(format!("A {}", l)))
                        .collect::<Vec<_>>(),
                ),
            }
        }
        let all_lines = all_lines.into_iter().flat_map(|i| i).collect::<Vec<_>>();

        let mut ctx_counter = 0;
        let mut ctx_lines = vec![];
        for l in &all_lines {
            match l {
                Lines::Expected(..) | Lines::Actual(..) => ctx_counter = ctx_size * 2 + 1,
                _ if ctx_counter > 0 => ctx_counter -= 1,
                _ => {}
            }
            ctx_lines.push(ctx_counter);
        }
        for _ in 0..ctx_size {
            if ctx_counter > 0 {
                ctx_counter -= 1
            }
            ctx_lines.push(ctx_counter);
        }

        let mut skip = false;
        for i in 0..all_lines.len() {
            if ctx_lines[i + ctx_size] == 0 {
                if !skip {
                    writeln!(t, "...snip").expect("Write failed");
                    skip = true;
                }
                continue;
            }
            skip = false;
            match all_lines[i] {
                Lines::Same(ref x) => {
                    t.reset().unwrap();
                    writeln!(t, "{}", x).expect("Write failed");
                }
                Lines::Expected(ref x) => {
                    t.fg(term::color::GREEN).unwrap();
                    writeln!(t, "{}", x).expect("Write failed");
                }
                Lines::Actual(ref x) => {
                    t.fg(term::color::RED).unwrap();
                    writeln!(t, "{}", x).expect("Write failed");
                }
            }
        }
        t.reset().unwrap();
    }
}

/// Compiles a single file using rustc using similar options than what
/// cargo would have used.
fn build(cwd: &str, path: &str, mode: TestMode) -> (bool, String, String)
{
    #[cfg(debug_assertions)]
    let conf = "debug";

    #[cfg(not(debug_assertions))]
    let conf = "release";

    let crate_type = match mode {
        TestMode::Run => "bin",
        _ => "lib",
    };

    // Launch rustc.
    let mut cmd = std::process::Command::new("rustc");
    #[rustfmt::skip]
    cmd.current_dir(cwd)
        .env("CARGO_PKG_NAME", "TestLib")
        .args(&[
            "--crate-name", "testcrate",
            "--crate-type", crate_type,
            "--out-dir", "tests/out",
            "-L", &format!("all=../target/{}/deps", conf),
            "--extern", &format!("intercom=../target/{}/libintercom.rlib", conf),
            path,
        ]);

    // In expansion mode add the 'pretty=expanded' option.
    if let TestMode::Macro = mode {
        cmd.arg("-Zunpretty=expanded");
    }

    // Get the output.
    let output = cmd.output().expect("Failed to execute");

    // stdout/err is utf8 byte stream. Parse it into a string.
    (
        output.status.success(),
        String::from_utf8(output.stdout).expect("Bad output"),
        String::from_utf8(output.stderr).expect("Bad stderr"),
    )
}

fn format(code: &str) -> String
{
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
fn build_crate(module: &str)
{
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

fn find_root() -> std::io::Result<PathBuf>
{
    let mut root_path = std::env::current_exe()?;
    loop {
        if root_path.join("Cargo.toml").exists() {
            break;
        }
        assert!(root_path.pop());
    }

    Ok(root_path)
}

fn find_intercom_fmt() -> std::io::Result<PathBuf>
{
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
            "Could not locate intercom-fmt. Search started from \"{0}\".",
            original_path
        );
    }
}

/// Determines if the given directory has intercom-fmt
fn has_intercom_fmt(dir: &PathBuf) -> Option<PathBuf>
{
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

#[derive(Clone, Copy, PartialEq)]
enum TestMode
{
    Macro,
    UI,
    Run,
}

fn test_path(config: &TestConfig, sub_path: &str, mode: TestMode) -> usize
{
    // Get the source test data files.
    let test_data = fs::read_dir(config.data_path.join(sub_path)).unwrap();
    let source_paths = test_data
        .into_iter()
        .map(|e| e.expect("Failed to read entry").path())
        .map(|p| p.to_str().unwrap().to_owned())
        .map(|s| s.replace("\\", "/"))
        .filter(|p| p.ends_with(".rs"));
    let mut failed = 0;

    let timestamp_re = regex::RegexBuilder::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2},\d{3}")
        .multi_line(true)
        .build()
        .unwrap();
    let bracket_re = regex::Regex::new(r"\[0x.*?\]").unwrap();
    for source_path in source_paths {
        println!("Testing {}", source_path);

        // Get the source and target code.

        // The source is compiled using rustc
        let (success, mut result_stdout, mut result_stderr) =
            build(config.crate_path.to_str().unwrap(), &source_path, mode);

        // If this is a run-test, run the resulting binary.
        if success && mode == TestMode::Run {
            let mut cmd = std::process::Command::new("tests/out/testcrate");
            let output = cmd.output().expect("Failed to execute");
            assert!(output.status.success());

            // stdout/err is utf8 byte stream. Parse it into a string.
            result_stdout = String::from_utf8(output.stdout).expect("Bad output");
            result_stderr = String::from_utf8(output.stderr).expect("Bad stderr");

            // Remove values that are likely to change between runs.

            // Log Timestamps
            let result_stdout_clean = timestamp_re.replace_all(&result_stdout, "<TIMESTAMP>");
            let result_stderr_clean = timestamp_re.replace_all(&result_stderr, "<TIMESTAMP>");

            // Pointer values.
            let result_stdout_clean = bracket_re.replace_all(&result_stdout_clean, "[xxxxxxxx]");
            let result_stderr_clean = bracket_re.replace_all(&result_stderr_clean, "[xxxxxxxx]");

            result_stdout = result_stdout_clean.to_string();
            result_stderr = result_stderr_clean.to_string();
        }

        // Generate diffs for both sources
        // Ensure the linebreaks are the same for both. This seems to be
        // somewhat of an issue on AppVeyor.
        let result_stdout = result_stdout.replace("\r", "");
        let result_stderr = result_stderr.replace("\r", "");

        let mut results = vec![];

        // Use rustfmt to format both pieces of code so that we have a
        // canonical format for them. Without rustfmt we'd need to match the
        // compiler pretty print format in the reference target files - which,
        // despite its name, isn't very pretty.
        match mode {
            TestMode::Macro => results.extend(assert_test_output(
                result_stdout,
                result_stderr,
                &source_path,
                |s| format(s.trim()),
            )),
            TestMode::Run | TestMode::UI => results.extend(assert_test_output(
                result_stdout,
                result_stderr,
                &source_path,
                |s| s,
            )),
        }

        // Acquire a terminal.
        //
        // Especially in CI environments there is no real terminal but since we
        // are still passing the TerminfoTerminal through the functions, we need
        // one. We'll try ANSI terminal as a fallback.
        let mut t = term::stdout().unwrap_or_else(|| {
            Box::new(term::TerminfoTerminal::new_with_terminfo(
                std::io::stdout(),
                term::terminfo::TermInfo::from_name("ansi").expect("ansi terminal not supported"),
            ))
        });
        for r in results {
            if std::env::var("UPDATE_TARGETS").is_ok() && r.expected_path.is_some() {
                // The user wants to update the targets.
                let mut target_file = fs::File::create(r.expected_path.as_ref().unwrap()).unwrap();
                target_file.write_all(r.actual.as_bytes()).expect(&format!(
                    "Writing target file {} failed",
                    r.expected_path.as_ref().unwrap()
                ));
                writeln!(
                    t,
                    "-----------------------------------------------------------------"
                )
                .unwrap();
                writeln!(t, "UPDATED: {}", r.expected_path.unwrap()).unwrap();
                writeln!(
                    t,
                    "-----------------------------------------------------------------"
                )
                .unwrap();
            } else {
                writeln!(
                    t,
                    "-----------------------------------------------------------------"
                )
                .unwrap();
                writeln!(t, "FAIL: {}", r.message).unwrap();
                writeln!(t, "").unwrap();
                r.show_diff(5, &mut *t);
                writeln!(t, "").unwrap();
                writeln!(
                    t,
                    "-----------------------------------------------------------------"
                )
                .unwrap();
                failed += 1;
            }
        }
    }
    failed
}

fn assert_test_output(
    result_stdout: String,
    result_stderr: String,
    source_path: &str,
    sanitize: impl Fn(String) -> String,
) -> Vec<OutputResult>
{
    // Construct the target file path by replacing the ".source.rs" with a
    // ".target.rs". There's a small discrepancy here as the .source.rs had
    // to be at the end for the file to count as source file, but here
    // we are replacing the .target. everywhere in the file name.
    //
    // This shouldn't matter in practice as these are test files and we can
    // decide on their naming as we write them.
    vec![
        assert_output_with(result_stdout, source_path, "stdout", |s| {
            sanitize(strip_path(source_path, s))
        }),
        assert_output_with(result_stderr, source_path, "stderr", |s| {
            strip_path(source_path, s)
        }),
    ]
    .into_iter()
    .flat_map(|v| v)
    .collect()
}

fn assert_output_with(
    actual: String,
    actual_path: &str,
    output_kind: &str,
    sanitize: impl Fn(String) -> String,
) -> Option<OutputResult>
{
    let name = Path::new(actual_path)
        .file_name()
        .unwrap()
        .to_string_lossy();
    let actual = sanitize(actual.clone());
    let expected_path_string = format!("{}.{}", actual_path, output_kind);
    let expected_path = Path::new(&expected_path_string);
    if expected_path.exists() {
        let expected = std::fs::read_to_string(expected_path)
            .unwrap()
            .replace("\r", "");
        let expected = sanitize(expected);
        if std::env::var("FORCE_UPDATE_TARGETS").is_ok() {
            Some(OutputResult {
                message: format!("Forcing update"),
                changeset: Changeset::new(&actual, &expected, "\n"),
                actual: actual,
                expected_path: Some(expected_path_string),
            })
        } else if expected != actual {
            Some(OutputResult {
                message: format!("{} {} output differs", name, output_kind),
                changeset: Changeset::new(&actual, &expected, "\n"),
                actual: actual,
                expected_path: Some(expected_path_string),
            })
        } else {
            None
        }
    } else if !actual.is_empty() {
        Some(OutputResult {
            message: format!("{} {} output was not expected", name, output_kind),
            changeset: Changeset::new(&actual, "", "\n"),
            actual: actual,
            expected_path: None,
        })
    } else {
        None
    }
}

fn strip_path(path: &str, source: String) -> String
{
    let root = Path::new(path).parent().unwrap().to_string_lossy() + "/";
    source.replace(root.as_ref(), "")
}
