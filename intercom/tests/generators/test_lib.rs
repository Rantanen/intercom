
#[com_library( AUTO_GUID )]

#[com_interface( AUTO_GUID )]
trait Foo {
    fn method( &self, a : u32 ) -> u32;
}

#[com_class( AUTO_GUID, Foo )]
struct Bar;

#[com_impl]
impl Foo for Bar {
    fn method( &self, a : u32 ) -> u32 { 0 }
}
