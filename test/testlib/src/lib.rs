#![crate_type="dylib"]
#![feature(type_ascription, use_extern_macros, try_from, attr_literals)]


extern crate intercom;
use intercom::*;
use std::convert::TryFrom;
extern crate winapi;

// Declare available COM classes.
#[com_library( AUTO_GUID,
    RefCountOperations,
    PrimitiveOperations,
    StatefulOperations,
    ResultOperations,
    ClassCreator,
    CreatedClass,
    SharedImplementation,
    ErrorSource,
    AllocTests,
    StringTests,
)]

#[com_interface( AUTO_GUID )]
trait IRefCount
{
    fn get_ref_count( &self ) -> u32;
}

macro_rules! impl_irefcount {
    ( $t:ty ) => {
        #[com_impl]
        impl IRefCount for $t {
            fn get_ref_count( &self ) -> u32
            {
                let combox = unsafe { ComBox::of( self ) };
                combox.get_ref_count()
            }
        }
    }
}

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

#[com_class( AUTO_GUID, StatefulOperations )]
pub struct StatefulOperations {
    state : i32
}

#[com_interface( AUTO_GUID )]
#[com_impl]
impl StatefulOperations {
    pub fn new() -> StatefulOperations { StatefulOperations { state : 0xABBACD } }
    pub fn put_value( &mut self, v : i32 ) {
        self.state = v;
    }
    pub fn get_value( &mut self ) -> i32 { self.state }
}

#[com_class( AUTO_GUID, ResultOperations )]
pub struct ResultOperations { }

#[com_interface( AUTO_GUID )]
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

    pub fn tuple( &self, value : u32 ) -> ComResult<( u16, u16 )> {
        let first = u16::try_from( ( value & 0xffff_0000 ) >> 16 ).unwrap();
        let second = u16::try_from( value & 0xffff ).unwrap();

        Ok( ( first, second ) )
    }
}

#[com_class( AUTO_GUID, ClassCreator )]
pub struct ClassCreator { }

#[com_interface( AUTO_GUID )]
#[com_impl]
impl ClassCreator {
    pub fn new() -> ClassCreator { ClassCreator {} }

    pub fn create_root( &self, id : i32 ) -> ComResult<ComItf<CreatedClass>> {
        Ok( ComStruct::new( CreatedClass::new_with_id( id ) ).into() )
    }

    pub fn create_child(
        &self,
        id : i32,
        parent : ComItf<IParent>
    ) -> ComResult<ComItf<CreatedClass>>
    {
        Ok( ComStruct::new(
            CreatedClass::new_child( id, parent.get_id() )
        ).into() )
    }
}

#[com_class( AUTO_GUID, CreatedClass, IParent, IRefCount )]
pub struct CreatedClass { id : i32, parent: i32 }

#[com_interface( AUTO_GUID )]
#[com_impl]
impl CreatedClass {
    pub fn new() -> CreatedClass { unreachable!() }
    pub fn new_with_id( id : i32 ) -> CreatedClass { CreatedClass { id, parent: 0 } }
    pub fn new_child( id : i32, parent : i32 ) -> CreatedClass { CreatedClass { id, parent } }

    pub fn get_id( &self ) -> ComResult<i32> { Ok( self.id ) }
    pub fn get_parent_id( &self ) -> ComResult<i32> { Ok( self.parent ) }
}
impl_irefcount!( CreatedClass );

#[com_interface( AUTO_GUID )]
pub trait IParent {
    fn get_id( &self ) -> i32;
}

#[com_impl]
impl IParent for CreatedClass {
    fn get_id( &self ) -> i32 { self.id }
}

#[com_class( AUTO_GUID, RefCountOperations )]
pub struct RefCountOperations {}

#[com_interface( AUTO_GUID )]
#[com_impl]
impl RefCountOperations {
    pub fn new() -> RefCountOperations { RefCountOperations { } }

    pub fn get_new( &self ) -> ComResult<ComItf<RefCountOperations>> {
        Ok( ComStruct::new( RefCountOperations::new() ).into() )
    }

    pub fn get_ref_count( &self ) -> u32 {
        let combox = unsafe { ComBox::of( self ) };
        combox.get_ref_count()
    }
}

#[com_interface( AUTO_GUID )]
pub trait ISharedInterface {
    fn get_value( &self ) -> u32;
    fn set_value( &mut self, v : u32 );
    fn divide_by( &self, divisor: ComItf<ISharedInterface> ) -> ComResult<u32>;
}

#[com_class( AUTO_GUID, ISharedInterface)]
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

#[com_class( AUTO_GUID, ErrorSource)]
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

#[com_interface( AUTO_GUID )]
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
            source.description().unwrap_or( "" ).to_owned() )
    }
}

#[com_class( AUTO_GUID, AllocTests)]
pub struct AllocTests;

#[com_interface( AUTO_GUID )]
#[com_impl]
impl AllocTests
{
    pub fn new() -> AllocTests { AllocTests }

    pub fn get_bstr( &self, value: u32 ) -> String {
        format!( "{}", value )
    }

    pub fn get_bstr_result( &self, value: u32 ) -> ComResult<String> {
        Ok( format!( "{}", value ) )
    }
}

#[com_class( AUTO_GUID, StringTests)]
pub struct StringTests;

static STRING_DATA: &[ &str ] = &[
    "",
    "Test",
    "öäå",
    "\u{1F980}",
];

#[com_interface( AUTO_GUID )]
#[com_impl]
impl StringTests
{
    pub fn new() -> StringTests { StringTests }

    pub fn string_to_index( &self, s : &str ) -> ComResult<u32> {

        for candidate in 0..STRING_DATA.len() {
            if s == STRING_DATA[ candidate ] {
                return Ok( candidate as u32 )
            }
        }

        println!( "Unrecognized string: {}", s );
        Err( intercom::E_FAIL )
    }

    pub fn index_to_string( &self, i : u32 ) -> ComResult<String> {

        for candidate in 0..STRING_DATA.len() {
            if i as usize == candidate {
                return Ok( STRING_DATA[ candidate ].to_owned() )
            }
        }

        println!( "Unrecognized index: {}", i );
        Err( intercom::E_FAIL )
    }

    pub fn bstr_parameter( &self, s : &BStr, ptr : usize ) -> ComResult<()> {

        if s.as_ptr() as usize == ptr {
            Ok(())
        } else {
            Err( intercom::E_FAIL )
        }
    }

    pub fn bstr_return_value( &self ) -> ComResult<( BString, usize )> {

        let bs : BString = BString::from( "some string" );
        let ptr = bs.as_ptr() as usize;

        Ok( ( bs, ptr ) )
    }

    pub fn invalid_string( &self, s : &str ) -> ComResult<()> {

        // Don't do any validation here.
        // Intercom should do validation automatically.
        println!( "String parameter was not invalid: {}", s );

        // Caller expects E_INVALIDARG, use E_FAIL to indicate something
        // went wrong.
        Err( intercom::E_FAIL )
    }
}
