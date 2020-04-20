#![crate_type = "dylib"]

extern crate intercom;
extern crate winapi;
use intercom::prelude::*;
use std::convert::TryInto;
use winapi::shared::{minwindef::DWORD, ntdef::ULARGE_INTEGER, windef::HBITMAP};
use winapi::um::{
    objidlbase::STATSTG,
    wingdi::{CreateBitmap, DeleteObject},
};

com_library! {
    on_load=on_load,
    class ThumbnailProvider
}

/// Called when the DLL is loaded.
///
/// Sets up logging to the Cargo.toml directory for debug purposes.
fn on_load()
{
    // Set up logging to the proejct directory.
    use log::LevelFilter;
    simple_logging::log_to_file(
        &format!("{}\\debug.log", env!("CARGO_MANIFEST_DIR")),
        LevelFilter::Trace,
    )
    .unwrap();
}

#[com_class(IInitializeWithStream, IThumbnailProvider)]
#[derive(Default)]
struct ThumbnailProvider
{
    bitmap: std::cell::Cell<Option<HBITMAP>>,
}

impl IInitializeWithStream for ThumbnailProvider
{
    fn initialize(&self, _stream: &ComItf<dyn IStream>, _mode: DWORD) -> ComResult<()>
    {
        // This sample always returns the same thumbnail so we don't need to read
        // the stream itself.
        Ok(())
    }
}

impl IThumbnailProvider for ThumbnailProvider
{
    fn get_thumbnail(&self, cx: u32) -> ComResult<(ComHBITMAP, WTS_ALPHATYPE)>
    {
        // Render a faded circle as an example thumbnail.
        let mut data = Vec::new();
        data.resize((cx * cx) as usize, 0);
        let midpoint = cx as f64 / 2.0;
        for x in 0..cx {
            for y in 0..cx {
                let x_coord = x as f64;
                let y_coord = y as f64;
                let dist_squared = (midpoint - x_coord).powf(2.0) + (midpoint - y_coord).powf(2.0);
                let mut value = (dist_squared.sqrt() / midpoint) * 255.0;
                if value > 255.0 {
                    value = 255.0
                }
                let value = value as u32;
                data[(x * cx + y) as usize] = value + (value << 8) + (value << 16) + (value << 24);
            }
        }

        // Create a bitmap from the data and return it.
        //
        // We'll store the bitmap handle in the struct so it can destroy the data when it's
        // not needed anymore.
        let icx: i32 = cx.try_into().map_err(|_| ComError::E_INVALIDARG)?;
        let bitmap = unsafe { CreateBitmap(icx, icx, 1, 32, data.as_ptr() as *const _) };
        self.bitmap.set(Some(bitmap));
        Ok((ComHBITMAP(bitmap), 0))
    }
}

impl Drop for ThumbnailProvider
{
    fn drop(&mut self)
    {
        // Delete the bitmap once it's not needed anymore.
        if let Some(bitmap) = self.bitmap.get() {
            unsafe { DeleteObject(bitmap as _) };
        }
    }
}

// New types for deriving Intercom traits.

#[derive(intercom::ForeignType, intercom::ExternOutput)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
struct ComHBITMAP(HBITMAP);

#[derive(intercom::ForeignType, intercom::ExternOutput, intercom::ExternInput)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
struct ComULARGE_INTEGER(ULARGE_INTEGER);

#[derive(intercom::ForeignType, intercom::ExternOutput, intercom::ExternInput)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
struct ComSTATSTG(STATSTG);

#[allow(non_camel_case_types)]
type WTS_ALPHATYPE = u32;

// COM interface definitions.

#[com_interface(com_iid = "e357fccd-a995-4576-b01f-234630154e96")]
trait IThumbnailProvider
{
    fn get_thumbnail(&self, cx: u32) -> ComResult<(ComHBITMAP, WTS_ALPHATYPE)>;
}

// The IStream definition is untested since this provider doesn't initialize itself
// from the stream.
#[com_interface(com_iid = "0000000c-0000-0000-C000-000000000046")]
trait IStream
{
    fn clone(&self) -> ComResult<ComRc<dyn IStream>>;
    fn commit(&self, commit_flags: DWORD) -> ComResult<()>;
    fn copy_to(
        &self,
        target: &ComItf<dyn IStream>,
        count: ComULARGE_INTEGER,
    ) -> ComResult<(ComULARGE_INTEGER, ComULARGE_INTEGER)>;
    fn lock_region(
        &self,
        lock_offset: ComULARGE_INTEGER,
        count: ComULARGE_INTEGER,
        lock_type: DWORD,
    ) -> ComResult<()>;
    fn revert(&self) -> ComResult<()>;
    fn seek(&self, move_to: ComULARGE_INTEGER, origin: DWORD) -> ComResult<ComULARGE_INTEGER>;
    fn set_size(&self, new_size: ComULARGE_INTEGER) -> ComResult<()>;
    fn stat(&self, stat: *mut ComSTATSTG, stat_flag: DWORD) -> ComResult<()>;
    fn unlock_region(
        &self,
        lock_offset: ComULARGE_INTEGER,
        count: ComULARGE_INTEGER,
        lock_type: DWORD,
    ) -> ComResult<()>;
}

#[com_interface(com_iid = "b824b49d-22ac-4161-ac8a-9916e8fa3f7f")]
trait IInitializeWithStream
{
    fn initialize(&self, stream: &ComItf<dyn IStream>, mode: DWORD) -> ComResult<()>;
}
