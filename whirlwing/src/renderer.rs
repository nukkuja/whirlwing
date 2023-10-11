use crate::shader::Shader;
use glutin::display::{Display, GlDisplay};
use std::ffi::CString;

pub(crate) struct Renderer {
    vertex_array: u32,
    shader: Shader,
}

impl Renderer {
    pub(crate) fn new(display: &Display) -> Self {
        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            display.get_proc_address(&symbol)
        });

        unsafe {
            gl::Viewport(0, 0, 800, 600);
        }

        unsafe {
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);

            let vertex_shader = include_str!("../../res/shaders/vertex_shader.glsl");
            let fragment_shader = include_str!("../../res/shaders/fragment_shader.glsl");
            let shader = Shader::from_str(vertex_shader, fragment_shader);
            if let Err(e) = &shader {
                wwg_log::wwg_err!("{e}");
            }
            let shader = shader.unwrap();

            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&VERTICES) as isize,
                VERTICES.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            Renderer {
                vertex_array: vao,
                shader,
            }
        }
    }
    pub(crate) fn resize(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }
    pub(crate) fn redraw(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);
            self.shader.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {}
}

#[rustfmt::skip]
const VERTICES: [f32; 9] = [
    -0.5f32, -0.5f32, 0.0f32,
     0.5f32, -0.5f32, 0.0f32,
     0.0f32,  0.5f32, 0.0f32,
];
