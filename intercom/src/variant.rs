use crate::attributes::{ComInterface, HasInterface};
use crate::type_system::{ExternInput, ExternOutput, TypeSystem};
use crate::*;
use intercom_attributes::ForeignType;
use std::convert::TryFrom;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy)]
pub struct Currency(i64);

#[derive(Debug, Clone, ForeignType)]
pub enum Variant
{
    None,
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Currency(Currency),
    String(IntercomString),
    SystemTime(SystemTime),
    IUnknown(ComRc<dyn IUnknown>),
}

impl Variant
{
    pub fn raw_type(&self) -> u16
    {
        match self {
            Variant::None => raw::var_type::EMPTY,
            Variant::I8(..) => raw::var_type::I1,
            Variant::I16(..) => raw::var_type::I2,
            Variant::I32(..) => raw::var_type::I4,
            Variant::I64(..) => raw::var_type::I8,
            Variant::U8(..) => raw::var_type::UI1,
            Variant::U16(..) => raw::var_type::UI2,
            Variant::U32(..) => raw::var_type::UI4,
            Variant::U64(..) => raw::var_type::UI8,
            Variant::F32(..) => raw::var_type::R4,
            Variant::F64(..) => raw::var_type::R8,
            Variant::Bool(..) => raw::var_type::BOOL,
            Variant::String(..) => raw::var_type::BSTR,
            Variant::SystemTime(..) => raw::var_type::DATE,
            Variant::Currency(..) => raw::var_type::CY,
            Variant::IUnknown(..) => raw::var_type::UNKNOWN,
        }
    }

    /// # Safety
    ///
    /// The source variant must be a valid variant.
    pub unsafe fn from_raw<TS: TypeSystem>(src: raw::Variant<TS>) -> ComResult<Self>
    {
        Ok(if src.vt.0 & raw::var_type::BYREF == 0 {
            match src.vt.0 & raw::var_type::TYPEMASK {
                raw::var_type::EMPTY | raw::var_type::NULL => Variant::None,
                raw::var_type::I1 => Variant::I8(src.data.bVal),
                raw::var_type::I2 => Variant::I16(src.data.iVal),
                raw::var_type::I4 => Variant::I32(src.data.lVal),
                raw::var_type::I8 => Variant::I64(src.data.llVal),
                raw::var_type::UI1 => Variant::U8(src.data.cVal),
                raw::var_type::UI2 => Variant::U16(src.data.uiVal),
                raw::var_type::UI4 => Variant::U32(src.data.ulVal),
                raw::var_type::UI8 => Variant::U64(src.data.ullVal),
                raw::var_type::R4 => Variant::F32(src.data.fltVal),
                raw::var_type::R8 => Variant::F64(src.data.dblVal),
                raw::var_type::BOOL => Variant::Bool(src.data.boolVal.into()),
                raw::var_type::BSTR => Variant::String(crate::IntercomString::BString(
                    crate::BString::from_ptr(src.data.bstrVal),
                )),
                raw::var_type::CY => Variant::Currency(Currency(src.data.cyVal)),
                raw::var_type::DATE => Variant::SystemTime(src.data.date.into()),
                raw::var_type::UNKNOWN => match src.data.punkVal {
                    Some(ptr) => Variant::IUnknown(ComRc::wrap(ptr)),
                    None => Variant::None,
                },
                _ => return Err(ComError::E_NOTIMPL),
            }
        } else {
            match src.vt.0 & raw::var_type::TYPEMASK {
                raw::var_type::EMPTY | raw::var_type::NULL => Variant::None,
                raw::var_type::I1 => Variant::I8(*src.data.pbVal),
                raw::var_type::I2 => Variant::I16(*src.data.piVal),
                raw::var_type::I4 => Variant::I32(*src.data.plVal),
                raw::var_type::I8 => Variant::I64(*src.data.pllVal),
                raw::var_type::UI1 => Variant::U8(*src.data.pcVal),
                raw::var_type::UI2 => Variant::U16(*src.data.puiVal),
                raw::var_type::UI4 => Variant::U32(*src.data.pulVal),
                raw::var_type::UI8 => Variant::U64(*src.data.pullVal),
                raw::var_type::R4 => Variant::F32(*src.data.pfltVal),
                raw::var_type::R8 => Variant::F64(*src.data.pdblVal),
                raw::var_type::BOOL => Variant::Bool((*src.data.pboolVal).into()),
                raw::var_type::BSTR => Variant::String(crate::IntercomString::BString(
                    crate::BString::from_ptr(*src.data.pbstrVal),
                )),
                raw::var_type::DATE => Variant::SystemTime((*src.data.pdate).into()),
                raw::var_type::CY => Variant::Currency(Currency(*src.data.pcyVal)),
                raw::var_type::UNKNOWN => match *src.data.ppunkVal {
                    Some(ptr) => Variant::IUnknown(ComRc::wrap(ptr)),
                    None => Variant::None,
                },
                _ => return Err(ComError::E_NOTIMPL),
            }
        })
    }
}

