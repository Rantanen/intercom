use crate::prelude::*;

use std::env;
use std::iter::FromIterator;

/// Resolve the name of the package being compiled.
pub fn lib_name() -> String
{
    // Cargo stores the currently compiled package in the CARGO_PKG_NAME
    // environment variable.
    env::var("CARGO_PKG_NAME").expect(
        "Could not resolve package name. \
         Ensure CARGO_PKG_NAME environment variable is defined.",
    )
}

pub fn tokens_to_tokenstream<T: IntoIterator<Item = TokenStream>>(
    original: TokenStreamNightly,
    tokens: T,
) -> TokenStreamNightly
{
    TokenStreamNightly::from_iter(
        std::iter::once(original).chain(tokens.into_iter().map(Into::into)),
    )
}

// These functions are left in for debugging purposes.
//
// They can be swapped in for the tokens_to_tokenstreams to get a printout of
// span information (bad vs ok) when tracking down tokens still referring to
// call site spans and thus messing the error messages.
/*
extern crate proc_macro;
pub fn tokens_to_validated_tokenstream<T: IntoIterator<Item = TokenStream>>(
    call_site: proc_macro::Span,
    original: TokenStreamNightly,
    tokens: T,
) -> TokenStreamNightly
{
    TokenStreamNightly::from_iter(
        std::iter::once(original)
            .chain(tokens.into_iter().map(|tt| do_valid(&call_site, tt.into())))
    )
}

pub fn do_valid(
    call_site: &proc_macro::Span,
    ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream
{
    let mut v = vec![];
    for tt in ts {
        let s = tt.span();
        if s.start().line == call_site.start().line &&
            s.start().column == call_site.start().column {

            eprint!("BAD: ");
        } else {
            eprint!("OK:  ");
        }
        match tt {
            proc_macro::TokenTree::Group(grp) => {
                let (left, right) = match grp.delimiter() {
                    proc_macro::Delimiter::Parenthesis => ( "(", ")" ),
                    proc_macro::Delimiter::Brace => ( "{", "}" ),
                    proc_macro::Delimiter::Bracket => ( "[", "]" ),
                    proc_macro::Delimiter::None => ( "@", "€" ),
                };
                eprintln!("{}", left);
                v.push(proc_macro::TokenTree::Group(
                        proc_macro::Group::new(grp.delimiter(), do_valid(call_site, grp.stream()))));
                eprintln!("--- {}", right);
            },
            tt => {
                eprintln!("{}", tt);
                v.push(tt)
            }
        }
    }
    proc_macro::TokenStream::from_iter(v)
}
*/
