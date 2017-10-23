#![feature( plugin, custom_attribute )]
#![plugin( com_library )]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

#[com_library( Foo, Bar )]

extern crate com_runtime;

use std::os::raw::c_void;

struct Foo {}

#[com_visible("{12341234-1234-1234-1234-123412340001}")]
impl Foo
{
    fn new() -> Foo { eprintln!( "Created Foo" ); Foo {} }
    fn bar( &self, a : u32 ) -> com_runtime::ComResult<u8> { Ok(10) }
}

struct Bar {
    value : u32
}

impl Drop for Bar
{
    fn drop( &mut self ) {
        eprintln!( "Dropped 'Bar' with value: {}", self.value )
    }
}

#[com_visible("{12341234-1234-1234-1234-123412340002}")]
impl Bar
{
    fn new() -> Bar { eprintln!( "Created Bar" ); Bar { value : 0 } }
    fn bar( &self, a : u32 ) -> com_runtime::ComResult<u8> {
        eprintln!( "bar" );
        Ok( ( ( a + self.value ) & 0xff ) as u8 )
    }
    fn baz( &mut self, b : u32 ) -> u16 {
        eprintln!( "baz" );
        self.value = b;
        ( b & 0xffff ) as u16
    }
    fn static_method( c : u32 ) -> u32 { 10 }
    fn empty_method() -> u32 { 10 }
}

fn main()
{
    // Horrible Rust code ahead. This mimics the C++ calls.
    unsafe {

        // DllGetClassObject params. Null value for the return value.
        // It will be assigned by the DllGetClassObject.
        let mut clsid = com_runtime::GUID::parse( "{12341234-1234-1234-1234-123412340002}" ).unwrap();
        let mut iid = com_runtime::GUID::parse( "{12341234-1234-1234-1234-123412340002}" ).unwrap();
        let mut classFactory_ptr = std::mem::transmute( std::ptr::null::<c_void>() );

        // Acquire the class factory.
        eprintln!( "DllGetClassObject: {}",
                  DllGetClassObject( &mut clsid, &mut iid, &mut classFactory_ptr ) );

        // Got the class factory pointer. Cast to class factory.
        let classFactory : &mut com_runtime::ClassFactory
                = std::mem::transmute( classFactory_ptr );

        // Invoke the create instance method.
        let mut bar_ptr = std::mem::transmute( std::ptr::null::<c_void>() );
        eprintln!( "create_instance: {}",
                (classFactory.__vtable.create_instance)(
                    std::mem::transmute( classFactory ),  // &this
                    std::mem::transmute( std::ptr::null::<c_void>() ),
                    &mut iid,
                    &mut bar_ptr ) );

        // Got the interface pointer.
        let mut ibar : &mut __Bar_ptr
                = std::mem::transmute( bar_ptr );
        eprintln!( "transmuted" );
        let fun = &ibar.__vtable.baz;
        let fun2 = &ibar.__vtable.baz;

        // Invoke baz()
        let mut baz_val = 0u16;
        eprintln!( "baz: {}",
                (fun)(
                    std::mem::transmute( &ibar ),
                    53 ) );

        // Invoke bar()
        let mut bar_val = 0u8;
        eprintln!( "bar: {}",
                (ibar.__vtable.bar)(
                    std::mem::transmute( &ibar ),
                    10,
                    &mut bar_val ) );

        eprintln!( "Result: {}", bar_val );
    }

    /*
    let f = Foo {};
    println!( "{}", f.bar( 0 ).unwrap() );

    let mut f = Box::new( __Foo_ptr::new() );
    {
        let f_ptr = Box::into_raw( f );
        let f_ptr_cvoid = f_ptr as *mut c_void;
        let mut ret_val : u8 = 0;
        let result = {
            let ret_val_ptr : *mut u8
                    = &mut ret_val as *mut _;
            // unsafe { ((*f_ptr).__vtable.bar)( f_ptr_cvoid, 0, ret_val_ptr ) }
            unsafe { __Foo_IFoo_bar( f_ptr_cvoid, 0, ret_val_ptr ) }
        };
        println!( "Result: {} ({})", ret_val, result );
    }
    */
}
