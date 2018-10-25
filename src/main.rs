#![allow(dead_code)]

extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate image;

mod shaders;
mod textures;
mod triangle;
mod utils;

use std::ffi::{CStr, CString};
use std::path::Path;
use std::time::{Duration, Instant};

use glutin::dpi::*;
use glutin::{Api, EventsLoop, GlContext, GlRequest, GlWindow};

use cgmath::prelude::*;
use cgmath::{Matrix4, vec3, Rad, Deg};

use utils::shader::Shader;

// settings
const SCR_WIDTH: f64 = 800.0;
const SCR_HEIGHT: f64 = 600.0;

const MS_PER_FRAME: u64 = 16;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let gl_window = create_gl_window(&events_loop);
    unsafe {
        configure_opengl(&gl_window);
    }

    run_event_loop(&mut events_loop, &gl_window);
}

fn create_gl_window(events_loop: &EventsLoop) -> GlWindow {
    let window = glutin::WindowBuilder::new()
        .with_title("SMOKE WEED EVERYDAY")
        .with_dimensions(LogicalSize::new(SCR_WIDTH, SCR_HEIGHT));

    let context = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync(true);

    return glutin::GlWindow::new(window, context, events_loop).unwrap();
}

unsafe fn configure_opengl(gl_window: &GlWindow) {
    gl_window.make_current().unwrap();
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}

fn run_event_loop(events_loop: &mut EventsLoop, gl_window: &GlWindow) {
    let mut running = true;
    let shader_program = Shader::new(
        Path::new("src/shaders/transform.vs"),
        Path::new("src/shaders/transform.fs"),
    );

    let vao = unsafe { textures::create_vertex_array_object() };
    let texture = unsafe { textures::load_and_create_texture( Path::new("resources/snoop_dogg.jpg") ) };

    let mut transform_matrix: Matrix4<f32> = Matrix4::identity();

    while running {
        let start_time = Instant::now();
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                glutin::WindowEvent::Resized(logical_size) => {
                    let dpi_factor = gl_window.get_hidpi_factor();
                    gl_window.resize(logical_size.to_physical(dpi_factor));
                }
                _ => (),
            },
            _ => (),
        });
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            //create transformations
            
            //transform_matrix = transform_matrix * Matrix4::<f32>::from_translation(vec3(0.5, -0.5, 0.0));
            transform_matrix = transform_matrix * Matrix4::<f32>::from_angle_z(Deg(2.0));
            let transform_matrix_window_scale = transform_matrix * Matrix4::<f32>::from_nonuniform_scale(1.0, (SCR_HEIGHT / SCR_WIDTH) as f32, 1.0);

            shader_program.use_program();
            shader_program.set_matrix4(&CString::new("transform").unwrap(), &transform_matrix_window_scale);

            gl::BindVertexArray(vao);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
        gl_window.swap_buffers().unwrap();
        if let Some(remaining_duration) = Duration::from_millis(MS_PER_FRAME).checked_sub(start_time.elapsed()) {
            std::thread::sleep(remaining_duration);
        } 
    }
}
