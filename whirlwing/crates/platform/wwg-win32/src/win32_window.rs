use crate::win32_error::*;
use crate::win32_utils::*;
use crate::win32_utils::{GL_FALSE, GL_TRUE};
use crate::WindowsError;
use windows::core::PCWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::Graphics::OpenGL::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::{s, w};

macro_rules! load_wgl_extension {
    ($name_str:literal into $name:ident($($args:ty),*)) => {
        let function_holder = $crate::win32_utils::wgl_get_proc_address($crate::win32_window::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*)>
        (function_holder) };
    };

    ($name_str:literal into $name:ident($($args:ty),*) -> $ret:ty) => {
        let function_holder = $crate::win32_utils::wgl_get_proc_address($crate::win32_window::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*) -> $ret>
        (function_holder) };
    };

        ($name_str:literal into $name:ident($($args:ty),*,)) => {
        let function_holder = $crate::win32_utils::wgl_get_proc_address($crate::win32_window::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*)>
        (function_holder) };
    };

    ($name_str:literal into $name:ident($($args:ty),*,) -> $ret:ty) => {
        let function_holder = $crate::win32_utils::wgl_get_proc_address($crate::win32_window::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*) -> $ret>
        (function_holder) };
    };
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct WindowWin32 {
    pub hwnd: HWND,
    pub device_context: HDC,
    rendering_context: HGLRC,

    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,

    hinstance: HMODULE,
    wnd_class_name: PCWSTR,
}

impl wwg_window_internal::Window for WindowWin32 {
    type Error = crate::WindowsError;

    fn init(
        title: &str,
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
    ) -> Result<Self, WindowsError> {
        let hinstance = get_instance_handle();
        let fake_wnd_class_name = w!("Fake window class");

        let fake_wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_OWNDC,
            lpfnWndProc: Some(crate::wnd_proc::fake_wnd_proc),
            hInstance: hinstance,
            lpszClassName: fake_wnd_class_name,
            ..Default::default()
        };

        register_window_class(fake_wnd_class)?;

        let fake_wnd = create_window(
            WINDOW_EX_STYLE::default(),
            fake_wnd_class_name,
            w!("Fake Window"),
            WINDOW_STYLE::default(),
            0,
            0,
            200,
            200,
            None,
            None,
            hinstance,
            None,
        )?;

        let fake_wnd_dc = get_device_context(fake_wnd)?;

        let pfd = PIXELFORMATDESCRIPTOR {
            nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 32,
            cDepthBits: 24,
            cStencilBits: 8,
            iLayerType: PFD_MAIN_PLANE,
            ..Default::default()
        };
        let fake_wnd_pixel_format: i32 = choose_pixel_format(fake_wnd_dc, &pfd)?;
        set_pixel_format(fake_wnd_dc, fake_wnd_pixel_format, &pfd)?;

        let fake_wgl_context = wgl_create_context(fake_wnd_dc)?;
        wgl_make_current(fake_wnd_dc, fake_wgl_context)?;
        load_gl_functions()?;

        load_wgl_extension!("wglChoosePixelFormatARB" into
        wglChoosePixelFormatARB(HDC, *const i32, *const f32, u32, *mut i32, *mut u32) -> BOOL);
        load_wgl_extension!("wglCreateContextAttribsARB" into
        wglCreateContextAttribsARB(HDC, HGLRC, *const i32) -> HGLRC);

        wwg_log::wwg_trace!("Fake window is created and opengl functions are loaded.");

