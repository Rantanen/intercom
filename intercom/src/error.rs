use std::convert::TryFrom;
use std::error::Error;

use super::*;
use crate::attributes::{ComInterface, ComInterfaceVariant};
use crate::type_system::{AutomationTypeSystem, ExternOutput, RawTypeSystem, TypeSystem};

/// Error structure containing the available information on a COM error.
#[derive(Debug)]
pub struct ComError
{
    /// `HRESULT` that triggered the error.
    pub hresult: raw::HRESULT,

    /// Possible detailed error info.
    pub error_info: Option<ErrorInfo>,
}

impl std::error::Error for ComError
{
    fn description(&self) -> &str
    {
        "ComError (Use Display for more information)"
    }
    fn cause(&self) -> Option<&dyn Error>
    {
        None
    }
    fn source(&self) -> Option<&(dyn Error + 'static)>
    {
        None
    }
}

impl std::fmt::Display for ComError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self.description() {
            Some(desc) => write!(f, "{}", desc),
            None => write!(f, "COM error ({:#x})", self.hresult.hr),
        }
    }
}

unsafe impl<TS: TypeSystem> ExternOutput<TS> for ComError
{
    type ForeignType = raw::HRESULT;

    fn into_foreign_output(self) -> ComResult<Self::ForeignType>
    {
        Ok(self.hresult)
    }

    unsafe fn from_foreign_output(source: Self::ForeignType) -> ComResult<Self>
    {
        Ok(ComError {
            hresult: source,
            error_info: None,
        })
    }
}

unsafe impl<TS: TypeSystem> ExternOutput<TS> for std::io::Error
{
    type ForeignType = raw::HRESULT;

    fn into_foreign_output(self) -> ComResult<Self::ForeignType>
    {
        let com_error: ComError = ComError::from(self);
        <ComError as ExternOutput<TS>>::into_foreign_output(com_error)
    }

    unsafe fn from_foreign_output(source: Self::ForeignType) -> ComResult<Self>
    {
        let com_error: ComError = <ComError as ExternOutput<TS>>::from_foreign_output(source)?;
        Ok(Self::from(com_error))
    }
}

impl ComError
{
    /// Constructs a new `ComError` from a `HRESULT` code.
    pub fn new_hr(hresult: raw::HRESULT) -> ComError
    {
        ComError {
            hresult,
            error_info: None,
        }
    }

    /// Construts a new `ComError` with a given message.
    pub fn new_message(hresult: raw::HRESULT, description: String) -> ComError
    {
        ComError {
            hresult,
            error_info: Some(ErrorInfo::new(description)),
        }
    }

    pub fn with_message<S: Into<String>>(mut self, msg: S) -> Self
    {
        self.error_info = Some(ErrorInfo::new(msg.into()));
        self
    }

    /// Gets the description if it's available.
    pub fn description(&self) -> Option<&str>
    {
        self.error_info.as_ref().map(|e| e.description.as_str())
    }

    pub const E_NOTIMPL: ComError = ComError {
        hresult: raw::E_NOTIMPL,
        error_info: None,
    };
    pub const E_NOINTERFACE: ComError = ComError {
        hresult: raw::E_NOINTERFACE,
        error_info: None,
    };
    pub const E_POINTER: ComError = ComError {
        hresult: raw::E_POINTER,
        error_info: None,
    };
    pub const E_ABORT: ComError = ComError {
        hresult: raw::E_ABORT,
        error_info: None,
    };
    pub const E_FAIL: ComError = ComError {
        hresult: raw::E_FAIL,
        error_info: None,
    };
    pub const E_INVALIDARG: ComError = ComError {
        hresult: raw::E_INVALIDARG,
        error_info: None,
    };
    pub const E_ACCESSDENIED: ComError = ComError {
        hresult: raw::E_ACCESSDENIED,
        error_info: None,
    };
    pub const STG_E_FILENOTFOUND: ComError = ComError {
        hresult: raw::STG_E_FILENOTFOUND,
        error_info: None,
    };
    pub const RPC_E_DISCONNECTED: ComError = ComError {
        hresult: raw::RPC_E_DISCONNECTED,
        error_info: None,
    };
    pub const RPC_E_CALL_REJECTED: ComError = ComError {
        hresult: raw::RPC_E_CALL_REJECTED,
        error_info: None,
    };
    pub const RPC_E_CALL_CANCELED: ComError = ComError {
        hresult: raw::RPC_E_CALL_CANCELED,
        error_info: None,
    };
    pub const RPC_E_TIMEOUT: ComError = ComError {
        hresult: raw::RPC_E_TIMEOUT,
        error_info: None,
    };
}

