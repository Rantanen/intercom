
#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use self::windows::build;

#[cfg(not(windows))]
pub fn build() {}
