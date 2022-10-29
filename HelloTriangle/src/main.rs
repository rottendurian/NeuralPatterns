
extern crate lba;
use lba::shader;
use lba::gl;
use lba::glfw;

use glfw::{Action, Context, Key};
use shader::Shader;
use std::{mem::size_of};
use lba::gl_errors::message_callback;



fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(std::option::Option::Some(message_callback),0 as *const gl::types::GLvoid);
    }



    let mut vao:u32 = 0;
    let mut vbo:u32 = 0;
    let array:Vec<f32> = vec![
         -0.5, -0.5, 0.0,
             0.5, -0.5, 0.0,
             0.0,  0.5, 0.0
        ];
    

    let shader = shader::Program::new::<usize,2>(
    [Shader::new("res/shaders/vertex.shader",gl::VERTEX_SHADER),
             Shader::new("res/shaders/fragment.shader",gl::FRAGMENT_SHADER)]);
    shader.bind();
    
    
    unsafe {
        gl::CreateVertexArrays(1,&mut vao);
        gl::CreateBuffers(1,&mut vbo);
        
        gl::NamedBufferData(vbo,(array.len() * size_of::<f32>()) as gl::types::GLsizeiptr,
        array.as_ptr() as *const gl::types::GLvoid,gl::STATIC_DRAW);        

        gl::EnableVertexArrayAttrib(vao,0);
        gl::VertexArrayAttribBinding(vao,0,0);
        gl::VertexArrayAttribFormat(vao,0,3,gl::FLOAT,gl::FALSE,0);

        gl::VertexArrayVertexBuffer(vao,0,vbo,0,5*size_of::<f32>() as i32);

    }



    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            shader.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