impl From<ComError> for std::io::Error
{
    fn from(com_error: ComError) -> std::io::Error
    {
        let error_kind = match com_error.hresult {
            raw::STG_E_FILENOTFOUND => std::io::ErrorKind::NotFound,
            raw::E_ACCESSDENIED => std::io::ErrorKind::PermissionDenied,
            raw::RPC_E_CALL_REJECTED => std::io::ErrorKind::ConnectionRefused,
            raw::RPC_E_DISCONNECTED => std::io::ErrorKind::ConnectionReset,
            raw::RPC_E_CALL_CANCELED => std::io::ErrorKind::ConnectionAborted,
            raw::RPC_E_TIMEOUT => std::io::ErrorKind::TimedOut,
            raw::E_INVALIDARG => std::io::ErrorKind::InvalidInput,
            _ => std::io::ErrorKind::Other,
        };

        std::io::Error::new(error_kind, com_error.to_string())
    }
}

impl From<std::io::Error> for ComError
{
    fn from(io_error: std::io::Error) -> ComError
    {
        match io_error.kind() {
            std::io::ErrorKind::NotFound => ComError::STG_E_FILENOTFOUND,
            std::io::ErrorKind::PermissionDenied => ComError::E_ACCESSDENIED,
            std::io::ErrorKind::ConnectionRefused => ComError::RPC_E_CALL_REJECTED,
            std::io::ErrorKind::ConnectionReset => ComError::RPC_E_DISCONNECTED,
            std::io::ErrorKind::ConnectionAborted => ComError::RPC_E_CALL_CANCELED,
            std::io::ErrorKind::TimedOut => ComError::RPC_E_TIMEOUT,
            std::io::ErrorKind::InvalidInput => ComError::E_INVALIDARG,
            _ => ComError::E_FAIL,
        }
        .with_message(io_error.to_string())
    }
}

impl From<raw::HRESULT> for ComResult<()>
{
    fn from(hresult: raw::HRESULT) -> ComResult<()>
    {
        match hresult {
            // TODO: We should have a proper 'succeeded' method on HRESULT.
            raw::S_OK | raw::S_FALSE => Ok(()),
            e => Err(e.into()),
        }
    }
}

impl From<raw::HRESULT> for ComError
{
    fn from(hresult: raw::HRESULT) -> ComError
    {
        ComError::new_hr(hresult)
    }
}

impl From<ComError> for raw::HRESULT
{
    fn from(error: ComError) -> raw::HRESULT
    {
        error.hresult
    }
}

impl<'a> From<&'a str> for crate::ComError
{
    fn from(s: &'a str) -> Self
    {
        s.to_string().into()
    }
}

impl From<String> for crate::ComError
{
    fn from(s: String) -> Self
    {
        Self::new_message(raw::E_FAIL, s)
    }
}

#[cfg(windows)]
#[allow(non_snake_case)]
mod error_store
{

    use super::*;

    #[link(name = "oleaut32")]
    extern "system" {
        pub(super) fn SetErrorInfo(
            dw_reserved: u32,
            errorinfo: Option<crate::raw::InterfacePtr<AutomationTypeSystem, dyn IErrorInfo>>,
        ) -> raw::HRESULT;

        #[allow(private_in_public)]
        pub(super) fn GetErrorInfo(
            dw_reserved: u32,
            errorinfo: *mut Option<crate::raw::InterfacePtr<AutomationTypeSystem, dyn IErrorInfo>>,
        ) -> raw::HRESULT;
    }
}

#[cfg(not(windows))]
#[allow(non_snake_case)]
mod error_store
{

    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static ERROR_STORE: RefCell< Option< ComRc<dyn IErrorInfo> > > = RefCell::new( None );
    }

