use std::ptr::{null, null_mut};

use wwg_error::{WhirlwingError, WhirlwingErrorKind};
use wwg_math::core::Matrix4;

#[cfg(debug_assertions)]
use std::str::from_utf8_unchecked;

pub(crate) struct Shader {
    program_id: u32,
}

#[allow(dead_code)]
impl Shader {
    pub fn new(
        vertex_shader_path: &str,
        fragment_shader_path: &str,
    ) -> Result<Shader, WhirlwingError> {
        unsafe {
            let mut vertex_shader_vector;
            match std::fs::read(vertex_shader_path) {
                Ok(vec) => {
                    vertex_shader_vector = vec;
                    vertex_shader_vector.push(b'\0');
                }
                Err(error) => {
                    let error_content = format!("Failed to read from path: {vertex_shader_path}");
                    return Err(WhirlwingError::new_with_source(
                        error_content,
                        WhirlwingErrorKind::ShaderCompilationFailure,
                        Box::new(error),
                    ));
                }
            }
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &(vertex_shader_vector.as_ptr() as *const i8),
                null(),
            );
            gl::CompileShader(vertex_shader);

            #[cfg(debug_assertions)]
            {
                let error_message =
                    format!("Vertex Shader Compilation Error: {vertex_shader_path}");
                Shader::check_shader_compilation(vertex_shader, &error_message)?;
            }

            let mut fragment_shader_vector;
            match std::fs::read(fragment_shader_path) {
                Ok(vec) => {
                    fragment_shader_vector = vec;
                    fragment_shader_vector.push(b'\0');
                }
                Err(error) => {
                    let error_content = format!("Failed to read from path: {fragment_shader_path}");
                    return Err(WhirlwingError::new_with_source(
                        error_content,
                        WhirlwingErrorKind::ShaderCompilationFailure,
                        Box::new(error),
                    ));
                }
            }
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &(fragment_shader_vector.as_ptr() as *const i8),
                null(),
            );
            gl::CompileShader(fragment_shader);

