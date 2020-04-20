use crate::raw::HRESULT;
use crate::typelib::*;
use std::convert::TryInto;
use std::ffi::{c_void, CStr, CString};

type HANDLE = *mut c_void;

/// Convert the Rust identifier from `snake_case` to `PascalCase`
fn pascal_case<T: AsRef<str>>(input: T) -> String
{
    let input = input.as_ref();

    // Allocate the output string. We'll never increase the amount of
    // characters so we can reserve string buffer using the input string length.
    let mut output = String::new();
    output.reserve(input.len());

    // Process each character from the input.
    let mut capitalize = true;
    for c in input.chars() {
        // Check the capitalization requirement.
        if c == '_' {
            // Skip '_' but capitalize the following character.
            capitalize = true;
        } else if capitalize {
            // Capitalize. Add the uppercase characters.
            for c_up in c.to_uppercase() {
                output.push(c_up)
            }

            // No need to capitalize any more.
            capitalize = false;
        } else {
            // No need to capitalize. Just add the character as is.
            output.push(c);
        }
    }
    output
}

#[link(name = "ole32")]
extern "system" {

    /// Opens/creates registry keys.
    pub fn RegCreateKeyExA(
        hKey: HANDLE,
        sub_key_path: *const i8,
        reserved: u32,
        class: *mut u8,
        options: u32,
        samDesired: u32,
        lpSecurityAttributes: *mut c_void,
        phkResult: *mut HANDLE,
        lpdwDisposition: *mut u32,
    ) -> HRESULT;

    /// Closes an open registry key.
    pub fn RegCloseKey(hKey: HANDLE) -> HRESULT;

    /// Sets a value under a registry key.
    pub fn RegSetValueExA(
        hKey: HANDLE,
        lpValueName: *const i8,
        reserved: u32,
        dwType: u32,
        lpData: *const u8,
        cbData: u32,
    ) -> HRESULT;

    /// Deletes a registry key.
    pub fn RegDeleteKeyA(hKey: HANDLE, lpSubKey: *const i8) -> HRESULT;
}

#[link(name = "kernel32")]
extern "system" {
    /// Resolves a module file name based on module handle.
    ///
    /// The current module handle is received through DllMain.
    pub fn GetModuleFileNameA(hModule: HANDLE, lpFilename: *mut u8, nSize: u32) -> u32;
}

/// A safe wrapper around the Windows registry functions.
struct Key(HANDLE);
const CLASSES_ROOT: Key = Key(0x80000000 as HANDLE);

impl Key
{
    /// Opens a sub-key.
    pub fn open_key(&self, path: &str) -> Result<Key, HRESULT>
    {
        let mut result: HANDLE = ::std::ptr::null_mut();
        let mut disposition: u32 = 0;
        let hr = unsafe {
            RegCreateKeyExA(
                self.0,
                CString::new(path)
                    .map_err(|_| crate::raw::E_INVALIDARG)?
                    .as_ptr(),
                0,
                ::std::ptr::null_mut(),
                0,
                2, // KEY_SET_VALUE
                ::std::ptr::null_mut(),
                &mut result,
                &mut disposition,
            )
        };

        match hr.is_success() {
            true => Ok(Key(result)),
            false => Err(hr),
        }
    }

    /// Deletes a sub-key.
    pub fn delete_key(&self, path: &str) -> Result<(), HRESULT>
    {
        let hr = unsafe {
            RegDeleteKeyA(
                self.0,
                CString::new(path)
                    .map_err(|_| crate::raw::E_INVALIDARG)?
                    .as_ptr(),
            )
        };

        match hr.is_success() {
            true => Ok(()),
            false => Err(hr),
        }
    }

    pub fn set_string_value(&self, name: &str, value: &str) -> Result<(), HRESULT>
    {
        let data = CString::new(value).map_err(|_| crate::raw::E_INVALIDARG)?;
        let hr = unsafe {
            RegSetValueExA(
                self.0,
                CString::new(name)
                    .map_err(|_| crate::raw::E_INVALIDARG)?
                    .as_ptr(),
                0,
                1, // REG_SZ
                data.as_ptr() as *const _,
                value
                    .len()
                    .try_into()
                    .map_err(|_| crate::raw::E_INVALIDARG)?,
            )
        };

        match hr.is_success() {
            true => Ok(()),
            false => Err(hr),
        }
    }
}

impl Drop for Key
{
    /// Closes the key handle.
    fn drop(&mut self)
    {
        unsafe {
            RegCloseKey(self.0);
        }
    }
}

/// Registers a type library.
pub fn register(dll: HANDLE, lib: TypeLib) -> Result<(), HRESULT>
{
    register_or_unregister(dll, lib, true)
}

/// Unregisters a type library.
pub fn unregister(dll: HANDLE, lib: TypeLib) -> Result<(), HRESULT>
{
    register_or_unregister(dll, lib, false)
}

