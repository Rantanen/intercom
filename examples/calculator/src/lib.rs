#![crate_type="dylib"]
#![feature( plugin, custom_attribute )]
#![plugin( com_library )]

// Declare available COM classes.
#[com_library(
    Calculator
)]

extern crate com_runtime;
extern crate winapi;

#[com_class("{12341234-1234-1234-1234-123412340001}", Calculator, Memory)]
struct Calculator {
    value : i32,
    store : Vec<i32>,
}

#[com_interface("{12341234-1234-1234-1234-123412340002}")]
#[com_impl]
impl Calculator
{
    pub fn new() -> Calculator {
        Calculator { value : 0, store : vec![] }
    }

    pub fn add( &mut self, value : i32 ) -> com_runtime::ComResult<i32>
    {
        println!(
            "Calculator::Add {} + {} = {}",
            self.value, value, self.value + value );

        self.value += value;
        Ok( self.value )
    }

    pub fn substract( &mut self, value : i32 ) -> com_runtime::ComResult<i32>
    {
        println!(
            "Calculator::Substract {} - {} = {}",
            self.value, value, self.value - value );
        
        self.value -= value;
        Ok( self.value )
    }

    pub fn multiply( &mut self, value : i32 ) -> com_runtime::ComResult<i32>
    {
        println!(
            "Calculator::Multiply {} * {} = {}",
            self.value, value, self.value * value );

        self.value *= value;
        Ok( self.value )
    }
}

#[com_interface("{12341234-1234-1234-1234-123412340003}")]
trait Memory {
    fn store( &mut self ) -> com_runtime::ComResult<usize>;
    fn recall( &mut self, slot : usize ) -> com_runtime::ComResult<()>;
}

#[com_impl]
impl Memory for Calculator {
    fn store( &mut self ) -> com_runtime::ComResult<usize> {
        self.store.push( self.value );
        Ok( self.store.len() - 1 )
    }

    fn recall( &mut self, slot : usize ) -> com_runtime::ComResult<()> {
        self.value = *self.store.get( slot ).ok_or( winapi::winerror::E_INVALIDARG )?;
        Ok(())
    }
}
