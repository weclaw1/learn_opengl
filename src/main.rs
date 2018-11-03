#![allow(dead_code)]

extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate image;

mod coordinate;
mod shaders;
mod textures;
mod triangle;
mod utils;

use std::ffi::{CStr, CString};
use std::path::Path;
use std::time::{Duration, Instant};

use glutin::dpi::*;
use glutin::ElementState::{Pressed, Released};
use glutin::VirtualKeyCode;
use glutin::WindowEvent::*;
use glutin::DeviceEvent::*;
use glutin::{Api, Event, EventsLoop, GlContext, GlRequest, GlWindow};

use cgmath::prelude::*;
use cgmath::{vec3, Deg, Matrix4, Point3, Rad, Vector3};

use utils::input::Input;
use utils::shader::Shader;

// settings
const SCR_WIDTH: f64 = 800.0;
const SCR_HEIGHT: f64 = 600.0;

const DURATION_PER_UPDATE: Duration = Duration::from_millis(16);

// camera
const CAMERA_UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let gl_window = create_gl_window(&events_loop);
    unsafe {
        configure_opengl(&gl_window);
    }

    run_game_loop(&mut events_loop, &gl_window);
}

fn create_gl_window(events_loop: &EventsLoop) -> GlWindow {
    let window = glutin::WindowBuilder::new()
        .with_title("SPOOKY")
        .with_dimensions(LogicalSize::new(SCR_WIDTH, SCR_HEIGHT));

    let context = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync(true);

    let gl_window = glutin::GlWindow::new(window, context, events_loop).unwrap();
    //gl_window.set_cursor_position(LogicalPosition::new(SCR_WIDTH / 2.0, SCR_HEIGHT / 2.0)).unwrap();
    gl_window.grab_cursor(true).unwrap();
    gl_window.hide_cursor(true);
    return gl_window;
}

unsafe fn configure_opengl(gl_window: &GlWindow) {
    gl_window.make_current().unwrap();
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}

fn run_game_loop(events_loop: &mut EventsLoop, gl_window: &GlWindow) {
    // configure global opengl state
    // -----------------------------
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // world space positions of our cubes
    let cube_positions: [Vector3<f32>; 10] = [
        vec3(0.0, 0.0, 0.0),
        vec3(2.0, 5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, -12.3),
        vec3(2.4, -0.4, -3.5),
        vec3(-1.7, 3.0, -7.5),
        vec3(1.3, -2.0, -2.5),
        vec3(1.5, 2.0, -2.5),
        vec3(1.5, 0.2, -1.5),
        vec3(-1.3, 1.0, -1.5),
    ];

    let mut cube_models: Vec<Matrix4<f32>> = cube_positions
        .iter()
        .map(|x| Matrix4::from_translation(*x))
        .enumerate()
        .map(|(i, x)| {
            x * Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(), Deg(i as f32 * 20.0))
        }).collect();

    let mut camera_position = Point3::new(0.0, 0.0, 3.0);
    let mut camera_front: Vector3<f32> = Vector3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    let mut running = true;
    let mut previous_time = Instant::now();
    let mut lag = Duration::new(0, 0);
    let shader_program = Shader::new(
        Path::new("src/shaders/coordinate.vs"),
        Path::new("src/shaders/coordinate.fs"),
    );

    let vao = unsafe { coordinate::create_vertex_array_object() };
    let (texture_1, texture_2) = unsafe {
        coordinate::load_and_create_textures(
            Path::new("resources/crate.jpg"),
            Path::new("resources/pumpkin.jpg"),
        )
    };

    // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
    // -------------------------------------------------------------------------------------------
    unsafe {
        shader_program.use_program();
        shader_program.set_int(&CString::new("texture_1").unwrap(), 0);
        shader_program.set_int(&CString::new("texture_2").unwrap(), 1);
    }

    let mut model: Matrix4<f32> =
        Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(), Deg(50.0));
    let projection: Matrix4<f32> =
        cgmath::perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);

    let mut input: Input = Input::new();

    while running {
        let elapsed = previous_time.elapsed();
        previous_time = Instant::now();
        lag += elapsed;

        process_input(&mut input, events_loop, gl_window);

        while lag >= DURATION_PER_UPDATE {
            //create transformations
            let camera_speed = 5.0 * DURATION_PER_UPDATE.subsec_millis() as f32 / 1000.0;

            if input.up() {
                camera_position += camera_speed * camera_front;
            }
            if input.down() {
                camera_position += -(camera_speed * camera_front);
            }
            if input.left() {
                camera_position += -(camera_front.cross(CAMERA_UP).normalize() * camera_speed);
            }
            if input.right() {
                camera_position += camera_front.cross(CAMERA_UP).normalize() * camera_speed;
            }
            if input.close() {
                running = false;
            }

            cube_models = cube_models
                .iter()
                .map(|x| x * Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(), Deg(2.0)))
                .collect();

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

            camera_front = Vector3 {
                x: input.yaw().to_radians().cos() * input.pitch().to_radians().cos(),
                y: input.pitch().to_radians().sin(),
                z: input.yaw().to_radians().sin() * input.pitch().to_radians().cos(),
            };

            camera_front.normalize();

            let view: Matrix4<f32> = Matrix4::look_at(camera_position, camera_position + camera_front, CAMERA_UP);
            shader_program.set_matrix4(&CString::new("view").unwrap(), &view);
            shader_program.set_matrix4(&CString::new("projection").unwrap(), &projection);

            gl::BindVertexArray(vao);

            for cube_model in cube_models.iter() {
                shader_program.set_matrix4(&CString::new("model").unwrap(), &cube_model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
        gl_window.swap_buffers().unwrap();
    }
}

fn process_input(input: &mut Input, events_loop: &mut EventsLoop, gl_window: &GlWindow) {
    events_loop.poll_events(|event| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                CloseRequested => input.set_close(true),
                Resized(logical_size) => {
                    let dpi_factor = gl_window.get_hidpi_factor();
                    gl_window.resize(logical_size.to_physical(dpi_factor));
                }
                KeyboardInput {
                    input:
                        glutin::KeyboardInput {
                            virtual_keycode: Some(key),
                            state: Pressed,
                            ..
                        },
                    ..
                } => match key {
                    VirtualKeyCode::W => input.set_up(true),
                    VirtualKeyCode::S => input.set_down(true),
                    VirtualKeyCode::A => input.set_left(true),
                    VirtualKeyCode::D => input.set_right(true),
                    VirtualKeyCode::Escape => input.set_close(true),
                    _ => (),
                },
                KeyboardInput {
                    input:
                        glutin::KeyboardInput {
                            virtual_keycode: Some(key),
                            state: Released,
                            ..
                        },
                    ..
                } => match key {
                    VirtualKeyCode::W => input.set_up(false),
                    VirtualKeyCode::S => input.set_down(false),
                    VirtualKeyCode::A => input.set_left(false),
                    VirtualKeyCode::D => input.set_right(false),
                    _ => (),
                },
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                MouseMotion { delta } => {
                    let (x_delta, y_delta) = (delta.0 as f32, delta.1 as f32);
                    let sensitivity: f32 = 0.1;

                    let current_yaw = input.yaw();
                    input.set_yaw(current_yaw + x_delta * sensitivity);

                    let current_pitch = input.pitch();
                    input.set_pitch(current_pitch + (-y_delta) * sensitivity);
                },
                _ => (),
            },
            _ => (),
        }
    });
}
