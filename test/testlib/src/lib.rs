#![crate_type="dylib"]
#![feature(type_ascription, proc_macro)]

extern crate intercom;
use intercom::*;
extern crate winapi;

// Declare available COM classes.
#[com_library( TestLib, "{12341234-1234-1234-1234-123412340000}",
    RefCountOperations,
    PrimitiveOperations,
    StatefulOperations,
    ResultOperations,
    ClassCreator,
    CreatedClass,
    SharedImplementation,
    ErrorSource,
)]

#[com_class("{12341234-1234-1234-1234-123412340001}", PrimitiveOperations )]
pub struct PrimitiveOperations { }

#[com_interface("{12341234-1234-1234-1234-123412340002}")]
#[com_impl]
impl PrimitiveOperations
{
    pub fn new() -> PrimitiveOperations {
        PrimitiveOperations { }
    }

    pub fn i8( &self, v : i8 ) -> i8 { !( v.wrapping_add( 1 ) ) }
    pub fn u8( &self, v : u8 ) -> u8 { !( v.wrapping_add( 1 ) ) }

    pub fn u16( &self, v : u16 ) -> u16 { !( v.wrapping_add( 1 ) ) }
    pub fn i16( &self, v : i16 ) -> i16 { !( v.wrapping_add( 1 ) ) }

    pub fn i32( &self, v : i32 ) -> i32 { !( v.wrapping_add( 1 ) ) }
    pub fn u32( &self, v : u32 ) -> u32 { !( v.wrapping_add( 1 ) ) }

    pub fn i64( &self, v : i64 ) -> i64 { !( v.wrapping_add( 1 ) ) }
    pub fn u64( &self, v : u64 ) -> u64 { !( v.wrapping_add( 1 ) ) }

    pub fn f64( &self, v : f64 ) -> f64 { 1f64 / v }
    pub fn f32( &self, v : f32 ) -> f32 { 1f32 / v }
}

#[com_class("{12341234-1234-1234-1234-123412340003}", StatefulOperations )]
pub struct StatefulOperations {
    state : i32
}

#[com_interface("{12341234-1234-1234-1234-123412340004}")]
#[com_impl]
impl StatefulOperations {
    pub fn new() -> StatefulOperations { StatefulOperations { state : 0xABBACD } }
    pub fn put_value( &mut self, v : i32 ) {
        self.state = v;
    }
    pub fn get_value( &mut self ) -> i32 { self.state }
}

#[com_class("{12341234-1234-1234-1234-123412340005}", ResultOperations )]
pub struct ResultOperations { }

#[com_interface("{12341234-1234-1234-1234-123412340006}")]
#[com_impl]
impl ResultOperations {
    pub fn new() -> ResultOperations { ResultOperations {} }

    pub fn s_ok( &mut self ) -> HRESULT {
        S_OK
    }

    pub fn not_impl( &mut self ) -> HRESULT {
        E_NOTIMPL
    }

    pub fn sqrt( &mut self, value : f64 ) -> ComResult<f64> {
        if value < 0.0 { return Err( E_INVALIDARG ) }
        Ok( value.sqrt() )
    }
}

#[com_class("{12341234-1234-1234-1234-123412340007}", ClassCreator )]
pub struct ClassCreator { }

#[com_class("{12341234-1234-1234-1234-123412340008}", CreatedClass )]
pub struct CreatedClass { id : i32, parent: i32 }

#[com_interface("{12341234-1234-1234-1234-123412340009}")]
#[com_impl]
impl ClassCreator {
    pub fn new() -> ClassCreator { ClassCreator {} }

    pub fn create_root( &self, id : i32 ) -> ComResult<ComRc<CreatedClass>> {
        Ok( ComRc::new( CreatedClass::new_with_id( id ) ) )
    }

