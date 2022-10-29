extern crate lba;
use std::time::Instant;


use lba::camera::Camera2D;
use lba::gl;

use lba::glfw::ffi::glfwGetMouseButton;
use lba::glam;
use lba::glfw;
use lba::glfw::ffi::glfwGetCursorPos;
use lba::glfw::ffi::glfwGetKey;
use lba::timer::Time;
extern crate rand;

mod render;
use render::Render;
//use lba::debug::message_callback;

use glfw::{Action, Context, Key};

static mut RUN_GAME_OF_LIFE:bool = true;
const SCREEN_WIDTH:f32 = 1000.0;
const SCREEN_HEIGHT:f32 = 1000.0;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, "GOL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    
    gl::load_with(|s| window.get_proc_address(s) as *const _);
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.set_swap_interval(lba::glfw::SwapInterval::Sync(1));
    
    window.set_framebuffer_size_polling(true);
    
    // unsafe {
    //     gl::Enable(gl::DEBUG_OUTPUT);
    //     gl::DebugMessageCallback(Some(message_callback),0 as *const gl::types::GLvoid);
    // }

    let mut render = Render::new("res/shaders/lifecompute.shader",
    "res/shaders/screenQuad.vs",
    "res/shaders/screenQuad.fs",
    glam::vec4(0.0,0.5,0.5,1.0));
    render.gen_square();
    render.gen_texture();
    render.bind_texture();

    let mut camera = Camera2D::new(1000.0,1000.0,0.0,50.0,1000.0);
    println!("{:?}",camera.proj);

    let mut delta = Instant::now();    
    let mut time = Time::new();

    while !window.should_close() {

        if time.check_delta(render.compute_interval) {
            if unsafe{RUN_GAME_OF_LIFE} == true {
                render.run_compute();
            }
            if unsafe {glfwGetKey(window.window_ptr(),glfw::Key::Equal as i32)} == glfw::Action::Press as i32 {
                render.change_draw_size(true);
            }
            if unsafe {glfwGetKey(window.window_ptr(),glfw::Key::Minus as i32)} == glfw::Action::Press as i32 {
                render.change_draw_size(false);
            }
            //println!("Frames {}",time.fps);
        }
        
        unsafe{gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);}

        render.draw_square(&camera);

        window.swap_buffers();

        handle_move_events(&mut window,&mut camera,&mut delta, &mut render);
        
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut camera,&mut render);
        }
    }
}

