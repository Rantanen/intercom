use intercom::prelude::*;

com_module! {
    class DoCallback,
    class NullableTests,
}

#[com_interface]
pub trait ICallback
{
    fn callback(&self) -> u32;
}

#[com_class(ICallback)]
#[derive(Default)]
struct DoCallback(u32);

impl ICallback for DoCallback
{
    fn callback(&self) -> u32
    {
        self.0
    }
}

#[com_interface]
pub trait INullableInterface
{
    fn nullable_parameter(&self, itf: Option<&ComItf<dyn ICallback>>) -> u32;

    fn nonnull_parameter(&self, itf: &ComItf<dyn ICallback>) -> ComResult<u32>;

    fn nullable_output(&self, value: u32) -> ComResult<Option<ComRc<dyn ICallback>>>;

    fn nonnull_output(&self, value: u32) -> ComResult<ComRc<dyn ICallback>>;
}

#[com_class(Self, INullableInterface)]
#[derive(Default)]
pub struct NullableTests;

#[com_interface]
impl NullableTests
{
    fn nullable_parameter(&self, v: u32, itf: &ComItf<dyn INullableInterface>) -> ComResult<()>
    {
        let opt = if v == 0 {
            None
        } else {
            Some(ComRc::from(ComBox::new(DoCallback(v))))
        };

        if itf.nullable_parameter(opt.as_deref()) != v {
            Err(ComError::E_FAIL)
        } else {
            Ok(())
        }
    }

    fn nonnull_parameter(&self, v: u32, itf: &ComItf<dyn INullableInterface>) -> ComResult<()>
    {
        let rc = ComRc::from(ComBox::new(DoCallback(v)));

        if itf.nonnull_parameter(&*rc)? != v {
            Err(ComError::E_FAIL)
        } else {
            Ok(())
        }
    }

    fn nullable_output(&self, value: u32, itf: &ComItf<dyn INullableInterface>) -> ComResult<()>
    {
        let cb = itf.nullable_output(value)?;
        match cb {
            None => {
                if value != 0 {
                    Err(ComError::E_INVALIDARG)
                } else {
                    Ok(())
                }
            }
            Some(cb) => {
                if value != 0 && cb.callback() == value {
                    Ok(())
                } else {
                    Err(ComError::E_FAIL)
                }
            }
        }
    }

    fn nonnull_output(&self, value: u32, itf: &ComItf<dyn INullableInterface>) -> ComResult<()>
    {
        let cb = itf.nonnull_output(value)?;
        match cb.callback() == value {
            true => Ok(()),
            false => Err(ComError::E_FAIL),
        }
    }
}

impl INullableInterface for NullableTests
{
    fn nullable_parameter(&self, itf: Option<&ComItf<dyn ICallback>>) -> u32
    {
        if let Some(itf) = itf {
            itf.callback()
        } else {
            0
        }
    }

    fn nonnull_parameter(&self, itf: &ComItf<dyn ICallback>) -> ComResult<u32>
    {
        Ok(itf.callback())
    }

    fn nullable_output(&self, value: u32) -> ComResult<Option<ComRc<dyn ICallback>>>
    {
        match value {
            0 => Ok(None),
            v => Ok(Some(ComRc::from(ComBox::new(DoCallback(v))))),
        }
    }

    fn nonnull_output(&self, value: u32) -> ComResult<ComRc<dyn ICallback>>
    {
        Ok(ComRc::from(ComBox::new(DoCallback(value))))
    }
}
