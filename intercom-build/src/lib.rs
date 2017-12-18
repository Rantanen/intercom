#![feature(proc_macro)]

extern crate winreg;
extern crate glob;

extern crate intercom;

mod os;
mod host;

pub fn build() {
    os::build()
}
