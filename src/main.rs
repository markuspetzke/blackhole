extern crate glfw;
use glfw::{Action, Context, Key, MouseButton, fail_on_errors};
extern crate gl;
use glam::{Mat4, Vec3};
use std::{ffi::CString, fs};

// mod square_obj;
// use square_obj::SquareObject;

mod light_obj;
use light_obj::LightObject;

mod render_text;
use render_text::TextRenderer;

mod ball_obj;
use ball_obj::BallObject;

mod collision;

mod line_renderer;

use crate::ball_obj::Color;

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
    window.set_mouse_button_polling(true);
    window.set_scroll_polling(true);
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
    let vertex_source = load_shader_source("./shader/text_vertex.glsl");

    let fragment_source = load_shader_source("./shader/text_fragment.glsl");

    let text_shader_program = unsafe {
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

    // let square = SquareObject::new(
    //     Vec3::new(200.0, 300.0, 0.0),
    //     std::f32::consts::PI / 6.,
    //     100.0,
    //     Vec3::new(0.5, 0.5, 0.2),
    // );

    let sun = LightObject::new(Vec3::new(600.0, 500.0, 0.0));
    let blackhole = BallObject::new(
        Vec3::new(400.0, 300.0, 0.0),
        Vec3::new(0., 0., 0.),
        100.,
        Color::new(0, 0, 0, 255),
        5000.,
        false,
        true,
        sun.clone(),
    );

    let ball1 = BallObject::new(
        Vec3::new(200.0, 100.0, 0.0),
        Vec3::new(0., 40., 0.),
        10.,
        Color::new(0, 200, 100, 255),
        10.0,
        true,
        true,
        sun.clone(),
    );
    let mut mouse_ball = BallObject::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0., 0., 0.),
        10.,
        Color::new(250, 250, 250, 100),
        0.0,
        false,
        false,
        sun.clone(),
    );

    let mut ball_objects: Vec<BallObject> = vec![ball1, blackhole];
    let mut light_objects: Vec<LightObject> = vec![sun];
    // let mut square_objects: Vec<SquareObject> = vec![];
    let text_renderer = TextRenderer::new(text_shader_program);

    let mut last_time = glfw.get_time() as f32;
    let mut frame_count = 0;
    let mut fps_timer = 0.0;

    let mut radius = 15.;
    let mut mass = 15.;
    let mut fps = 0.;

    // Render loop
    while !window.should_close() {
        //FPS
        let current_time = glfw.get_time() as f32;
        let delta_time = (current_time - last_time).min(0.1);

        frame_count += 1;
        fps_timer += delta_time;
        if fps_timer > 0.5 {
            fps = frame_count as f32 / fps_timer;
        }

        mouse_ball.radius = radius;
        mouse_ball.position = Vec3::new(
            window.get_cursor_pos().0 as f32,
            SRC_HEIGHT as f32 - window.get_cursor_pos().1 as f32,
            0.,
        );

        let len = ball_objects.len();

        //Update physics for balls
        for i in 0..len {
            ball_objects[i].update(delta_time);
            ball_objects[i].wall_collision();

            for j in 0..len {
                if i != j {
                    let other = ball_objects[j].clone();
                    if ball_objects[i].has_gravity {
                        ball_objects[i].gravity_update(&other, delta_time);
                    }
                }
            }
            for j in (i + 1)..len {
                let (left, right) = ball_objects.split_at_mut(j);

                if left[i].has_collision && right[0].has_collision {
                    left[i].check_ball_ball_collision(&mut right[0]);
                }
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let ortho =
                Mat4::orthographic_rh_gl(0.0, SRC_WIDTH as f32, 0.0, SRC_HEIGHT as f32, -1.0, 1.0);

            text_renderer.draw(&format!("FPS {fps:.0}"), 10.0, 70.0, 24.0, &ortho);
            text_renderer.draw(&format!("Radius {radius:.0}"), 10.0, 40.0, 24.0, &ortho);
            text_renderer.draw(&format!("Mass {mass:.0}"), 10.0, 10.0, 24.0, &ortho);

            mouse_ball.render(shader_program, &ortho);
            for ball in &mut ball_objects {
                ball.render(shader_program, &ortho);
            }

            for light in &mut light_objects {
                light.render(shader_program, &ortho);
            }
        }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Scroll(_, y) if y > 0.0 => {
                    let shift = window.get_key(Key::LeftShift) == Action::Press;
                    if shift {
                        mass += 1.;
                    } else {
                        radius += 1.;
                    }
                }
                glfw::WindowEvent::Scroll(_, y) if y < 0.0 => {
                    let shift = window.get_key(Key::LeftShift) == Action::Press;
                    if shift {
                        mass -= 1.;
                    } else {
                        radius -= 1.;
                    }
                }

                glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
                    spawn_ball(&mut ball_objects, &mut window, 1, radius, mass);
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

fn spawn_ball(
    array: &mut Vec<BallObject>,
    window: &mut glfw::Window,
    count: i32,
    radius: f32,
    mass: f32,
) {
    for _i in 0..count {
        let (xpos, ypos) = window.get_cursor_pos();
        let flipped_ypos = SRC_HEIGHT as f64 - ypos;

        // let ball = BallObject::new(
        //     Vec3::new(xpos as f32, flipped_ypos as f32, 0.0),
        //     Vec3::new(50.0, 0.0, 0.),
        //     radius,
        //     Color::new(
        //         rand::random_range(0..=255),
        //         rand::random_range(0..=255),
        //         rand::random_range(0..=255),
        //         255,
        //     ),
        //     mass,
        //     true,
        //     true,
        //     ,
        // );
        // array.push(ball);
    }
}
fn main() {
    window();
}