    fn reset_error_store(value: Option<ComRc<dyn IErrorInfo>>)
    {
        ERROR_STORE.with(|store| {
            store.replace(value);
        });
    }

    fn take_error() -> Option<ComRc<dyn IErrorInfo>>
    {
        ERROR_STORE.with(|store| store.replace(None))
    }

    pub(super) unsafe fn SetErrorInfo(
        _dw_reserved: u32,
        errorinfo: Option<crate::raw::InterfacePtr<AutomationTypeSystem, dyn IErrorInfo>>,
    ) -> raw::HRESULT
    {
        reset_error_store(errorinfo.map(ComRc::from));
        raw::S_OK
    }

    pub(super) unsafe fn GetErrorInfo(
        _dw_reserved: u32,
        errorinfo: *mut Option<crate::raw::InterfacePtr<AutomationTypeSystem, dyn IErrorInfo>>,
    ) -> raw::HRESULT
    {
        match take_error() {
            Some(rc) => {
                *errorinfo = ComItf::ptr(&ComRc::detach(rc));
                raw::S_OK
            }
            None => {
                *errorinfo = None;
                raw::S_FALSE
            }
        }
    }
}

/// Error info COM object data.
#[com_class( clsid = None, IErrorInfo )]
#[derive(Debug, Clone)]
pub struct ErrorInfo
{
    guid: GUID,
    source: String,
    description: String,
    help_file: String,
    help_context: u32,
}

impl ErrorInfo
{
    pub fn new(description: String) -> ErrorInfo
    {
        ErrorInfo {
            description,
            guid: GUID::zero_guid(),
            source: String::new(),
            help_file: String::new(),
            help_context: 0,
        }
    }

    pub fn guid(&self) -> &GUID
    {
        &self.guid
    }
    pub fn source(&self) -> &str
    {
        &self.source
    }
    pub fn description(&self) -> &str
    {
        &self.description
    }
    pub fn help_file(&self) -> &str
    {
        &self.help_file
    }
    pub fn help_context(&self) -> u32
    {
        self.help_context
    }
}

impl<'a> TryFrom<&'a dyn IErrorInfo> for ErrorInfo
{
    type Error = raw::HRESULT;

