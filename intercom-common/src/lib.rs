#![feature(non_exhaustive, tool_lints)]
#![recursion_limit="128"]

#![allow(clippy::match_bool)]

#[macro_use] extern crate quote;
#[macro_use] extern crate syn;
extern crate proc_macro2;
extern crate sha1;
extern crate ordermap;
extern crate toml;
#[macro_use] extern crate failure;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate handlebars;

pub mod generators;
pub mod guid;
pub mod error;
pub mod idents;
pub mod tyhandlers;
pub mod returnhandlers;
pub mod utils;
pub mod ast_converters;
pub mod methodinfo;
pub mod model;
pub mod builtin_model;
pub mod foreign_ty;
pub mod attributes;
pub mod type_parser;
pub mod prelude;
