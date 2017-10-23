#![crate_type="dylib"]
#![feature( plugin, custom_attribute )]
#![plugin( com_library )]

// Declare available COM classes.
#[com_library(
    Calculator
)]

extern crate com_runtime;

struct Calculator {
    value : i32
}

#[com_visible("{12341234-1234-1234-1234-123412340002}")]
impl Calculator
{
    pub fn new() -> Calculator {
        Calculator { value : 0 }
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
