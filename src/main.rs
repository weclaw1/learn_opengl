#![allow(dead_code)]

extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate image;

mod shaders;
mod textures;
mod triangle;
mod coordinate;
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

const DURATION_PER_UPDATE: Duration = Duration::from_millis(16);

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
    // configure global opengl state
    // -----------------------------
    unsafe { gl::Enable(gl::DEPTH_TEST); }

    let mut running = true;
    let mut previous_time = Instant::now();
    let mut lag = Duration::new(0, 0);
    let shader_program = Shader::new(
        Path::new("src/shaders/coordinate.vs"),
        Path::new("src/shaders/coordinate.fs"),
    );

    let vao = unsafe { coordinate::create_vertex_array_object() };
    let (texture_1, texture_2) = unsafe { coordinate::load_and_create_textures( Path::new("resources/crate.jpg"), Path::new("resources/snoop_dogg.jpg") ) };

    // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
    // -------------------------------------------------------------------------------------------
    unsafe {
        shader_program.use_program();
        shader_program.set_int(&CString::new("texture_1").unwrap(), 0);
        shader_program.set_int(&CString::new("texture_2").unwrap(), 1);
    }

    let mut model: Matrix4<f32> = Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(), Deg(50.0));
    let view: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, -3.0));
    let projection: Matrix4<f32> = cgmath::perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

    while running {
        let elapsed = previous_time.elapsed();
        previous_time = Instant::now();
        lag += elapsed;
        
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
        while lag >= DURATION_PER_UPDATE {
            //create transformations
            
            //transform_matrix = transform_matrix * Matrix4::<f32>::from_translation(vec3(0.5, -0.5, 0.0));
            //transform_matrix = transform_matrix * Matrix4::<f32>::from_angle_z(Deg(2.0));
            model = model * Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(), Deg(2.0));

            lag -= DURATION_PER_UPDATE;
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture_2);

            shader_program.use_program();
            shader_program.set_matrix4(&CString::new("model").unwrap(), &model);
            shader_program.set_matrix4(&CString::new("view").unwrap(), &view);
            shader_program.set_matrix4(&CString::new("projection").unwrap(), &projection);

            gl::BindVertexArray(vao);

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
        gl_window.swap_buffers().unwrap();
    }
}