impl Default for Variant
{
    fn default() -> Self
    {
        Variant::None
    }
}

unsafe impl<TS: TypeSystem> ExternInput<TS> for Variant
{
    type ForeignType = raw::Variant<TS>;

    type Lease = ();
    unsafe fn into_foreign_parameter(self) -> ComResult<(Self::ForeignType, ())>
    {
        Self::ForeignType::try_from(self).map(|variant| (variant, ()))
    }

    type Owned = Self;
    unsafe fn from_foreign_parameter(src: Self::ForeignType) -> ComResult<Self::Owned>
    {
        Self::from_raw(src)
    }
}

unsafe impl<TS: TypeSystem> ExternOutput<TS> for Variant
{
    type ForeignType = raw::Variant<TS>;

    fn into_foreign_output(self) -> ComResult<Self::ForeignType>
    {
        Self::ForeignType::try_from(self)
    }

    unsafe fn from_foreign_output(src: Self::ForeignType) -> ComResult<Self>
    {
        Self::from_raw(src)
    }
}

impl<TS: TypeSystem> TryFrom<Variant> for raw::Variant<TS>
{
    type Error = ComError;
    fn try_from(src: Variant) -> ComResult<Self>
    {
        Ok(match src {
            Variant::None => raw::Variant::new(
                raw::VariantType::new(raw::var_type::EMPTY),
                raw::VariantData { bVal: 0 },
            ),
            Variant::I8(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::I1),
                raw::VariantData { bVal: data },
            ),
            Variant::I16(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::I2),
                raw::VariantData { iVal: data },
            ),
            Variant::I32(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::I4),
                raw::VariantData { lVal: data },
            ),
            Variant::I64(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::I8),
                raw::VariantData { llVal: data },
            ),
            Variant::U8(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::UI1),
                raw::VariantData { cVal: data },
            ),
            Variant::U16(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::UI2),
                raw::VariantData { uiVal: data },
            ),
            Variant::U32(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::UI4),
                raw::VariantData { ulVal: data },
            ),
            Variant::U64(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::UI8),
                raw::VariantData { ullVal: data },
            ),
            Variant::F32(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::R4),
                raw::VariantData { fltVal: data },
            ),
            Variant::F64(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::R8),
                raw::VariantData { dblVal: data },
            ),
            Variant::Bool(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::BOOL),
                raw::VariantData {
                    boolVal: data.into(),
                },
            ),
            Variant::Currency(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::CY),
                raw::VariantData { cyVal: data.0 },
            ),
            Variant::String(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::BSTR),
                raw::VariantData {
                    bstrVal: crate::BString::try_from(data)?.into_ptr(),
                },
            ),
            Variant::SystemTime(data) => raw::Variant::new(
                raw::VariantType::new(raw::var_type::DATE),
                raw::VariantData { date: data.into() },
            ),

            Variant::IUnknown(data) => {
                let v = raw::Variant::new(
                    raw::VariantType::new(raw::var_type::UNKNOWN),
                    raw::VariantData {
                        punkVal: ComItf::ptr(&data),
                    },
                );

                // We didn't add_ref the punkVal so avoid release by forgetting
                // the ComRc.
                std::mem::forget(data);

                v
            }
        })
    }
}