    fn try_from(source: &'a dyn IErrorInfo) -> Result<Self, Self::Error>
    {
        Ok(ErrorInfo {
            guid: source.get_guid()?,
            source: source.get_source()?,
            description: source.get_description()?,
            help_file: source.get_help_file()?,
            help_context: source.get_help_context()?,
        })
    }
}

#[com_interface(com_iid = "1CF2B120-547D-101B-8E65-08002B2BD119")]
pub trait IErrorInfo: crate::IUnknown
{
    fn get_guid(&self) -> ComResult<GUID>;
    fn get_source(&self) -> ComResult<String>;
    fn get_description(&self) -> ComResult<String>;
    fn get_help_file(&self) -> ComResult<String>;
    fn get_help_context(&self) -> ComResult<u32>;
}

impl IErrorInfo for ErrorInfo
{
    fn get_guid(&self) -> ComResult<GUID>
    {
        Ok(self.guid.clone())
    }
    fn get_source(&self) -> ComResult<String>
    {
        Ok(self.source.clone())
    }
    fn get_description(&self) -> ComResult<String>
    {
        Ok(self.description.clone())
    }
    fn get_help_file(&self) -> ComResult<String>
    {
        Ok(self.help_file.clone())
    }
    fn get_help_context(&self) -> ComResult<u32>
    {
        Ok(self.help_context)
    }
}

/// Extracts the HRESULT from the error result and stores the extended error
/// information in thread memory so it can be fetched by the COM client.
pub fn store_error<E>(error: E) -> ComError
where
    E: Into<ComError>,
{
    // Convet the error.
    let com_error = error.into();

    match com_error.error_info {
        Some(ref error_info) => {
            // ComError contains ErrorInfo. We need to set this in the OS error
            // store.

            // Construct the COM class used for IErrorInfo. The class contains the
            // description in memory.
            let info = ComBox::<ErrorInfo>::new(error_info.clone());

            // Store the IErrorInfo pointer in the SetErrorInfo.
            let rc = ComRc::<dyn IErrorInfo>::from(info);
            let ptr = ComItf::ptr(&rc).expect("Intercom did not implement correct type system");
            unsafe {
                error_store::SetErrorInfo(0, Some(ptr));
            }
        }
        None => {
            // No error info in the ComError.
            unsafe {
                error_store::SetErrorInfo(0, None);
            }
        }
    }

    // Return the HRESULT of the original error.
    com_error
}

pub fn load_error<I: ComInterface + ?Sized>(
    iunk: &ComItf<I>,
    iid: &GUID,
    err: raw::HRESULT,
) -> ComError
{
    // Do not try to load error if this is IUnknown or ISupportErrorInfo.
    // Both of these are used during error handling and may fail.
    if iid == <dyn IUnknown as ComInterfaceVariant<AutomationTypeSystem>>::iid()
        || iid == <dyn IUnknown as ComInterfaceVariant<RawTypeSystem>>::iid()
        || iid == <dyn ISupportErrorInfo as ComInterfaceVariant<AutomationTypeSystem>>::iid()
        || iid == <dyn ISupportErrorInfo as ComInterfaceVariant<RawTypeSystem>>::iid()
    {
        return ComError {
            hresult: err,
            error_info: None,
        };
    }

    // Try to get the ISupportErrorInfo and query that for the IID.
    let supports_errorinfo = match ComItf::query_interface::<dyn ISupportErrorInfo>(iunk) {
        Ok(rc) => match rc.interface_supports_error_info(iid) {
            intercom::raw::S_OK => true,
            _ => false,
        },
        _ => false,
    };

    ComError {
        hresult: err,
        error_info: match supports_errorinfo {
            true => get_last_error(),
            false => None,
        },
    }
}

/// Gets the last COM error that occurred on the current thread.
pub fn get_last_error() -> Option<ErrorInfo>
{
    let ierrorinfo = match get_error_info() {
        Some(ierror) => ierror,
        None => return None,
    };

    // FIXME ComRc Deref
    let ierr: &dyn IErrorInfo = &*ierrorinfo;
    ErrorInfo::try_from(ierr).ok()
}

/// Defines a way to handle errors based on the method return value type.
///
/// The default implementation will terminate the process on the basis that
/// errors must not silently leak. The specialization for `HRESULT` will return
/// the `HRESULT`.
///
/// The user is free to implement this trait for their own types to handle
/// custom status codes gracefully.
pub trait ErrorValue
{
    /// Attempts to convert a COM error into a custom status code. Must not panic.
    fn from_error(_: ComError) -> Self;
}

impl<S, E: ErrorValue> ErrorValue for Result<S, E>
{
    fn from_error(e: ComError) -> Self
    {
        Err(E::from_error(e))
    }
}

impl<T: From<ComError>> ErrorValue for T
{
    fn from_error(err: ComError) -> Self
    {
        err.into()
    }
}

#[com_class(IErrorStore)]
#[derive(Default)]
pub struct ErrorStore;

#[com_interface(
    com_iid = "d7f996c5-0b51-4053-82f8-19a7261793a9",
    raw_iid = "7586c49a-abbd-4a06-b588-e3d02b431f01"
)]
pub trait IErrorStore: crate::IUnknown
{
    fn get_error_info(&self) -> ComResult<ComRc<dyn IErrorInfo>>;
    fn set_error_info(&self, info: &ComItf<dyn IErrorInfo>) -> ComResult<()>;
    fn set_error_message(&self, msg: &str) -> ComResult<()>;
}

impl IErrorStore for ErrorStore
{
    fn get_error_info(&self) -> ComResult<ComRc<dyn IErrorInfo>>
    {
        Ok(get_error_info().unwrap()) // FIXME Option
    }

