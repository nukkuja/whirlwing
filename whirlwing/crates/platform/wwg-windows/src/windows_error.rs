#[derive(Debug, PartialEq)]
pub enum WindowsErrorType {
    ClassRegistrationError,
    WindowCreationError,
    AdjustWindowRectError,
    DeviceContextRetrievalError,
    PixelFormatChooseError,
    PixelFormatSetError,
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
        }
    }
}

#[derive(Debug)]
pub struct WindowsError {
    pub err_type: WindowsErrorType,
    pub err_code: Option<u32>,
    pub err_body: String,
}

impl std::fmt::Display for WindowsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.err_code {
            Some(err_code) => {
                let output = format!("Windows Error Code: {}\nError Type: {}\nErrorMessage: {}",
                err_code,
                self.err_type,
                self.err_body);

                write!(f, "{output}")
            },
            None => {
                let output = format!("Windows Error!\nError Type: {}\nErrorMessage: {}",
                self.err_type,
                self.err_body);

                write!(f, "{output}")
            },
        }
    }
}