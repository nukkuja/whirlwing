macro_rules! load_wgl_extension {
    ($name_str:literal into $name:ident($($args:ty),*)) => {
        let function_holder = $crate::win32_utils::wgl_get_proc_address($crate::win32_windowing_context::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*)>
        (function_holder) };
    };

    ($name_str:literal into $name:ident($($args:ty),*) -> $ret:ty) => {
        let function_holder = $crate::win32_utils::wgl_get_proc_address($crate::win32_windowing_context::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*) -> $ret>
        (function_holder) };
    };

        ($name_str:literal into $name:ident($($args:ty),*,)) => {
        let function_holder = $crate::win32_utils::wgl_get_proc_address($crate::win32_windowing_context::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*)>
        (function_holder) };
    };

    ($name_str:literal into $name:ident($($args:ty),*,) -> $ret:ty) => {
        let function_holder = $crate::win32_utils::wgl_get_proc_address($crate::win32_windowing_context::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*) -> $ret>
        (function_holder) };
    };
}

#[allow(unused_imports)]
use windows::{
    core::{PCSTR, PCWSTR},
    s, w,
    Win32::{
        Foundation::*,
        Graphics::{Gdi::*, OpenGL::*},
        System::LibraryLoader::*,
        UI::WindowsAndMessaging::*,
    },
};

use crate::win32_error::*;
use crate::win32_utils::GL_TRUE;
use crate::win32_utils::*;
use crate::win32_window::WindowWin32;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct WindowingContextWin32 {
    hinstance: HMODULE,
    fake_wnd_class_name: PCWSTR,
    fake_wnd: HWND,
    fake_dc: HDC,
    fake_rc: HGLRC,

    wnd_class_name: PCWSTR,

    wglChoosePixelFormatARB:
        unsafe extern "system" fn(HDC, *const i32, *const f32, u32, *mut i32, *mut u32) -> BOOL,
    wglCreateContextAttribsARB: unsafe extern "system" fn(HDC, HGLRC, *const i32) -> HGLRC,
}

impl wwg_window_internal::WindowingContext for WindowingContextWin32 {
    type Window = WindowWin32;
    type Error = WindowsError;

    fn init() -> Result<Self, WindowsError> {
        let hinstance = get_instance_handle();
        let fake_wnd_class_name = w!("Fake window class");

        let fake_wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_OWNDC,
            lpfnWndProc: Some(fake_wnd_proc),
            hInstance: hinstance,
            lpszClassName: fake_wnd_class_name,
            ..Default::default()
        };

        register_window_class(fake_wnd_class)?;

        let fake_wnd = create_window(
            WINDOW_EX_STYLE::default(),
            fake_wnd_class_name,
            w!("Fake Window"),
            WINDOW_STYLE::default(),
            0,
            0,
            200,
            200,
            None,
            None,
            hinstance,
            None,
        )?;

        let fake_wnd_dc = get_device_context(fake_wnd)?;
        wwg_log::wwg_trace!("Device context of fake window retrieved!");

        let pfd = PIXELFORMATDESCRIPTOR {
            nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 32,
            cDepthBits: 24,
            cStencilBits: 8,
            iLayerType: PFD_MAIN_PLANE,
            ..Default::default()
        };
        let fake_wnd_pixel_format: i32 = choose_pixel_format(fake_wnd_dc, &pfd)?;
        set_pixel_format(fake_wnd_dc, fake_wnd_pixel_format, &pfd)?;

        let fake_wgl_context = wgl_create_context(fake_wnd_dc)?;
        wgl_make_current(fake_wnd_dc, fake_wgl_context)?;
        load_gl_functions()?;

        load_wgl_extension!("wglChoosePixelFormatARB" into
        wglChoosePixelFormatARB(HDC, *const i32, *const f32, u32, *mut i32, *mut u32) -> BOOL);
        load_wgl_extension!("wglCreateContextAttribsARB" into
        wglCreateContextAttribsARB(HDC, HGLRC, *const i32) -> HGLRC);

