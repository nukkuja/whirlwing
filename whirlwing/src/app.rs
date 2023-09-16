pub(super) mod app_internal {
    use wwg_events::{input_bindings::*, *};
    use wwg_window::{Window, WindowDescriptor};

    pub struct Application<W: Window> {
        pub window: W,
        should_close: bool,
    }

    impl<W, E> Application<W>
    where
        W: Window<Error = E>,
        E: std::fmt::Display,
    {
        pub fn new(descriptor: WindowDescriptor) -> Self {
            match W::init(
                descriptor.title,
                descriptor.pos_x,
                descriptor.pos_y,
                descriptor.width as i32,
                descriptor.height as i32,
            ) {
                Ok(window) => Application {
                    window,
                    should_close: false,
                },
                Err(e) => {
                    wwg_log::wwg_err!("Failed to create window:\n{e}");
                    panic!()
                }
            }
        }

        pub fn run(mut self) {
            while !self.should_close {
                let events = self.window.receive_events();
                for event in events {
                    match event.event_type() {
                        EventType::ApplicationExit => self.should_close = true,
                        EventType::KeyPressed { key: ESCAPE, .. } => self.should_close = true,
                        EventType::KeyPressed { key: KC_A, .. } => self.should_close = true,
                        _ => (),
                    }
                }
                self.window.render();
            }

            wwg_log::wwg_debug!("Exiting application loop");
        }
    }

    impl<W: Window> Drop for Application<W> {
        fn drop(&mut self) {
            if let Err(e) = self.window.destroy() {
                wwg_log::wwg_err!("{}", e);
            }
        }
    }
}

use wwg_window::*;

pub struct Application;

impl Application {
    #[cfg(target_os = "windows")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(descriptor: WindowDescriptor) -> app_internal::Application<WindowWin32> {
        app_internal::Application::new(descriptor)
    }
}
