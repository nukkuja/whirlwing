pub use wwg_window_internal::Window;

#[cfg(target_os = "windows")]
pub use wwg_win32::{WindowWin32, WindowsError};

#[derive(Debug, Clone)]
pub struct WindowDescriptor<'a> {
    pub title: &'a str,
    pub pos_x: i32,
    pub pos_y: i32,
    pub width: u32,
    pub height: u32,
}

impl WindowDescriptor<'_> {
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl Default for WindowDescriptor<'_> {
    fn default() -> Self {
        WindowDescriptor {
            title: "Whirlwing Window",
            pos_x: 50,
            pos_y: 50,
            width: 800,
            height: 600,
        }
    }
}
