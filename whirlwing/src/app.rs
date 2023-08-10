pub(super) mod app_internal {
    use std::collections::HashMap;
    use wwg_window::{Window, WindowID, WindowingContext};

    pub struct Application<C, W>
    where C: WindowingContext,
    W: Window,
    {
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
            self.map_windows.insert(window_id.clone(), window);
            Ok(window_id)
        }

        pub fn destroy_window(&mut self, id: WindowID) {
            let window = self.map_windows.remove(&id);
            match window {
                Some(window) => { let _ = window.destroy(); },
                None => { wwg_log::wwg_err!("Wrong window index is passed while trying to destroy window."); },
            };
        }

        pub fn make_current_window(&self, id: &WindowID) {
            let _ = self.map_windows.get(id).unwrap().make_current();
        }

        pub fn draw_unchecked(&self, id: &WindowID, r: f32, g: f32, b: f32, a: f32) {
            self.map_windows.get(id).unwrap().draw_background(r, g, b, a);
        }
    }

    impl<C: WindowingContext, W: Window> Drop for Application<C, W>
    
    {
        fn drop(&mut self) {
            for (_, window) in self.map_windows.drain() {
                if let Err(e) = window.destroy() {
                    wwg_log::wwg_err!("Error occured while trying to destroy the window:\n{e}");
                }
            }
            if let Err(e) = self.windowing_context.destroy_context() {
                wwg_log::wwg_err!("Error occured while trying to destroy windowing context:\n{e}");
            }
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