    fn set_error_info(&self, info: &ComItf<dyn IErrorInfo>) -> ComResult<()>
    {
        set_error_info(&info)
    }

    fn set_error_message(&self, msg: &str) -> ComResult<()>
    {
        let info = ComBox::<ErrorInfo>::new(ErrorInfo::new(msg.to_string()));
        let itf = ComRc::<dyn IErrorInfo>::from(&info);
        self.set_error_info(&*itf)
    }
}

fn get_error_info() -> Option<ComRc<dyn IErrorInfo>>
{
    // We're unsafe due to pointer lifetimes.
    //
    // The GetErrorInfo gives us a raw pointer which we own so we'll need to
    // wrap that in a `ComRc` to manage its memory.
    unsafe {
        // Get the error info and wrap it in an RC.
        let mut error_ptr = None;
        match error_store::GetErrorInfo(0, &mut error_ptr) {
            raw::S_OK => {}
            _ => return None,
        }
        match error_ptr {
            Some(ptr) => Some(ComRc::wrap(ptr)),
            None => None,
        }
    }
}

fn set_error_info(info: &ComItf<dyn IErrorInfo>) -> ComResult<()>
{
    unsafe { error_store::SetErrorInfo(0, ComItf::ptr(info)).into() }
}

pub mod raw
{

    /// COM method status code.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
    #[repr(C)]
    pub struct HRESULT
    {
        /// The numerical HRESULT code.
        pub hr: i32,
    }

    impl HRESULT
    {
        /// Constructs a new `HRESULT` with the given numerical code.
        pub fn new(hr: i32) -> HRESULT
        {
            #[allow(overflowing_literals)]
            HRESULT { hr: hr as i32 }
        }

        /// Returns true if the HRESULT represents a success-value.
        pub fn is_success(&self) -> bool
        {
            self.hr >= 0
        }

        /// Returns true if the HRESULT represents an error-value.
        pub fn is_error(&self) -> bool
        {
            !self.is_success()
        }
    }

    macro_rules! make_hr {
        ( $(#[$attr:meta] )* $hr_name: ident = $hr_value: expr ) => {
            $(#[$attr])*
            #[allow(overflowing_literals)]
            pub const $hr_name : HRESULT = HRESULT { hr: $hr_value as i32 };
        }
    }

    make_hr!(
        /// `HRESULT` indicating the operation completed successfully.
        S_OK = 0
    );

    make_hr!(
        /// `HRESULT` indicating the operation completed successfully and returned
        /// `false`.
        S_FALSE = 1
    );

    make_hr!(
        /// `HRESULT` for unimplemented functionality.
        E_NOTIMPL = 0x8000_4001
    );

    make_hr!(
        /// `HRESULT` indicating the type does not support the requested interface.
        E_NOINTERFACE = 0x8000_4002
    );

    make_hr!(
        /// `HRESULT` indicating a pointer parameter was invalid.
        E_POINTER = 0x8000_4003
    );

    make_hr!(
        /// `HRESULT` for aborted operation.
        E_ABORT = 0x8000_4004
    );

    make_hr!(
        /// `HRESULT` for unspecified failure.
        E_FAIL = 0x8000_4005
    );

    make_hr!(
        /// `HRESULT` for invalid argument.
        E_INVALIDARG = 0x8007_0057
    );

    make_hr!(
        /// `HRESULT` for unavailable CLSID.
        E_CLASSNOTAVAILABLE = 0x8004_0111
    );

    // These might be deprecated. They are a bit too specific for cross-platform
    // support. We'll just need to ensure the winapi HRESULTs are compatible.
    make_hr!(E_ACCESSDENIED = 0x8007_0005);
    make_hr!(STG_E_FILENOTFOUND = 0x8003_0002);
    make_hr!(RPC_E_DISCONNECTED = 0x8001_0108);
    make_hr!(RPC_E_CALL_REJECTED = 0x8001_0001);
    make_hr!(RPC_E_CALL_CANCELED = 0x8001_0002);
    make_hr!(RPC_E_TIMEOUT = 0x8001_011F);
}
