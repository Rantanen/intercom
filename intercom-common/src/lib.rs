#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro;
extern crate sha1;

pub mod guid;
pub mod error;
pub mod idents;
pub mod tyhandlers;
pub mod returnhandlers;
pub mod utils;
pub mod ast_converters;
pub mod methodinfo;
