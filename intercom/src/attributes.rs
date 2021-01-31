use crate::combox::ComBoxData;
use crate::raw::RawComPtr;
use crate::type_system;
use crate::RawComResult;
use crate::REFIID;
use crate::{type_system::TypeSystem, IID};

/// Trait required by any COM coclass type.
///
/// Used to specify the virtual table for the `ComBoxData`.
pub trait ComClass
{
    type VTableList: Copy;
    const VTABLE: Self::VTableList;
    fn query_interface(vtables: &Self::VTableList, riid: REFIID) -> RawComResult<RawComPtr>;
    fn interface_supports_error_info(riid: REFIID) -> bool;
}

pub trait HasInterface<T: ComInterface + ?Sized>: ComClass {}

pub trait ComClassInterface<TInterface: ?Sized, TS: TypeSystem>: ComClass + Sized
{
    fn offset() -> usize;

    /// # Safety
    ///
    /// The `vtable` must be a valid pointer that points to the `TInterface`
    /// part of the `self`'s virtual table list.
    unsafe fn get_box<'a>(vtable: RawComPtr) -> &'a mut ComBoxData<Self>
    {
        let offset = Self::offset();
        let self_ptr = (vtable as usize - offset) as *mut _;
        &mut *self_ptr
    }
}

pub trait ComInterfaceVTableFor<I: ?Sized, S, TS: TypeSystem>: ComInterfaceVariant<TS>
{
    const VTABLE: Self::VTable;
}

/// The `ComInterface` trait defines the COM interface details for a COM
/// interface trait.
pub trait ComInterface
{
    /// The current interface.
    ///
    /// This associated type exists only to provide better error messages. When
    /// the `ComInterfaceVariant` is accessed through this type, the compiler
    /// will first report the missing `ComInterface` implementation if `Self`
    /// is not a COM interface.
    type TSelf: ?Sized;

    /// IID of the COM interface.
    fn iid(ts: type_system::TypeSystemName) -> Option<&'static IID>;

    fn iid_ts<TS: intercom::type_system::TypeSystem>() -> &'static intercom::IID
    where
        Self: intercom::attributes::ComInterfaceVariant<TS>;

    /// Dereferences a `ComItf<T>` into a `&T`.
    ///
    /// While in most cases the user crate will implement `T` for `ComItf<T>`,
    /// this impl exists only in the user crate and cannot be used in generic
    /// contexts. For generic `ComItf<T>` use, Intercom ipmls `Deref<Target=T>`
    /// for `ComItf<T>` which requires this method.
    fn deref(com_itf: &crate::ComItf<Self>) -> &Self;
}

pub trait ComInterfaceVariant<TS: TypeSystem>
{
    type VTable: Copy + 'static;
    fn iid() -> &'static IID;
}

pub trait ComClassTypeInfo
{
    fn gather_type_info() -> Vec<crate::typelib::TypeInfo>;
}

pub trait ComInterfaceTypeInfo
{
    fn gather_type_info() -> Vec<crate::typelib::TypeInfo>;
}
