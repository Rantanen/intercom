#![crate_type="dylib"]
#![feature( plugin, custom_attribute, unique, type_ascription )]
#![plugin( com_library )]

// Declare available COM classes.
#[com_library( "{1234-1234-1234-1234-1234-1232412340000}",
    Calculator
)]

extern crate com_runtime;
extern crate winapi;

#[com_class("{12341234-1234-1234-1234-123412340001}", Calculator, Memory, Cloneable )]
pub struct Calculator {
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
        println!( "Memory::Store" );
        self.store.push( self.value );
        Ok( self.store.len() - 1 )
    }

    fn recall( &mut self, slot : usize ) -> com_runtime::ComResult<()> {
        println!( "Memory::Recall" );
        self.value = *self.store.get( slot ).ok_or( winapi::winerror::E_INVALIDARG )?;
        Ok(())
    }
}

#[com_interface("{12341234-1234-1234-1234-123412340004}")]
trait Cloneable {
    fn clone_calculator( &self ) -> com_runtime::ComResult< com_runtime::ComRc< Calculator > >;
}

#[com_impl]
impl Cloneable for Calculator {
    fn clone_calculator( &self ) -> com_runtime::ComResult< com_runtime::ComRc< Calculator > >
    {
        println!( "Cloneable::Clone" );
        Ok( com_runtime::ComRc::new( Calculator {
            value: self.value,
            store: self.store.clone(),
        } ) )
    }
}
