pub struct Window;

impl Window {
    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        match wwg_windows::create_window() {
            Ok(()) => (),
            Err(error) => { wwg_log::wwg_err!("{}", error); },
        };
        Window
    }
}
