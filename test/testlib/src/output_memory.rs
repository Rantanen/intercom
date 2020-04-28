#![allow(clippy::type_complexity)]

use intercom::prelude::*;
use intercom::type_system::{ExternOutput, TypeSystem};
use intercom::IUnknown;
use std::ffi::c_void;

#[com_class(IOutputMemoryTests)]
#[derive(Default)]
pub struct OutputMemoryTests;

#[com_interface]
pub trait IOutputMemoryTests
{
    fn succeed(
        &self,
        input: &ComItf<dyn IUnknown>,
    ) -> ComResult<(ComRc<dyn IUnknown>, ComRc<dyn IUnknown>)>;

    fn fail(
        &self,
        input: &ComItf<dyn IUnknown>,
    ) -> ComResult<(ComRc<dyn IUnknown>, FailingType, ComRc<dyn IUnknown>)>;

    fn call_succeed(
        &self,
        itf: &ComItf<dyn IOutputMemoryTests>,
        input: &ComItf<dyn IUnknown>,
    ) -> ComResult<()>;

    fn call_fail(
        &self,
        itf: &ComItf<dyn IOutputMemoryTests>,
        input: &ComItf<dyn IUnknown>,
    ) -> ComResult<()>;
}

impl IOutputMemoryTests for OutputMemoryTests
{
    fn succeed(
        &self,
        input: &ComItf<dyn IUnknown>,
    ) -> ComResult<(ComRc<dyn IUnknown>, ComRc<dyn IUnknown>)>
    {
        Ok((input.into(), input.into()))
    }

    fn fail(
        &self,
        input: &ComItf<dyn IUnknown>,
    ) -> ComResult<(ComRc<dyn IUnknown>, FailingType, ComRc<dyn IUnknown>)>
    {
        Ok((input.into(), FailingType, input.into()))
    }

    fn call_succeed(
        &self,
        itf: &ComItf<dyn IOutputMemoryTests>,
        input: &ComItf<dyn IUnknown>,
    ) -> ComResult<()>
    {
        itf.succeed(input).map(|_| ())
    }

    fn call_fail(
        &self,
        itf: &ComItf<dyn IOutputMemoryTests>,
        input: &ComItf<dyn IUnknown>,
    ) -> ComResult<()>
    {
        itf.fail(input).map(|_| ())
    }
}

/// A type that fails all conversions.
pub struct FailingType;
unsafe impl<TS: TypeSystem> ExternOutput<TS> for FailingType
{
    type ForeignType = *mut c_void;

    fn into_foreign_output(self) -> ComResult<Self::ForeignType>
    {
        Err(ComError::E_FAIL)
    }

    unsafe fn from_foreign_output(_: Self::ForeignType) -> ComResult<Self>
    {
        Err(ComError::E_FAIL)
    }
}
