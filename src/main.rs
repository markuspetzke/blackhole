extern crate glfw;
use glfw::{Action, Context, Key, fail_on_errors};
extern crate gl;
use glam::{Mat4, Vec3};
use std::{ffi::CString, fs};

const SRC_WIDTH: u32 = 800;
const SRC_HEIGHT: u32 = 600;

fn load_shader_source(path: &str) -> CString {
    let source = fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file"));
    CString::new(source).unwrap()
}

fn window() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    let (mut window, events) = glfw
        .create_window(
            SRC_WIDTH,
            SRC_HEIGHT,
            "Test Title",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create Window");
    window.make_current();
    gl::load_with(|s| {
        window
            .get_proc_address(s)
            .map_or(std::ptr::null(), |p| p as *const _)
    });
    window.set_key_polling(true);

    // Shader sources mit null terminator
    let vertex_source = load_shader_source("./shader/vertex.glsl");

    let fragment_source = load_shader_source("./shader/fragment.glsl");

    let shader_program = unsafe {
        // Vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex_ptr = vertex_source.as_ptr();
        gl::ShaderSource(vertex_shader, 1, &vertex_ptr, std::ptr::null());
        gl::CompileShader(vertex_shader);

        // Fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_ptr = fragment_source.as_ptr();
        gl::ShaderSource(fragment_shader, 1, &fragment_ptr, std::ptr::null());
        gl::CompileShader(fragment_shader);

        // Shader program
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    };

    let vertices: Vec<f32> = vec![
        50.0, 50.0, 0.0, // oben rechts
        50.0, -50.0, 0.0, // unten rechts
        -50.0, -50.0, 0.0, // unten links
        -50.0, 50.0, 0.0, // oben links
    ];
    let indices: Vec<u32> = vec![
        0, 1, 3, // erstes Dreieck
        1, 2, 3, // zweites Dreieck
    ];

    let mut vbo: u32 = 0;
    let mut vao: u32 = 0;
    let mut ebo: u32 = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        //VBO
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        //EBO
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        //position
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Render loop
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            let time = glfw.get_time() as f32;

            gl::UseProgram(shader_program);

            let ortho = glam::Mat4::orthographic_rh_gl(0.0, 800.0, 0.0, 600.0, -1.0, 1.0);

            let mut model = glam::Mat4::IDENTITY;

            model *= Mat4::from_translation(Vec3::new(400., 300., 0.0));
            model *= Mat4::from_axis_angle(Vec3::Z, time);

            let transform = ortho * model;

            let transformloc =
                gl::GetUniformLocation(shader_program, "transform".as_ptr() as *const i8);
            gl::UniformMatrix4fv(
                transformloc,
                1,
                gl::FALSE,
                &transform as *const Mat4 as *const f32,
            );
            gl::BindVertexArray(vao);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true)
            }
        }
        window.swap_buffers();
    }
}

fn main() {
    window();
}
