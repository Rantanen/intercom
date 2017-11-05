#![crate_type="dylib"]
#![feature(type_ascription, proc_macro)]

extern crate com_runtime;
use com_runtime::*;
extern crate winapi;

// Declare available COM classes.
#[com_library( TestLib, "{12341234-1234-1234-1234-123412340000}",
    PrimitiveOperations,
    StatefulOperations,
    ResultOperations,
    ClassCreator,
    CreatedClass,
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
    pub fn new() -> StatefulOperations { StatefulOperations { state : 0 } }
    pub fn put_value( &mut self, v : i32 ) { self.state = v; }
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
    // pub fn create_child( &self, id : i32, parent : ComRc<CreatedClass> ) -> ComResult<ComRc<CreatedClass>> {
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
