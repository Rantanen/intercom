use intercom::*;

#[com_class(clsid = "{12341234-1234-1234-1234-123412340001}", PrimitiveOperations)]
#[derive(Default)]
pub struct PrimitiveOperations {}

#[com_interface(
    com_iid = "{12341234-1234-1234-1234-123412340002}",
    raw_iid = "{12341234-1234-1234-1234-123412340003}"
)]
#[com_impl]
impl PrimitiveOperations
{
    pub fn i8(&self, v: i8) -> i8
    {
        !(v.wrapping_add(1))
    }
    pub fn u8(&self, v: u8) -> u8
    {
        !(v.wrapping_add(1))
    }

    pub fn u16(&self, v: u16) -> u16
    {
        !(v.wrapping_add(1))
    }
    pub fn i16(&self, v: i16) -> i16
    {
        !(v.wrapping_add(1))
    }

    pub fn i32(&self, v: i32) -> i32
    {
        !(v.wrapping_add(1))
    }
    pub fn u32(&self, v: u32) -> u32
    {
        !(v.wrapping_add(1))
    }

    pub fn i64(&self, v: i64) -> i64
    {
        !(v.wrapping_add(1))
    }
    pub fn u64(&self, v: u64) -> u64
    {
        !(v.wrapping_add(1))
    }

    pub fn f64(&self, v: f64) -> f64
    {
        1f64 / v
    }
    pub fn f32(&self, v: f32) -> f32
    {
        1f32 / v
    }
}
