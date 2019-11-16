use intercom::{type_system::TypeSystem, IID};

pub trait ComImpl<TInterface: ?Sized, TS: TypeSystem>
where
    TInterface: ComInterface<TS>,
{
    fn vtable() -> &'static TInterface::VTable;
}

pub trait ComClass<TInterface: ?Sized, TS: TypeSystem>
{
    fn offset() -> usize;
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
