#[cfg(target_os = "windows")]
pub use crate::windows::handle;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use crate::unix::handle;
