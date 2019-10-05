
use intercom::*;

#[com_interface]
pub trait ISharedInterface {
    fn get_value( &self ) -> u32;
    fn set_value( &mut self, v : u32 );
    fn divide_by( &self, divisor: ComItf<dyn ISharedInterface> ) -> ComResult<u32>;
}

#[com_class( ISharedInterface)]
pub struct SharedImplementation { value: u32 }

impl SharedImplementation {
    pub fn new() -> SharedImplementation { SharedImplementation { value: 0 } }
}

#[com_impl]
impl ISharedInterface for SharedImplementation {
    fn get_value( &self ) -> u32 { self.value }
    fn set_value( &mut self, v : u32 ) { self.value = v }
    fn divide_by( &self, other: ComItf<dyn ISharedInterface> ) -> ComResult<u32> {
        let divisor = other.get_value();
        match divisor {
            0 => Err( ComError::E_INVALIDARG ),
            _ => Ok( self.value / divisor ),
        }
    }
}
