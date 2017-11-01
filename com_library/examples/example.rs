#![feature(proc_macro)]
#![feature(attr_literals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate com_library;
use com_library::com_library;
#[com_library( Bar )]

extern crate com_runtime;
use com_library::com_interface;
use com_library::com_impl;
use com_library::com_class;

use std::os::raw::c_void;

#[com_class( "{12341234-1234-1234-1234-123412340003}", Bar, StringFunctions )]
struct Bar {
    value : i16
}

impl Bar
{
    fn new() -> Bar { eprintln!( "Created Bar" ); Bar { value : 0 } }
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
    fn accumulate( &mut self, a : i16 ) -> com_runtime::ComResult<i16> {
        self.value += a;
        Ok( self.value )
    }
    fn clip( &self, min : i16, max : i16 ) -> i16 {

        // Intentionally 'i16' instead of 'ComResult<i16>'.
        match self.value {
            n if n < min => min,
            n if n > max => max,
            n => n
        }
    }

    // Ensure these don't cause problems. They can't be called over COM.
    fn static_method( c : u32 ) -> u32 { 10 }
    fn empty_method() -> u32 { 10 }
}

#[com_interface("{12341234-1234-1234-1234-123412340005}")]
trait StringFunctions
{
    fn join( &self, a : String, b : String ) -> com_runtime::ComResult<String>;
}

#[com_impl]
impl StringFunctions for Bar {

    fn join( &self, mut a : String, b : String ) -> com_runtime::ComResult<String>
    {
        a.push_str( " + " );
        a.push_str( &b );
        Ok( a )
    }
}

fn create( clsid : com_runtime::GUID ) -> com_runtime::ComResult< com_runtime::RawComPtr >
{
    Err( com_runtime::S_OK )
}

struct CF {
    clsid : com_runtime::REFCLSID
}

macro_rules! com_call {
    ( ( $itf:path ) $vtbl:ident . $method:ident ( $( $param:expr ),* ) ) =>
        ( ( (**( $vtbl as *const *const $itf )). $method )( std::mem::transmute( $vtbl ), $( $param, )* ) )
}

fn main()
{
    let dummy_classfactory = com_runtime::ClassFactory::new( &CLSID_Bar, |clsid| Err( com_runtime::S_OK ) );
    println!("Class factory size: {}, expected: {}",
        std::mem::size_of_val(&dummy_classfactory),
        std::mem::size_of::<usize>());

    // Horrible Rust code ahead. This mimics the C++ calls.
    unsafe {

        // Acquire the class factory.
        let mut classFactory = std::mem::uninitialized();
        eprintln!( "DllGetClassObject: {}",
                  DllGetClassObject( &mut CLSID_Bar, &mut com_runtime::IID_IClassFactory, &mut classFactory ) );

        // Got the class factory pointer.
        //
        // The vtable is a the start of the struct and we asked for the
        // IClassFactory interface, we can consider the pointer to the class
        // factory as a pointer to a IClassFactory vtable pointer.
        let classFactory_vtbl_ptr_ptr = classFactory as *const *const com_runtime::ClassFactoryVtbl;

        // Invoke the create instance method.
        //
        // This is an example of a raw COM call without the com_call! macro.
        let mut bar_iunk = std::mem::uninitialized();
        eprintln!( "Creating Bar::IUnknown" );
        eprintln!( "- HRESULT: {}",
                ((**classFactory_vtbl_ptr_ptr).create_instance)(
                    classFactory,  // &this
                    std::mem::transmute( std::ptr::null::<c_void>() ),
                    &com_runtime::IID_IUnknown,
                    &mut bar_iunk ) );
        eprintln!( "- Received: {:p}, with vtable {:p} (Expected {:p})",
                   bar_iunk,
                   *( bar_iunk as *const com_runtime::RawComPtr ),
                   &__Bar_IUnknownVtbl_INSTANCE );

        // Invoke the create instance method.
        //
        // This is an example of a raw COM call without the com_call! macro.
        let mut bar_bar = std::mem::uninitialized();
        eprintln!( "Creating Bar::Bar" );
        eprintln!( "- HRESULT: {}",
                ((**classFactory_vtbl_ptr_ptr).create_instance)(
                    classFactory,  // &this
                    std::mem::transmute( std::ptr::null::<c_void>() ),
                    &IID_Bar,
                    &mut bar_bar ) );
        eprintln!( "- Received: {:p}, with vtable {:p} (Expected {:p})",
                   bar_bar,
                   *( bar_bar as *const com_runtime::RawComPtr ),
                   &__Bar_BarVtbl_INSTANCE );

        let mut bar_bar_iunk = std::mem::uninitialized();
        eprintln!( "Querying IUnknown on Bar::Bar" );
        eprintln!( "- HRESULT: {}", com_call!(
                (com_runtime::IUnknownVtbl)
                    bar_bar.query_interface( &com_runtime::IID_IUnknown, &mut bar_bar_iunk )
                ) );

        eprintln!( "Bar::IUnknown {:p}, Bar::Bar::IUnknown: {:p}, expected: {:p}",
                *( bar_iunk as *const *const com_runtime::IUnknownVtbl ),
                *( bar_bar_iunk as *const *const com_runtime::IUnknownVtbl ),
                &( __Bar_IUnknownVtbl_INSTANCE ) );

        let mut result = 0i16;
        eprintln!( "Calling Bar::accumulate" );
        eprintln!( "- HRESULT: {}", com_call!(
                (__BarVtbl) bar_bar.accumulate( 10, &mut result ) ) );
        eprintln!( "- Result: {}", result );
        eprintln!( "Calling Bar::accumulate" );
        eprintln!( "- HRESULT: {}", com_call!(
                (__BarVtbl) bar_bar.accumulate( 15, &mut result ) ) );
        eprintln!( "- Result: {}", result );

        eprintln!( "Calling Bar::clip" );
        eprintln!( "- Result( 10, 11 ): {}", com_call!(
                (__BarVtbl) bar_bar.clip( 10, 11 ) ) );
        eprintln!( "- Result( 10, 50 ): {}", com_call!(
                (__BarVtbl) bar_bar.clip( 10, 50 ) ) );
        eprintln!( "- Result( 50, 80 ): {}", com_call!(
                (__BarVtbl) bar_bar.clip( 50, 80 ) ) );

        let mut bar_strfuns = std::mem::uninitialized();
        eprintln!( "Querying StringFunctions interface" );
        eprintln!( "- HRESULT: {}", com_call!(
                (com_runtime::IUnknownVtbl)
                    bar_bar.query_interface( &IID_StringFunctions, &mut bar_strfuns ) ) );

        let bstr_param1 = com_runtime::BStr::string_to_bstr( &String::from( "First" ) );
        let bstr_param2 = com_runtime::BStr::string_to_bstr( &String::from( "Second" ) );
        let mut bstr_return = std::mem::uninitialized();
        eprintln!( "Calling Bar::join" );
        eprintln!( "- HRESULT: {}", com_call!(
                (__StringFunctionsVtbl) bar_strfuns.join(
                    bstr_param1, bstr_param2, &mut bstr_return ) ) );
        eprintln!( "- Result: {}", com_runtime::BStr::bstr_to_string( &bstr_return ) );
    }
}
