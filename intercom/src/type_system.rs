use crate::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialOrd, PartialEq)]
#[repr(C)]
pub enum TypeSystemName
{
    Automation = 0,
    Raw = 1,
}

/// Common trait for type systems.
pub trait TypeSystem: Clone + Copy
{
    const AUTOMATION: TypeSystemName = TypeSystemName::Automation;
    const RAW: TypeSystemName = TypeSystemName::Raw;

    fn key() -> TypeSystemName;
}

/// Automation type system.
#[derive(Clone, Copy)]
pub struct AutomationTypeSystem;
impl TypeSystem for AutomationTypeSystem
{
    fn key() -> TypeSystemName
    {
        TypeSystemName::Automation
    }
}

/// Raw type system.
#[derive(Clone, Copy)]
pub struct RawTypeSystem;
impl TypeSystem for RawTypeSystem
{
    fn key() -> TypeSystemName
    {
        TypeSystemName::Raw
    }
}

/// Defines a type that has identical representation for both input and output directions.
pub trait ForeignType
{
    /// The name of the type.
    fn type_name() -> &'static str;
    fn indirection_level() -> u32
    {
        0
    }
}

/// Defines a type that is compatible with Intercom interfaces.
pub trait ExternInput<TS: TypeSystem>: Sized
{
    type ForeignType: ForeignType;

    type Lease;
    fn into_foreign_parameter(self) -> ComResult<(Self::ForeignType, Self::Lease)>;

    type Owned;
    unsafe fn from_foreign_parameter(source: Self::ForeignType) -> ComResult<Self::Owned>;
}

pub trait ExternOutput<TS: TypeSystem>: Sized
{
    type ForeignType: ForeignType;

    fn into_foreign_output(self) -> ComResult<Self::ForeignType>;

    unsafe fn from_foreign_output(source: Self::ForeignType) -> ComResult<Self>;
}

/// A quick macro for implementing ExternInput/etc. for various basic types
/// that should represent themselves.
macro_rules! self_extern {
    ( $t:ty ) => {
        impl ForeignType for $t
        {
            /// The default name is the name of the type.
            fn type_name() -> &'static str
            {
                stringify!($t)
            }
        }

        impl<TS: TypeSystem> ExternInput<TS> for $t
        {
            type ForeignType = $t;
            type Lease = ();
            fn into_foreign_parameter(self) -> ComResult<(Self::ForeignType, ())>
            {
                Ok((self, ()))
            }

            type Owned = Self;
            unsafe fn from_foreign_parameter(source: Self::ForeignType) -> ComResult<Self::Owned>
            {
                Ok(source)
            }
        }

        impl<TS: TypeSystem> ExternOutput<TS> for $t
        {
            type ForeignType = $t;
            fn into_foreign_output(self) -> ComResult<Self::ForeignType>
            {
                Ok(self)
            }

            unsafe fn from_foreign_output(source: Self::ForeignType) -> ComResult<Self>
            {
                Ok(source)
            }
        }
    };
}

// Define all types that should have built-in Self extern type.
self_extern!(());
self_extern!(i8);
self_extern!(i16);
self_extern!(i32);
self_extern!(i64);
self_extern!(isize);
self_extern!(u8);
self_extern!(u16);
self_extern!(u32);
self_extern!(u64);
self_extern!(usize);
self_extern!(f32);
self_extern!(f64);
self_extern!(bool);

use crate::raw::HRESULT;
self_extern!(HRESULT);

use crate::GUID;
self_extern!(GUID);

self_extern!(TypeSystemName);

impl ForeignType for libc::c_void
{
    fn type_name() -> &'static str
    {
        "void"
    }
}

impl<TS: TypeSystem, TPtr: ForeignType + ?Sized> ExternOutput<TS> for *mut TPtr
{
    type ForeignType = Self;
    fn into_foreign_output(self) -> ComResult<Self::ForeignType>
    {
        Ok(self)
    }

    unsafe fn from_foreign_output(source: Self::ForeignType) -> ComResult<Self>
    {
        Ok(source)
    }
}

impl<TS: TypeSystem, TPtr: ForeignType + ?Sized> ExternOutput<TS> for *const TPtr
{
    type ForeignType = Self;
    fn into_foreign_output(self) -> ComResult<Self::ForeignType>
    {
        Ok(self)
    }

    unsafe fn from_foreign_output(source: Self::ForeignType) -> ComResult<Self>
    {
        Ok(source)
    }
}

impl<TS: TypeSystem, TPtr: ForeignType + ?Sized> ExternInput<TS> for *mut TPtr
{
    type ForeignType = Self;
    type Lease = ();
    fn into_foreign_parameter(self) -> ComResult<(Self::ForeignType, ())>
    {
        Ok((self, ()))
    }

    type Owned = Self;
    unsafe fn from_foreign_parameter(source: Self::ForeignType) -> ComResult<Self::Owned>
    {
        Ok(source)
    }
}

impl<TS: TypeSystem, TPtr: ForeignType + ?Sized> ExternInput<TS> for *const TPtr
{
    type ForeignType = Self;
    type Lease = ();
    fn into_foreign_parameter(self) -> ComResult<(Self::ForeignType, ())>
    {
        Ok((self, ()))
    }

    type Owned = Self;
    unsafe fn from_foreign_parameter(source: Self::ForeignType) -> ComResult<Self::Owned>
    {
        Ok(source)
    }
}

impl<TPtr: ForeignType + ?Sized> ForeignType for *mut TPtr
{
    fn type_name() -> &'static str
    {
        <TPtr as ForeignType>::type_name()
    }

    fn indirection_level() -> u32
    {
        <TPtr as ForeignType>::indirection_level() + 1
    }
}

impl<TPtr: ForeignType + ?Sized> ForeignType for *const TPtr
{
    fn type_name() -> &'static str
    {
        <TPtr as ForeignType>::type_name()
    }

    fn indirection_level() -> u32
    {
        <TPtr as ForeignType>::indirection_level() + 1
    }
}

impl<TS: TypeSystem, I: crate::ComInterface + ?Sized> ForeignType
    for crate::raw::InterfacePtr<TS, I>
where
    I: ForeignType,
{
    /// The name of the type.
    fn type_name() -> &'static str
    {
        <I as ForeignType>::type_name()
    }
    fn indirection_level() -> u32
    {
        <I as ForeignType>::indirection_level() + 1
    }
}

/// Defines the uninitialized values for out parameters when calling into
/// Intercom interfaces.
pub trait ExternDefault
{
    /// # Safety
    ///
    /// This results in zeroed values. This should only be used for types that
    /// are okay being zeroed (mainly `#[repr(C)]` types).
    unsafe fn extern_default() -> Self;
}

impl<T> ExternDefault for T
{
    default unsafe fn extern_default() -> Self
    {
        std::mem::zeroed()
    }
}

impl<TPtr> ExternDefault for *const TPtr
{
    default unsafe fn extern_default() -> Self
    {
        std::ptr::null()
    }
}
