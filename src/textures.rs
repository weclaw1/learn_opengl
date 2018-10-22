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
    let vertices: [f32; 32] = [
        // positions     // colors        // texture coords
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
    ];

    let indices = [
        0, 1, 3, // first Triangle
        1, 2, 3, // second Triangle
    ];

    let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW,
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &indices[0] as *const i32 as *const c_void,
        gl::STATIC_DRAW,
    );

    let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
    // position attribute
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    // color attribute
    gl::VertexAttribPointer(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );
    gl::EnableVertexAttribArray(1);
    // texture coord attribute
    gl::VertexAttribPointer(
        2,
        2,
        gl::FLOAT,
        gl::FALSE,
        stride,
        (6 * mem::size_of::<GLfloat>()) as *const c_void,
    );
    gl::EnableVertexAttribArray(2);

    return vao;
}

pub unsafe fn load_and_create_texture(file_path: &Path) -> GLuint {
    let mut texture = 0;
    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
                                              // set the texture wrapping parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    // set texture filtering parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    // load image, create texture and generate mipmaps
    let img = image::open(file_path).expect("Failed to load texture");
    
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

    return texture;
}