            #[cfg(debug_assertions)]
            {
                let error_message =
                    format!("Fragment Shader Compilation Error: {fragment_shader_path}");
                Shader::check_shader_compilation(fragment_shader, &error_message)?;
            }

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);

            #[cfg(debug_assertions)]
            {
                let mut success = 0;
                let mut buffer = [0i8; 512];
                gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
                if success == 0 {
                    gl::GetProgramInfoLog(program_id, 512, null_mut(), &mut buffer[0]);
                    let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                    let string = format!(
                        "Shader Program Linking Failed:\n
                                          Vertex Shader: {vertex_shader_path}\n
                                          Fragment Shader: {fragment_shader_path}\n
                                          OpenGL Error: {string}"
                    );
                    return Err(WhirlwingError::new(
                        string.to_string(),
                        WhirlwingErrorKind::ShaderCompilationFailure,
                    ));
                }
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(Shader { program_id })
        }
    }

    /// Slices should have b'\0' at the end.
    pub fn from_utf8_slices(
        vertex_shader_slice: &[u8],
        fragment_shader_slice: &[u8],
    ) -> Result<Shader, WhirlwingError> {
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &(vertex_shader_slice.as_ptr() as *const i8),
                null(),
            );
            gl::CompileShader(vertex_shader);

            #[cfg(debug_assertions)]
            {
                let error_message = format!("Vertex Shader Compilation Error: Shader was compiled from slice{vertex_shader_slice:?}");
                Shader::check_shader_compilation(vertex_shader, &error_message)?;
            }

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &(fragment_shader_slice.as_ptr() as *const i8),
                null(),
            );
            gl::CompileShader(fragment_shader);

            #[cfg(debug_assertions)]
            {
                let error_message = format!("Fragment Shader Compilation Error: Shader was compiled from slice{fragment_shader_slice:?}");
                Shader::check_shader_compilation(fragment_shader, &error_message)?;
            }

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);

            #[cfg(debug_assertions)]
            {
                let mut success = 0;
                let mut buffer = [0i8; 512];
                gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
                if success == 0 {
                    gl::GetProgramInfoLog(program_id, 512, null_mut(), &mut buffer[0]);
                    let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                    let string = format!(
                        "Shader Program Linking Failed:\n
                                          Vertex Shader: {vertex_shader_slice:?}\n
                                          Fragment Shader: {fragment_shader_slice:?}\n
                                          OpenGL Error: {string}"
                    );
                    return Err(WhirlwingError::new(
                        string.to_string(),
                        WhirlwingErrorKind::ShaderCompilationFailure,
                    ));
                }
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(Shader { program_id })
        }
    }

    pub fn from_str(
        vertex_shader_str: &str,
        fragment_shader_str: &str,
    ) -> Result<Shader, WhirlwingError> {
        unsafe {
            let mut vertex_shader_str = vertex_shader_str.to_string();
            vertex_shader_str.push('\0');

            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &(vertex_shader_str.as_ptr() as *const i8),
                null(),
            );
            gl::CompileShader(vertex_shader);

            #[cfg(debug_assertions)]
            {
                let error_message = format!("Vertex Shader Compilation Error: Shader was compiled from string:\n{vertex_shader_str:?}");
                Shader::check_shader_compilation(vertex_shader, &error_message)?;
            }

            let mut fragment_shader_str = fragment_shader_str.to_string();
            fragment_shader_str.push('\0');

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &(fragment_shader_str.as_ptr() as *const i8),
                null(),
            );
            gl::CompileShader(fragment_shader);

            #[cfg(debug_assertions)]
            {
                let error_message = format!("Fragment Shader Compilation Error: Shader was compiled from string:\n{fragment_shader_str:?}");
                Shader::check_shader_compilation(fragment_shader, &error_message)?;
            }

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);

            #[cfg(debug_assertions)]
            {
                let mut success = 0;
                let mut buffer = [0i8; 512];
                gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
                if success == 0 {
                    gl::GetProgramInfoLog(program_id, 512, null_mut(), &mut buffer[0]);
                    let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                    let string = format!(
                        "Shader Program Linking Failed:\n
                                          Vertex Shader:\n{vertex_shader_str}\n
                                          Fragment Shader:\n{fragment_shader_str}\n
                                          OpenGL Error: {string}"
                    );
                    return Err(WhirlwingError::new(
                        string.to_string(),
                        WhirlwingErrorKind::ShaderCompilationFailure,
                    ));
                }
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(Shader { program_id })
        }
    }

    pub fn set_bool(&self, name: &str, uniform: bool) {
        unsafe {
            let mut name = name.to_string();
            name.push('\0');
            let location = gl::GetUniformLocation(self.program_id, name.as_ptr() as *const i8);
            gl::Uniform1i(location, uniform as i32);
        }
    }

    pub fn set_int(&self, name: &str, uniform: i32) {
        unsafe {
            let mut name = name.to_string();
            name.push('\0');
            let location = gl::GetUniformLocation(self.program_id, name.as_ptr() as *const i8);
            gl::Uniform1i(location, uniform);
        }
    }

    pub fn set_float(&self, name: &str, uniform: f32) {
        unsafe {
            let mut name = name.to_string();
            name.push('\0');
            let location = gl::GetUniformLocation(self.program_id, name.as_ptr() as *const i8);
            gl::Uniform1f(location, uniform);
        }
    }

    pub fn set_mat4(&self, name: &str, uniform: &Matrix4) {
        unsafe {
            let mut name = name.to_string();
            name.push('\0');
            let location = gl::GetUniformLocation(self.program_id, name.as_ptr() as *const i8);
            gl::UniformMatrix4fv(location, 1, gl::FALSE, uniform.data.as_ptr());
        }
    }

    pub fn id(&self) -> u32 {
        self.program_id
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    #[cfg(debug_assertions)]
    fn check_shader_compilation(
        shader: u32,
        possible_error_message: &str,
    ) -> Result<(), WhirlwingError> {
        unsafe {
            let mut success = 0;
            let mut buffer = [0i8; 512];
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                gl::GetShaderInfoLog(shader, 512, null_mut(), &mut buffer[0]);
                let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                let string = format!("{possible_error_message}\nOpenGL Error: {string}");
                return Err(WhirlwingError::new(
                    string.to_string(),
                    WhirlwingErrorKind::ShaderCompilationFailure,
                ));
            }
            Ok(())
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}
