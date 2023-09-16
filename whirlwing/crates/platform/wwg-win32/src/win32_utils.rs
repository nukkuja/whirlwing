#![allow(dead_code)]

pub(crate) const WGL_NUMBER_PIXEL_FORMATS_ARB: i32 = 0x2000;
pub(crate) const WGL_DRAW_TO_WINDOW_ARB: i32 = 0x2001;
pub(crate) const WGL_DRAW_TO_BITMAP_ARB: i32 = 0x2002;
pub(crate) const WGL_ACCELERATION_ARB: i32 = 0x2003;
pub(crate) const WGL_NEED_PALETTE_ARB: i32 = 0x2004;
pub(crate) const WGL_NEED_SYSTEM_PALETTE_ARB: i32 = 0x2005;
pub(crate) const WGL_SWAP_LAYER_BUFFERS_ARB: i32 = 0x2006;
pub(crate) const WGL_SWAP_METHOD_ARB: i32 = 0x2007;
pub(crate) const WGL_NUMBER_OVERLAYS_ARB: i32 = 0x2008;
pub(crate) const WGL_NUMBER_UNDERLAYS_ARB: i32 = 0x2009;
pub(crate) const WGL_TRANSPARENT_ARB: i32 = 0x200A;
pub(crate) const WGL_TRANSPARENT_RED_VALUE_ARB: i32 = 0x2037;
pub(crate) const WGL_TRANSPARENT_GREEN_VALUE_ARB: i32 = 0x2038;
pub(crate) const WGL_TRANSPARENT_BLUE_VALUE_ARB: i32 = 0x2039;
pub(crate) const WGL_TRANSPARENT_ALPHA_VALUE_ARB: i32 = 0x203A;
pub(crate) const WGL_TRANSPARENT_INDEX_VALUE_ARB: i32 = 0x203B;
pub(crate) const WGL_SHARE_DEPTH_ARB: i32 = 0x200C;
pub(crate) const WGL_SHARE_STENCIL_ARB: i32 = 0x200D;
pub(crate) const WGL_SHARE_ACCUM_ARB: i32 = 0x200E;
pub(crate) const WGL_SUPPORT_GDI_ARB: i32 = 0x200F;
pub(crate) const WGL_SUPPORT_OPENGL_ARB: i32 = 0x2010;
pub(crate) const WGL_DOUBLE_BUFFER_ARB: i32 = 0x2011;
pub(crate) const WGL_STEREO_ARB: i32 = 0x2012;
pub(crate) const WGL_PIXEL_TYPE_ARB: i32 = 0x2013;
pub(crate) const WGL_COLOR_BITS_ARB: i32 = 0x2014;
pub(crate) const WGL_RED_BITS_ARB: i32 = 0x2015;
pub(crate) const WGL_RED_SHIFT_ARB: i32 = 0x2016;
pub(crate) const WGL_GREEN_BITS_ARB: i32 = 0x2017;
pub(crate) const WGL_GREEN_SHIFT_ARB: i32 = 0x2018;
pub(crate) const WGL_BLUE_BITS_ARB: i32 = 0x2019;
pub(crate) const WGL_BLUE_SHIFT_ARB: i32 = 0x201A;
pub(crate) const WGL_ALPHA_BITS_ARB: i32 = 0x201B;
pub(crate) const WGL_ALPHA_SHIFT_ARB: i32 = 0x201C;
pub(crate) const WGL_ACCUM_BITS_ARB: i32 = 0x201D;
pub(crate) const WGL_ACCUM_RED_BITS_ARB: i32 = 0x201E;
pub(crate) const WGL_ACCUM_GREEN_BITS_ARB: i32 = 0x201F;
pub(crate) const WGL_ACCUM_BLUE_BITS_ARB: i32 = 0x2020;
pub(crate) const WGL_ACCUM_ALPHA_BITS_ARB: i32 = 0x2021;
pub(crate) const WGL_DEPTH_BITS_ARB: i32 = 0x2022;
pub(crate) const WGL_STENCIL_BITS_ARB: i32 = 0x2023;
pub(crate) const WGL_AUX_BUFFERS_ARB: i32 = 0x2024;

