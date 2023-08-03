pub struct Window;

impl Window {
    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        wwg_windows::create_window();
        Window
    }
}
