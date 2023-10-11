use wwg_error::{WhirlwingError, WhirlwingErrorKind};
use std::str::from_utf8_unchecked;

pub(crate) struct Shader {
    program_id: u32,
}

impl Shader {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Result<Shader, WhirlwingError> {
        unsafe {
            let mut vertex_shader_vector;
            match std::fs::read(vertex_shader_path) {
                Ok(vec) => {
                    vertex_shader_vector = vec;
                    vertex_shader_vector.push(b'\0');
                },
                Err(error) => {
                    let error_content = format!("Failed to read from path: {vertex_shader_path}");
                    return Err(WhirlwingError::new_with_source(error_content, WhirlwingErrorKind::ShaderCompilationFailure, Box::new(error)))
                },
            }
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &(vertex_shader_vector.as_ptr() as *const i8), std::ptr::null());
            gl::CompileShader(vertex_shader);

            #[cfg(debug_assertions)]
            {
                let mut success = 0;
                let mut buffer = [0i8; 512];
                gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
                if success == 0 {
                    gl::GetShaderInfoLog(vertex_shader, 512, std::ptr::null_mut(), &mut buffer[0]);
                    let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                    let string = format!("Vertex Shader Compilation Error: {vertex_shader_path}\nOpenGL Error: {string}");
                    return Err(WhirlwingError::new(string.to_string(), WhirlwingErrorKind::ShaderCompilationFailure))
                }
            }

            let mut fragment_shader_vector;
            match std::fs::read(fragment_shader_path) {
                Ok(vec) => {
                    fragment_shader_vector = vec;
                    fragment_shader_vector.push(b'\0');
                },
                Err(error) => {
                    let error_content = format!("Failed to read from path: {fragment_shader_path}");
                    return Err(WhirlwingError::new_with_source(error_content, WhirlwingErrorKind::ShaderCompilationFailure, Box::new(error)))
                },
            }
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &(fragment_shader_vector.as_ptr() as *const i8), std::ptr::null());
            gl::CompileShader(fragment_shader);

            #[cfg(debug_assertions)]
            {
                let mut success = 0;
                let mut buffer = [0i8; 512];
                gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
                if success == 0 {
                    gl::GetShaderInfoLog(fragment_shader, 512, std::ptr::null_mut(), &mut buffer[0]);
                    let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                    let string = format!("Fragment Shader Compilation Error: {fragment_shader_path}\nOpenGL Error: {string}");
                    return Err(WhirlwingError::new(string.to_string(), WhirlwingErrorKind::ShaderCompilationFailure))
                }
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
                    gl::GetProgramInfoLog(program_id, 512, std::ptr::null_mut(), &mut buffer[0]);
                    let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                    let string = format!("Shader Program Linking Failed:\n
                                          Vertex Shader: {vertex_shader_path}\n
                                          Fragment Shader: {fragment_shader_path}\n
                                          OpenGL Error: {string}");
                    return Err(WhirlwingError::new(string.to_string(), WhirlwingErrorKind::ShaderCompilationFailure))
                }
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(Shader { program_id })
        }
    }

    #[allow(dead_code)]
    pub fn from_utf8_slices(vertex_shader_slice: &[u8], fragment_shader_slice: &[u8]) -> Result<Shader, WhirlwingError> {
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &(vertex_shader_slice.as_ptr() as *const i8), std::ptr::null());
            gl::CompileShader(vertex_shader);

            #[cfg(debug_assertions)]
            {
                let mut success = 0;
                let mut buffer = [0i8; 512];
                gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
                if success == 0 {
                    gl::GetShaderInfoLog(vertex_shader, 512, std::ptr::null_mut(), &mut buffer[0]);
                    let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                    let string = format!("Vertex Shader Compilation Error: Shader was compiled from slice: {vertex_shader_slice:?}\nOpenGL Error: {string}");
                    return Err(WhirlwingError::new(string.to_string(), WhirlwingErrorKind::ShaderCompilationFailure))
                }
            }

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &(fragment_shader_slice.as_ptr() as *const i8), std::ptr::null());
            gl::CompileShader(fragment_shader);

            #[cfg(debug_assertions)]
            {
                let mut success = 0;
                let mut buffer = [0i8; 512];
                gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
                if success == 0 {
                    gl::GetShaderInfoLog(fragment_shader, 512, std::ptr::null_mut(), &mut buffer[0]);
                    let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                    let string = format!("Fragment Shader Compilation Error: Shader was compiled from slice{fragment_shader_slice:?}\nOpenGL Error: {string}");
                    return Err(WhirlwingError::new(string.to_string(), WhirlwingErrorKind::ShaderCompilationFailure))
                }
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
                    gl::GetProgramInfoLog(program_id, 512, std::ptr::null_mut(), &mut buffer[0]);
                    let string = from_utf8_unchecked(&*(buffer.as_ptr() as *const [u8; 512]));
                    let string = format!("Shader Program Linking Failed:\n
                                          Vertex Shader: {vertex_shader_slice:?}\n
                                          Fragment Shader: {fragment_shader_slice:?}\n
                                          OpenGL Error: {string}");
                    return Err(WhirlwingError::new(string.to_string(), WhirlwingErrorKind::ShaderCompilationFailure))
                }
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(Shader { program_id })
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.program_id); }       
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program_id); }
    }
}