pub(crate) const WGL_NO_ACCELERATION_ARB: i32 = 0x2025;
pub(crate) const WGL_GENERIC_ACCELERATION_ARB: i32 = 0x2026;
pub(crate) const WGL_FULL_ACCELERATION_ARB: i32 = 0x2027;

pub(crate) const WGL_SWAP_EXCHANGE_ARB: i32 = 0x2028;
pub(crate) const WGL_SWAP_COPY_ARB: i32 = 0x2029;
pub(crate) const WGL_SWAP_UNDEFINED_ARB: i32 = 0x202A;

pub(crate) const WGL_TYPE_RGBA_ARB: i32 = 0x202B;
pub(crate) const WGL_TYPE_COLORINDEX_ARB: i32 = 0x202C;

pub(crate) const WGL_CONTEXT_MAJOR_VERSION_ARB: i32 = 0x2091;
pub(crate) const WGL_CONTEXT_MINOR_VERSION_ARB: i32 = 0x2092;
pub(crate) const WGL_CONTEXT_LAYER_PLANE_ARB: i32 = 0x2093;
pub(crate) const WGL_CONTEXT_FLAGS_ARB: i32 = 0x2094;
pub(crate) const WGL_CONTEXT_PROFILE_MASK_ARB: i32 = 0x9126;

pub(crate) const WGL_CONTEXT_DEBUG_BIT_ARB: i32 = 0x0001;
pub(crate) const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: i32 = 0x0002;

pub(crate) const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: i32 = 0x00000001;
pub(crate) const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: i32 = 0x00000002;

pub(crate) const GL_TRUE: i32 = 1;
pub(crate) const GL_FALSE: i32 = 0;

#[allow(unused_imports)]
use windows::{
    core::{IntoParam, PCSTR, PCWSTR},
    s, w,
    Win32::{
        Foundation::*,
        Graphics::{Gdi::*, OpenGL::*},
        System::LibraryLoader::*,
        UI::WindowsAndMessaging::*,
    },
};

use crate::win32_error::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn create_window<
    P0: IntoParam<PCWSTR>,
    P1: IntoParam<PCWSTR>,
    P2: IntoParam<HWND>,
    P3: IntoParam<HMENU>,
    P4: IntoParam<HMODULE>,
>(
    dwexstyle: WINDOW_EX_STYLE,
    lpclassname: P0,
    lpwindowname: P1,
    dwstyle: WINDOW_STYLE,
    pos_x: i32,
    pos_y: i32,
    nwidth: i32,
    nheight: i32,
    hwndparent: P2,
    hmenu: P3,
    hinstance: P4,
    lpparam: Option<*const std::ffi::c_void>,
) -> Result<HWND, WindowsError> {
    let mut wnd_rect = RECT {
        left: 0,
        right: nwidth,
        top: 0,
        bottom: nheight,
    };
    let res = unsafe { AdjustWindowRect(&mut wnd_rect, dwstyle, None) };
    if res == false {
        let err_code = unsafe { GetLastError() };
        return Err(WindowsError {
            err_type: WindowsErrorType::AdjustWindowRectError,
            err_code: Some(Win32ErrorCode(err_code.0)),
            err_body: "Failed to adjust window size while creating the window.".to_string(),
        });
    }

    let h_wnd = unsafe {
        CreateWindowExW(
            dwexstyle,
            lpclassname,
            lpwindowname,
            dwstyle,
            pos_x,
            pos_y,
            wnd_rect.right - wnd_rect.left,
            wnd_rect.bottom - wnd_rect.top,
            hwndparent,
            hmenu,
            hinstance,
            lpparam,
        )
    };
    if h_wnd.0 == 0 {
        let err_code = unsafe { GetLastError() };
        return Err(WindowsError {
            err_type: WindowsErrorType::WindowCreationError,
            err_code: Some(Win32ErrorCode(err_code.0)),
            err_body: "Failed to create the window.".to_string(),
        });
    }
    Ok(h_wnd)
}

