
use std::convert::TryFrom;
use super::*;
mod intercom {
    pub use ::*;
}

/// Error structure containing the available information on a COM error.
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

    /// Gets the message if it's available.
    pub fn message( &self ) -> Option< &str >
    {
        self.error_info.as_ref().map( |e| e.description.as_str() )
    }
}

#[cfg(windows)]
#[allow(non_snake_case)]
mod error_store {

    #[link(name = "oleaut32")]
    extern "system" {
        pub fn SetErrorInfo(
            dw_reserved: u32,
            errorinfo: ::RawComPtr,
        ) -> ::HRESULT;

        pub fn GetErrorInfo(
            dw_reserved: u32,
            errorinfo: &mut ::RawComPtr,
        ) -> ::HRESULT;
    }
}

#[cfg(not(windows))]
#[allow(non_snake_case)]
mod error_store {

    pub fn SetErrorInfo(
        _dw_reserved: u32,
        _errorinfo: ::RawComPtr,
    ) -> ::HRESULT { S_OK }

    pub fn GetErrorInfo(
        dw_reserved: u32,
        errorinfo: &mut ::RawComPtr,
    ) -> ::HRESULT { S_OK }
}

/// Error info COM object data.
#[com_class( NO_CLSID, IErrorInfo )]
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

#[com_interface( "1CF2B120-547D-101B-8E65-08002B2BD119" )]
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
            let mut info = ComBox::< ErrorInfo >::new( error_info );

            // Get the IErrorInfo interface and set it in thread memory.
            let mut error_ptr : RawComPtr = std::ptr::null_mut();
            unsafe {

                // We are intentionally ignoring the HRESULT codes here. We don't
                // want to override the original error HRESULT with these codes.
                ComBox::query_interface(
                        info.as_mut(),
                        &IID_IErrorInfo,
                        &mut error_ptr );
                error_store::SetErrorInfo( 0, error_ptr );

                // SetErrorInfo took ownership of the error.
                // Forget it from the Box.
                Box::into_raw( info );
            }
        },
        None => {
            // No error info in the ComError.
            unsafe { error_store::SetErrorInfo( 0, std::ptr::null_mut() ); }
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
            let mut error_ptr : RawComPtr = std::ptr::null_mut();
            let hr = error_store::GetErrorInfo( 0, &mut error_ptr );

            if hr == S_OK {

                let ierrorinfo = ComItf::< IErrorInfo >::wrap( error_ptr );;

                // Construct a proper ErrorInfo struct from the COM interface.
                let error_info = ErrorInfo::try_from(
                        &ierrorinfo as &IErrorInfo ).ok();

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
