#![allow(dead_code)]

extern crate gl;
extern crate glutin;
extern crate cgmath;
extern crate image;

mod triangle;
mod shaders;
mod utils;
mod textures;

use std::time::{Duration, Instant};
use std::ffi::{CString};

use glutin::dpi::*;
use glutin::{GlContext, GlRequest, GlWindow, EventsLoop, Api};

// settings
const SCR_WIDTH: f64 = 800.0;
const SCR_HEIGHT: f64 = 600.0;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let gl_window = create_gl_window(&events_loop);
    unsafe { configure_opengl(&gl_window); }

    run_event_loop(&mut events_loop, &gl_window);
}

fn create_gl_window(events_loop: &EventsLoop) -> GlWindow {
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(LogicalSize::new(SCR_WIDTH, SCR_HEIGHT));

    let context = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3,3)))
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
    //let start_time = Instant::now();
    let mut running = true;
    let shader_program = unsafe { shaders::create_shader_program_with_color() };
    let vao = unsafe { shaders::create_vertex_array_object_with_colors() };
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => ()
                },
                _ => ()
            }
        });
        unsafe { 
            gl::Clear(gl::COLOR_BUFFER_BIT); 

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);

            //let time_since_start = (start_time.elapsed().as_secs() * 10 + (start_time.elapsed().subsec_millis() / 100) as u64) as f32;
            //let green_value = time_since_start.sin() / 2.0 + 0.5;
            //let our_color = CString::new("ourColor").unwrap();
            //let vertex_color_location = gl::GetUniformLocation(shader_program, our_color.as_ptr());
            //gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        gl_window.swap_buffers().unwrap();
    }
}