pub(crate) fn get_instance_handle() -> HMODULE {
    unsafe { GetModuleHandleW(None).unwrap() }
}

pub(crate) fn get_device_context(h_wnd: HWND) -> Result<HDC, WindowsError> {
    let dc = unsafe { GetDC(h_wnd) };
    if dc.0 == 0 {
        Err(WindowsError {
            err_type: WindowsErrorType::DeviceContextRetrievalError,
            err_code: None,
            err_body: "Failed to get device context for window.".to_string(),
        })
    } else {
        Ok(dc)
    }
}

pub(crate) fn register_window_class(class: WNDCLASSEXW) -> Result<u16, WindowsError> {
    let atom = unsafe { RegisterClassExW(&class) };
    if atom != 0 {
        Ok(atom)
    } else {
        let err_code = unsafe { GetLastError() };
        Err(WindowsError {
            err_type: WindowsErrorType::ClassRegistrationError,
            err_code: Some(Win32ErrorCode(err_code.0)),
            err_body: "Failed to register window class.".to_string(),
        })
    }
}

pub(crate) fn unregister_window_class(
    class_name: PCWSTR,
    h_instance: HMODULE,
) -> Result<(), WindowsError> {
    let result = unsafe { UnregisterClassW(class_name, h_instance) };
    if result == TRUE {
        Ok(())
    } else {
        let err_code = unsafe { GetLastError() };
        Err(WindowsError {
            err_type: WindowsErrorType::ClassUnregistrationError,
            err_code: Some(Win32ErrorCode(err_code.0)),
            err_body: "Failed to unregister window class.".to_string(),
        })
    }
}

pub(crate) fn choose_pixel_format(
    hdc: HDC,
    ppfd: *const PIXELFORMATDESCRIPTOR,
) -> Result<i32, WindowsError> {
    let pixel_format = unsafe { ChoosePixelFormat(hdc, ppfd) };
    if pixel_format != 0 {
        Ok(pixel_format)
    } else {
        let err_code = unsafe { GetLastError() };
        Err(WindowsError {
            err_type: WindowsErrorType::PixelFormatChooseError,
            err_code: Some(Win32ErrorCode(err_code.0)),
            err_body: "Failed to choose pixel format.".to_string(),
        })
    }
}

pub(crate) fn set_pixel_format(
    hdc: HDC,
    format: i32,
    ppfd: *const PIXELFORMATDESCRIPTOR,
) -> Result<(), WindowsError> {
    let res = unsafe { SetPixelFormat(hdc, format, ppfd) };
    if res == TRUE {
        Ok(())
    } else {
        let err_code = unsafe { GetLastError() };
        Err(WindowsError {
            err_type: WindowsErrorType::PixelFormatSetError,
            err_code: Some(Win32ErrorCode(err_code.0)),
            err_body: "Failed to set pixel format.".to_string(),
        })
    }
}

pub(crate) fn wgl_create_context(dc: HDC) -> Result<HGLRC, WindowsError> {
    let result = unsafe { wglCreateContext(dc) };
    match result {
        Ok(hglrc) => Ok(hglrc),
        Err(_) => {
            let err_code = unsafe { GetLastError() };
            Err(WindowsError {
                err_type: WindowsErrorType::WGLContextCreationError,
                err_code: Some(Win32ErrorCode(err_code.0)),
                err_body: "Failed to create wgl context.".to_string(),
            })
        }
    }
}

pub(crate) fn wgl_delete_context(hglrc: HGLRC) -> Result<(), WindowsError> {
    let result = unsafe { wglDeleteContext(hglrc) };
    if result == TRUE {
        Ok(())
    } else {
        let err_code = unsafe { GetLastError() };
        Err(WindowsError {
            err_type: WindowsErrorType::WGLContextDeletionError,
            err_code: Some(Win32ErrorCode(err_code.0)),
            err_body: "Failed to delete wgl context.".to_string(),
        })
    }
}

