use intercom::*;

#[com_struct]
pub struct BasicStruct
{
    a: u32,
    b: u32,
    c: u32,
}

#[com_struct]
pub struct StringStruct
{
    a: String,
    b: String,
}

#[com_struct]
pub struct Rectangle
{
    top_left: Point,
    bottom_right: Point,
}

#[com_struct]
pub struct Point
{
    x: f64,
    y: f64,
}

#[com_class(StructParameterTests)]
pub struct StructParameterTests;
impl StructParameterTests
{
    pub fn new() -> StructParameterTests
    {
        StructParameterTests
    }
}

#[com_impl]
#[com_interface]
impl StructParameterTests
{
    fn get_basic_struct(&self, a: u32, b: u32, c: u32) -> ComResult<BasicStruct>
    {
        Ok(BasicStruct { a, b, c })
    }

    fn get_string_struct(&self, a: &str, b: &str) -> ComResult<StringStruct>
    {
        Ok(StringStruct {
            a: a.to_string(),
            b: b.to_string(),
        })
    }

    fn get_complex_struct(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> ComResult<Rectangle>
    {
        Ok(Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        })
    }

    fn verify_basic_struct(&self, data: BasicStruct, a: u32, b: u32, c: u32) -> ComResult<()>
    {
        if data.a != a {
            return Err(intercom::ComError::E_INVALIDARG);
        }
        if data.b != b {
            return Err(intercom::ComError::E_INVALIDARG);
        }
        if data.c != c {
            return Err(intercom::ComError::E_INVALIDARG);
        }
        Ok(())
    }

    fn verify_string_struct(&self, data: StringStruct, a: &str, b: &str) -> ComResult<()>
    {
        if data.a != a {
            return Err(intercom::ComError::E_INVALIDARG);
        }
        if data.b != b {
            return Err(intercom::ComError::E_INVALIDARG);
        }
        Ok(())
    }

    fn verify_complex_struct(
        &self,
        data: Rectangle,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> ComResult<()>
    {
        if data.top_left.x != x1 {
            return Err(intercom::ComError::E_INVALIDARG);
        }
        if data.top_left.y != y1 {
            return Err(intercom::ComError::E_INVALIDARG);
        }
        if data.bottom_right.x != x2 {
            return Err(intercom::ComError::E_INVALIDARG);
        }
        if data.bottom_right.y != y2 {
            return Err(intercom::ComError::E_INVALIDARG);
        }

        Ok(())
    }
}