#[derive(Debug)]
pub struct VariantError;
impl<'a> From<&'a Variant> for VariantError
{
    fn from(_: &Variant) -> Self
    {
        VariantError
    }
}

impl From<VariantError> for ComError
{
    fn from(_: VariantError) -> Self
    {
        ComError::E_INVALIDARG
    }
}

impl TryFrom<Variant> for ()
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<(), Self::Error>
    {
        match src {
            Variant::None => Ok(()),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<()> for Variant
{
    fn from(_: ()) -> Self
    {
        Variant::None
    }
}

impl TryFrom<Variant> for u8
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<u8, Self::Error>
    {
        match src {
            Variant::U8(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<u8> for Variant
{
    fn from(src: u8) -> Self
    {
        Variant::U8(src)
    }
}

impl TryFrom<Variant> for i8
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<i8, Self::Error>
    {
        match src {
            Variant::I8(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<i8> for Variant
{
    fn from(src: i8) -> Self
    {
        Variant::I8(src)
    }
}

impl TryFrom<Variant> for u16
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<u16, Self::Error>
    {
        match src {
            Variant::U8(data) => Ok(data.into()),
            Variant::U16(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<u16> for Variant
{
    fn from(src: u16) -> Self
    {
        Variant::U16(src)
    }
}

impl TryFrom<Variant> for i16
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<i16, Self::Error>
    {
        match src {
            Variant::I8(data) => Ok(data.into()),
            Variant::U8(data) => Ok(data.into()),
            Variant::I16(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<i16> for Variant
{
    fn from(src: i16) -> Self
    {
        Variant::I16(src)
    }
}

impl TryFrom<Variant> for u32
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<u32, Self::Error>
    {
        match src {
            Variant::U8(data) => Ok(data.into()),
            Variant::U16(data) => Ok(data.into()),
            Variant::U32(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<u32> for Variant
{
    fn from(src: u32) -> Self
    {
        Variant::U32(src)
    }
}

impl TryFrom<Variant> for i32
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<i32, Self::Error>
    {
        match src {
            Variant::I8(data) => Ok(data.into()),
            Variant::U8(data) => Ok(data.into()),
            Variant::I16(data) => Ok(data.into()),
            Variant::U16(data) => Ok(data.into()),
            Variant::I32(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<i32> for Variant
{
    fn from(src: i32) -> Self
    {
        Variant::I32(src)
    }
}

impl TryFrom<Variant> for u64
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<u64, Self::Error>
    {
        match src {
            Variant::U8(data) => Ok(data.into()),
            Variant::U16(data) => Ok(data.into()),
            Variant::U32(data) => Ok(data.into()),
            Variant::U64(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<u64> for Variant
{
    fn from(src: u64) -> Self
    {
        Variant::U64(src)
    }
}

impl TryFrom<Variant> for i64
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<i64, Self::Error>
    {
        match src {
            Variant::I8(data) => Ok(data.into()),
            Variant::U8(data) => Ok(data.into()),
            Variant::I16(data) => Ok(data.into()),
            Variant::U16(data) => Ok(data.into()),
            Variant::I32(data) => Ok(data.into()),
            Variant::U32(data) => Ok(data.into()),
            Variant::I64(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<i64> for Variant
{
    fn from(src: i64) -> Self
    {
        Variant::I64(src)
    }
}

impl TryFrom<Variant> for f32
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<f32, Self::Error>
    {
        match src {
            Variant::I8(data) => Ok(data.into()),
            Variant::U8(data) => Ok(data.into()),
            Variant::I16(data) => Ok(data.into()),
            Variant::U16(data) => Ok(data.into()),
            Variant::F32(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<f32> for Variant
{
    fn from(src: f32) -> Self
    {
        Variant::F32(src)
    }
}

impl<T: HasInterface<dyn IUnknown>> From<ComBox<T>> for Variant
{
    fn from(src: ComBox<T>) -> Self
    {
        Variant::IUnknown(ComRc::from(src))
    }
}

impl<T: ComInterface + ?Sized> From<&ComItf<T>> for Variant
{
    fn from(src: &ComItf<T>) -> Self
    {
        let iunk: &ComItf<dyn IUnknown> = src.as_iunknown();
        Variant::IUnknown(ComRc::from(iunk))
    }
}

impl<T: ComInterface + ?Sized> From<ComRc<T>> for Variant
{
    fn from(src: ComRc<T>) -> Self
    {
        // We need to go through query_interface so objects that implement
        // multiple interfaces are handled properly (IUnknown pointers
        // must be equal for the same object no matter which interface is
        // used to acquire them).
        let iunkrc = ComItf::query_interface::<dyn IUnknown>(&src)
            .expect("All COM objects must implement IUnknown");
        Variant::IUnknown(iunkrc)
    }
}

impl TryFrom<Variant> for f64
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<f64, Self::Error>
    {
        match src {
            Variant::I8(data) => Ok(data.into()),
            Variant::U8(data) => Ok(data.into()),
            Variant::I16(data) => Ok(data.into()),
            Variant::U16(data) => Ok(data.into()),
            Variant::I32(data) => Ok(data.into()),
            Variant::U32(data) => Ok(data.into()),
            Variant::F32(data) => Ok(data.into()),
            Variant::F64(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<f64> for Variant
{
    fn from(src: f64) -> Self
    {
        Variant::F64(src)
    }
}

impl TryFrom<Variant> for bool
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<bool, Self::Error>
    {
        match src {
            Variant::Bool(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<bool> for Variant
{
    fn from(src: bool) -> Self
    {
        Variant::Bool(src)
    }
}

impl TryFrom<Variant> for SystemTime
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<SystemTime, Self::Error>
    {
        match src {
            Variant::SystemTime(data) => Ok(data),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl From<SystemTime> for Variant
{
    fn from(src: SystemTime) -> Self
    {
        Variant::SystemTime(src)
    }
}

impl TryFrom<Variant> for String
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<String, Self::Error>
    {
        match src {
            Variant::String(data) => String::try_from(data).map_err(|_| VariantError),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl TryFrom<Variant> for BString
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<BString, Self::Error>
    {
        match src {
            Variant::String(data) => BString::try_from(data).map_err(|_| VariantError),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl TryFrom<Variant> for CString
{
    type Error = VariantError;
    fn try_from(src: Variant) -> Result<CString, Self::Error>
    {
        match src {
            Variant::String(data) => CString::try_from(data).map_err(|_| VariantError),
            _ => Err(VariantError::from(&src)),
        }
    }
}

impl<T: Into<IntercomString>> From<T> for Variant
{
    fn from(src: T) -> Self
    {
        Variant::String(src.into())
    }
}

pub mod raw
{

    use super::intercom_attributes::ForeignType;
    use crate::type_system::TypeSystem;
    use std::time::{Duration, SystemTime};

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VariantBool(u16);

    impl From<VariantBool> for bool
    {
        fn from(src: VariantBool) -> bool
        {
            src.0 != 0
        }
    }

    impl From<bool> for VariantBool
    {
        fn from(src: bool) -> VariantBool
        {
            VariantBool(if src { 0xffff } else { 0 })
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VariantDate(f64);

    impl VariantDate
    {
        pub fn com_epoch() -> SystemTime
        {
            SystemTime::UNIX_EPOCH - Duration::from_secs(2_209_161_600)
        }
    }

    impl From<VariantDate> for SystemTime
    {
        fn from(src: VariantDate) -> SystemTime
        {
            let days = src.0.trunc() as i64;
            let time = src.0.fract().abs();

            let com_epoch = VariantDate::com_epoch();
            const DAY_SECONDS: u64 = 24 * 60 * 60;
            const DAY_SECONDS_F: f64 = DAY_SECONDS as f64;

            let date = if days > 0 {
                com_epoch + Duration::from_secs(days as u64 * DAY_SECONDS)
            } else {
                com_epoch - Duration::from_secs((-days) as u64 * DAY_SECONDS)
            };

            // Rust's SystemTime is accurate to 100ns in Windows as it uses the
            // system's native time format. We'll truncate the time to 100ns
            // accuracy here to reduce the differences between platforms.
            date + Duration::from_nanos((time * DAY_SECONDS_F * 10_000_000f64).trunc() as u64 * 100)
        }
    }

    impl From<SystemTime> for VariantDate
    {
        fn from(src: SystemTime) -> VariantDate
        {
            let com_epoch = VariantDate::com_epoch();
            const DAY_SECONDS: u64 = 24 * 60 * 60;
            const DAY_SECONDS_F: f64 = DAY_SECONDS as f64;

            let v = match src.duration_since(com_epoch) {
                Ok(duration) => {
                    // Proper positive duration. The scale here matches that of
                    // VariantDate so we can just turn the tics into a float
                    // and be done with it.
                    let duration_secs = duration.as_secs();
                    let duration_secs_f = duration_secs as f64 / DAY_SECONDS_F;
                    let nanos = f64::from(duration.subsec_nanos()) / 1_000_000_000f64;
                    duration_secs_f + nanos
                }
                Err(err) => {
                    // Negative duration. Here we need to consider the date/time
                    // split in the floating point number.
                    let duration = err.duration();
                    let duration_secs = duration.as_secs();
                    let duration_secs_f = duration_secs as f64 / DAY_SECONDS_F;
                    let nanos = f64::from(duration.subsec_nanos()) / 1_000_000_000f64;

                    // First of all, the current duration is positive.
                    // day -1, 0:00:00 -> 1
                    // day -1, 6:00:00 -> 0.75
                    let f = -(duration_secs_f + nanos);

                    // day -1, 0:00:00 -> -1, correct
                    // day -1, 6:00:00 -> -0.75 and should be -1.25

                    // To get the days properly, we'll floor the f. This gives
                    // us the correct days in all the cases.
                    let days = f.floor();

                    // At this point the difference f - days will be the remaining
                    // time fraction. Which we'll sub from the original days to
                    // accumulate the fraction.
                    let time = f - days;
                    days - time
                }
            };

            VariantDate(v)
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    #[allow(non_snake_case)]
    pub struct UserDefinedTypeValue
    {
        pub pvRecord: *mut std::ffi::c_void,
        pub pRecInfo: crate::raw::RawComPtr,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    #[allow(non_snake_case)]
    pub union VariantData<TS: TypeSystem>
    {
        pub llVal: i64,
        pub lVal: i32,
        pub bVal: i8,
        pub iVal: i16,
        pub fltVal: f32,
        pub dblVal: f64,
        pub boolVal: VariantBool,
        pub scode: crate::raw::HRESULT,
        pub cyVal: i64,
        pub date: VariantDate,
        pub bstrVal: *mut u16,
        pub punkVal: Option<crate::raw::InterfacePtr<TS, dyn crate::IUnknown>>,
        //*pdispVal : ComItf<IDispatch>,
        //parray : SafeArray,
        pub pbVal: *mut i8,
        pub piVal: *mut i16,
        pub plVal: *mut i32,
        pub pllVal: *mut i64,
        pub pfltVal: *mut f32,
        pub pdblVal: *mut f64,
        pub pboolVal: *mut VariantBool,
        //*pscode : SCODE,
        pub pcyVal: *mut i64,
        pub pdate: *mut VariantDate,
        pub pbstrVal: *mut *mut u16,
        pub ppunkVal: *mut Option<crate::raw::InterfacePtr<TS, dyn crate::IUnknown>>,
        //ppdispVal : *mut ComItf<IDispatch>,
        //pparray : *mut SafeArray,
        pub pvarVal: *mut Variant<TS>,
        pub byref: *mut std::os::raw::c_void,
        pub cVal: u8,
        pub uiVal: u16,
        pub ulVal: u32,
        pub ullVal: u64,
        pub intVal: i32,
        pub uintVal: u32,
        //*pdecVal : DECIMAL,
        pub pcVal: *mut u8,
        pub puiVal: *mut u16,
        pub pulVal: *mut u32,
        pub pullVal: *mut u64,
        pub pintVal: *mut i32,
        pub puintVal: *mut u32,
        pub record: UserDefinedTypeValue,
    }

    #[repr(C)]
    #[derive(Copy, Clone, ForeignType)]
    pub struct Variant<TS: TypeSystem>
    {
        pub vt: VariantType,
        reserved1: u16,
        reserved2: u16,
        reserved3: u16,
        pub data: VariantData<TS>,
    }

    impl<TS: TypeSystem> Variant<TS>
    {
        pub fn new(vt: VariantType, data: VariantData<TS>) -> Variant<TS>
        {
            Variant {
                vt,
                data,
                reserved1: 0,
                reserved2: 0,
                reserved3: 0,
            }
        }
    }

    impl<TS: TypeSystem> Default for Variant<TS>
    {
        fn default() -> Variant<TS>
        {
            Variant::new(VariantType::new(var_type::EMPTY), VariantData { lVal: 0 })
        }
    }

    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct VariantType(pub u16);

    impl VariantType
    {
        pub fn new(vt: u16) -> VariantType
        {
            VariantType(vt as u16)
        }
    }

    #[allow(unused)]
    pub mod var_type
    {
        pub const EMPTY: u16 = 0;
        pub const NULL: u16 = 1;
        pub const I2: u16 = 2;
        pub const I4: u16 = 3;
        pub const R4: u16 = 4;
        pub const R8: u16 = 5;
        pub const CY: u16 = 6;
        pub const DATE: u16 = 7;
        pub const BSTR: u16 = 8;
        pub const DISPATCH: u16 = 9;
        pub const ERROR: u16 = 10;
        pub const BOOL: u16 = 11;
        pub const VARIANT: u16 = 12;
        pub const UNKNOWN: u16 = 13;
        pub const DECIMAL: u16 = 14;
        pub const I1: u16 = 16;
        pub const UI1: u16 = 17;
        pub const UI2: u16 = 18;
        pub const UI4: u16 = 19;
        pub const I8: u16 = 20;
        pub const UI8: u16 = 21;
        pub const INT: u16 = 22;
        pub const UINT: u16 = 23;
        pub const VOID: u16 = 24;
        pub const HRESULT: u16 = 25;
        pub const PTR: u16 = 26;
        pub const SAFEARRAY: u16 = 27;
        pub const CARRAY: u16 = 28;
        pub const USERDEFINED: u16 = 29;
        pub const LPSTR: u16 = 30;
        pub const LPWSTR: u16 = 31;
        pub const RECORD: u16 = 36;
        pub const INTPTR: u16 = 37;
        pub const UINTPTR: u16 = 38;
        pub const FILETIME: u16 = 64;
        pub const BLOB: u16 = 65;
        pub const STREAM: u16 = 66;
        pub const STORAGE: u16 = 67;
        pub const STREAMEDOBJECT: u16 = 68;
        pub const STOREDOBJECT: u16 = 69;
        pub const BLOBOBJECT: u16 = 70;
        pub const CF: u16 = 71;
        pub const CLSID: u16 = 72;
        pub const VERSIONEDSTREAM: u16 = 73;
        pub const BSTRBLOB: u16 = 0xFFF;

        pub const VECTOR: u16 = 0x1000;
        pub const ARRAY: u16 = 0x2000;
        pub const BYREF: u16 = 0x4000;
        pub const RESERVED: u16 = 0x8000;
        pub const ILLEGAL: u16 = 0xffff;
        pub const ILLEGALMASKED: u16 = 0xfff;
        pub const TYPEMASK: u16 = 0xfff;
    }

    pub struct VariantError(VariantType);

    impl From<VariantError> for crate::ComError
    {
        fn from(_: VariantError) -> Self
        {
            crate::ComError::E_INVALIDARG
        }
    }

    impl<TS: TypeSystem> std::fmt::Debug for Variant<TS>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
        {
            write!(f, "Variant::Raw(type = {})", self.vt.0)
        }
    }
}

#[cfg(test)]
mod test
{

    use super::*;

    #[test]
    fn i8_to_variant()
    {
        match Variant::from(-123i8) {
            Variant::I8(-123i8) => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn i16_to_variant()
    {
        match Variant::from(-12323i16) {
            Variant::I16(-12323i16) => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn i32_to_variant()
    {
        match Variant::from(-1232323i32) {
            Variant::I32(-1232323i32) => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn i64_to_variant()
    {
        match Variant::from(-1232323i64) {
            Variant::I64(-1232323i64) => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn u8_to_variant()
    {
        match Variant::from(123u8) {
            Variant::U8(123u8) => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn u16_to_variant()
    {
        match Variant::from(12323u16) {
            Variant::U16(12323u16) => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn u32_to_variant()
    {
        match Variant::from(1232323u32) {
            Variant::U32(1232323u32) => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn u64_to_variant()
    {
        match Variant::from(1232323u64) {
            Variant::U64(1232323u64) => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn bool_to_variant()
    {
        match Variant::from(true) {
            Variant::Bool(true) => {}
            _ => panic!("Bad variant"),
        }
        match Variant::from(false) {
            Variant::Bool(false) => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn none_to_variant()
    {
        match Variant::from(()) {
            Variant::None => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn string_to_variant()
    {
        match Variant::from("input string".to_string()) {
            Variant::String(IntercomString::String(ref s)) if s == "input string" => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn bstring_to_variant()
    {
        match Variant::from(BString::from("input string")) {
            Variant::String(IntercomString::BString(ref s))
                if s.to_string().unwrap() == "input string" => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn cstring_to_variant()
    {
        match Variant::from(CString::new("input string").unwrap()) {
            Variant::String(IntercomString::CString(ref s))
                if s.to_str().unwrap() == "input string" => {}
            _ => panic!("Bad variant"),
        }
    }

    #[test]
    fn variant_to_i64()
    {
        assert_eq!(
            -100000000i64,
            i64::try_from(Variant::I64(-100000000i64)).unwrap()
        );
        assert_eq!(
            -1000000i64,
            i64::try_from(Variant::I32(-1000000i32)).unwrap()
        );
        assert_eq!(1000000i64, i64::try_from(Variant::U32(1000000u32)).unwrap());
        assert_eq!(-10000i64, i64::try_from(Variant::I16(-10000i16)).unwrap());
        assert_eq!(10000i64, i64::try_from(Variant::U16(10000u16)).unwrap());
        assert_eq!(-100i64, i64::try_from(Variant::I8(-100i8)).unwrap());
        assert_eq!(100i64, i64::try_from(Variant::U8(100u8)).unwrap());
    }

    #[test]
    fn variant_to_u64()
    {
        assert_eq!(
            100000000u64,
            u64::try_from(Variant::U64(100000000u64)).unwrap()
        );
        assert_eq!(1000000u64, u64::try_from(Variant::U32(1000000u32)).unwrap());
        assert_eq!(10000u64, u64::try_from(Variant::U16(10000u16)).unwrap());
        assert_eq!(100u64, u64::try_from(Variant::U8(100u8)).unwrap());
    }

    #[test]
    fn variant_to_i32()
    {
        assert_eq!(
            -1000000i32,
            i32::try_from(Variant::I32(-1000000i32)).unwrap()
        );
        assert_eq!(-10000i32, i32::try_from(Variant::I16(-10000i16)).unwrap());
        assert_eq!(10000i32, i32::try_from(Variant::U16(10000u16)).unwrap());
        assert_eq!(-100i32, i32::try_from(Variant::I8(-100i8)).unwrap());
        assert_eq!(100i32, i32::try_from(Variant::U8(100u8)).unwrap());
    }

    #[test]
    fn variant_to_u32()
    {
        assert_eq!(1000000u32, u32::try_from(Variant::U32(1000000u32)).unwrap());
        assert_eq!(10000u32, u32::try_from(Variant::U16(10000u16)).unwrap());
        assert_eq!(100u32, u32::try_from(Variant::U8(100u8)).unwrap());
    }

    #[test]
    fn variant_to_i16()
    {
        assert_eq!(-10000i16, i16::try_from(Variant::I16(-10000i16)).unwrap());
        assert_eq!(-100i16, i16::try_from(Variant::I8(-100i8)).unwrap());
        assert_eq!(100i16, i16::try_from(Variant::U8(100u8)).unwrap());
    }

    #[test]
    fn variant_to_u16()
    {
        assert_eq!(10000u16, u16::try_from(Variant::U16(10000u16)).unwrap());
        assert_eq!(100u16, u16::try_from(Variant::U8(100u8)).unwrap());
    }

    #[test]
    fn variant_to_i8()
    {
        assert_eq!(-100i8, i8::try_from(Variant::I8(-100i8)).unwrap());
    }

    #[test]
    fn variant_to_u8()
    {
        assert_eq!(100u8, u8::try_from(Variant::U8(100u8)).unwrap());
    }

    #[test]
    fn variant_to_f64()
    {
        assert_eq!(
            -100000000f64,
            f64::try_from(Variant::F64(-100000000f64)).unwrap()
        );
        assert_eq!(
            -1000000f64,
            f64::try_from(Variant::F32(-1000000f32)).unwrap()
        );
        assert_eq!(
            -1000000f64,
            f64::try_from(Variant::I32(-1000000i32)).unwrap()
        );
        assert_eq!(1000000f64, f64::try_from(Variant::U32(1000000u32)).unwrap());
        assert_eq!(-10000f64, f64::try_from(Variant::I16(-10000i16)).unwrap());
        assert_eq!(10000f64, f64::try_from(Variant::U16(10000u16)).unwrap());
        assert_eq!(-100f64, f64::try_from(Variant::I8(-100i8)).unwrap());
        assert_eq!(100f64, f64::try_from(Variant::U8(100u8)).unwrap());
    }

    #[test]
    fn variant_to_f32()
    {
        assert_eq!(
            -1000000f32,
            f32::try_from(Variant::F32(-1000000f32)).unwrap()
        );
        assert_eq!(-10000f32, f32::try_from(Variant::I16(-10000i16)).unwrap());
        assert_eq!(10000f32, f32::try_from(Variant::U16(10000u16)).unwrap());
        assert_eq!(-100f32, f32::try_from(Variant::I8(-100i8)).unwrap());
        assert_eq!(100f32, f32::try_from(Variant::U8(100u8)).unwrap());
    }

    #[test]
    fn variant_to_bool()
    {
        assert_eq!(true, bool::try_from(Variant::Bool(true)).unwrap());
        assert_eq!(false, bool::try_from(Variant::Bool(false)).unwrap());
    }

    #[test]
    fn variant_to_none()
    {
        assert_eq!((), <()>::try_from(Variant::None).unwrap());
    }

    #[test]
    fn variant_to_string()
    {
        assert_eq!(
            "variant value".to_string(),
            String::try_from(Variant::String(IntercomString::String(
                "variant value".to_string()
            )))
            .unwrap()
        );
    }

    #[test]
    fn variant_to_bstring()
    {
        assert_eq!(
            BString::from("variant value"),
            BString::try_from(Variant::String(IntercomString::String(
                "variant value".to_string()
            )))
            .unwrap()
        );
    }

    #[test]
    fn variant_to_cstring()
    {
        assert_eq!(
            CString::new("variant value").unwrap(),
            CString::try_from(Variant::String(IntercomString::String(
                "variant value".to_string()
            )))
            .unwrap()
        );
    }
}
