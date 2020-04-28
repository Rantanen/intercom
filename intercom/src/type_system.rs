use crate::prelude::*;
use crate::ComItf;

#[derive(Debug, Clone, Copy, Hash, PartialOrd, PartialEq)]
#[repr(C)]
pub enum TypeSystemName
{
    Automation = 0,
    Raw = 1,
}

impl TypeSystemName
{
    pub fn get_ptr<I: ?Sized>(self, itf: &ComItf<I>) -> crate::raw::RawComPtr
    {
        let opt = match self {
            TypeSystemName::Automation => AutomationTypeSystem::get_ptr(itf).map(|p| p.ptr),
            TypeSystemName::Raw => RawTypeSystem::get_ptr(itf).map(|p| p.ptr),
        };

        match opt {
            Some(ptr) => ptr.as_ptr(),
            None => std::ptr::null_mut(),
        }
    }
}

/// Common trait for type systems.
pub trait TypeSystem: Clone + Copy
{
    const AUTOMATION: TypeSystemName = TypeSystemName::Automation;
    const RAW: TypeSystemName = TypeSystemName::Raw;

    fn key() -> TypeSystemName;

    /// Gets the type system pointer from a ComItf.
    fn get_ptr<I: ?Sized>(itf: &ComItf<I>) -> Option<crate::raw::InterfacePtr<Self, I>>;

    /// Constructs a ComItf from a pointer.
    fn wrap_ptr<I: ?Sized>(ptr: crate::raw::InterfacePtr<Self, I>) -> ComItf<I>;
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

    /// Gets the type system pointer from a ComItf.
    fn get_ptr<I: ?Sized>(itf: &ComItf<I>) -> Option<crate::raw::InterfacePtr<Self, I>>
    {
        itf.automation_ptr
    }

