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

    let square = SquareObject::new(
        Vec3::new(200.0, 300.0, 0.0),
        100.0,
        Vec3::new(0.5, 0.5, 0.2),
    );

    let ball1 = BallObject::new(
        Vec3::new(400.0, 300.0, 0.0),
        Vec2::new(150.0, 200.0),
        10.0,
        Vec3::new(0.5, 0.5, 0.2),
    );

    let ball2 = BallObject::new(
        Vec3::new(500.0, 200.0, 0.0),
        Vec2::new(150.0, 200.0),
        10.0,
        Vec3::new(0.5, 0.5, 0.2),
    );

    let mut ball_objects = vec![ball1, ball2];
    let mut square_objects = vec![square];

    let mut last_time = glfw.get_time() as f32;

    // Render loop
    while !window.should_close() {
        let current_time = glfw.get_time() as f32;
        let delta_time = current_time - last_time;

        for ball in &mut ball_objects {
            ball.update(delta_time);

            let wall_collision = check_wall_collision(
                ball.position,
                ball.radius,
                SRC_WIDTH as f32,
                SRC_HEIGHT as f32,
            );

            if wall_collision.left || wall_collision.right {
                ball.velocity.x *= -1.0;
                if wall_collision.left {
                    ball.position.x = ball.radius;
                }
                if wall_collision.right {
                    ball.position.x = SRC_WIDTH as f32 - ball.radius;
                }
            }

            if wall_collision.top || wall_collision.bottom {
                ball.velocity.y *= -1.0;
                if wall_collision.bottom {
                    ball.position.y = ball.radius;
                }
                if wall_collision.top {
                    ball.position.y = SRC_HEIGHT as f32 - ball.radius;
                }
            }
        }

        for ball in &mut ball_objects {
            let mut collision_detected = false;

            for square in &square_objects {
                if check_ball_square_collision(
                    ball.position,
                    ball.radius,
                    square.position,
                    square.size,
                ) {
                    ball.velocity *= -1.0;
                    collision_detected = true;
                    break;
                }
            }
        }

        for square in &mut square_objects {
            let mut collision = false;
            for ball in &ball_objects {
                if check_ball_square_collision(
                    ball.position,
                    ball.radius,
                    square.position,
                    square.size,
                ) {
                    collision = true;
                    break;
                }
            }

            if collision {
                square.color = Vec3::new(1.0, 0.5, 0.0);
            } else {
                square.color = Vec3::new(0.3, 0.8, 1.0);
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let ortho =
                Mat4::orthographic_rh_gl(0.0, SRC_WIDTH as f32, 0.0, SRC_HEIGHT as f32, -1.0, 1.0);
            for ball in &ball_objects {
                ball.render(shader_program, &ortho);
            }

            for square in &square_objects {
                square.render(shader_program, &ortho);
            }
        }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true);
            }
        }

        last_time = current_time;
        window.swap_buffers();
    }
}

fn main() {
    window();
}
