
use std::error::Error;
use std::convert::TryFrom;

use super::*;

/// Error structure containing the available information on a COM error.
#[derive(Debug)]
pub struct ComError {

    /// `HRESULT` that triggered the error.
    pub hresult : HRESULT,

    /// Possible detailed error info.
    pub error_info : Option<ErrorInfo>,
}

impl ComError {

    /// Constructs a new `ComError` from a `HRESULT` code.
    pub fn new_hr( hresult : HRESULT ) -> ComError
    {
        ComError { hresult, error_info: None }
    }

    /// Construts a new `ComError` with a given message.
    pub fn new_message(
        hresult: HRESULT,
        description: String
    ) -> ComError
    {
        ComError {
            hresult,
            error_info: Some( ErrorInfo::new( description ) )
        }
    }

    /// Gets the description if it's available.
    pub fn description( &self ) -> Option< &str >
    {
        self.error_info.as_ref().map( |e| e.description.as_str() )
    }
}

impl From<ComError> for std::io::Error {

    fn from( com_error : ComError ) -> std::io::Error {

        let error_kind = match com_error.hresult {

            ::STG_E_FILENOTFOUND => std::io::ErrorKind::NotFound,
            ::E_ACCESSDENIED => std::io::ErrorKind::PermissionDenied,
            ::RPC_E_CALL_REJECTED => std::io::ErrorKind::ConnectionRefused,
            ::RPC_E_DISCONNECTED => std::io::ErrorKind::ConnectionReset,
            ::RPC_E_CALL_CANCELED => std::io::ErrorKind::ConnectionAborted,
            ::RPC_E_TIMEOUT => std::io::ErrorKind::TimedOut,
            ::E_INVALIDARG => std::io::ErrorKind::InvalidInput,
            _ => std::io::ErrorKind::Other,
        };

        std::io::Error::new(
                error_kind,
                com_error.description().unwrap_or( "Unknown error" ) )
    }
}

impl From<std::io::Error> for ComError {

    fn from( io_error : std::io::Error ) -> ComError {

        let hresult = match io_error.kind() {

            std::io::ErrorKind::NotFound => ::STG_E_FILENOTFOUND,
            std::io::ErrorKind::PermissionDenied => ::E_ACCESSDENIED,
            std::io::ErrorKind::ConnectionRefused => ::RPC_E_CALL_REJECTED,
            std::io::ErrorKind::ConnectionReset => ::RPC_E_DISCONNECTED,
            std::io::ErrorKind::ConnectionAborted => ::RPC_E_CALL_CANCELED,
            std::io::ErrorKind::TimedOut => ::RPC_E_TIMEOUT,
            std::io::ErrorKind::InvalidInput => ::E_INVALIDARG,
            _ => ::E_FAIL,
        };

        ComError::new_message( hresult, io_error.description().to_owned() )
    }
}

impl From<::HRESULT> for ComError {
    fn from( hresult : ::HRESULT ) -> ComError {
        ComError::new_hr( hresult )
    }
}

impl From<ComError> for ::HRESULT {
    fn from( error : ComError ) -> ::HRESULT {
        error.hresult
    }
}

#[cfg(windows)]
#[allow(non_snake_case)]
mod error_store {

    use super::*;

    #[link(name = "oleaut32")]
    extern "system" {
        pub(super) fn SetErrorInfo(
            dw_reserved: u32,
            errorinfo: raw::InterfacePtr<IErrorInfo>,
        ) -> ::HRESULT;

        #[allow(private_in_public)]
        pub(super) fn GetErrorInfo(
            dw_reserved: u32,
            errorinfo: *mut raw::InterfacePtr<IErrorInfo>,
        ) -> ::HRESULT;
    }
}

#[cfg(not(windows))]
#[allow(non_snake_case)]
mod error_store {

    use super::*;

    pub(super) unsafe fn SetErrorInfo(
        _dw_reserved: u32,
        _errorinfo: raw::InterfacePtr<IErrorInfo>,
    ) -> ::HRESULT { ::S_OK }

    pub(super) unsafe fn GetErrorInfo(
        _dw_reserved: u32,
        _errorinfo: *mut raw::InterfacePtr<IErrorInfo>,
    ) -> ::HRESULT { ::S_OK }
}

/// Error info COM object data.
#[com_class( clsid = None, IErrorInfo )]
#[derive(Debug)]
pub struct ErrorInfo {
    guid : GUID,
    source : String,
    description : String,
    help_file: String,
    help_context: u32,
}

impl ErrorInfo {
    pub fn new( description : String ) -> ErrorInfo {
        ErrorInfo {
            description,
            guid: GUID::zero_guid(),
            source: String::new(),
            help_file: String::new(),
            help_context: 0,
        }
    }

