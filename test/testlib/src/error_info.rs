
use intercom::*;

#[com_class( ErrorSource)]
pub struct ErrorSource;

#[derive(Debug)]
pub struct TestError( raw::HRESULT, String );
impl std::error::Error for TestError {
    fn description( &self ) -> &str { &self.1 }
    fn cause( &self ) -> Option<&std::error::Error> { None }
}
impl std::fmt::Display for TestError {
    fn fmt( &self, f : &mut std::fmt::Formatter ) -> std::fmt::Result
    {
        write!( f, "{}", self.1 )
    }
}

#[com_interface]
#[com_impl]
impl ErrorSource
{
    pub fn new() -> ErrorSource { ErrorSource }

    pub fn store_error(
        &self, 
        hr : raw::HRESULT,
        desc : String
    ) -> Result<(), TestError>
    {
        Err( TestError( hr, desc ) )
    }
}

impl From<TestError> for intercom::ComError
{
    fn from( source : TestError ) -> intercom::ComError {
        intercom::ComError::new_message( source.0, source.1 )
    }
}

impl From<intercom::ComError> for TestError
{
    fn from( source : intercom::ComError ) -> TestError {
        TestError(
            source.hresult,
            source.description().unwrap_or( "" ).to_owned() )
    }
}
