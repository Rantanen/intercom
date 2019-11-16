use crate::combox::{CoClass, ComBoxData};
use crate::raw::RawComPtr;
use crate::{type_system::TypeSystem, IID};

pub trait ComImpl<TInterface: ?Sized, TS: TypeSystem>
where
    TInterface: ComInterface<TS>,
{
    fn vtable() -> &'static TInterface::VTable;
}

pub trait ComClass<TInterface: ?Sized, TS: TypeSystem>: CoClass + Sized
{
    fn offset() -> usize;
    unsafe fn get_box<'a>(vtable: RawComPtr) -> &'a mut ComBoxData<Self>
    {
        let offset = Self::offset();
        let self_ptr = (vtable as usize - offset) as *mut _;
        &mut *self_ptr
    }
}

pub trait VTableFor<I: ?Sized, S, TS: TypeSystem>: ComInterface<TS>
{
    const VTABLE: Self::VTable;
}

pub trait ComInterface<TS: TypeSystem>
{
    type VTable: 'static;
    fn iid() -> &'static IID;
}

pub trait HasTypeInfo
{
    fn gather_type_info() -> Vec<crate::typelib::TypeInfo>;
}

pub trait InterfaceHasTypeInfo
{
    fn gather_type_info() -> Vec<crate::typelib::TypeInfo>;
}