        let wnd_class_name = w!("Whirlwing Window Class");
        let wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_OWNDC,
            lpfnWndProc: Some(crate::wnd_proc::wnd_proc),
            hInstance: hinstance,
            hIcon: HICON(0),
            hCursor: load_cursor_w(IDC_ARROW)?,
            lpszClassName: wnd_class_name,
            ..Default::default()
        };

        register_window_class(wnd_class)?;

        let mut title = title.encode_utf16().collect::<Vec<u16>>();
        title.push(0);
        let title = PCWSTR(title.as_ptr());

        let hwnd = create_window(
            WS_EX_LEFT,
            wnd_class_name,
            title,
            WS_OVERLAPPED | WS_CAPTION | WS_MINIMIZEBOX | WS_SYSMENU,
            pos_x,
            pos_y,
            width,
            height,
            HWND(0),
            HMENU(0),
            hinstance,
            None,
        )?;

        let dc = get_device_context(hwnd)?;

        let pfd = PIXELFORMATDESCRIPTOR {
            nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 32,
            cDepthBits: 24,
            cStencilBits: 8,
            iLayerType: PFD_MAIN_PLANE,
            ..Default::default()
        };
        let attrib_list = [
            WGL_DRAW_TO_WINDOW_ARB,
            GL_TRUE,
            WGL_SUPPORT_OPENGL_ARB,
            GL_TRUE,
            WGL_DOUBLE_BUFFER_ARB,
            GL_TRUE,
            WGL_PIXEL_TYPE_ARB,
            WGL_TYPE_RGBA_ARB,
            WGL_COLOR_BITS_ARB,
            32,
            WGL_DEPTH_BITS_ARB,
            24,
            WGL_STENCIL_BITS_ARB,
            8,
            0,
        ];
        let mut pixel_format = 0;
        let mut num_formats = 0u32;

        let result = unsafe {
            (wglChoosePixelFormatARB)(
                dc,
                attrib_list.as_ptr(),
                std::ptr::null(),
                1,
                &mut pixel_format,
                &mut num_formats,
            )
        };
        if result == FALSE {
            return Err(WindowsError {
                err_type: WindowsErrorType::WGLExtensionLoadError,
                err_code: None,
                err_body: "Failed to choose WGL pixel format.".to_string(),
            });
        };
        set_pixel_format(dc, pixel_format, &pfd)?;

        let attribs_list = [
            WGL_CONTEXT_MAJOR_VERSION_ARB,
            3,
            WGL_CONTEXT_MINOR_VERSION_ARB,
            3,
            WGL_CONTEXT_PROFILE_MASK_ARB,
            WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
        ];

        let hglrc = unsafe { (wglCreateContextAttribsARB)(dc, HGLRC(0), attribs_list.as_ptr()) };
        if hglrc.0 == 0 {
            let err_code = unsafe { GetLastError() };
            return Err(WindowsError {
                err_type: WindowsErrorType::WGLContextCreationErrorARB,
                err_code: Some(Win32ErrorCode(err_code.0)),
                err_body: "Failed to create WGL ARB context.".to_string(),
            });
        }

        wgl_make_current(fake_wnd_dc, fake_wgl_context)?;
        wgl_make_current_null()?;
        wgl_delete_context(fake_wgl_context)?;
        destroy_window(fake_wnd)?;
        unregister_window_class(fake_wnd_class_name, hinstance)?;

        wwg_log::wwg_trace!("Fake window is destroyed.");

        wgl_make_current(dc, hglrc)?;
        unsafe {
            ShowWindow(hwnd, SW_SHOW);
        }

        wwg_log::wwg_debug!("Window is successfully created!");

        Ok(WindowWin32 {
            hwnd,
            device_context: dc,
            rendering_context: hglrc,
            pos_x,
            pos_y,
            width,
            height,
            hinstance,
            wnd_class_name,
        })
    }

    fn destroy(&mut self) -> Result<(), WindowsError> {
        wwg_log::wwg_debug!("Window is being properly destroyed.");
        wgl_make_current_null()?;
        wgl_delete_context(self.rendering_context)?;
        destroy_window(self.hwnd)?;
        unregister_window_class(self.wnd_class_name, self.hinstance)
    }

    fn render(&mut self) {
        unsafe {
            wwg_log::wwg_trace!("render function is called.");
            gl::Viewport(0, 0, self.width, self.height);

            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let vertex_buffer = [-0.5f32, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

            let mut vbo: u32 = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_buffer.len() * std::mem::size_of::<f32>()) as isize,
                vertex_buffer.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            let vertex_shader_source = std::ffi::CString::new(
                r#"
                #version 330 core
                layout (location = 0) in vec3 aPos;
            
                void main() {
                    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
                }
                "#,
            )
            .expect("CString creation failed.");

            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &vertex_shader_source.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(vertex_shader);

            let mut success = i32::default();
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut buffer = [0i8; 512];
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr(),
                );
                let string = std::str::from_utf8(std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u8,
                    buffer.len(),
                ))
                .unwrap();
                wwg_log::wwg_err!("Shader compilation failed!\n{}", string);
            }

            let fragment_shader_source = std::ffi::CString::new(
                r#"
                #version 330 core
                out vec4 FragColor;
            
                void main() {
                    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
                }
                "#,
            )
            .expect("CString creation failed.");

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &fragment_shader_source.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(fragment_shader);

            let mut success = i32::default();
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut buffer = [0i8; 512];
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr(),
                );
                let string = std::str::from_utf8(std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u8,
                    buffer.len(),
                ))
                .unwrap();
                wwg_log::wwg_err!("Shader compilation failed!\n{}", string);
            }

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success = i32::default();
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut buffer = [0i8; 512];
                gl::GetProgramInfoLog(
                    shader_program,
                    512,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr(),
                );
                let string = std::str::from_utf8(std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u8,
                    buffer.len(),
                ))
                .unwrap();
                wwg_log::wwg_err!("Shader compilation failed!\n{}", string);
            }

            let mut vao = u32::default();
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::VertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                GL_FALSE as u8,
                3 * std::mem::size_of::<f32>() as i32,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(0);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            let error = gl::GetError();
            if error != 0 {
                wwg_log::wwg_err!("OpenGL error: {error}");
            } else {
                wwg_log::wwg_debug!("No OpenGL errors.");
            }

            SwapBuffers(self.device_context);
        }
    }

    fn receive_events(&self) -> std::collections::VecDeque<wwg_events::Event> {
        use std::collections::VecDeque;
        use wwg_events::*;
        let mut events: VecDeque<wwg_events::Event> = VecDeque::new();
        unsafe {
            let mut message = MSG::default();
            while PeekMessageW(&mut message, HWND(0), 0, 0, PM_REMOVE) != FALSE {
                TranslateMessage(&message);
                match message.message {
                    WM_QUIT => {
                        let event_type = EventType::ApplicationExit;
                        events.push_back(Event::new(event_type, EventCategory::WindowEvent));
                        wwg_log::wwg_debug!("Event received: {:#?}", event_type);
                    }
                    WM_KEYDOWN => {
                        let event_type = wwg_events::EventType::KeyPressed {
                            key: message.wParam.0 as u8 as char,
                            repeats: 0,
                        };
                        wwg_log::wwg_debug!("Event received: {:#?}", event_type);
                        events.push_back(wwg_events::Event::new(
                            event_type,
                            wwg_events::EventCategory::KeyboardEvent,
                        ));
                    }
                    _ => {
                        DispatchMessageA(&message);
                    }
                }
            }
        }
        events
    }
}
