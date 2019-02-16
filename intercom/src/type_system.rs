
use prelude::*;

pub enum TypeSystemName {
    Automation,
    Raw,
}

/// Common trait for type systems.
pub trait TypeSystem : Clone + Copy {
    const Automation : TypeSystemName = TypeSystemName::Automation;
    const Raw : TypeSystemName = TypeSystemName::Raw;

    fn key() -> TypeSystemName;
}

/// Automation type system.
#[derive(Clone, Copy)]
pub struct AutomationTypeSystem;
impl TypeSystem for AutomationTypeSystem {
    fn key() -> TypeSystemName { TypeSystemName::Automation }
}

/// Raw type system.
#[derive(Clone, Copy)]
pub struct RawTypeSystem;
impl TypeSystem for RawTypeSystem {
    fn key() -> TypeSystemName { TypeSystemName::Raw }
}

/// Defines a type that is compatible with Intercom interfaces.
pub trait ExternType<TS: TypeSystem> : Sized {

    /// Type used when the Self type is encountered as an input parameter.
    type ExternInputType;

    /// Type used when the Self type is encountered as an output type.
    type ExternOutputType;

    /// A possible temporary type used for converting `Self` into
    /// `ExternInputType` when calling Intercom interfaces from Rust.
    type OwnedExternType : IntercomFrom< Self > = Self;

    /// A possible temporary type used for converting `ExternInputType` into
    /// `Self` type when calling Rust through an Intercom interface.
    type OwnedNativeType : IntercomFrom< Self::ExternInputType > = Self;
}

/// A conversion that may fail by resulting in a `ComError .
pub trait IntercomFrom<TSource> : Sized {
    fn intercom_from( source : TSource ) -> ComResult<Self>;
}

/// Default identity blanket implementation.
impl<T> IntercomFrom<T> for T {
    default fn intercom_from( source: T ) -> ComResult<T> { Ok( source ) }
}

/// Blanket implementation for all cloneable instance references.
impl<TSource: Clone> IntercomFrom<&TSource> for TSource {
    fn intercom_from( source: &TSource ) -> ComResult<Self> {
        Ok( source.clone() )
    }
}

/// A conversion that may fail by resulting in a `ComError .
pub trait IntercomInto<TTarget> {
    fn intercom_into( self : Self ) -> ComResult<TTarget>;
}

/// Blanket implementation for reversing IntercomFrom into IntercomInto.
impl<TSource, TTarget: IntercomFrom<TSource>>
        IntercomInto<TTarget> for TSource
{
    default fn intercom_into( self: Self ) -> ComResult<TTarget> {
        TTarget::intercom_from( self )
    }
}

/// A quick macro for implementing ExternType for various basic types that
/// should represent themselves.
///
/// Ideally we would use specialization here to implement ExternType for T,
/// but that prevents other crates from implementing a specialized version for
/// some reason.
macro_rules! self_extern {
    ( $t:ty ) => {
        impl<TS: TypeSystem> ExternType<TS> for $t {
            type ExternInputType = $t;
            type ExternOutputType = $t;
            type OwnedExternType = $t;
            type OwnedNativeType = $t;
        }
    }
}

// Define all types that should have built-in Self extern type.
self_extern!( () );
self_extern!( i8 );
self_extern!( i16 );
self_extern!( i32 );
self_extern!( i64 );
self_extern!( isize );
self_extern!( u8 );
self_extern!( u16 );
self_extern!( u32 );
self_extern!( u64 );
self_extern!( usize );
self_extern!( f32 );
self_extern!( f64 );
self_extern!( ::raw::HRESULT );
self_extern!( ::GUID );

// Any raw pointer is passed as is.

impl<TS: TypeSystem, TPtr> ExternType<TS> for *mut TPtr {
    type ExternInputType = *mut TPtr;
    type ExternOutputType = *mut TPtr;
    type OwnedExternType = *mut TPtr;
    type OwnedNativeType = *mut TPtr;
}

impl<TS: TypeSystem, TPtr> ExternType<TS> for *const TPtr {
    type ExternInputType = *const TPtr;
    type ExternOutputType = *const TPtr;
    type OwnedExternType = *const TPtr;
    type OwnedNativeType = *const TPtr;
}

/// `ComItf` extern type implementation.
impl<TS: TypeSystem, I: ::ComInterface + ?Sized> ExternType<TS>
        for ::ComItf<I> {

    type ExternInputType = ::raw::InterfacePtr<TS, I>;
    type ExternOutputType = ::raw::InterfacePtr<TS, I>;
    type OwnedExternType = ::raw::InterfacePtr<TS, I>;
    type OwnedNativeType = ::raw::InterfacePtr<TS, I>;
}

impl<TS: TypeSystem, I: ::ComInterface + ?Sized>
    IntercomFrom<::ComItf<I>> for ::raw::InterfacePtr<TS, I>
{
    fn intercom_from( source: ::ComItf<I> ) -> ComResult<Self> {
        Ok( ::ComItf::ptr( &source ) )
    }
}

impl<TS: TypeSystem, I: ::ComInterface + ?Sized>
    IntercomFrom<&::ComItf<I>> for ::raw::InterfacePtr<TS, I>
{
    fn intercom_from( source: &::ComItf<I> ) -> ComResult<Self> {
        Ok( ::ComItf::ptr( source ) )
    }
}

impl<TS: TypeSystem, I: ::ComInterface + ?Sized>
    IntercomFrom<::raw::InterfacePtr<TS, I>> for ::ComItf<I>
{
    fn intercom_from( source: ::raw::InterfacePtr<TS, I> ) -> ComResult<Self> {
        ::ComItf::maybe_wrap( source )
                .ok_or_else( || ::ComError::E_INVALIDARG )
    }
}

impl<TS: TypeSystem, I: ::ComInterface + ?Sized>
    IntercomFrom<&::raw::InterfacePtr<TS, I>> for ::ComItf<I>
{
    fn intercom_from( source: &::raw::InterfacePtr<TS, I> ) -> ComResult<Self> {
        ::ComItf::maybe_wrap( source.clone() )
                .ok_or_else( || ::ComError::E_INVALIDARG )
    }
}

/// Defines the uninitialized values for out parameters when calling into
/// Intercom interfaces.
pub trait ExternDefault {
    unsafe fn extern_default() -> Self;
}

impl<T> ExternDefault for T {
    default unsafe fn extern_default() -> Self { std::mem::zeroed() }
}

impl<TPtr> ExternDefault for *const TPtr {
    default unsafe fn extern_default() -> Self { std::ptr::null() }
}
