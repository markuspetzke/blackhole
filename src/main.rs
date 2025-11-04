extern crate glfw;
use glfw::{Action, Context, Key, fail_on_errors};
extern crate gl;
use glam::{Mat4, Vec3};
use std::{ffi::CString, fs};

mod square_obj;
use square_obj::SquareObject;

mod ball_obj;
use ball_obj::BallObject;

mod collision;
use collision::*;

mod line_renderer;
use line_renderer::LineRenderer;

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
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
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

    let line_renderer = LineRenderer::new();

    let square = SquareObject::new(
        Vec3::new(200.0, 300.0, 0.0),
        std::f32::consts::PI / 6.,
        100.0,
        Vec3::new(0.5, 0.5, 0.2),
    );

    let ball_rechts = BallObject::new(
        Vec3::new(400.0, 300.0, 0.0),
        Vec3::new(-100., 0., 0.),
        10.,
        Vec3::new(0.5, 0.5, 0.2),
    );

    let ball_top = BallObject::new(
        Vec3::new(200.0, 400.0, 0.0),
        Vec3::new(0., 100., 0.),
        10.,
        Vec3::new(0.5, 0.5, 0.2),
    );

    let ball_top2 = BallObject::new(
        Vec3::new(200.0, 450.0, 0.0),
        Vec3::new(0., 100., 0.),
        10.,
        Vec3::new(0.5, 0.5, 0.2),
    );

    let ball_links = BallObject::new(
        Vec3::new(100.0, 300.0, 0.0),
        Vec3::new(100., 0., 0.),
        10.,
        Vec3::new(0.5, 0.5, 0.2),
    );

    let ball_unten = BallObject::new(
        Vec3::new(200.0, 100.0, 0.0),
        Vec3::new(0., 100., 0.),
        10.,
        Vec3::new(0.5, 0.5, 0.2),
    );

    let mut ball_objects: Vec<BallObject> = vec![ball_top, ball_top2];

    for i in 0..10 {
        let ball = BallObject::new(
            Vec3::new(150.0 + (i as f32 * 10.), 400.0 + (i as f32 * 10.), 0.0),
            Vec3::new(0., 25., 0.),
            10.,
            Vec3::new(0.5, 0.5, 0.2),
        );

        ball_objects.push(ball);
    }

    let mut square_objects: Vec<SquareObject> = vec![square];

    let mut last_time = glfw.get_time() as f32;
    let mut frame_count = 0;
    let mut fps_timer = 0.0;

    let count = 1;

    let mut normal_square = Vec3::ZERO;

    // Render loop
    while !window.should_close() {
        let current_time = glfw.get_time() as f32;
        let delta_time = (current_time - last_time).min(0.1);

        //FPS
        frame_count += 1;
        fps_timer += delta_time;
        if fps_timer >= 0.5 {
            let fps = frame_count as f32 / fps_timer;
            window.set_title(&format!(
                "Test Title - FPS: {fps:.1} - Ball Count: {}",
                ball_objects.len()
            ));
            frame_count = 0;
            fps_timer = 0.0;
        }

        //Update physics for balls
        for ball in &mut ball_objects {
            ball.update(delta_time);

            let wall = check_wall_collision(
                ball.position,
                ball.radius,
                SRC_WIDTH as f32,
                SRC_HEIGHT as f32,
            );

            if wall.left || wall.right {
                ball.velocity.x *= -1.0;
                if wall.left {
                    ball.position.x = ball.radius;
                } else if wall.right {
                    ball.position.x = SRC_WIDTH as f32 - ball.radius;
                }
            }

            if wall.top || wall.bottom {
                ball.velocity.y *= -1.0;
                if wall.bottom {
                    ball.position.y = ball.radius;
                } else if wall.top {
                    ball.position.y = SRC_HEIGHT as f32 - ball.radius;
                }
            }
        }

        for ball in &mut ball_objects {
            for square in &mut square_objects {
                let (collided, side_index, ball_pos) = check_ball_square_collision(
                    ball.position,
                    ball.radius,
                    square.position,
                    square.size,
                );
                if collided {
                    let normal = square.get_normal_relative_to(side_index);

                    ball.position = ball_pos;

                    ball.velocity -= 2.0 * ball.velocity.dot(normal) * normal;
                    // ball.velocity *= 0.9; //Damping

                    normal_square = Vec3::new(ball.velocity.x, ball.velocity.y, 0.0);

                    square.color = Vec3::new(1.0, 0.5, 0.0);
                    break;
                } else {
                    square.color = Vec3::new(0.3, 0.8, 1.0);
                }
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let ortho =
                Mat4::orthographic_rh_gl(0.0, SRC_WIDTH as f32, 0.0, SRC_HEIGHT as f32, -1.0, 1.0);

            for ball in &ball_objects {
                ball.render(shader_program, &ortho);

                line_renderer.draw_vector(
                    ball.position,
                    Vec3::new(ball.velocity.x, ball.velocity.y, 0.0),
                    50.0,
                    Vec3::new(1.0, 0.0, 0.0),
                    shader_program,
                    &ortho,
                );
            }

            if normal_square != Vec3::ZERO {
                line_renderer.draw_vector(
                    square_objects[0].position,
                    normal_square,
                    100.0,
                    Vec3::new(0.0, 1.0, 0.0),
                    shader_program,
                    &ortho,
                );
            }

            for square in &square_objects {
                let normals = square.get_normals();
                let half = square.size / 2.0;

                let colors = [
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 1.0, 0.0),
                ];

                for (i, normal) in normals.iter().enumerate() {
                    line_renderer.draw_vector(
                        square.position,
                        Vec3::new(normal.x, normal.y, 0.0),
                        half + 30.0,
                        colors[i],
                        shader_program,
                        &ortho,
                    );
                }

                square.render(shader_program, &ortho);
            }
        }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                    random_balls(&mut ball_objects, count);
                }
                glfw::WindowEvent::Key(Key::C, _, Action::Press, _) => {
                    ball_objects.clear();
                }

                _ => {}
            }
        }

        last_time = current_time;
        window.swap_buffers();
    }
}

fn random_balls(array: &mut Vec<BallObject>, count: i32) {
    for _i in 0..count {
        let rng_size = rand::random_range(1.0..=10.);
        let rng_velox = rand::random_range(1.0..=5.);
        let rng_veloy = rand::random_range(1.0..=5.);
        let rng_posx = rand::random_range(0.0..=1.);
        let rng_posy = rand::random_range(0.0..=1.);

        let ball = BallObject::new(
            Vec3::new(400.0 * rng_posx, 300.0 * rng_posy, 0.0),
            Vec3::new(150.0 * rng_velox, 200.0 * rng_veloy, 0.),
            rng_size,
            Vec3::new(
                rand::random_range(0.0..=1.0),
                rand::random_range(0.0..=1.0),
                rand::random_range(0.0..=1.0),
            ),
        );
        array.push(ball);
    }
}

fn main() {
    window();
}
