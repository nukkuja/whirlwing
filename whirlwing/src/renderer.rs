use std::ffi::*;
use std::mem::{size_of, size_of_val};
use std::ptr::null;

use crate::{shader::Shader, time::Time};
use glutin::display::{Display, GlDisplay};

pub(crate) struct Renderer {
    vertex_array: u32,
    element_buffer: u32,
    texture1: u32,
    texture2: u32,
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
            gl::Enable(gl::DEPTH_TEST);
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
                5 * size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // gl::VertexAttribPointer(
            //     1,
            //     3,
            //     gl::FLOAT,
            //     gl::FALSE,
            //     8 * size_of::<f32>() as i32,
            //     (3 * size_of::<f32>()) as *const c_void,
            // );
            // gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * size_of::<f32>() as i32,
                (3 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);

            // TEXTURES CODE STARTS HERE
            let path = std::env::current_dir().unwrap();

            let mut container_texture_path = path.clone();
            container_texture_path.push("res/textures/container.jpg");
            let img = image::open(container_texture_path).unwrap();
            let width = img.width();
            let height = img.height();
            let pixels = img.into_rgb8().into_raw();

            let mut texture1 = 0;
            gl::GenTextures(1, &mut texture1);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            let border_color: [f32; 4] = [0.3, 0.2, 0.5, 1.0];
            gl::TexParameterfv(
                gl::TEXTURE_2D,
                gl::TEXTURE_BORDER_COLOR,
                border_color.as_ptr(),
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

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

            let mut awesomeface_path = path.clone();
            awesomeface_path.push("res/textures/awesomeface.png");
            let img = image::open(awesomeface_path).unwrap().flipv();
            let width = img.width();
            let height = img.height();
            let pixels = img.into_rgba8().into_raw();

            let mut texture2 = 0;
            gl::GenTextures(1, &mut texture2);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);

            shader.bind();
            shader.set_int("texture1", 0);
            shader.set_int("texture2", 1);

            // Element buffer
            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                size_of_val(&INDICES) as isize,
                INDICES.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            Renderer {
                vertex_array: vao,
                element_buffer: ebo,
                texture1,
                texture2,
                shader,
            }
        }
    }

    pub(crate) fn resize(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub(crate) fn redraw(&self, time: &Time) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.texture2);

            self.shader.bind();
            use wwg_math::*;

            let _angle = time.now().as_secs_f32() * 1.5f32;
            // let rot = Rotor3::from_rotation_xz(f32::to_radians(60.0)) * Rotor3::from_rotation_yz(f32::to_radians(30.0));
            let rot = Rotor3::from_rotation_xz(f32::to_radians(30.0));
            let model = Similarity3::new(Vec3::zero(), rot, 1.0);

            // Finally it works as expected!!!
            let eye = Vec3::new(0.0, 2.0, 5.0);
            let target = eye + Vec3::new(0.0, 0.0, -1.0);
            // let target = Vec3::zero();

            let camera = Isometry3::new(eye, Rotor3::from_rotation_yz(f32::to_radians(-20.0)));
            // let rot_mat = camera.rotation.into_matrix().into_homogeneous();
            let view = camera.into_homogeneous_matrix().inversed();


            // let view = Mat4::look_at(eye, target, Vec3::unit_y());

            // Projection
            let near = 0.1f32;
            let far = 100.0f32;
            let angle_rad = 0.7f32;
            let aspect = 800.0f32 / 600.0f32;
            let projection = perspective_gl(angle_rad, aspect, near, far);

            self.shader.set_mat4("model", &model.into_homogeneous_matrix());
            self.shader.set_mat4("view", &view);
            self.shader.set_mat4("projection", &projection);

            gl::BindVertexArray(self.vertex_array);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_buffer);
            gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, null());
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {}
}

#[rustfmt::skip]
const VERTICES: [f32; 120] = [
    -0.5, -0.5, -0.5,   0.0, 0.0,
    -0.5,  0.5, -0.5,   0.0, 1.0,
     0.5, -0.5, -0.5,   1.0, 0.0,
     0.5,  0.5, -0.5,   1.0, 1.0,

    -0.5, -0.5,  0.5,   0.0, 0.0,
    -0.5,  0.5,  0.5,   0.0, 1.0,
     0.5, -0.5,  0.5,   1.0, 0.0,
     0.5,  0.5,  0.5,   1.0, 1.0,

    -0.5, -0.5, -0.5,   0.0, 0.0,
    -0.5,  0.5, -0.5,   0.0, 1.0,
    -0.5, -0.5,  0.5,   1.0, 0.0,
    -0.5,  0.5,  0.5,   1.0, 1.0,

     0.5, -0.5, -0.5,   0.0, 0.0,
     0.5,  0.5, -0.5,   0.0, 1.0,
     0.5, -0.5,  0.5,   1.0, 0.0,
     0.5,  0.5,  0.5,   1.0, 1.0,

    -0.5, -0.5, -0.5,   0.0, 0.0,
    -0.5, -0.5,  0.5,   0.0, 1.0,
     0.5, -0.5, -0.5,   1.0, 0.0,
     0.5, -0.5,  0.5,   1.0, 1.0,

    -0.5, 0.5, -0.5,   0.0, 0.0,
    -0.5, 0.5,  0.5,   0.0, 1.0,
     0.5, 0.5, -0.5,   1.0, 0.0,
     0.5, 0.5,  0.5,   1.0, 1.0,
];

const INDICES: [u32; 36] = [
    0, 1, 2, 1, 2, 3,
    4, 5, 6, 5, 6, 7,
    8, 9, 10, 9, 10, 11,
    12, 13, 14, 13, 14, 15,
    16, 17, 18, 17, 18, 19,
    20, 21, 22, 21, 22, 23,
];
