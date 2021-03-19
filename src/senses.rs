pub mod senses {
    use gl;
    use std;
    use std::ffi::{CStr, CString};

    struct Program {
        id: gl::types::GLuint;
    }
    
    impl Program {
        pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
            let program_id = unsafe { gl::CreateProgram() };

            for shader in shaders {
                unsafe { gl::AttachShader(program_id, shader.id); }
        }
    }


    struct Shader {
        id: gl::types::GLuint,
    }

    impl Shader {
        pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
            let id = shader_from_source(source, kind)?;
            Ok(Shader { id })
        }
        pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
            Shader::from_source(source, gl::VERTEX_SHADER)
        }
        pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
            Shader::from_source(source, gl::FRAGMENT_SHADER)
        }
    }

    impl Drop for Shader {
        fn drop(&mut self) {
            unsafe { gl::DeleteShader(self.id) };
        }
    }

    fn shader_from_source(
        source: &CStr,
        kind: gl::types::GLenum,
    ) -> Result<gl::types::GLuint, String> {
        let id = unsafe { gl::CreateShader(kind) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;

        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            //NOTE(stanisz): length of the error message needed
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }
        Ok(id)
    }

    fn create_whitespace_cstring_with_len(len: usize) -> CString {
        //NOTE(stanisz): creates a vector, that is null-terminated
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        //NOTE(stanisz): [b' '] is a single-item array, that is looped
        // over using cycle() until 'len' elements are taken
        buffer.extend([b' '].iter().cycle().take(len as usize));
        //NOTE(stanisz): create an error string converting the Vec into CString
        // for OpenGL
        unsafe { CString::from_vec_unchecked(buffer) }
    }
}
