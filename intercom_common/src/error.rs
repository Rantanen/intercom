
use proc_macro::{TokenStream, LexError};

pub struct MacroError {
    pub msg : String,
}

impl<'a> From<&'a str> for MacroError {
    fn from(m:&'a str) -> MacroError { MacroError { msg : m.to_owned() } }
}

impl From<String> for MacroError {
    fn from(m:String) -> MacroError { MacroError { msg : m } }
}

impl From<LexError> for MacroError {
    fn from(e:LexError) -> MacroError {
        MacroError { msg : "Error parsing token stream".to_owned() }
    }
}

