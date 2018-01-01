
pub struct MacroError {
    pub msg : String,
}

impl<'a> From<&'a str> for MacroError {
    fn from(m:&'a str) -> MacroError { MacroError { msg : m.to_owned() } }
}

impl From<String> for MacroError {
    fn from(m:String) -> MacroError { MacroError { msg : m } }
}

impl From<::model::ParseError> for MacroError {
    fn from(e : ::model::ParseError) -> MacroError { MacroError { msg : e.0 } }
}
