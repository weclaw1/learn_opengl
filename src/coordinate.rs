use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;
use std::path::Path;

use gl;
use gl::types::{GLchar, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint};

use image;
use image::GenericImageView;

pub unsafe fn create_vertex_array_object() -> GLuint {
    let vertices: [f32; 180] = [
         -0.5, -0.5, -0.5,  0.0, 0.0,
          0.5, -0.5, -0.5,  1.0, 0.0,
          0.5,  0.5, -0.5,  1.0, 1.0,
          0.5,  0.5, -0.5,  1.0, 1.0,
         -0.5,  0.5, -0.5,  0.0, 1.0,
         -0.5, -0.5, -0.5,  0.0, 0.0,

         -0.5, -0.5,  0.5,  0.0, 0.0,
          0.5, -0.5,  0.5,  1.0, 0.0,
          0.5,  0.5,  0.5,  1.0, 1.0,
          0.5,  0.5,  0.5,  1.0, 1.0,
         -0.5,  0.5,  0.5,  0.0, 1.0,
         -0.5, -0.5,  0.5,  0.0, 0.0,

         -0.5,  0.5,  0.5,  1.0, 0.0,
         -0.5,  0.5, -0.5,  1.0, 1.0,
         -0.5, -0.5, -0.5,  0.0, 1.0,
         -0.5, -0.5, -0.5,  0.0, 1.0,
         -0.5, -0.5,  0.5,  0.0, 0.0,
         -0.5,  0.5,  0.5,  1.0, 0.0,

          0.5,  0.5,  0.5,  1.0, 0.0,
          0.5,  0.5, -0.5,  1.0, 1.0,
          0.5, -0.5, -0.5,  0.0, 1.0,
          0.5, -0.5, -0.5,  0.0, 1.0,
          0.5, -0.5,  0.5,  0.0, 0.0,
          0.5,  0.5,  0.5,  1.0, 0.0,

         -0.5, -0.5, -0.5,  0.0, 1.0,
          0.5, -0.5, -0.5,  1.0, 1.0,
          0.5, -0.5,  0.5,  1.0, 0.0,
          0.5, -0.5,  0.5,  1.0, 0.0,
         -0.5, -0.5,  0.5,  0.0, 0.0,
         -0.5, -0.5, -0.5,  0.0, 1.0,

         -0.5,  0.5, -0.5,  0.0, 1.0,
          0.5,  0.5, -0.5,  1.0, 1.0,
          0.5,  0.5,  0.5,  1.0, 0.0,
          0.5,  0.5,  0.5,  1.0, 0.0,
         -0.5,  0.5,  0.5,  0.0, 0.0,
         -0.5,  0.5, -0.5,  0.0, 1.0
    ];

    let (mut vbo, mut vao) = (0, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW,
    );

    let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
    // position attribute
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    // texture coord attribute
    gl::VertexAttribPointer(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );
    gl::EnableVertexAttribArray(1);

    return vao;
}

pub unsafe fn load_and_create_textures(file_path_1: &Path, file_path_2: &Path) -> (GLuint, GLuint) {
    let (mut texture_1, mut texture_2) = (0, 0);

    //texture 1
    gl::GenTextures(1, &mut texture_1);
    gl::BindTexture(gl::TEXTURE_2D, texture_1);
    // set texture wrapping parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    // set texture filtering parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    // load image, create texture and generate mipmaps
    let img = image::open(file_path_1).expect("Failed to load texture");
    let img = img.flipv();
    let data = img.raw_pixels();

    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGB as i32,
        img.width() as i32,
        img.height() as i32,
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);

    //texture 2
    gl::GenTextures(1, &mut texture_2);
    gl::BindTexture(gl::TEXTURE_2D, texture_2);
    // set texture wrapping parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    // set texture filtering parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    // load image, create texture and generate mipmaps
    let img = image::open(file_path_2).expect("Failed to load texture");
    let img = img.flipv();
    let data = img.raw_pixels();

    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGB as i32,
        img.width() as i32,
        img.height() as i32,
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);

    return (texture_1, texture_2);
}
