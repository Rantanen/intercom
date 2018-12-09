
use super::*;
use std::marker::PhantomData;
use type_system::{TypeSystem, TypeSystemName, RawTypeSystem, AutomationTypeSystem};

/// An incoming COM interface pointer.
///
/// Intercom will implement the various `[com_interface]` traits for the
/// corresponding `ComItf<T>` type.
///
/// This applies only to the pure interfaces.  Implicit interfaces created
/// through `#[com_interface] impl MyStruct` constructs are not supported for
/// `ComItf<T>`.
pub struct ComItf<T> where T: ?Sized {
    raw_ptr: raw::InterfacePtr<RawTypeSystem, T>,
    automation_ptr: raw::InterfacePtr<AutomationTypeSystem, T>,
    phantom: PhantomData<T>,
}

impl<T: ?Sized> std::fmt::Debug for ComItf<T> {
    fn fmt( &self, f : &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( f, "ComItf(automation = {:?}, raw = {:?})",
                self.automation_ptr, self.raw_ptr )
    }
}

impl<T: ?Sized> Clone for ComItf<T> {
    fn clone( &self ) -> Self {
        ComItf {
            raw_ptr: self.raw_ptr,
            automation_ptr: self.automation_ptr,
            phantom: PhantomData
        }
    }
}

impl<T: ?Sized> Copy for ComItf<T> { }

impl<T: ?Sized> ComItf<T> {

    /// Creates a `ComItf<T>` from a raw type system COM interface pointer..
    ///
    /// # Safety
    ///
    /// The `ptr` __must__ be a valid COM interface pointer for an interface
    /// of type `T`.
    pub unsafe fn new(
        automation : raw::InterfacePtr<AutomationTypeSystem, T>,
        raw : raw::InterfacePtr<RawTypeSystem, T>
    ) -> ComItf<T> {
        ComItf {
            raw_ptr: raw,
            automation_ptr: automation,
            phantom: PhantomData,
        }
    }

    /// Creates a `ComItf<T>` from a raw type system COM interface pointer..
    ///
    /// # Safety
    ///
    /// The `ptr` __must__ be a valid COM interface pointer for an interface
    /// of type `T`.
    pub fn maybe_wrap<TS: TypeSystem>(
        ptr : raw::InterfacePtr<TS, T>,
    ) -> Option<ComItf<T>>
    {
        if ptr.is_null() {
            None
        } else {
            Some( TS::wrap_ptr( ptr ) )
        }
    }

    /// Gets the raw COM pointer from the `ComItf<T>`.
    pub fn ptr<TS: TypeSystem>( this : &Self ) -> raw::InterfacePtr<TS, T> {
        TS::get_ptr( this )
    }

    pub fn maybe_ptr<TS: TypeSystem>(
        this : &Self
    ) -> Option<raw::InterfacePtr<TS, T>> {

        // Acquire the pointer.
        let ptr = Self::ptr( this );

        // Check for null.
        if ptr.is_null() {
            None
        } else {
            Some( ptr )
        }
    }

    /// Returns a `ComItf<T>` value that references a null pointer.
    ///
    /// # Safety
    ///
    /// The `ComItf<T>` returned by the function will be invalid for any
    /// method calls. Its purpose is to act as a return value from COM
    /// methods in the case of an error result.
    pub unsafe fn null_itf() -> ComItf<T> {
        ComItf {
            raw_ptr: raw::InterfacePtr::null(),
            automation_ptr: raw::InterfacePtr::null(),
            phantom: PhantomData,
        }
    }

    /// Checks whether the interface represents a null pointer.
    ///
    /// This should not be a case normally but may occur after certain unsafe
    /// operations.
    pub fn is_null( itf : &Self ) -> bool {
        itf.raw_ptr.is_null() && itf.automation_ptr.is_null()
    }
}

impl ComItf<IUnknown> {