    /// Constructs a ComItf from a pointer.
    fn wrap_ptr<I: ?Sized>(ptr: crate::raw::InterfacePtr<Self, I>) -> ComItf<I>
    {
        ComItf {
            automation_ptr: Some(ptr),
            raw_ptr: None,
            phantom: std::marker::PhantomData,
        }
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

    /// Gets the type system pointer from a ComItf.
    fn get_ptr<I: ?Sized>(itf: &ComItf<I>) -> Option<crate::raw::InterfacePtr<Self, I>>
    {
        itf.raw_ptr
    }

    /// Constructs a ComItf from a pointer.
    fn wrap_ptr<I: ?Sized>(ptr: crate::raw::InterfacePtr<Self, I>) -> ComItf<I>
    {
        ComItf {
            automation_ptr: None,
            raw_ptr: Some(ptr),
            phantom: std::marker::PhantomData,
        }
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

/// Defines a type that may be used as a parameter type in Intercom interfaces.
///
/// # Safety
///
/// Implementing this trait allows Intercom to use the type as an input type.
/// This trait will be used within the code generated in the procedural macros.
/// It is important to ensure this trait is implemented in such a way that its
/// use in the macros is sound.
pub unsafe trait ExternInput<TS: TypeSystem>: Sized
{
    type ForeignType: ForeignType;

    type Lease;

    /// # Safety
    ///
    /// The returned `ForeignType` value is valid only as long as the `Lease`
    /// is held.
    unsafe fn into_foreign_parameter(self) -> ComResult<(Self::ForeignType, Self::Lease)>;

    type Owned;

    /// # Safety
    ///
    /// The validity of the returned `Owned` value depends on the source type.
    /// In general it shouldn't be used past the lifetime of the `source`
    /// reference.
    unsafe fn from_foreign_parameter(source: Self::ForeignType) -> ComResult<Self::Owned>;
}

/// Defines a type that may be used as an output type in Intercom interfaces.
///
/// # Safety
///
/// Implementing this trait allows Intercom to use the type as an output type.
/// This trait will be used within the code generated in the procedural macros.
/// It is important to ensure this trait is implemented in such a way that its
/// use in the macros is sound.
pub unsafe trait ExternOutput<TS: TypeSystem>: Sized
{
    type ForeignType: ForeignType;

    fn into_foreign_output(self) -> ComResult<Self::ForeignType>;

    /// # Safety
    ///
    /// The source ownership is transferred to the function invoker. In case of
    /// pointers, the function (or the `Self` type) is given the ownership of
    /// the memory. The caller must ensure that it owns the source parameter
    /// and is allowed to pass the ownership in this way.
    unsafe fn from_foreign_output(source: Self::ForeignType) -> ComResult<Self>;

    /// # Safety
    ///
    /// The source ownership is transferred to the function invoker. In case of
    /// pointers, the function (or the `Self` type) is given the ownership of
    /// the memory. The caller must ensure that it owns the source parameter
    /// and is allowed to pass the ownership in this way.
    unsafe fn drop_foreign_output(source: Self::ForeignType)
    {
        // Default implementation just converts this back to the original.
        //
        // The `from_foreign_output` is supposed to ensure unused memory isn't leaked and dropping
        // the return value will clean up the remaining memory.
        //
        // Type-specific implementation can clean up the source memory without going through the
        // trouble of creating  new `Self` value.
        let _ = Self::from_foreign_output(source);
    }
}

/// Defines a type that may be used as a parameter type in Intercom interfaces.
///
/// # Safety
///
/// Implementing this trait allows Intercom to use the type as an input type.
/// This trait will be used within the code generated in the procedural macros.
/// It is important to ensure this trait is implemented in such a way that its
/// use in the macros is sound.
pub unsafe trait InfallibleExternInput<TS: TypeSystem>: Sized
{
    type ForeignType: ForeignType;

    type Lease;

    /// # Safety
    ///
    /// The returned `ForeignType` value is valid only as long as the `Lease`
    /// is held.
    unsafe fn into_foreign_parameter(self) -> (Self::ForeignType, Self::Lease);

    type Owned;

    /// # Safety
    ///
    /// The validity of the returned `Owned` value depends on the source type.
    /// In general it shouldn't be used past the lifetime of the `source`
    /// reference.
    unsafe fn from_foreign_parameter(source: Self::ForeignType) -> Self::Owned;
}

/// Defines a type that may be used as an output type in Intercom interfaces.
///
/// # Safety
///
/// Implementing this trait allows Intercom to use the type as an output type.
/// This trait will be used within the code generated in the procedural macros.
/// It is important to ensure this trait is implemented in such a way that its
/// use in the macros is sound.
pub unsafe trait InfallibleExternOutput<TS: TypeSystem>: Sized
{
    type ForeignType: ForeignType;

    fn into_foreign_output(self) -> Self::ForeignType;

    /// # Safety
    ///
    /// The source ownership is transferred to the function invoker. In case of
    /// pointers, the function (or the `Self` type) is given the ownership of
    /// the memory. The caller must ensure that it owns the source parameter
    /// and is allowed to pass the ownership in this way.
    unsafe fn from_foreign_output(source: Self::ForeignType) -> Self;
}

/// Holds a conversion result foreign value and cleans it up unless consumed
pub struct OutputGuard<TS, TType>
where
    TS: TypeSystem,
    TType: ExternOutput<TS>,
{
    value: std::mem::ManuallyDrop<TType::ForeignType>,
}

impl<TS, TType> OutputGuard<TS, TType>
where
    TS: TypeSystem,
    TType: ExternOutput<TS>,
{
    /// Wrap a foreign value in the guard.
    pub fn wrap(value: TType::ForeignType) -> OutputGuard<TS, TType>
    {
        OutputGuard {
            value: std::mem::ManuallyDrop::new(value),
        }
    }

    /// Consume the guard to acquire the final value and giving up on having to clean it later.
    pub fn consume(self) -> TType::ForeignType
    {
        unsafe {
            // Read the value out of the guard and forget the guard to avoid
            // dropping it, which would clean the value.
            let value = std::ptr::read(&self.value);
            std::mem::forget(self);
            std::mem::ManuallyDrop::into_inner(value)
        }
    }
}

impl<TS, TType> Drop for OutputGuard<TS, TType>
where
    TS: TypeSystem,
    TType: ExternOutput<TS>,
{
    fn drop(&mut self)
    {
        unsafe {
            // Clean the value on drop..
            let v = std::mem::ManuallyDrop::take(&mut self.value);
            TType::drop_foreign_output(v);
        }
    }
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

        unsafe impl<TS: TypeSystem> ExternInput<TS> for $t
        {
            type ForeignType = $t;
            type Lease = ();
            unsafe fn into_foreign_parameter(self) -> ComResult<(Self::ForeignType, ())>
            {
                Ok((self, ()))
            }

            type Owned = Self;
            unsafe fn from_foreign_parameter(source: Self::ForeignType) -> ComResult<Self::Owned>
            {
                Ok(source)
            }
        }

        unsafe impl<TS: TypeSystem> ExternOutput<TS> for $t
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

        unsafe impl<TS: TypeSystem> InfallibleExternInput<TS> for $t
        {
            type ForeignType = $t;
            type Lease = ();
            unsafe fn into_foreign_parameter(self) -> (Self::ForeignType, ())
            {
                (self, ())
            }

            type Owned = Self;
            unsafe fn from_foreign_parameter(source: Self::ForeignType) -> Self::Owned
            {
                source
            }
        }

        unsafe impl<TS: TypeSystem> InfallibleExternOutput<TS> for $t
        {
            type ForeignType = $t;
            fn into_foreign_output(self) -> Self::ForeignType
            {
                self
            }

            unsafe fn from_foreign_output(source: Self::ForeignType) -> Self
            {
                source
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

self_extern!(std::ffi::c_void);

macro_rules! extern_ptr {
    ( $mut:tt ) => {
        unsafe impl<TS: TypeSystem, TPtr: ForeignType + ?Sized> ExternOutput<TS> for *$mut TPtr
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

        unsafe impl<TS: TypeSystem, TPtr: ForeignType + ?Sized> ExternInput<TS> for *$mut TPtr
        {
            type ForeignType = Self;
            type Lease = ();
            unsafe fn into_foreign_parameter(self) -> ComResult<(Self::ForeignType, ())>
            {
                Ok((self, ()))
            }

            type Owned = Self;
            unsafe fn from_foreign_parameter(source: Self::ForeignType) -> ComResult<Self::Owned>
            {
                Ok(source)
            }
        }

        unsafe impl<TS: TypeSystem, TPtr: ForeignType + ?Sized> InfallibleExternOutput<TS> for *$mut TPtr
        {
            type ForeignType = Self;
            fn into_foreign_output(self) -> Self::ForeignType
            {
                self
            }

            unsafe fn from_foreign_output(source: Self::ForeignType) -> Self
            {
                source
            }
        }

        unsafe impl<TS: TypeSystem, TPtr: ForeignType + ?Sized> InfallibleExternInput<TS> for *$mut TPtr
        {
            type ForeignType = Self;
            type Lease = ();
            unsafe fn into_foreign_parameter(self) -> (Self::ForeignType, ())
            {
                (self, ())
            }

            type Owned = Self;
            unsafe fn from_foreign_parameter(source: Self::ForeignType) -> Self::Owned
            {
                source
            }
        }

        impl<TPtr: ForeignType + ?Sized> ForeignType for *$mut TPtr
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
    }
}

extern_ptr!(mut);
extern_ptr!(const);

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
    unsafe fn extern_default() -> Self
    {
        std::mem::zeroed()
    }
}
