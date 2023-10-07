use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin::context::{ContextAttributesBuilder, GlProfile};
use glutin::display::Display;
use glutin::display::GetGlDisplay;
use glutin::prelude::{GlDisplay, NotCurrentGlContextSurfaceAccessor, PossiblyCurrentGlContext};
use glutin::surface::GlSurface;
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use std::ffi::CString;
use std::num::NonZeroU32;
use std::str::from_utf8_unchecked;
use winit::event::{Event, WindowEvent};

struct Renderer;

impl Renderer {
    fn new(display: &Display) -> Self {
        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            display.get_proc_address(&symbol)
        });

        unsafe {
            gl::Viewport(0, 0, 800, 600);
        }

        #[rustfmt::skip]
        let vertices = [
            -0.5f32, -0.5f32, 0.0f32,
             0.5f32, -0.5f32, 0.0f32,
             0.0f32,  0.5f32, 0.0f32
        ];

        unsafe {
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as isize,
                vertices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &(VERTEX_SHADER.as_ptr() as *const i8), std::ptr::null());
            gl::CompileShader(vertex_shader);

            let mut success = 0;
            let mut buffer = [0i8; 512];
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                gl::GetShaderInfoLog(vertex_shader, 512, std::ptr::null_mut(), &mut buffer[0]);
                let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                wwg_log::wwg_err!("VERTEX SHADER COMPILATION FAILED: {0}", string);
            }

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &(FRAGMENT_SHADER.as_ptr() as *const i8), std::ptr::null());
            gl::CompileShader(fragment_shader);

            let mut success = 0;
            let mut buffer = [0i8; 512];
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                gl::GetShaderInfoLog(fragment_shader, 512, std::ptr::null_mut(), &mut buffer[0]);
                let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                wwg_log::wwg_err!("VERTEX SHADER COMPILATION FAILED: {0}", string);
            }

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success = 0;
            let mut buffer = [0i8; 512];
            gl::GetShaderiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                gl::GetProgramInfoLog(shader_program, 512, std::ptr::null_mut(), &mut buffer[0]);
                let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                wwg_log::wwg_err!("VERTEX SHADER COMPILATION FAILED: {0}", string);
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            gl::UseProgram(shader_program);
        }
        Renderer
    }
    fn resize(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }
}

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

    let mut renderer: Option<Renderer> = None;
    let mut state = None;

    event_loop.run(move |event, elwt, control_flow| match event {
        Event::Resumed => {
            let window = window.take().unwrap_or_else(|| {
                let window_builder = winit::window::WindowBuilder::new()
                    .with_title("Whirlwing Window")
                    .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
                    .with_transparent(true);
                glutin_winit::finalize_window(&elwt, window_builder, &gl_config).unwrap()
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

            if let None = renderer {
                renderer = Some(Renderer::new(&gl_display));
            }

            if let Err(res) = gl_surface.set_swap_interval(
                &gl_context,
                glutin::surface::SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
            ) {
                wwg_log::wwg_warn!("Error setting vsync: {res}");
            }

            assert!(state.replace((gl_context, gl_surface, window)).is_none());
        }
        Event::Suspended => {
            let (gl_context, ..) = state.take().unwrap();
            assert!(not_current_gl_context
                .replace(gl_context.make_not_current().unwrap())
                .is_none());
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                if size.width != 0 && size.height != 0 {
                    if let Some((gl_context, gl_surface, _)) = &state {
                        gl_surface.resize(
                            &gl_context,
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
                    gl::ClearColor(0.4, 0.9, 0.4, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                window.request_redraw();

                gl_surface.swap_buffers(gl_context).unwrap();
            }
        }
        _ => (),
    });
}

const VERTEX_SHADER: &[u8] = b"
#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
\0";

const FRAGMENT_SHADER: &[u8] = b"
#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
\0";