pub enum WindowsErrorType {
    ClassRegistrationFailure,
}

pub struct WindowsError {
    pub err_type: WindowsErrorType,
    pub err_code: Option<u32>,
    pub err_body: String,
}