    // ComRc input not supported yet.
    // First we should support random COM interfaces. After that we should
    // support specific Rust COM classes through special QueryInterface.
    //
    // pub fn create_child(
    //     &self,
    //     id : i32,
    //     parent : ComRc<CreatedClass>
    // ) -> ComResult<ComRc<CreatedClass>>
    // {
    //     Ok( ComRc::new( CreatedClass::new_child( id, &*parent ) ) )
    // }
}

#[com_interface("{12341234-1234-1234-1234-123412340010}")]
#[com_impl]
impl CreatedClass {
    pub fn new() -> CreatedClass { unreachable!() }
    pub fn new_with_id( id : i32 ) -> CreatedClass { CreatedClass { id, parent: 0 } }
    pub fn new_child( id : i32, parent : &CreatedClass ) -> CreatedClass { CreatedClass { id, parent: parent.id } }

    pub fn get_id( &self ) -> ComResult<i32> { Ok( self.id ) }
    pub fn get_parent_id( &self ) -> ComResult<i32> { Ok( self.parent ) }
}

#[com_class("{12341234-1234-1234-1234-123412340011}", RefCountOperations )]
pub struct RefCountOperations {}

#[com_interface("{12341234-1234-1234-1234-123412340012}")]
#[com_impl]
impl RefCountOperations {
    pub fn new() -> RefCountOperations { RefCountOperations { } }

    pub fn get_ref_count( &self ) -> u32 {
        let combox = unsafe { ComBox::of( self ) };
        combox.get_ref_count()
    }

    pub fn get_new( &self ) -> ComResult<ComRc<RefCountOperations>> {
        Ok( ComRc::new( RefCountOperations::new() ) )
    }
}

#[com_interface("{12341234-1234-1234-1234-123412340013}")]
pub trait ISharedInterface {
    fn get_value( &self ) -> u32;
    fn set_value( &mut self, v : u32 );
    fn divide_by( &self, divisor: ComItf<ISharedInterface> ) -> ComResult<u32>;
}

#[com_class("{12341234-1234-1234-1234-123412340014}", ISharedInterface)]
pub struct SharedImplementation { value: u32 }

impl SharedImplementation {
    pub fn new() -> SharedImplementation { SharedImplementation { value: 0 } }
}

#[com_impl]
impl ISharedInterface for SharedImplementation {
    fn get_value( &self ) -> u32 { self.value }
    fn set_value( &mut self, v : u32 ) { self.value = v }
    fn divide_by( &self, other: ComItf<ISharedInterface> ) -> ComResult<u32> {
        let divisor = other.get_value();
        match divisor {
            0 => Err( intercom::E_INVALIDARG ),
            _ => Ok( self.value / divisor ),
        }
    }
}

#[com_class("{12341234-1234-1234-1234-123412340015}", ErrorSource)]
pub struct ErrorSource;

#[derive(Debug)]
pub struct TestError( HRESULT, String );
impl std::error::Error for TestError {
    fn description( &self ) -> &str { &self.1 }
    fn cause( &self ) -> Option<&std::error::Error> { None }
}
impl std::fmt::Display for TestError {
    fn fmt( &self, f : &mut std::fmt::Formatter ) -> std::fmt::Result
    {
        write!( f, "{}", self.1 )
    }
}

#[com_interface("{12341234-1234-1234-1234-123412340016}")]
#[com_impl]
impl ErrorSource
{
    pub fn new() -> ErrorSource { ErrorSource }

    pub fn store_error( &self, hr : HRESULT, desc : String ) -> Result<(), TestError>
    {
        Err( TestError( hr, desc ) )
    }
}

impl From<TestError> for intercom::ComError
{
    fn from( source : TestError ) -> intercom::ComError {
        intercom::ComError::new_message( source.0, source.1 )
    }
}

impl From<intercom::ComError> for TestError
{
    fn from( source : intercom::ComError ) -> TestError {
        TestError(
            source.hresult,
            source.message().unwrap_or( "" ).to_owned() )
    }
}
