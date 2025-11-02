extern crate glfw;
use glfw::{Action, Context, Key, fail_on_errors};
extern crate gl;
use glam::{Mat4, Vec2, Vec3};
use std::{ffi::CString, fs};

mod square_obj;
use square_obj::SquareObject;

mod ball_obj;
use ball_obj::BallObject;

mod collision;
use collision::*;

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

    let mut square = SquareObject::new(
        Vec3::new(400.0, 300.0, 0.0),
        100.0,
        Vec3::new(0.5, 0.5, 0.2),
    );

    let mut ball = BallObject::new(
        Vec3::new(400.0, 300.0, 0.0),
        Vec2::ZERO,
        100.0,
        Vec3::new(0.5, 0.5, 0.2),
    );
    let mut last_time = glfw.get_time() as f32;
    let speed = 20000.0;

    // Render loop
    while !window.should_close() {
        let current_time = glfw.get_time() as f32;
        let delta_time = current_time - last_time;

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // square.rotation = glfw.get_time() as f32;

            let ortho =
                Mat4::orthographic_rh_gl(0.0, SRC_WIDTH as f32, 0.0, SRC_HEIGHT as f32, -1.0, 1.0);
            square.render(shader_program, &ortho);
            ball.render(shader_program, &ortho);
        }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                    ball.position.x += speed * delta_time;
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                    ball.position.x -= speed * delta_time;
                }
                glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                    ball.position.y += speed * delta_time;
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                    ball.position.y -= speed * delta_time;
                }
                _ => {}
            }
        }

        if check_ball_square_collision(ball.position, ball.radius, square.position, square.size) {
            ball.color = Vec3::new(1.0, 1.0, 0.0);
            square.color = Vec3::new(1.0, 0.5, 0.0);
        } else {
            ball.color = Vec3::new(1.0, 0.3, 0.3);
            square.color = Vec3::new(0.3, 0.8, 1.);
        }
        last_time = current_time;
        window.swap_buffers();
    }
}

fn main() {
    window();
}
