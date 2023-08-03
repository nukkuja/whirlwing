#[allow(unused_imports)]
use windows::{
    s, w,
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

pub(crate) fn get_instance_handle() -> HMODULE {
    unsafe { GetModuleHandleW(None).unwrap() }
}

pub(crate) fn register_window_class(class: WNDCLASSEXW) -> Result<u16, WindowsError> {
    let atom = unsafe { RegisterClassExW(&class) };
    if atom != 0 {
        Ok(atom)
    } else {
        let err_code = unsafe { GetLastError() };
        Err(WindowsError {
            err_type: WindowsErrorType::ClassRegistrationFailure,
            err_code: Some(err_code.0),
            err_body: "Failed to register Windows class.".to_string(),
        })
    }
}