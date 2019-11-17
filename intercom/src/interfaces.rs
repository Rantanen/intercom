use super::*;
use crate::raw::HRESULT;
use crate::type_system::{AutomationTypeSystem, RawTypeSystem};

/// The `IUnknown` COM interface.
///
/// All COM interfaces must inherit from `IUnknown` interface directly or
/// indirectly. The interface provides the basis of COM reference counting
/// and interface discovery.
///
/// For Rust code, Intercom implements the interface automatically.
#[com_interface(
    com_iid = "00000000-0000-0000-C000-000000000046",
    raw_iid = "11111111-0000-0000-C000-000000000046",
    base = NO_BASE,
    vtable_of = RawIUnknown )]
pub trait IUnknown
{
}

#[com_interface(
    com_iid = "00000000-0000-0000-C000-000000000046",
    raw_iid = "11111111-0000-0000-C000-000000000046",
    base = NO_BASE )]
pub trait RawIUnknown
{
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
    fn query_interface(&self, riid: crate::REFIID) -> crate::RawComResult<crate::raw::RawComPtr>;

    /// Increments the reference count of the object.
    ///
    /// Returns the reference count after the incrementation.
    fn add_ref(&self) -> u32;

    /// Decreases the reference count of the object.
    ///
    /// Returns the reference count after the decrement.
    ///
    /// If the reference count reaches zero, the object will deallocate
    /// itself. As the call might deallocate the object, the caller must
    /// ensure that the released reference is not used afterwards.
    fn release(&self) -> u32;
}

impl<I, S> crate::attributes::ComInterfaceVTableFor<I, S, RawTypeSystem> for dyn IUnknown
where
    I: ?Sized,
    S: intercom::attributes::ComClassInterface<I, RawTypeSystem> + intercom::attributes::ComClass,
{
    const VTABLE: Self::VTable = Self::VTable {
        query_interface: query_interface::<I, S, RawTypeSystem>,
        add_ref: add_ref::<I, S, RawTypeSystem>,
        release: release::<I, S, RawTypeSystem>,
    };
}
impl<I, S> crate::attributes::ComInterfaceVTableFor<I, S, AutomationTypeSystem> for dyn IUnknown
where
    I: ?Sized,
    S: intercom::attributes::ComClassInterface<I, AutomationTypeSystem>
        + intercom::attributes::ComClass,
{
    const VTABLE: Self::VTable = Self::VTable {
        query_interface: query_interface::<I, S, AutomationTypeSystem>,
        add_ref: add_ref::<I, S, AutomationTypeSystem>,
        release: release::<I, S, AutomationTypeSystem>,
    };
}

pub unsafe extern "system" fn query_interface<I, S, TS>(
    self_vtable: crate::raw::RawComPtr,
    riid: *const crate::GUID,
    out: *mut *mut std::ffi::c_void,
) -> HRESULT
where
    I: ?Sized,
    S: intercom::attributes::ComClassInterface<I, TS> + intercom::attributes::ComClass,
    TS: crate::type_system::TypeSystem,
{
    intercom::ComBoxData::<S>::query_interface(S::get_box(self_vtable), riid, out)
}

pub unsafe extern "system" fn add_ref<I, S, TS>(self_vtable: crate::raw::RawComPtr) -> u32
where
    I: ?Sized,
    S: intercom::attributes::ComClassInterface<I, TS> + intercom::attributes::ComClass,
    TS: crate::type_system::TypeSystem,
{
    intercom::ComBoxData::<S>::add_ref(S::get_box(self_vtable))
}

pub unsafe extern "system" fn release<I, S, TS>(self_vtable: crate::raw::RawComPtr) -> u32
where
    I: ?Sized,
    S: intercom::attributes::ComClassInterface<I, TS> + intercom::attributes::ComClass,
    TS: crate::type_system::TypeSystem,
{
    intercom::ComBoxData::<S>::release(S::get_box(self_vtable))
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
#[com_interface(
    com_iid = "DF0B3D60-548F-101B-8E65-08002B2BD119",
    raw_iid = "4C667A45-1C4F-4761-8EBF-34E7699BD06E",
    implemented_by = isupporterrorinfo
)]
pub trait ISupportErrorInfo: IUnknown
{
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
    fn interface_supports_error_info(&self, riid: crate::REFIID) -> crate::raw::HRESULT;
}

pub mod isupporterrorinfo
{
    use crate::{combox::ComBoxData, raw, REFIID};

    /// Checks whether the given interface identified by the IID supports error
    /// info through IErrorInfo.
    pub fn interface_supports_error_info<S>(_this: &ComBoxData<S>, riid: REFIID) -> raw::HRESULT
    where
        S: intercom::attributes::ComClass,
    {
        match S::interface_supports_error_info(riid) {
            true => raw::S_OK,
            false => raw::S_FALSE,
        }
    }
}
