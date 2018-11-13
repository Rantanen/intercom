
#[cfg(windows)]
mod os {

    #[cfg(windows)]
    #[link(name = "ole32")]
    extern "system" {
        #[doc(hidden)]
        pub fn CoInitializeEx(
            reserved : *const ::std::os::raw::c_void,
            init : u32,
        ) -> ::raw::HRESULT;

        #[doc(hidden)]
        pub fn CoUninitialize();
    }

    pub fn initialize() -> ::raw::HRESULT {
        unsafe {
            let hr = CoInitializeEx( ::std::ptr::null(), 2 /* APARTMENTTHREADED */ );
            match hr {
                ::raw::S_FALSE => ::raw::S_OK,
                other => other
            }
        }
    }

    pub fn uninitialize() {
        unsafe {
            CoUninitialize();
        }
    }
}

#[cfg(not(windows))]
mod os {
    pub fn initialize() -> ::raw::HRESULT { ::raw::S_OK }

    pub fn uninitialize() {}
}

pub fn initialize() -> ::RawComResult<()> {
    match os::initialize() {
        ::raw::S_OK => Ok( () ),
        e => Err( e )
    }
}

pub fn uninitialize() {
    os::uninitialize();
}
