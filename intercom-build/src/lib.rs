#![feature(proc_macro)]

extern crate winreg;
extern crate glob;

extern crate intercom;

mod os;

pub fn build() {
    os::build()
}
