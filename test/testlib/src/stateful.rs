use intercom::*;

#[com_class(StatefulOperations)]
pub struct StatefulOperations
{
    state: i32,
}

impl Default for StatefulOperations {
    fn default() -> Self {
        StatefulOperations { state: 0xABBACD }
    }
}

#[com_interface]
#[com_impl]
impl StatefulOperations
{
    pub fn put_value(&mut self, v: i32)
    {
        self.state = v;
    }
    pub fn get_value(&mut self) -> i32
    {
        self.state
    }
}
