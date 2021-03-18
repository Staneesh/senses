extern crate gl;
extern crate sdl2;

use std::convert::TryInto;
use std::ffi::{CStr, CString};

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let (mut window_width, mut window_height) = (900, 700);
    let window = video_subsystem
        .window("senses", window_width, window_height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    //TODO(stanisz): remove this unsafe block
    unsafe {
        gl::Viewport(
            0,
            0,
            window_width.try_into().unwrap(),
            window_height.try_into().unwrap(),
        );
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'game_loop,
                _ => {}
            }
        }

        {
            let wsz = window.size();
            window_width = wsz.0;
            window_height = wsz.1;

            println!("{}, {}", window_width, window_height);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}

fn create_empty_cstring_with_len(len: usize) -> CString {
    //NOTE(stanisz): creates a vector, that is null-terminated
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    //NOTE(stanisz): [b' '] is a single-item array, that is looped
    // over using cycle() until 'len' elements are taken
    buffer.extend([b' '].iter().cycle().take(len as usize));
    //NOTE(stanisz): create an error string converting the Vec into CString
    // for OpenGL
    unsafe { CString::from_vec_unchecked(buffer) }
}

struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }
    fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }
    fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
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

        let error = create_empty_cstring_with_len(len as usize);

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
