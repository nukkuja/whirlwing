use std::ffi::*;
use std::mem::{size_of, size_of_val};
use std::ptr::null;

use crate::{shader::Shader, time::Time};
use glutin::display::{Display, GlDisplay};

pub(crate) struct Renderer {
    vertex_array: u32,
    element_buffer: u32,
    texture: u32,
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
                panic!();
            }
            let shader = shader.unwrap();

            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&VERTICES) as isize,
                VERTICES.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * size_of::<f32>() as i32,
                0usize as *const c_void,
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * size_of::<f32>() as i32,
                (3 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            // TEXTURES CODE STARTS HERE
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT as i32,
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let mut path = std::env::current_dir().unwrap();
            path.push("res/textures/container.jpg");
            let img = image::open(path).unwrap();
            let width = img.width();
            let height = img.height();
            let pixels = img.into_bytes();
            
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            // drop(pixels);

            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * size_of::<f32>() as i32, (6 * size_of::<f32>()) as *const c_void);
            gl::EnableVertexAttribArray(2);

            // Element buffer

            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&INDICES) as isize, INDICES.as_ptr() as *const c_void, gl::STATIC_DRAW);

            Renderer {
                vertex_array: vao,
                element_buffer: ebo,
                texture,
                shader,
            }
        }
    }

    pub(crate) fn resize(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub(crate) fn redraw(&self, _time: &Time) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.shader.bind();

            gl::BindVertexArray(self.vertex_array);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_buffer);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {}
}

#[rustfmt::skip]
const VERTICES: [f32; 32] = [
    -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,  0.0, 0.0,
    -0.5,  0.5, 0.0,    0.0, 1.0, 0.0,  0.0, 1.0,
     0.5, -0.5, 0.0,    0.0, 0.0, 1.0,  1.0, 0.0,
     0.5,  0.5, 0.0,    1.0, 1.0, 0.0,  1.0, 1.0,
];

const INDICES: [u32; 6] = [
    0, 1, 2,
    1, 2, 3,
];