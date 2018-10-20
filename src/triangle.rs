use std::ffi::{CString};
use std::ptr;
use std::mem;
use std::str;
use std::os::raw::c_void;

use gl;
use gl::types::{GLchar, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint};



const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const ORANGE_FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

const YELLOW_FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 1.0f, 0.0f, 1.0f);
    }
"#;

pub unsafe fn create_shader_program() -> GLuint {
    // build and compile our shader program
    // ------------------------------------
    // vertex shader
    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
    gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(vertex_shader);

    // check for shader compile errors
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(vertex_shader, 512,ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }

    // fragment shader
    let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    let c_str_frag = CString::new(ORANGE_FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
    gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
    gl::CompileShader(fragment_shader);
    // check for shader compile errors
    gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }

    // link shaders
    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);
    // check for linking errors
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }
    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    return shader_program;
}

pub unsafe fn create_two_shader_programs() -> [GLuint; 2] {
    // build and compile our shader program
    // ------------------------------------
    // vertex shader
    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
    gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(vertex_shader);

    // check for shader compile errors
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(vertex_shader, 512,ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }

    // fragment shader
    let fragment_shader_1 = gl::CreateShader(gl::FRAGMENT_SHADER);
    let c_str_frag = CString::new(ORANGE_FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
    gl::ShaderSource(fragment_shader_1, 1, &c_str_frag.as_ptr(), ptr::null());
    gl::CompileShader(fragment_shader_1);
    // check for shader compile errors
    gl::GetShaderiv(fragment_shader_1, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(fragment_shader_1, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }

    // fragment shader_2
    let fragment_shader_2 = gl::CreateShader(gl::FRAGMENT_SHADER);
    let c_str_frag_2 = CString::new(YELLOW_FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
    gl::ShaderSource(fragment_shader_2, 1, &c_str_frag_2.as_ptr(), ptr::null());
    gl::CompileShader(fragment_shader_2);
    // check for shader compile errors
    gl::GetShaderiv(fragment_shader_2, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(fragment_shader_2, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }

    // link shaders
    let shader_program_1 = gl::CreateProgram();
    gl::AttachShader(shader_program_1, vertex_shader);
    gl::AttachShader(shader_program_1, fragment_shader_1);
    gl::LinkProgram(shader_program_1);
    // check for linking errors
    gl::GetProgramiv(shader_program_1, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(shader_program_1, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }
    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader_1);


    let shader_program_2 = gl::CreateProgram();
    gl::AttachShader(shader_program_2, vertex_shader);
    gl::AttachShader(shader_program_2, fragment_shader_2);
    gl::LinkProgram(shader_program_2);
    // check for linking errors
    gl::GetProgramiv(shader_program_2, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(shader_program_2, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
    }
    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader_2);

    return [shader_program_1, shader_program_2];
}

pub unsafe fn create_vertex_array_object() -> GLuint {
    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    // HINT: type annotation is crucial since default for float literals is f64
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top
    ];

    let (mut vbo, mut vao) = (0, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER, 
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, 
                   &vertices[0] as *const f32 as *const c_void, 
                   gl::STATIC_DRAW);

    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    gl::EnableVertexAttribArray(0);

    // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

    // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
    // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
    gl::BindVertexArray(0);

    // uncomment this call to draw in wireframe polygons.
    // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

    return vao;
}

pub unsafe fn create_vertex_array_object_two_triangles() -> GLuint {
    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    // HINT: type annotation is crucial since default for float literals is f64
    let vertices: [f32; 18] = [
        -1.0, -0.5, 0.0, 
        0.0, -0.5, 0.0,
        -0.5, 0.5, 0.0,
        0.0, -0.5, 0.0, 
        1.0, -0.5, 0.0,
        0.5, 0.5, 0.0,
    ];

    let (mut vbo, mut vao) = (0, 0);
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER, 
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, 
                   &vertices[0] as *const f32 as *const c_void, 
                   gl::STATIC_DRAW);

    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    gl::EnableVertexAttribArray(0);

    // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

    // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
    // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
    gl::BindVertexArray(0);

    // uncomment this call to draw in wireframe polygons.
    // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

    return vao;
}

pub unsafe fn create_two_vertex_array_objects_two_triangles() -> [GLuint; 2] {
    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    // HINT: type annotation is crucial since default for float literals is f64
    let vertices_1: [f32; 9] = [
        -1.0, -0.5, 0.0, 
        0.0, -0.5, 0.0,
        -0.5, 0.5, 0.0,
    ];

    let vertices_2: [f32; 9] = [
        0.0, -0.5, 0.0, 
        1.0, -0.5, 0.0,
        0.5, 0.5, 0.0,
    ];

    let (mut vaos, mut vbos) = ([0; 2], [0; 2]);
    gl::GenVertexArrays(2, &mut vaos[0]);
    gl::GenBuffers(2, &mut vbos[0]);
    // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
    gl::BindVertexArray(vaos[0]);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[0]);
    gl::BufferData(gl::ARRAY_BUFFER, 
                   (vertices_1.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, 
                   &vertices_1[0] as *const f32 as *const c_void, 
                   gl::STATIC_DRAW);

    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    gl::EnableVertexAttribArray(0);

    gl::BindVertexArray(vaos[1]);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[1]);
    gl::BufferData(gl::ARRAY_BUFFER, 
                   (vertices_2.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, 
                   &vertices_2[0] as *const f32 as *const c_void, 
                   gl::STATIC_DRAW);

    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    gl::EnableVertexAttribArray(0);

    // uncomment this call to draw in wireframe polygons.
    // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

    return vaos;
}