    pub fn guid( &self ) -> &GUID { &self.guid }
    pub fn source( &self ) -> &str { &self.source }
    pub fn description( &self ) -> &str { &self.description }
    pub fn help_file( &self ) -> &str { &self.help_file }
    pub fn help_context( &self ) -> u32 { self.help_context }
}

impl<'a> TryFrom<&'a IErrorInfo> for ErrorInfo {

    type Error = ::HRESULT;

    fn try_from( source : &'a IErrorInfo ) -> Result<Self, Self::Error> {

        Ok( ErrorInfo {
            guid: source.get_guid()?,
            source: source.get_source()?.to_owned(),
            description: source.get_description()?.to_owned(),
            help_file: source.get_help_file()?.to_owned(),
            help_context: source.get_help_context()?,
        } )
    }
}

#[com_interface( com_iid = "1CF2B120-547D-101B-8E65-08002B2BD119" )]
trait IErrorInfo
{
    fn get_guid( &self ) -> ComResult< GUID >;
    fn get_source( &self ) -> ComResult< String >;
    fn get_description( &self ) -> ComResult< String >;
    fn get_help_file( &self ) -> ComResult< String >;
    fn get_help_context( &self ) -> ComResult< u32 >;
}

#[com_impl]
impl IErrorInfo for ErrorInfo
{
    fn get_guid( &self ) -> ComResult< GUID > { Ok( self.guid.clone() ) }
    fn get_source( &self ) -> ComResult< String > { Ok( self.source.clone() ) }
    fn get_description( &self ) -> ComResult< String > { Ok( self.description.clone() ) }
    fn get_help_file( &self ) -> ComResult< String > { Ok( self.help_file.clone() ) }
    fn get_help_context( &self ) -> ComResult< u32 > { Ok( self.help_context ) }
}

/// Extracts the HRESULT from the error result and stores the extended error
/// information in thread memory so it can be fetched by the COM client.
pub fn return_hresult< E >( error : E ) -> HRESULT
    where E : Into< ComError >
{
    // Convet the error.
    let com_error = error.into();

    match com_error.error_info {

        Some( error_info ) => {

            // ComError contains ErrorInfo. We need to set this in the OS error
            // store.

            // Construct the COM class used for IErrorInfo. The class contains the
            // description in memory.
            let mut info = ComStruct::< ErrorInfo >::new( error_info );

            // Get the IErrorInfo interface and set it in thread memory.
            let mut error_ptr : RawComPtr = std::ptr::null_mut();
            unsafe {

                // We are intentionally ignoring the HRESULT codes here. We don't
                // want to override the original error HRESULT with these codes.
                ComBox::query_interface(
                        info.as_mut(),
                        &IID_IErrorInfo,
                        &mut error_ptr );
                error_store::SetErrorInfo( 0, raw::InterfacePtr::new( error_ptr ) );
            }
        },
        None => {
            // No error info in the ComError.
            unsafe { error_store::SetErrorInfo( 0, raw::InterfacePtr::null() ); }
        }
    }

    // Return the HRESULT of the original error.
    com_error.hresult
}

/// Gets the last COM error that occurred on the current thread.
pub fn get_last_error< E >( last_hr : HRESULT ) -> E
    where E : From< ComError >
{
    let com_error = ComError {
        hresult: last_hr,
        error_info: unsafe {

            // Get the last error COM interface.
            let mut error_ptr : raw::InterfacePtr<IErrorInfo>
                    = raw::InterfacePtr::null();
            let hr = error_store::GetErrorInfo( 0, &mut error_ptr );

            if hr == S_OK && ! error_ptr.is_null(){

                let ierrorinfo = ComItf::< IErrorInfo >::wrap(
                        error_ptr,
                        TypeSystem::Automation );

                // Construct a proper ErrorInfo struct from the COM interface.
                let error_info = ErrorInfo::try_from( &*ierrorinfo ).ok();

                // Release the interface.
                let iunk : &ComItf<IUnknown> = ierrorinfo.as_ref();
                iunk.release();

                error_info

            } else {

                // GetErrorInfo didn't return proper error. Don't provide one
                // in the ComError.
                None
            }
        },
    };

    E::from( com_error )
}

/// Defines a way to handle errors based on the method return value type.
///
/// The default implementation will terminate the process on the basis that
/// errors must not silently leak. The specialization for `HRESULT` will return
/// the `HRESULT`.
///
/// The user is free to implement this trait for their own types to handle
/// custom status codes gracefully.
pub trait ErrorValue {

    /// Attempts to convert a COM error into a custom status code.
    fn from_error( HRESULT ) -> Self;
}

impl<T> ErrorValue for T {
    default fn from_error( _ : HRESULT ) -> Self {
        panic!( "Function does not support error values" )
    }
}

impl ErrorValue for HRESULT {
    fn from_error( hr : HRESULT ) -> Self { hr }
}
