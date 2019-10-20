use crate::generators::{idl, ModelOptions};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;

use std::fmt;

mod setup_configuration;

#[derive(Fail, Debug)]
struct EmbedError(String);
impl fmt::Display for EmbedError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "EmbedError: {}", self.0)
    }
}

pub fn embed_typelib(path: &Path, opts: ModelOptions) -> Result<(), failure::Error>
{
    let idl_path = Path::new("crate.idl");
    let tlb_path = Path::new("crate.tlb");
    let dll_name = "test_lib.dll";
    let manifest_path = Path::new("crate.manifest");

    let lib = crate::typelib::read_typelib(path)?;

    {
        let mut idl_file = File::create(&idl_path).unwrap();
        idl::write(lib, opts, &mut idl_file)?;
    }

    let paths = setup_configuration::get_tool_paths().map_err(EmbedError)?;

    // Turn libs and incs into environment format.
    let libs = paths
        .libs
        .iter()
        .map(|l| l.to_string_lossy())
        .collect::<Vec<_>>()
        .join(";");
    let incs = paths
        .incs
        .iter()
        .map(|l| l.to_string_lossy())
        .collect::<Vec<_>>()
        .join(";");

    let output = Command::new(paths.midl)
        .env(
            "PATH",
            format!(
                "{};{}",
                &paths.vs_bin.to_string_lossy(),
                env::var("PATH").unwrap_or_else(|_| "".to_string())
            ),
        )
        .env("LIB", libs)
        .env("INCLUDE", incs)
        .arg(idl_path)
        .arg("/tlb")
        .arg(tlb_path)
        .output()?;
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("midl fail");
    }

    let mut tlb_buffer = {
        let mut tlb_file = File::open(&tlb_path)?;
        let mut buffer = Vec::<u8>::new();
        tlb_file.read_to_end(&mut buffer)?;
        buffer
    };

    let output = Command::new(paths.mt)
        .arg(format!("-tlb:{}", tlb_path.to_string_lossy()))
        .arg(format!("-dll:{}", dll_name))
        .arg(format!("-out:{}", manifest_path.to_string_lossy()))
        .output()?;
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("mt fail");
    }

    let mut manifest_buffer = {
        let mut manifest_file = File::open(&manifest_path)?;
        let mut buffer = Vec::<u8>::new();
        manifest_file.read_to_end(&mut buffer)?;
        buffer
    };

    unsafe {
        use std::os::windows::ffi::OsStrExt;
        use winapi::shared::minwindef::FALSE;
        use winapi::um::winbase::*;
        let path_wide = path
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let update_handle = BeginUpdateResourceW(path_wide.as_slice().as_ptr(), FALSE);
        if update_handle.is_null() {
            panic!("Bad handle");
        }

        let restype = std::ffi::OsString::from("TYPELIB")
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let result = UpdateResourceW(
            update_handle,
            restype.as_ptr(),
            1 as _,
            0,
            tlb_buffer.as_mut_ptr() as _,
            tlb_buffer.len() as u32,
        );
        if result == FALSE {
            panic!("Could not update");
        }

        let result = UpdateResourceW(
            update_handle,
            24 as _,
            1 as _,
            0,
            manifest_buffer.as_mut_ptr() as _,
            manifest_buffer.len() as u32,
        );
        if result == FALSE {
            panic!("Could not update");
        }

        let result = EndUpdateResourceW(update_handle, FALSE);
        if result == FALSE {
            panic!("Could not end update");
        }

        eprintln!("Update handle: {:?}", path);
        eprintln!("Update handle: {:?}", update_handle);
    }

    Ok(())
}
