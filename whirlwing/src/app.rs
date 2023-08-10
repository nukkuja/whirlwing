pub(super) mod app_internal {
    use std::collections::HashMap;
    use wwg_window::{Window, WindowID, WindowingContext};

    pub struct Application<C, W> {
        windowing_context: C,
        next_window_id: u32,
        map_windows: HashMap<WindowID, W>,
    }

    impl<C, W, E> Application<C, W>
    where
        C: WindowingContext<Window = W, Error = E>,
        W: Window<Error = E>,
    {
        pub fn new() -> Result<Self, E> {
            let windowing_context = C::init()?;
            let next_window_id = 0u32;
            let map_windows = HashMap::new();
            Ok(Application {
                windowing_context,
                next_window_id,
                map_windows,
            })
        }

        pub fn create_window(
            &mut self,
            title: &str,
            pos_x: u32,
            pos_y: u32,
            width: u32,
            height: u32,
        ) -> Result<WindowID, E> {
            let window = self
                .windowing_context
                .create_window(title, pos_x, pos_y, width, height)?;
            let window_id = WindowID(self.next_window_id);
            self.next_window_id += 1u32;
            self.map_windows.insert(window_id, window);
            Ok(window_id)
        }
    }
}

#[cfg(target_os = "windows")]
use wwg_window::*;

pub struct Application;

impl Application {
    #[cfg(target_os = "windows")]
    pub fn new(
    ) -> Result<app_internal::Application<WindowingContextWin32, WindowWin32>, WindowsError> {
        app_internal::Application::new()
    }
}
