pub struct Window;

impl Window {
    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        match wwg_win32::create_wc_and_window() {
            Ok(()) => (),
            Err(error) => {
                wwg_log::wwg_err!("{}", error);
            }
        };
        Window
    }
}
