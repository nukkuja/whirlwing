#[allow(unused_imports)]
use windows::{
    s, w,
    core::{PCSTR, PCWSTR, IntoParam},
    Win32::{
        Foundation::*,
        UI::WindowsAndMessaging::*,
        System::LibraryLoader::*,
        Graphics::{
            Gdi::*,
            OpenGL::*,
        },
    },
};

use crate::windows_error::*;

pub(crate) fn new_window
<
P0: IntoParam<PCWSTR>,
P1: IntoParam<PCWSTR>,
P2: IntoParam<HWND>,
P3: IntoParam<HMENU>,
P4: IntoParam<HMODULE>,
>
(
    dwexstyle: WINDOW_EX_STYLE,
    lpclassname: P0,
    lpwindowname: P1,
    dwstyle: WINDOW_STYLE,
    nwidth: i32,
    nheight: i32,
    hwndparent: P2,
    hmenu: P3,
    hinstance: P4,
    lpparam: Option<*const std::ffi::c_void>,
 ) -> Result<HWND, WindowsError> {
    let mut wnd_rect = RECT { left: 0, right: nwidth, top: 0, bottom: nheight };
    let res = unsafe { AdjustWindowRect(&mut wnd_rect, dwstyle, None) };
    if res == false {
        let err_code = unsafe { GetLastError() };
        return Err(WindowsError {
            err_type: WindowsErrorType::AdjustWindowRectError,
            err_code: Some(Win32ErrorCode(err_code.0)),
            err_body: "Failed to adjust window size while creating the window.".to_string(),
        });
    }

    let h_wnd = unsafe { CreateWindowExW(
        dwexstyle,
        lpclassname,
        lpwindowname,
        dwstyle,
        100, 100,
        wnd_rect.right - wnd_rect.left,
        wnd_rect.bottom - wnd_rect.top,
        hwndparent,
        hmenu,
        hinstance,
        lpparam,
    )};
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

pub(crate) fn choose_pixel_format(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> Result<i32, WindowsError> {
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

pub(crate) fn set_pixel_format(hdc: HDC, format: i32, ppfd: *const PIXELFORMATDESCRIPTOR) -> Result<(), WindowsError> {
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
                Some(val) => return val as *const std::ffi::c_void,
                None => {
                    match wglGetProcAddress(fn_name) {
                        Some(val) => return val as *const std::ffi::c_void,
                        None => {
                            wwg_log::wwg_debug!("Failed to load {string} function.");
                            return std::ptr::null();
                        }
                    }
                }
            }
        }
    });
    Ok(())
}