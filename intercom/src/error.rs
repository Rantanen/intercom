
use super::*;

/// Error structure used to represent the COM `IErrorInfo`.
pub struct ComError {
    pub description : String,
    pub hresult : HRESULT,
}

#[cfg(windows)]
#[link(name = "oleaut32")]
extern "system" {
    pub fn SetErrorInfo(
        dw_reserved: u32,
        errorinfo: RawComPtr,
    ) -> HRESULT;
}

#[cfg(not(windows))]
#[allow(non_snake_case)]
pub fn SetErrorInfo(
    _dw_reserved: u32,
    _errorinfo: RawComPtr,
) -> HRESULT { S_OK }

/// Error info COM object data.
struct ErrorInfo { pub description : String }

/// `CoClass` implementation for the `ErrorInfo` COM object.
impl CoClass for ErrorInfo {

    /// Virtual table type.
    type VTableList = &'static IErrorInfoVtbl;

    /// Virtual table constructor.
    fn create_vtable_list() -> Self::VTableList {
        &IErrorInfoVtbl {
            __base : IUnknownVtbl {
                query_interface : ComBox::< Self >::query_interface_ptr,
                add_ref : ComBox::< Self >::add_ref_ptr,
                release : ComBox::< Self >::release_ptr,
            },
            get_guid : get_guid_impl,
            get_source : get_source_impl,
            get_description : get_description_impl,
            get_help_file : get_help_file_impl,
            get_help_context : get_help_context_impl,
        }
    }

    /// Query interface implementation. Supports only `IUnknown` and `IErrorInfo`.
    fn query_interface(
        vtables : &Self::VTableList,
        riid : REFIID,
    ) -> ComResult< RawComPtr >
    {
        if riid.is_null() { return Err( E_NOINTERFACE ) }
        unsafe { match *riid {
            super::IID_IUnknown | super::IID_IErrorInfo =>
                Ok( vtables as *const _ as RawComPtr ),
            _ => Err( E_NOINTERFACE ),
        } }
    }

    /// `IErrorInfo` itself doesn't support error info.
    ///
    /// We don't want to override errors when doing error processing.
    fn interface_supports_error_info( _riid : REFIID ) -> bool { false }
}

/// `IErrorInfo` virtual table.
struct IErrorInfoVtbl {
    pub __base: IUnknownVtbl,
    pub get_guid: unsafe extern "stdcall" fn(
            RawComPtr, *mut GUID ) -> HRESULT,
    pub get_source: unsafe extern "stdcall" fn(
            RawComPtr, *mut BStr ) -> HRESULT,
    pub get_description: unsafe extern "stdcall" fn(
            RawComPtr, *mut BStr ) -> HRESULT,
    pub get_help_file: unsafe extern "stdcall" fn(
            RawComPtr, *mut BStr ) -> HRESULT,
    pub get_help_context: unsafe extern "stdcall" fn(
            RawComPtr, *mut u32 ) -> HRESULT
}

/// `IErrorInfo::GetGuid` raw COM implementation.
unsafe extern "stdcall" fn get_guid_impl(
    _this : RawComPtr,
    guid : *mut GUID
) -> HRESULT
{
    if guid.is_null() {
        return E_INVALIDARG;
    }

    *guid = GUID::zero_guid();
    S_OK
}

/// `IErrorInfo::GetSource` raw COM implementation.
unsafe extern "stdcall" fn get_source_impl(
    _this : RawComPtr,
    source : *mut BStr
) -> HRESULT
{
    if source.is_null() {
        return E_INVALIDARG;
    }

    *source = BStr::string_to_bstr( "Intercom-Component" );
    S_OK
}

/// `IErrorInfo::GetDescription` raw COM implementation.
unsafe extern "stdcall" fn get_description_impl(
    this : RawComPtr,
    desc : *mut BStr
) -> HRESULT
{
    if desc.is_null() {
        return E_INVALIDARG;
    }

    let cb = ComBox::< ErrorInfo >::from_ptr( this );
    *desc = BStr::string_to_bstr( &cb.description );
    S_OK
}

/// `IErrorInfo::GetHelpFile` raw COM implementation.
unsafe extern "stdcall" fn get_help_file_impl(
    _this : RawComPtr,
    help_file : *mut BStr
) -> HRESULT
{
    if help_file.is_null() {
        return E_INVALIDARG;
    }

    *help_file = BStr::string_to_bstr( "" );
    S_OK
}

/// `IErrorInfo::GetHelpContext` raw COM implementation.
unsafe extern "stdcall" fn get_help_context_impl(
    _this : RawComPtr,
    help_context : *mut u32
) -> HRESULT
{
    if help_context.is_null() {
        return E_INVALIDARG;
    }

    *help_context = 0;
    S_OK
}

/// Extracts the HRESULT from the error result and stores the extended error
/// information in thread memory so it can be fetched by the COM client.
pub fn return_hresult< E >( error : E ) -> HRESULT
    where E : Into< ComError >
{
    // Convet the error.
    let com_error = error.into();

    // Construct the COM class used for IErrorInfo. The class contains the
    // description in memory.
    let mut info = ComBox::< ErrorInfo >::new( ErrorInfo {
        description: com_error.description
    } );

    // Get the IErrorInfo interface and set it in thread memory.
    let mut error_ptr : RawComPtr = std::ptr::null_mut();
    unsafe {

        // We are intentionally ignoring the HRESULT codes here. We don't
        // want to override the original error HRESULT with these codes.
        ComBox::query_interface(
                info.as_mut(),
                &IID_IErrorInfo,
                &mut error_ptr );
        SetErrorInfo( 0, error_ptr );

        // SetErrorInfo took ownership of the error.
        // Forget it from the Box.
        Box::into_raw( info );
    }

    // Return the HRESULT of the original error.
    com_error.hresult
}

/// Gets the last COM error that occurred on the current thread.
pub fn get_last_error< E >() -> E
    where E : From< ComError >
{
    panic!( "Not implemented" );
}
