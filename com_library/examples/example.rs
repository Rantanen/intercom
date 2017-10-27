#![feature(unique, shared)]
#![feature( plugin, custom_attribute, attr_literals )]
#![plugin( com_library )]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

#[com_library( Foo, Bar )]

extern crate com_runtime;

use std::os::raw::c_void;

#[com_class("{12341234-1234-1234-1234-123412340001}", Foo)]
struct Foo {}

#[com_interface("{12341234-1234-1234-1234-123412340002}")]
#[com_impl]
impl Foo
{
    fn new() -> Foo { eprintln!( "Created Foo" ); Foo {} }
    fn bar( &self, a : u32 ) -> com_runtime::ComResult<u8> { Ok(10) }
    fn baz1( &self, a : u32 ) -> com_runtime::ComResult<()> { Ok(()) }
    fn baz2( &self, a : u32 ) -> com_runtime::ComResult<()> { Ok(()) }
    fn baz3( &self, a : u32 ) -> com_runtime::ComResult<()> { Ok(()) }
    fn baz4( &self, a : u32 ) -> com_runtime::ComResult<()> { Ok(()) }
    fn baz5( &self, a : u32 ) -> com_runtime::ComResult<()> { Ok(()) }
    fn baz6( &self, a : u32 ) -> com_runtime::ComResult<()> { Ok(()) }
    fn baz7( &self, a : u32 ) -> com_runtime::ComResult<()> { Ok(()) }
    fn baz8( &self, a : u32 ) -> com_runtime::ComResult<()> { Ok(()) }
    fn baz9( &self, a : u32 ) -> com_runtime::ComResult<()> { Ok(()) }
    fn baz0( &self, a : u32 ) -> com_runtime::ComResult<()> { Ok(()) }
}

#[com_class( "{12341234-1234-1234-1234-123412340003}", Bar, BarItf )]
struct Bar {
    value : u32
}

impl Drop for Bar
{
    fn drop( &mut self ) {
        eprintln!( "Dropped 'Bar' with value: {}", self.value )
    }
}

#[com_interface("{12341234-1234-1234-1234-123412340004}")]
#[com_impl]
impl Bar
{
    fn new() -> Bar { eprintln!( "Created Bar" ); Bar { value : 0 } }
    fn bar( &self, a : u32 ) -> com_runtime::ComResult<u8> {
        eprintln!( "bar" );
        Ok( ( ( a + self.value ) & 0xff ) as u8 )
    }
    fn baz( &mut self, b : u32 ) -> u16 {
        eprintln!( "baz" );
        eprintln!( "{:p}", &self );
        self.value = b;
        ( b & 0xffff ) as u16
    }
    fn static_method( c : u32 ) -> u32 { 10 }
    fn empty_method() -> u32 { 10 }
}

#[com_interface("{12341234-1234-1234-1234-123412340005}")]
trait BarItf
{
    fn stuff( &self, a : u32 ) -> com_runtime::ComResult<u16>;
}

#[com_impl]
impl BarItf for Bar {
    fn stuff( &self, a : u32 ) -> com_runtime::ComResult<u16> { Ok(123) }
}

fn create( clsid : com_runtime::GUID ) -> com_runtime::ComResult< com_runtime::RawComPtr >
{
    Err( com_runtime::S_OK )
}

struct CF {
    clsid : com_runtime::REFCLSID
}

fn main()
{
    let dummy_classfactory = com_runtime::ClassFactory::new( &CLSID_Bar, |clsid| Err( com_runtime::S_OK ) );
    println!("Class factory size: {}, expected: {}",
        std::mem::size_of_val(&dummy_classfactory),
        std::mem::size_of::<usize>());

    // Horrible Rust code ahead. This mimics the C++ calls.
    unsafe {

        // DllGetClassObject params. Null value for the return value.
        // It will be assigned by the DllGetClassObject.
        let mut clsid = com_runtime::GUID::parse( "{12341234-1234-1234-1234-123412340003}" ).unwrap();
        let mut iid = com_runtime::GUID::parse( "{12341234-1234-1234-1234-123412340004}" ).unwrap();
        let mut classFactory_ptr = std::mem::transmute( std::ptr::null::<c_void>() );

        // Acquire the class factory.
        eprintln!( "DllGetClassObject: {}",
                  DllGetClassObject( &mut clsid, &mut com_runtime::IID_IClassFactory, &mut classFactory_ptr ) );

        // Got the class factory pointer.
        //
        // The vtable is a the start of the struct and we asked for the
        // IClassFactory interface, we can consider the pointer to the class
        // factory as a pointer to a IClassFactory vtable pointer.
        let classFactory_vtbl_ptr_ptr = classFactory_ptr as *const *const com_runtime::ClassFactoryVtbl;

        // Invoke the create instance method.
        let mut bar_ptr = std::mem::transmute( std::ptr::null::<c_void>() );
        eprintln!( "create_instance: {}",
                ((**classFactory_vtbl_ptr_ptr).create_instance)(
                    classFactory_ptr,  // &this
                    std::mem::transmute( std::ptr::null::<c_void>() ),
                    &mut iid,
                    &mut bar_ptr ) );

        // Got the interface pointer.
        let ibar : &mut com_runtime::ComBox< Bar > = std::mem::transmute( bar_ptr );
        eprintln!( "transmuted" );
        eprintln!( "ComBox: {:p}, Value: {:p}", bar_ptr, ibar as &Bar );
        ibar.baz( 53 );

        let Bar_vtbl = com_runtime::ComBox::vtable( &ibar ).Bar;
        let fun = Bar_vtbl.baz;
        let fun2 = Bar_vtbl.bar;

        // Invoke baz()
        let baz_val = 0u16;
        eprintln!( "baz: {}",
                (fun)( &Bar_vtbl as *const _ as usize as *mut _, 53 ) );

        // Invoke bar()
        let mut bar_val = 0u8;
        eprintln!( "bar: {}",
                ( com_runtime::ComBox::vtable( &ibar ).Bar.bar)(
                    std::mem::transmute( &ibar ),
                    10,
                    &mut bar_val ) );

        eprintln!( "Result: {}", bar_val );
    }
}
