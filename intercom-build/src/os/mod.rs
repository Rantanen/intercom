

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use self::windows::build;

#[cfg(not(windows))]
use crate::BuildError;

#[cfg(not(windows))]
pub fn build(_: bool) -> Result<(), BuildError> { Ok(()) }
