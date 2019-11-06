#![crate_type = "dylib"]
#![feature(type_ascription)]

extern crate intercom;
use intercom::*;
extern crate winapi;

// Declare available COM classes.
com_library!(HelloWorld);

#[com_interface]
trait IHelloWorld
{
    fn get_hello(&self) -> String;
}

#[com_class(clsid = "{25ccb3f6-b782-4b2d-933e-54ab447da0aa}", IHelloWorld)]
#[derive(Default)]
pub struct HelloWorld {}

impl HelloWorld
{
    pub fn new() -> HelloWorld
    {
        HelloWorld {}
    }
}

#[com_impl]
impl IHelloWorld for HelloWorld
{
    fn get_hello(&self) -> String
    {
        "Hello World!".to_string()
    }
}

#[test]
fn hello_world_returns_hello_world()
{
    let hello = HelloWorld::new();
    assert_eq!(hello.get_hello(), "Hello World!");
}
