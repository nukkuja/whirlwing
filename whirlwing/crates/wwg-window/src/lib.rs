pub struct Window;

use std::collections::VecDeque;

use wwg_events::*;

impl Window {
    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        wwg_windows::create_window();
        Window
    }
}
