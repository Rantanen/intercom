use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use std::io::{BufWriter, Write};
use std::path::PathBuf;

extern crate intercom_common;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgMatches};

/// Runs rustfmt for the given source file
fn main()
{
    let parser = App::new("Rust Formatter Utility")
        .version(crate_version!())
        .author("Juha Lepola <fluxie@fluxie.fi>")
        .arg(
            Arg::with_name("input")
                .short("i")
                .help("File to format")
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .required(false)
                .help("Target for the formatted file")
                .index(2),
        )
        .arg(
            Arg::with_name("preserve")
                .short("p")
                .long("preserve")
                .help("Preserves the temporary crate created for the formatting."),
        );

    // Define the command line arguments using clap.
    let matches = parser.get_matches();

    // Run the command and report possiblecd  errors.
    match run_cmd(&matches) {
        Ok(path) => {
            if !matches.is_present("preserve") {
                std::fs::remove_dir_all(path).expect("Removing temporary directory failed");
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            ::std::process::exit(-1);
        }
    }
}

/// Executes the command based on the given command line parameters.
fn run_cmd(matches: &ArgMatches) -> Result<PathBuf, failure::Error>
{
    // Prepare a temporary location for generating the Rust package.
    // We will execute "cargo fmt" within this package to format the input file.
    let temp = std::env::temp_dir();
    let unique_name;
    {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        unique_name = format!("intercom-fmt-{0}_{1}", now.as_secs(), now.subsec_micros());
    }
    let temp = temp.join(&unique_name);
    std::fs::create_dir_all(&temp)?;

    // Prepare the cargo package.
    generate_cargo_toml(&unique_name, &temp)?;
    let for_output = copy_as_source(&mut get_input(matches)?, &temp)?;

    // Format the output with "cargo fmt".
    run_cargo_fmt(&temp)?;

    std::io::copy(
        &mut File::open(for_output.as_ref())?,
        get_output(matches)?.as_mut(),
    )?;
    Ok(temp)
}

/// Gets input stream for reaing the Rust code that needs to be formatted.
fn get_input(matches: &ArgMatches) -> Result<Box<dyn std::io::Read>, failure::Error>
{
    // Fallback to stdin if the input parameter was not specified.
    let input_stream: Box<dyn std::io::Read> = match matches.value_of("input") {
        Some(ref input) => {
            let input = Path::new(input);
            if !input.exists() {
                return Err(failure::err_msg("Specified input file does not exist."));
            }
            Box::new(File::open(input)?)
        }
        None => Box::new(std::io::stdin()),
    };

    Ok(input_stream)
}

/// Gets output stream for writing the formatted Rust code.
fn get_output(matches: &ArgMatches) -> Result<Box<dyn std::io::Write>, failure::Error>
{
    // Fallback to stdout if the output file was not specified.
    let output_stream: Box<dyn std::io::Write> = match matches.value_of("output") {
        Some(ref output) => {
            let output = Path::new(output);
            if output.exists() {
                return Err(failure::err_msg("Target file already exists"));
            };
            Box::new(File::create(output)?)
        }
        None => Box::new(std::io::stdout()),
    };

    Ok(output_stream)
}

/// Generates a minimal Cargo.toml for the crate in "dir".
fn generate_cargo_toml(package_name: &str, dir: &Path) -> Result<(), failure::Error>
{
    let cargo_toml = File::create(dir.join("Cargo.toml"))?;
    {
        let mut w = BufWriter::new(cargo_toml);

        writeln!(w, "[package]")?;
        writeln!(w, "name = \"{0}\"", package_name)?;
        writeln!(w, "version = \"0.1.0\"")?;
    }

    Ok(())
}

/// Copies the input file as a source file for the project.
fn copy_as_source(input: &mut dyn std::io::Read, dir: &Path)
    -> Result<Box<PathBuf>, failure::Error>
{
    // To support cargo source files must be in "src" directory.
    let src = dir.join("src");
    std::fs::create_dir_all(&src)?;

    // By default the crate must have either lib.rs or main rs.
    let target = src.join("lib.rs");
    std::io::copy(input, &mut File::create(&target)?)?;

    Ok(Box::new(target))
}

/// Executes the "cargo fmt" for the crate in "dir"
fn run_cargo_fmt(dir: &Path) -> Result<(), failure::Error>
{
    let status = Command::new("cargo").arg("fmt").current_dir(dir).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format_err!(
            "Executing \"cargo fmt\" failed with exit code \"{0}\".",
            status.code().unwrap()
        ))
    }
}
