#![feature(use_extern_macros)]

extern crate intercom;
extern crate intercom_common;
mod os;
mod host;

pub fn build() {
    os::build()
}