        let class_name = w!("Whirlwind Window Class");
        let wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_OWNDC,
            lpfnWndProc: Some(fake_wnd_proc),
            hInstance: hinstance,
            hIcon: HICON(0),
            hCursor: load_cursor_w(IDC_ARROW)?,
            lpszClassName: class_name,
            ..Default::default()
        };

        register_window_class(wnd_class)?;

        Ok(WindowingContextWin32 {
            hinstance: hinstance,
            fake_wnd_class_name: fake_wnd_class_name,
            fake_wnd: fake_wnd,
            fake_dc: fake_wnd_dc,
            fake_rc: fake_wgl_context,
            wnd_class_name: class_name,
            wglChoosePixelFormatARB,
            wglCreateContextAttribsARB,
        })
    }

    fn create_window(
        &self,
        title: &str,
        pos_x: u32,
        pos_y: u32,
        width: u32,
        height: u32,
    ) -> Result<WindowWin32, WindowsError> {
        let mut title = title.encode_utf16().collect::<Vec<u16>>();
        title.push(0);
        let title = PCWSTR(title.as_ptr());

        let hwnd = create_window(
            WS_EX_LEFT,
            self.wnd_class_name,
            title,
            WS_OVERLAPPED | WS_CAPTION | WS_MINIMIZEBOX | WS_SYSMENU,
            pos_x as i32,
            pos_y as i32,
            width as i32,
            height as i32,
            HWND(0),
            HMENU(0),
            self.hinstance,
            None,
        )?;

        let dc = get_device_context(hwnd)?;

        let pfd = PIXELFORMATDESCRIPTOR {
            nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 32,
            cDepthBits: 24,
            cStencilBits: 8,
            iLayerType: PFD_MAIN_PLANE,
            ..Default::default()
        };
        let attrib_list = [
            WGL_DRAW_TO_WINDOW_ARB,
            GL_TRUE,
            WGL_SUPPORT_OPENGL_ARB,
            GL_TRUE,
            WGL_DOUBLE_BUFFER_ARB,
            GL_TRUE,
            WGL_PIXEL_TYPE_ARB,
            WGL_TYPE_RGBA_ARB,
            WGL_COLOR_BITS_ARB,
            32,
            WGL_DEPTH_BITS_ARB,
            24,
            WGL_STENCIL_BITS_ARB,
            8,
            0,
        ];
        let mut pixel_format = 0;
        let mut num_formats = 0u32;

        let result = unsafe {
            (self.wglChoosePixelFormatARB)(
                dc,
                attrib_list.as_ptr(),
                std::ptr::null(),
                1,
                &mut pixel_format,
                &mut num_formats,
            )
        };
        if result == FALSE {
            return Err(WindowsError {
                err_type: WindowsErrorType::WGLExtensionLoadError,
                err_code: None,
                err_body: "Failed to choose WGL pixel format.".to_string(),
            });
        };
        set_pixel_format(dc, pixel_format, &pfd)?;

        let attribs_list = [
            WGL_CONTEXT_MAJOR_VERSION_ARB,
            3,
            WGL_CONTEXT_MINOR_VERSION_ARB,
            3,
            WGL_CONTEXT_PROFILE_MASK_ARB,
            WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
        ];

        let hglrc =
            unsafe { (self.wglCreateContextAttribsARB)(dc, HGLRC(0), attribs_list.as_ptr()) };
        if hglrc.0 == 0 {
            let err_code = unsafe { GetLastError() };
            return Err(WindowsError {
                err_type: WindowsErrorType::WGLContextCreationErrorARB,
                err_code: Some(Win32ErrorCode(err_code.0)),
                err_body: "Failed to create WGL ARB context.".to_string(),
            });
        }

        unsafe { ShowWindow(hwnd, SW_SHOW); }
        Ok(WindowWin32::new(hwnd, dc, hglrc))
    }

    fn destroy_context(&mut self) -> Result<(), Self::Error> {
        unregister_window_class(self.wnd_class_name, self.hinstance)?;

        wgl_make_current(self.fake_dc, self.fake_rc)?;
        wgl_make_current_null()?;
        wgl_delete_context(self.fake_rc)?;
        destroy_window(self.fake_wnd)?;
        unregister_window_class(self.fake_wnd_class_name, self.hinstance)?;

        Ok(())
    }
}

unsafe extern "system" fn fake_wnd_proc(
    h_wnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    DefWindowProcW(h_wnd, msg, w_param, l_param)
}
