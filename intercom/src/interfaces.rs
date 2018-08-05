use super::*;

/// The `IUnknown` COM interface.
///
/// All COM interfaces must inherit from `IUnknown` interface directly or
/// indirectly. The interface provides the basis of COM reference counting
/// and interface discovery.
///
/// For Rust code, Intercom implements the interface automatically.
#[com_interface( com_iid = "00000000-0000-0000-C000-000000000046", base = NO_BASE )]
pub trait IUnknown {

    /// Tries to get a different COM interface for the current object.
    ///
    /// COM objects may (and do) implement multiple interfaces. COM defines
    /// `QueryInterface` as the mechanism for acquiring an interface pointer
    /// to a different interface the object implements.
    ///
    /// * `riid` - The `IID` of the interface to query.
    ///
    /// Returns `Ok( interface_ptr )` if the object supports the specified
    /// interface or `Err( E_NOINTERFACE )` if it doesn't.
    fn query_interface( &self, riid : ::REFIID ) -> ::ComResult< ::RawComPtr >;

    /// Increments the reference count of the object.
    ///
    /// Returns the reference count after the incrementation.
    fn add_ref( &self ) -> u32;

    /// Decreases the reference count of the object.
    ///
    /// Returns the reference count after the decrement.
    ///
    /// If the reference count reaches zero, the object will deallocate
    /// itself. As the call might deallocate the object, the caller must
    /// ensure that the released reference is not used afterwards.
    fn release( &self ) -> u32;
}

/// The `ISupportErrorInfo` COM interface.
///
/// The `ISupportErrorInfo` is part of COM error handling concept. As the
/// methods are traditionally limited to `HRESULT` return values, they may
/// make more detailed `IErrorInfo` data available through the error info
/// APIs.
///
/// The `ISupportErrorInfo` interface communicates which interfaces that an
/// object implements support detailed error info. When a COM client
/// receives an error-HRESULT, it may query for error info support through
/// this interface. If the interface returns an `S_OK` as opposed to
/// `S_FALSE` return value, the client can then use separate error info
/// APIs to retrieve a detailed `IErrorInfo` object that contains more
/// details about the error, such as the error message.
///
/// Intercom COM classes support the detailed error info for all user
/// specified interfaces automatically. Only methods that return a
/// two-parameter `Result<S,E>` value will store the detailed `IErrorInfo`.
/// Other methods will set a null `IErrorInfo` value.
#[com_interface( com_iid = "DF0B3D60-548F-101B-8E65-08002B2BD119" )]
pub trait ISupportErrorInfo {

    /// Informs the current COM class supports `IErrorInfo` for a specific
    /// interface.
    ///
    /// * `riid` - The `IID` of the interface to query.
    ///
    /// Returns `S_OK` if the object supports `IErrorInfo` for the
    /// interface specified by the `riid` parameter. Otherwise returns
    /// `S_FALSE` - even in the case the object doesn't implement `riid`
    /// at all.
    ///
    /// # Description
    ///
    /// If the object returns `S_OK` for an interface, then any methods
    /// the object implements for that interface must store the
    /// `IErrorInfo` on failure.
    ///
    /// Intercom will implement the support for `IErrorInfo` automatically
    /// for all custom interfaces the user defines. This includes returning
    /// `S_OK` from this method.
    ///
    fn interface_supports_error_info( &self, riid : ::REFIID ) -> ::HRESULT;
}
