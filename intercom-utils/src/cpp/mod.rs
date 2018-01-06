
extern crate std;

use std::collections::HashMap;
use parse::*;
use clap::ArgMatches;
use error::AppResult;
use std::fs::File;
use std::io::Write;
use intercom_common::guid::*;

mod header;
mod implementation;

/// Runs the 'cpp' subcommand.
pub fn run( cpp_params : &ArgMatches ) -> AppResult {

    // Parse the sources and convert the result into an IDL.
    let ( renames, result ) = parse_crate(
            cpp_params.value_of( "path" ).unwrap() )?;
    let target = cpp_params.value_of( "output" ).expect( "Output path not specified." );
    result_to_cpp( &result, &renames, target );
    Ok(())
}

/// Gets an empty string for ind One level of indentation is four spaces.
pub fn get_indentation(
    level: usize
) -> String {
    let spaces = level * 4;
    let indentation = format!( r###"{: <1$}"###, "", spaces );
    indentation
}

/// Converts a Rust type into applicable C++ type.
pub fn to_cpp_type(
    ty: &str
) -> &str {

    match ty {
        "int8" => "int8_t",
        "uint8" => "uint8_t",
        "int16" => "int16_t",
        "uint16" => "uint16_t",
        "int32" => "int32_t",
        "uint32" => "uint32_t",
        "int64" => "int64_t",
        "uint64" => "uint64_t",
        "BStr" => "intercom::BSTR",
        "HRESULT" => "intercom::HRESULT",
        _ => ty,
    }
}

/// Converts a guid to binarys representation.
pub fn guid_to_binary(
    g: &GUID
) -> String {

    format!( "{{0x{:08x},0x{:04x},0x{:04x},{{0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x},0x{:02x}}}}}",
            g.data1, g.data2, g.data3,
            g.data4[0], g.data4[1], g.data4[2], g.data4[3],
            g.data4[4], g.data4[5], g.data4[6], g.data4[7] )
}

fn result_to_cpp(
    r : &ParseResult,
    rn : &HashMap<String, String>,
    output : &str
)
{
    // Generate the header.
    let header = header::generate( r, rn );
    let header_target = format!( "{}/{}.h", output, r.libname );
    let header_target = File::create( &header_target )
            .expect( &format!( "Creating file \"{}\" failed.", &header_target ) );
    writeln!( &header_target, "{}", &header ).expect( "Writing header failed." );

    // Generate the implementation.
    let implementation = implementation::generate( r, rn );
    let implementation_target = format!( "{}/{}.cpp", output, r.libname );
    let implementation_target = File::create( &implementation_target )
            .expect( &format!( "Creating file \"{}\" failed.", &implementation_target ) );
    writeln!( &implementation_target, "{}", &implementation ).expect( "Writing implementation failed." );
}
