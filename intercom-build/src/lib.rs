#![feature(proc_macro)]

extern crate intercom;
mod os;
mod host;

pub fn build() {
    os::build()
}
