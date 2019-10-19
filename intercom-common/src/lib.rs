#![feature(non_exhaustive)]
#![recursion_limit="128"]

#![allow(clippy::match_bool)]

#[macro_use] extern crate quote;
#[macro_use] extern crate syn;
#[macro_use] extern crate failure;

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
