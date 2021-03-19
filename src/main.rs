extern crate gl;
extern crate sdl2;

use std::convert::TryInto;

pub mod senses;

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

    unsafe {
        gl::Viewport(
            0,
            0,
            window_width.try_into().unwrap(),
            window_height.try_into().unwrap(),
        );
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
    shader_program.set_used();

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
