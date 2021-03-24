extern crate gl;
extern crate sdl2;

use std::convert::TryInto;
//use resources::Resources;
use std::path::Path;

pub mod senses;
//pub mod resources;

fn main() {
    //let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

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

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    use std::ffi::CString;

    let vertex_shader =
        senses::Shader::from_vert_source(&CString::new(include_str!("triangle.vs")).unwrap())
            .unwrap();
    let fragment_shader =
        senses::Shader::from_frag_source(&CString::new(include_str!("triangle.fs")).unwrap())
            .unwrap();

    let shader_program = senses::Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();

    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
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
            gl::Viewport(
                0,
                0,
                window_width.try_into().unwrap(),
                window_height.try_into().unwrap(),
            );
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }
}
