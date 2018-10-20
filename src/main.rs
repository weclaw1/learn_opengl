#![allow(dead_code)]

extern crate gl;
extern crate glutin;

mod triangle;

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
    let mut running = true;
    let shader_programs = unsafe { triangle::create_two_shader_programs() };
    let vaos = unsafe { triangle::create_two_vertex_array_objects_two_triangles() };
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

            gl::UseProgram(shader_programs[0]);
            gl::BindVertexArray(vaos[0]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::UseProgram(shader_programs[1]);
            gl::BindVertexArray(vaos[1]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        gl_window.swap_buffers().unwrap();
    }
}
