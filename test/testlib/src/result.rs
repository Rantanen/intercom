use intercom::*;
use std::convert::TryFrom;

#[com_class(ResultOperations)]
#[derive(Default)]
pub struct ResultOperations {}

#[com_interface]
#[com_impl]
impl ResultOperations
{
    pub fn s_ok(&mut self) -> raw::HRESULT
    {
        raw::S_OK
    }

    pub fn not_impl(&mut self) -> raw::HRESULT
    {
        raw::E_NOTIMPL
    }

    pub fn sqrt(&mut self, value: f64) -> ComResult<f64>
    {
        if value < 0.0 {
            return Err(ComError::E_INVALIDARG);
        }
        Ok(value.sqrt())
    }

    pub fn tuple(&self, value: u32) -> ComResult<(u16, u16)>
    {
        let first = u16::try_from((value & 0xffff_0000) >> 16).unwrap();
        let second = u16::try_from(value & 0xffff).unwrap();

        Ok((first, second))
    }
}
