use intercom::*;

#[com_interface]
pub trait IErrorSource
{
    fn return_comerror(&self, hr: raw::HRESULT, desc: &str) -> ComResult<()>;

    fn return_testerror(&self, hr: raw::HRESULT, desc: &str) -> Result<(), TestError>;

    fn return_ioerror(&self, hr: raw::HRESULT, desc: &str) -> Result<(), std::io::Error>;
}

#[com_class(ErrorTests, IErrorSource)]
#[derive(Default)]
pub struct ErrorTests;

#[com_interface]
impl ErrorTests
{
    pub fn test_comerror(&self, source: &ComItf<dyn IErrorSource>) -> ComResult<()>
    {
        let err = source.return_comerror(raw::HRESULT::new(123), "Error message");

        match err {
            Ok(..) => Err(ComError::E_FAIL),
            Err(e) => {
                if e.hresult.hr != 123 {
                    return Err(ComError::E_INVALIDARG
                        .with_message(format!("Bad HRESULT: {}", e.hresult.hr)));
                }

                if e.description() != Some("Error message") {
                    return Err(ComError::E_INVALIDARG.with_message(format!("Bad message: {}", e)));
                }

                Ok(())
            }
        }
    }

    pub fn test_testerror(&self, source: &ComItf<dyn IErrorSource>) -> ComResult<()>
    {
        let err = source.return_testerror(raw::HRESULT::new(123), "Error message");

        match err {
            Ok(..) => Err(ComError::E_FAIL),
            Err(e) => {
                if e.0.hr != 123 {
                    return Err(
                        ComError::E_INVALIDARG.with_message(format!("Bad HRESULT: {}", e.0.hr))
                    );
                }

                if e.1 != "Error message" {
                    return Err(
                        ComError::E_INVALIDARG.with_message(format!("Bad message: {:?}", e.1))
                    );
                }

                Ok(())
            }
        }
    }

    pub fn test_ioerror(&self, source: &ComItf<dyn IErrorSource>) -> ComResult<()>
    {
        let err = source.return_ioerror(raw::HRESULT::new(raw::E_ACCESSDENIED.hr), "Access denied");

        match err {
            Ok(..) => Err(ComError::E_FAIL),
            Err(e) => {
                if e.kind() != std::io::ErrorKind::PermissionDenied {
                    return Err(
                        ComError::E_INVALIDARG.with_message(format!("Bad kind: {:?}", e.kind()))
                    );
                }

                if e.to_string() != "Access denied" {
                    return Err(ComError::E_INVALIDARG.with_message(format!("Bad message: {}", e)));
                }

                Ok(())
            }
        }
    }
}

impl IErrorSource for ErrorTests
{
    fn return_comerror(&self, hr: raw::HRESULT, desc: &str) -> ComResult<()>
    {
        Err(ComError::new_message(hr, desc.to_string()))
    }

    fn return_testerror(&self, hr: raw::HRESULT, desc: &str) -> Result<(), TestError>
    {
        Err(TestError(hr, desc.to_string()))
    }

    fn return_ioerror(&self, _hr: raw::HRESULT, _desc: &str) -> Result<(), std::io::Error>
    {
        Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "permission denied",
        ))
    }
}

#[derive(Debug)]
pub struct TestError(raw::HRESULT, String);

unsafe impl<TS: intercom::type_system::TypeSystem> intercom::type_system::ExternOutput<TS>
    for TestError
{
    type ForeignType = raw::HRESULT;

    fn into_foreign_output(self) -> ComResult<Self::ForeignType>
    {
        Ok(self.0)
    }

    unsafe fn from_foreign_output(source: Self::ForeignType) -> ComResult<Self>
    {
        Ok(TestError(source, "".to_string()))
    }
}

impl std::error::Error for TestError
{
    fn description(&self) -> &str
    {
        &self.1
    }
    fn cause(&self) -> Option<&dyn std::error::Error>
    {
        None
    }
}

impl std::fmt::Display for TestError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", self.1)
    }
}

impl From<TestError> for intercom::ComError
{
    fn from(source: TestError) -> intercom::ComError
    {
        intercom::ComError::new_message(source.0, source.1)
    }
}

impl From<intercom::ComError> for TestError
{
    fn from(source: intercom::ComError) -> TestError
    {
        TestError(
            source.hresult,
            source.description().unwrap_or("").to_owned(),
        )
    }
}