fn register_or_unregister(dll: HANDLE, lib: TypeLib, do_register: bool) -> Result<(), HRESULT>
{
    // Format the typelib name and version according to usual COM convention.
    let lib_name = pascal_case(&lib.name);
    let lib_version = lib.version.replace(".", "_");
    let path = get_module_path(dll)?;

    register_typelib(&path, &lib, &lib_name, do_register)?;

    for cls in lib.types.iter().filter_map(|t| match t {
        TypeInfo::Class(cls) => Some(cls),
        _ => None,
    }) {
        register_class(&path, &lib, &lib_name, &lib_version, &cls, do_register)?;
    }

    Ok(())
}

// Function for getting runtime output. regsvr32 is a non-console process so
// stdout is lost.
/*
fn output(msg: &str)
{
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("output.txt")
        .unwrap();
    writeln!(f, "{}", msg).unwrap();
    f.flush().unwrap();
}
*/

pub fn register_typelib(
    path: &str,
    lib: &TypeLib,
    lib_name: &str,
    do_register: bool,
) -> Result<(), HRESULT>
{
    let description = format!("{} TypeLib", lib_name);

    #[cfg(target_arch = "x86")]
    let arch = "win32";
    #[cfg(target_arch = "x86_64")]
    let arch = "win64";

    let data = vec![
        (format!("TypeLib\\{}", lib.libid), "", description),
        (
            format!("TypeLib\\{}\\{}", lib.libid, lib.version),
            "",
            format!("{} {}", lib_name, lib.version),
        ),
        (
            format!("TypeLib\\{}\\{}\\0", lib.libid, lib.version),
            "",
            String::new(),
        ),
        (
            format!("TypeLib\\{}\\{}\\0\\{}", lib.libid, lib.version, arch),
            "",
            path.to_string(),
        ),
        (
            format!("TypeLib\\{}\\{}\\FLAGS", lib.libid, lib.version),
            "",
            "0".to_string(),
        ),
    ];

    // Iterate in reverse to ensure the sub-keys are deleted before the parent
    // keys. Otherwise the parent keys won't get deleted.
    for d in data.iter().rev() {
        if do_register {
            let key = CLASSES_ROOT.open_key(&d.0)?;
            if d.2 != "" {
                key.set_string_value(d.1, &d.2)?;
            }
        } else if d.1 == "" {
            // Delete key.
            // Only process default-values so we won't delete keys multiple times.
            CLASSES_ROOT.delete_key(&d.0)?;
        }
    }

    Ok(())
}
pub fn register_class(
    path: &str,
    lib: &TypeLib,
    lib_name: &str,
    lib_version: &str,
    cls: &CoClass,
    do_register: bool,
) -> Result<(), HRESULT>
{
    let latest = format!("{}.{}", lib_name, cls.name);
    let curver = format!("{}.{}.{}", lib_name, cls.name, lib_version);
    let description = format!("{} {} Class", lib_name, cls.name);

    let data = vec![
        (latest.clone(), "", description.clone()),
        (curver.clone(), "", description.clone()),
        (format!("{}\\CLSID", curver), "", cls.clsid.to_string()),
        (format!("{}\\CurVer", latest), "", curver.clone()),
        (format!("CLSID\\{}", cls.clsid), "", description),
        (
            format!("CLSID\\{}\\InprocServer32", cls.clsid),
            "",
            path.to_string(),
        ),
        (
            format!("CLSID\\{}\\InprocServer32", cls.clsid),
            "ThreadingModel",
            "Both".to_string(),
        ),
        (format!("CLSID\\{}\\ProgID", cls.clsid), "", curver),
        (
            format!("CLSID\\{}\\TypeLib", cls.clsid),
            "",
            lib.libid.to_string(),
        ),
        (
            format!("CLSID\\{}\\VersionIndependentProgID", cls.clsid),
            "",
            latest,
        ),
    ];

    // Iterate in reverse to ensure the sub-keys are deleted before the parent
    // keys. Otherwise the parent keys won't get deleted.
    for d in data.iter().rev() {
        if do_register {
            let key = CLASSES_ROOT.open_key(&d.0)?;
            key.set_string_value(d.1, &d.2)?;
        } else if d.1 == "" {
            // Delete key.
            // Only process default-values so we won't delete keys multiple times.
            CLASSES_ROOT.delete_key(&d.0)?;
        }
    }

    Ok(())
}

fn get_module_path(dll_handle: HANDLE) -> Result<String, HRESULT>
{
    Ok(unsafe {
        let mut path = Vec::new();
        path.reserve(1024);
        let path_len = GetModuleFileNameA(
            dll_handle,
            path.as_mut_ptr(),
            path.capacity()
                .try_into()
                .map_err(|_| intercom::raw::E_INVALIDARG)?,
        );

        let path_len = path_len as usize;
        if path_len == 0 || path_len >= path.capacity() {
            return Err(intercom::raw::E_FAIL);
        }
        path.set_len(path_len + 1);
        CStr::from_bytes_with_nul(&path)
            .map_err(|_| intercom::raw::E_FAIL)?
            .to_owned()
    }
    .to_str()
    .map_err(|_| intercom::raw::E_FAIL)?
    .to_string())
}
