use std::char::decode_utf16;
use windows::core::PWSTR;
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::Memory::LocalFree;
use windows::Win32::Foundation::HLOCAL;

#[derive(Debug, PartialEq)]
pub enum WindowsErrorType {
    ClassRegistrationError,
    WindowCreationError,
    AdjustWindowRectError,
    DeviceContextRetrievalError,
    PixelFormatChooseError,
    PixelFormatSetError,
    WGLContextCreationError,
    WGLContextSelectingError,
    LibraryLoadError,
}

impl std::fmt::Display for WindowsErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WindowsErrorType::ClassRegistrationError => write!(f, "Window Class Registration Error"),
            WindowsErrorType::WindowCreationError => write!(f, "Window Creation Error"),
            WindowsErrorType::AdjustWindowRectError => write!(f, "Window Size Error"),
            WindowsErrorType::DeviceContextRetrievalError => write!(f, "Device Context Retrieval Error"),
            WindowsErrorType::PixelFormatChooseError => write!(f, "Pixel Format Choose Error"),
            WindowsErrorType::PixelFormatSetError => write!(f, "Pixel Format Set Error"),
            WindowsErrorType::WGLContextCreationError => write!(f, "WGL Context Creation Error"),
            WindowsErrorType::WGLContextSelectingError => write!(f, "WGL Context Selecting Error"),
            WindowsErrorType::LibraryLoadError => write!(f, "Library Load Error"),
        }
    }
}

#[derive(Debug)]
pub struct WindowsError {
    pub err_type: WindowsErrorType,
    pub err_code: Option<Win32ErrorCode>,
    pub err_body: String,
}

impl std::fmt::Display for WindowsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.err_code {
            Some(err_code) => {
                let output = format!("Windows Error Code: {}\nError Type: {}\nError Message: {}",
                err_code,
                self.err_type,
                self.err_body);

                write!(f, "{output}")
            },
            None => {
                let output = format!("Windows Error!\nError Type: {}\nError Message: {}",
                self.err_type,
                self.err_body);

                write!(f, "{output}")
            },
        }
    }
}

#[derive(Debug)]
pub struct Win32ErrorCode(pub u32);

impl std::fmt::Display for Win32ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut buffer = PWSTR(std::ptr::null_mut());
        unsafe {
            FormatMessageW(
                FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
                None,
                self.0,
                0,
                PWSTR(&mut buffer.0 as *mut _ as *mut _),
                0,
                None,
            );
        }
        if buffer.0.is_null() {
            Err(std::fmt::Error)
        } else {
            let buffer_array = unsafe { buffer.as_wide() };
            let mut string = String::new();
            for decode_result in decode_utf16(buffer_array.iter().copied()) {
                let char = decode_result.unwrap_or('�');
                string.push(char);
            }
            write!(f, "{}", string.trim())?;
        unsafe { let _ = LocalFree(HLOCAL(buffer.0 as isize)); }
            Ok(())
        }
    }
}
