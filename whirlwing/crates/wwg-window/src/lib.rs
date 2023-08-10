#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct WindowID(pub u32);

pub use wwg_window_internal::*;

#[cfg(target_os = "windows")]
pub use wwg_win32::{WindowWin32, WindowingContextWin32, WindowsError};
