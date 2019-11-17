extern crate intercom;
use intercom::*;

trait NotComInterface
{
}

#[com_class(NotComInterface)]
pub struct S;

impl NotComInterface for S {}