pub(crate) fn wgl_make_current(dc: HDC, wgl_context: HGLRC) -> Result<(), WindowsError> {
    let result = unsafe { wglMakeCurrent(dc, wgl_context) };
    if result == TRUE {
        Ok(())
    } else {
        let err_code = unsafe { GetLastError() };
        Err(WindowsError {
            err_type: WindowsErrorType::WGLContextSelectingError,
            err_code: Some(Win32ErrorCode(err_code.0)),
            err_body: "Failed to select wgl context.".to_string(),
        })
    }
}

/// This function releases Device Context that is used by current rendering context!
/// Be careful when use it!
pub(crate) fn wgl_make_current_null() -> Result<(), WindowsError> {
    let result = unsafe { wglMakeCurrent(HDC(0), HGLRC(0)) };
    if result == TRUE {
        Ok(())
    } else {
        let err_code = unsafe { GetLastError() };
        Err(WindowsError {
            err_type: WindowsErrorType::WGLContextSelectingError,
            err_code: Some(Win32ErrorCode(err_code.0)),
            err_body: "Failed to select wgl context.".to_string(),
        })
    }
}

pub(crate) fn load_library_w(name: PCWSTR) -> Result<HMODULE, WindowsError> {
    let hmodule = unsafe { LoadLibraryW(name) };
    match hmodule {
        Ok(hmodule) => Ok(hmodule),
        Err(_) => {
            let err_code = unsafe { GetLastError() };
            Err(WindowsError {
                err_type: WindowsErrorType::LibraryLoadError,
                err_code: Some(Win32ErrorCode(err_code.0)),
                err_body: "Failed to load library.".to_string(),
            })
        }
    }
}

pub(crate) fn load_gl_functions() -> Result<(), WindowsError> {
    let hmodule = load_library_w(w!("opengl32.dll"))?;

    gl::load_with(|string| {
        let fn_name = PCSTR(format!("{string}\0").as_ptr());
        unsafe {
            match GetProcAddress(hmodule, fn_name) {
                Some(val) => val as *const std::ffi::c_void,
                None => match wglGetProcAddress(fn_name) {
                    Some(val) => val as *const std::ffi::c_void,
                    None => {
                        wwg_log::wwg_debug!("Failed to load {string} function.");
                        std::ptr::null()
                    }
                },
            }
        }
    });
    Ok(())
}

pub(crate) fn load_cursor_w<P: IntoParam<PCWSTR>>(cursor: P) -> Result<HCURSOR, WindowsError> {
    let result = unsafe { LoadCursorW(HMODULE(0), cursor) };
    match result {
        Ok(cursor) => Ok(cursor),
        Err(_) => {
            let err_code = unsafe { GetLastError() };
            Err(WindowsError {
                err_type: WindowsErrorType::CursorLoadError,
                err_code: Some(Win32ErrorCode(err_code.0)),
                err_body: "Failed to load cursor.".to_string(),
            })
        }
    }
}

pub(crate) fn wgl_get_proc_address(
    name: PCSTR,
) -> Result<unsafe extern "system" fn() -> isize, WindowsError> {
    let result = unsafe { wglGetProcAddress(name) };
    match result {
        Some(function) => Ok(function),
        None => {
            let err_code = unsafe { GetLastError() };
            Err(WindowsError {
                err_type: WindowsErrorType::WGLExtensionLoadError,
                err_code: Some(Win32ErrorCode(err_code.0)),
                err_body: "Failed to load wgl extension.".to_string(),
            })
        }
    }
}

pub(crate) fn destroy_window(hwnd: HWND) -> Result<(), WindowsError> {
    let result = unsafe { DestroyWindow(hwnd) };
    if result == TRUE {
        Ok(())
    } else {
        let err_code = unsafe { GetLastError() };
        Err(WindowsError {
            err_type: WindowsErrorType::WGLContextSelectingError,
            err_code: Some(Win32ErrorCode(err_code.0)),
            err_body: "Failed to select wgl context.".to_string(),
        })
    }
}

pub(crate) fn vk_into_keycode() {}
