pub struct MacroError
{
    pub msg: String,
}

impl<'a> From<&'a str> for MacroError
{
    fn from(m: &'a str) -> MacroError
    {
        MacroError { msg: m.to_owned() }
    }
}

impl From<String> for MacroError
{
    fn from(m: String) -> MacroError
    {
        MacroError { msg: m }
    }
}
