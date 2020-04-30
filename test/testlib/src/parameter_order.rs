use intercom::prelude::*;

#[com_class(Self)]
#[derive(Default)]
pub struct ParameterOrderTests;

#[com_interface]
impl ParameterOrderTests
{
    #[com_signature(value, OUT[0])]
    pub fn reciprocal(&mut self, value: u32) -> ComResult<f64>
    {
        Ok(1.0 / f64::from(value))
    }

    #[com_signature(OUT[0], value)]
    pub fn reciprocal_reversed(&mut self, value: u32) -> ComResult<f64>
    {
        Ok(1.0 / f64::from(value))
    }

    #[com_signature(v1, OUT[0], v2, OUT[1])]
    pub fn reciprocal_two(&mut self, v1: u32, v2: u32) -> ComResult<(f64, f64)>
    {
        Ok((1.0 / f64::from(v1), 1.0 / f64::from(v2)))
    }
}
