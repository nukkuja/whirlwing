use crate::{renderer::Renderer, time::Time};

use std::num::NonZeroU32;

use glutin::{
    prelude::{NotCurrentGlContextSurfaceAccessor, PossiblyCurrentGlContext},
    config::{ConfigTemplateBuilder, GlConfig},
    display::{GlDisplay, GetGlDisplay},
    context::{ContextAttributesBuilder, GlProfile},
    surface::GlSurface,
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::event::{Event, WindowEvent, VirtualKeyCode};

pub fn run() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window_builder = winit::window::WindowBuilder::new()
        .with_title("Whirlwing Window")
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .with_transparent(true);

    // I stole next chunk of code from glutin example
    // And I don't know what's going on there
    #[cfg(cgl_backend)]
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(true);
    #[cfg(not(cgl_backend))]
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(false);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (mut window, gl_config) = display_builder
        .build(&event_loop, template, |configs| {
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                    if transparency_check || config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());
    let gl_display = gl_config.display();

    #[cfg(not(wgl_backend))]
    let context_attributes = ContextAttributesBuilder::new()
        .with_profile(GlProfile::Core)
        .with_context_api(glutin::context::ContextApi::Gles(Some(
            glutin::context::Version::new(3, 3),
        )));
    #[cfg(debug_assertions)]
    let context_attributes = context_attributes
        .with_debug(true)
        .with_robustness(glutin::context::Robustness::RobustLoseContextOnReset);

    let context_attributes = context_attributes.build(raw_window_handle);

    let mut not_current_gl_context = Some(unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .expect("Failed to create OpenGL Context.")
    });

    let mut renderer = None;
    let mut state = None;
    let mut time = Time::start();

    event_loop.run(move |event, elwt, control_flow| {
        control_flow.set_poll();

        match event {
            Event::Resumed => {
                let window = window.take().unwrap_or_else(|| {
                    let window_builder = winit::window::WindowBuilder::new()
                        .with_title("Whirlwing Window")
                        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
                        .with_transparent(true);
                    glutin_winit::finalize_window(elwt, window_builder, &gl_config).unwrap()
                });
                let attributes = window.build_surface_attributes(<_>::default());

                let gl_surface = unsafe {
                    gl_display
                        .create_window_surface(&gl_config, &attributes)
                        .unwrap()
                };
                let gl_context = not_current_gl_context
                    .take()
                    .unwrap()
                    .make_current(&gl_surface)
                    .unwrap();

                if renderer.is_none() {
                    renderer = Some(Renderer::new(&gl_display));

                }

                if let Err(res) = gl_surface.set_swap_interval(
                    &gl_context,
                    glutin::surface::SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
                ) {
                    wwg_log::wwg_warn!("Error setting vsync: {res}");
                }

                assert!(state.replace((gl_context, gl_surface, window)).is_none());
                time.reset();
            }
            Event::Suspended => {
                let (gl_context, ..) = state.take().unwrap();
                assert!(not_current_gl_context
                    .replace(gl_context.make_not_current().unwrap())
                    .is_none());
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.virtual_keycode.unwrap() == VirtualKeyCode::Escape {
                        control_flow.set_exit();
                    }
                }
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        if let Some((gl_context, gl_surface, _)) = &state {
                            gl_surface.resize(
                                gl_context,
                                NonZeroU32::new(size.width).unwrap(),
                                NonZeroU32::new(size.height).unwrap(),
                            );
                            if let Some(rend) = &renderer {
                                rend.resize(size.width as i32, size.height as i32);
                            }
                        }
                    }
                }
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                if let Some((gl_context, gl_surface, window)) = &state {
                    unsafe {
                        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }
                    if let Some(rend) = &renderer {
                        rend.redraw(&time);
                    }
                    window.request_redraw();
                    gl_surface.swap_buffers(gl_context).unwrap();
                    time.tick();
                    wwg_log::wwg_trace!("FPS: {}", 1f32 / time.delta_time().as_secs_f32());
                    wwg_log::wwg_trace!("Frame took {} seconds.", time.delta_time().as_secs_f32());
                }
            }
            _ => (),
        }
    });
}