    /// Tries to convert the ComRc into a different interface within a single
    /// type system. Used to implement the generic conversion method.
    fn query_interface_ts<TS: TypeSystem, TTarget: ComInterface + ?Sized>(
        &self
    ) -> ComResult<ComRc<TTarget>>
    {
        // Try to get the IID.
        let iid = match TTarget::iid( TS::key() ) {
            None => return Err( ComError::E_NOINTERFACE ),
            Some( iid ) => iid
        } ;

        // Try to query interface using the iid.
        let iunk : &IUnknown = &*self;
        match iunk.query_interface( iid ) {
            Ok( ptr ) => {

                let target_itf = unsafe {
                    raw::InterfacePtr::<TS, TTarget>::new( ptr )
                };
                let itf = ComItf::maybe_wrap( target_itf )
                        .ok_or_else( || ComError::E_POINTER )?;
                Ok( ComRc::attach( itf ) )
            },
            Err( e ) => Err( e.into() )
        }
    }
}

trait PointerOperations : TypeSystem + Sized {
    fn wrap_ptr<I: ?Sized>(
        ptr: ::raw::InterfacePtr<Self, I>
    ) -> ComItf<I>;

    fn get_ptr<I: ?Sized>(
        itf: &ComItf<I>
    ) -> ::raw::InterfacePtr<Self, I>;
}

impl<TS: TypeSystem> PointerOperations for TS {
    default fn wrap_ptr<I: ?Sized>(
        ptr: ::raw::InterfacePtr<Self, I>
    ) -> ComItf<I>
    {
        panic!( "Not implemented" );
    }

    default fn get_ptr<I: ?Sized>(
        itf: &ComItf<I>
    ) -> ::raw::InterfacePtr<Self, I>
    {
        panic!( "Not implemented" );
    }
}

impl PointerOperations for AutomationTypeSystem {
    fn wrap_ptr<I: ?Sized>(
        ptr: ::raw::InterfacePtr<Self, I>
    ) -> ComItf<I>
    {
        ComItf {
            raw_ptr: raw::InterfacePtr::null(),
            automation_ptr: ptr,
            phantom: PhantomData,
        }
    }

    fn get_ptr<I: ?Sized>(
        itf: &ComItf<I>
    ) -> ::raw::InterfacePtr<Self, I>
    {
        itf.automation_ptr
    }
}

impl PointerOperations for RawTypeSystem {
    fn wrap_ptr<I: ?Sized>(
        ptr: ::raw::InterfacePtr<Self, I>
    ) -> ComItf<I>
    {
        ComItf {
            raw_ptr: ptr,
            automation_ptr: raw::InterfacePtr::null(),
            phantom: PhantomData,
        }
    }

    fn get_ptr<I: ?Sized>(
        itf: &ComItf<I>
    ) -> ::raw::InterfacePtr<Self, I>
    {
        itf.raw_ptr
    }
}

impl<T: ComInterface + ?Sized> ComItf<T> {

    pub fn query_interface<TTarget: ComInterface + ?Sized>( this : &Self ) -> ComResult<ComRc<TTarget>>
    {
        let iunk : &ComItf<IUnknown> = this.as_ref();

        if let Ok( itf ) = iunk.query_interface_ts::<RawTypeSystem, TTarget>() {
            return Ok( itf );
        }

        if let Ok( itf ) = iunk.query_interface_ts::<AutomationTypeSystem, TTarget>() {
            return Ok( itf );
        }

        // If we got here, none of the query interfaces we invoked returned
        // anything.
        Err( ComError::E_NOINTERFACE )
    }

    // ComItf is a smart pointer and shouldn't introduce methods on 'self'.
    #[allow(clippy::wrong_self_convention)]
    pub fn as_unknown( this : &Self ) -> ComItf<IUnknown> {
        ComItf {
            raw_ptr: this.raw_ptr.as_unknown(),
            automation_ptr: this.automation_ptr.as_unknown(),
            phantom: PhantomData,
        }
    }
}

impl<T: ComInterface + ?Sized> std::ops::Deref for ComItf<T> {
    type Target = T;

    fn deref( &self ) -> &T {
        ComInterface::deref( self )
    }
}

#[cfg(windows)]
#[link(name = "ole32")]
extern "system" {

    #[doc(hidden)]
    pub fn CoCreateInstance(
        clsid : ::guid::GUID,
        outer : RawComPtr,
        cls_context: u32,
        riid : ::REFIID,
        out : &mut RawComPtr,
    ) -> ::raw::HRESULT;
}

impl<T: ComInterface + ?Sized> AsRef<ComItf<IUnknown>> for ComItf<T>
{
    fn as_ref( &self ) -> &ComItf<IUnknown> {
        unsafe { &*( self as *const _ as *const ComItf<IUnknown> ) }
    }
}
