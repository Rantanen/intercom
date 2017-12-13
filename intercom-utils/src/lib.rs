
extern crate syn;
extern crate intercom_common;
extern crate clap;
extern crate toml;
extern crate glob;

mod error;
mod parse;
mod idl;
pub use idl::create_idl;