fn handle_move_events(window: &mut glfw::Window,camera:&mut Camera2D,deltai:&mut Instant, render:&mut Render) {
    let delta = deltai.elapsed().as_secs_f32()/10.0;    

    if unsafe {glfwGetKey(window.window_ptr(),glfw::Key::A as i32)} == glfw::Action::Press as i32 {
        camera.move_left(delta);
    }
    if unsafe {glfwGetKey(window.window_ptr(),glfw::Key::D as i32)} == glfw::Action::Press as i32 {
        camera.move_right(delta);
    }
    if unsafe {glfwGetKey(window.window_ptr(),glfw::Key::W as i32)} == glfw::Action::Press as i32 {
        camera.move_up(delta);
    }
    if unsafe {glfwGetKey(window.window_ptr(),glfw::Key::S as i32)} == glfw::Action::Press as i32 {
        camera.move_down(delta);
    }
    if unsafe {glfwGetKey(window.window_ptr(),glfw::Key::Space as i32)} == glfw::Action::Press as i32 {
        camera.zoom_in(delta);
    }
    if unsafe {glfwGetKey(window.window_ptr(),glfw::Key::LeftShift as i32)} == glfw::Action::Press as i32 {
        camera.zoom_out(delta);
    }
    if unsafe{glfwGetMouseButton(window.window_ptr(), glfw::MouseButtonLeft as i32)} == glfw::Action::Press as i32 {
        let mut xpos:f64= 0.0;
        let mut ypos:f64= 0.0;
        
        unsafe {glfwGetCursorPos(window.window_ptr(), &mut xpos as *mut f64, &mut ypos as *mut f64);}
        
        camera.scale_glfw_cursor(&mut xpos,&mut ypos);

        render.point_texture(xpos as f32, ypos as f32,false);
    }
    if unsafe{glfwGetMouseButton(window.window_ptr(), glfw::MouseButtonRight as i32)} == glfw::Action::Press as i32 {
        let mut xpos:f64= 0.0;
        let mut ypos:f64= 0.0;
        
        unsafe {glfwGetCursorPos(window.window_ptr(), &mut xpos as *mut f64, &mut ypos as *mut f64);}

        camera.scale_glfw_cursor(&mut xpos,&mut ypos);

        render.point_texture(xpos as f32, ypos as f32,true);
    }

    *deltai = Instant::now();

}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, camera:&mut Camera2D, render:&mut Render) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::Key(Key::R, _, Action::Press, _) => {
            unsafe {RUN_GAME_OF_LIFE = !RUN_GAME_OF_LIFE};
        }
        glfw::WindowEvent::Key(Key::T,_,Action::Press,_) => {
            println!("Camera proj {:?}",camera.proj);
            println!("Camera pos {:?}",camera.view);
            //*view += glam::Mat4::from_translation(glam::vec3(1.1, 1.1, 0.0));
        }
        glfw::WindowEvent::Key(Key::Minus,_,Action::Press,_) => {
            render.change_draw_size(false);
        }
        glfw::WindowEvent::Key(Key::Equal,_,Action::Press,_) => {
            render.change_draw_size(true);
        }
        glfw::WindowEvent::Key(Key::Up,_,Action::Press,_) => {
            render.compute_interval*=10.0;
            println!("Compute interval {}",render.compute_interval);
        }
        glfw::WindowEvent::Key(Key::Down,_,Action::Press,_) => {
            render.compute_interval/=10.0;
            println!("Compute interval {}",render.compute_interval);
        }
        glfw::WindowEvent::Key(Key::Num1,_,Action::Press,_) => {
            render.change_compute_shader("res/shaders/lifecompute.shader");
        }
        glfw::WindowEvent::Key(Key::Num2,_,Action::Press,_) => {
            render.change_compute_shader("res/shaders/wavescompute.shader");
        }
        glfw::WindowEvent::Key(Key::Num3,_,Action::Press,_) => {
            render.change_compute_shader("res/shaders/wormscompute.shader");
        }
        glfw::WindowEvent::Key(Key::Num4,_,Action::Press,_) => {
            render.change_compute_shader("res/shaders/slimecompute.shader");
        }
        glfw::WindowEvent::Key(Key::Num5,_,Action::Press,_) => {
            render.change_compute_shader("res/shaders/mitosiscompute.shader");
        }
        glfw::WindowEvent::Key(Key::Num0,_,Action::Press,_) => {
            render.reset_texture();
            render.bind_texture();
        }
        glfw::WindowEvent::Key(Key::Comma,_,Action::Press,_) => {
            render.color[0] += 0.1;
        }
        glfw::WindowEvent::Key(Key::Period,_,Action::Press,_) => {
            render.color[1] += 0.1;
        }
        glfw::WindowEvent::Key(Key::Slash,_,Action::Press,_) => {
            render.color[2] += 0.1;
        }
        glfw::WindowEvent::Key(Key::L,_,Action::Press,_) => {
            render.color[0] -= 0.1;
        }
        glfw::WindowEvent::Key(Key::Semicolon,_,Action::Press,_) => {
            render.color[1] -= 0.1;
        }
        glfw::WindowEvent::Key(Key::Apostrophe,_,Action::Press,_) => {
            render.color[2] -= 0.1;
        }
        glfw::WindowEvent::FramebufferSize(w,h) => {
            unsafe{gl::Viewport(0, 0, w, h)};
            //unsafe{SCREEN_WIDTH = w as f32; SCREEN_HEIGHT = h as f32;}
            camera.update_projection(w as f32,h as f32);
            println!("{:?}",camera.proj);

            //camera.update_projection(w as f32, h as f32);
            //camera.proj = glam::Mat4::perspective_rh_gl(1.0, w as f32/h as f32, 0.0, 50.0);
        }
        _ => {}
    }
}

