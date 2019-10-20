#[cfg(windows)]
mod os
{

    #[cfg(windows)]
    #[link(name = "ole32")]
    extern "system" {
        #[doc(hidden)]
        pub fn CoInitializeEx(
            reserved: *const ::std::os::raw::c_void,
            init: u32,
        ) -> crate::raw::HRESULT;

        #[doc(hidden)]
        pub fn CoUninitialize();
    }

    pub fn initialize() -> crate::raw::HRESULT
    {
        unsafe {
            let hr = CoInitializeEx(::std::ptr::null(), 2 /* APARTMENTTHREADED */);
            match hr {
                crate::raw::S_FALSE => crate::raw::S_OK,
                other => other,
            }
        }
    }

    pub fn uninitialize()
    {
        unsafe {
            CoUninitialize();
        }
    }
}

#[cfg(not(windows))]
mod os
{
    pub fn initialize() -> crate::raw::HRESULT
    {
        crate::raw::S_OK
    }

    pub fn uninitialize() {}
}

pub fn initialize() -> crate::RawComResult<()>
{
    match os::initialize() {
        crate::raw::S_OK => Ok(()),
        e => Err(e),
    }
}

pub fn uninitialize()
{
    os::uninitialize();
}
