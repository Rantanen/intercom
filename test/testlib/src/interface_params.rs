use intercom::*;

#[com_interface]
pub trait ISharedInterface
{
    fn get_value(&self) -> u32;
    fn set_value(&mut self, v: u32);
    fn divide_by(&self, divisor: &ComItf<dyn ISharedInterface>) -> ComResult<u32>;
}

#[com_class(ISharedInterface)]
#[derive(Default)]
pub struct SharedImplementation
{
    value: u32,
}

impl ISharedInterface for SharedImplementation
{
    fn get_value(&self) -> u32
    {
        self.value
    }
    fn set_value(&mut self, v: u32)
    {
        self.value = v
    }
    fn divide_by(&self, other: &ComItf<dyn ISharedInterface>) -> ComResult<u32>
    {
        let divisor = other.get_value();
        match divisor {
            0 => Err(ComError::E_INVALIDARG),
            _ => Ok(self.value / divisor),
        }
    }
}
