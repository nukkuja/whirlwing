use crate::win32_utils::*;
use crate::WindowsError;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::Graphics::OpenGL::*;

#[derive(Debug)]
pub struct WindowWin32 {
    pub hwnd: HWND,
    pub device_context: HDC,
    rendering_context: HGLRC,
}

impl WindowWin32 {
    pub fn new(hwnd: HWND, device_context: HDC, rendering_context: HGLRC) -> Self {
        WindowWin32 {
            hwnd,
            device_context,
            rendering_context,
        }
    }
}

impl wwg_window_internal::Window for WindowWin32 {
    type Error = crate::WindowsError;

    fn make_current(&self) -> Result<(), WindowsError> {
        wgl_make_current(self.device_context, self.rendering_context)
    }

    fn destroy(self) -> Result<(), WindowsError> {
        wgl_make_current(self.device_context, self.rendering_context)?;
        wgl_make_current_null()?;
        wgl_delete_context(self.rendering_context)?;
        destroy_window(self.hwnd)
    }
}
