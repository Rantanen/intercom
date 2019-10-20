
use crate::prelude::*;

use std;
use std::env;
use std::iter::FromIterator;

/// Resolve the name of the package being compiled.
pub fn lib_name() -> String {

    // Cargo stores the currently compiled package in the CARGO_PKG_NAME
    // environment variable.
    env::var( "CARGO_PKG_NAME" )
        .expect( "Could not resolve package name. \
                 Ensure CARGO_PKG_NAME environment variable is defined." )
}

pub fn tokens_to_tokenstream<T: IntoIterator<Item=TokenStream>>(
    original : TokenStreamNightly,
    tokens : T,
) -> TokenStreamNightly
{
    TokenStreamNightly::from_iter(
        std::iter::once( original )
            .chain( tokens.into_iter().map( Into::into ) ) )
}

pub fn get_calling_convetion() -> &'static str {
    // https://msdn.microsoft.com/en-us/library/984x0h58.aspx
    //
    // This equals __stdcall on 32-bit Windows, 'C' call everywhere else.
    // Microsoft only has one 64-bit calling convention so it doesn't
    // matter there.
    "system"
}

