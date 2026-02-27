extern crate glfw;
use glam::{Mat4, Vec3, Vec4};

use crate::{
    SRC_HEIGHT, SRC_WIDTH, collision::*, light_obj::LightObject, line_renderer::LineRenderer,
};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_vec(self) -> Vec4 {
        Vec4::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }
}

#[derive(Clone)]
pub struct BallObject {
    pub position: Vec3,
    pub velocity: Vec3,
    pub radius: f32,
    pub color: Color,
    pub mass: f32,
    pub has_collision: bool,
    pub has_gravity: bool,
    pub light_source: LightObject,
    vao: u32,
    vbo: u32,
    ebo: u32,
    vertex_count: i32,
}

impl BallObject {
    pub fn new(
        position: Vec3,
        velocity: Vec3,
        radius: f32,
        color: Color,
        mass: f32,
        has_collision: bool,
        has_gravity: bool,
        light_source: LightObject,
    ) -> Self {
        BallObject {
            position,
            velocity,
            radius,
            color,
            mass,
            has_collision,
            has_gravity,
            light_source,
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertex_count: 0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position.x += self.velocity.x * delta_time;
        self.position.y += self.velocity.y * delta_time;
    }

    pub fn gravity_update(&mut self, another_ball: &BallObject, delta_time: f32) {
        if !self.has_gravity {
            return;
        }
        //F = G * (m1 * m2) / r^2
        let g = 100.0;
        let direction = another_ball.position - self.position;
        let r = direction.length();
        if r < 1.0 {
            return;
        }

        let f = g * (self.mass * another_ball.mass) / r.powi(2);

        let acceleration = (direction.normalize() * f) / self.mass;
        self.velocity += acceleration * delta_time;
    }

    pub fn render(&mut self, shader_program: u32, projection: &Mat4) {
        self.mesh();
        let line_renderer = LineRenderer::new();
        unsafe {
            gl::UseProgram(shader_program);

            let mut model = glam::Mat4::IDENTITY;

            model *= Mat4::from_translation(self.position);

            let transform = *projection * model;

            let transform_name = std::ffi::CString::new("transform").unwrap();
            let transformloc = gl::GetUniformLocation(shader_program, transform_name.as_ptr());
            gl::UniformMatrix4fv(
                transformloc,
                1,
                gl::FALSE,
                &transform as *const Mat4 as *const f32,
            );

            let color_name = std::ffi::CString::new("objectColor").unwrap();
            let colorloc = gl::GetUniformLocation(shader_program, color_name.as_ptr());
            gl::Uniform4f(
                colorloc,
                self.color.to_vec().x,
                self.color.to_vec().y,
                self.color.to_vec().z,
                self.color.to_vec().w,
            );

            let light_name = std::ffi::CString::new("lightPos").unwrap();
            let lightloc = gl::GetUniformLocation(shader_program, light_name.as_ptr());
            gl::Uniform2f(
                lightloc,
                self.light_source.position.x,
                self.light_source.position.y,
            );

            let ball_pos_name = std::ffi::CString::new("ballPos").unwrap();
            let ball_pos_loc = gl::GetUniformLocation(shader_program, ball_pos_name.as_ptr());
            gl::Uniform2f(ball_pos_loc, self.position.x, self.position.y);

            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.vertex_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            line_renderer.draw_vector(
                self.position,
                Vec3::new(self.velocity.x, self.velocity.y, 0.0),
                50.0 + self.radius,
                Vec3::new(1.0, 0.0, 0.0),
                shader_program,
                projection,
            );
        }
    }

    fn mesh(&mut self) {
        let segments = 32;
        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        vertices.extend_from_slice(&[0.0, 0.0, 0.0]);

        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
            let x = angle.cos() * self.radius;
            let y = angle.sin() * self.radius;
            vertices.extend_from_slice(&[x, y, 0.0]);
        }

        for i in 1..=segments {
            indices.push(0);
            indices.push(i);
            indices.push(i + 1);
        }
        self.vertex_count = indices.len() as i32;
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            //VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            //EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
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
    }

    pub fn wall_collision(&mut self) {
        let damping = 0.85;
        let wall = check_wall_collision(
            self.position,
            self.radius,
            SRC_WIDTH as f32,
            SRC_HEIGHT as f32,
        );

        if wall.left || wall.right {
            self.velocity.x *= -damping * 1.0;
            if wall.left {
                self.position.x = self.radius;
            } else if wall.right {
                self.position.x = SRC_WIDTH as f32 - self.radius;
            }
        }

        if wall.top || wall.bottom {
            self.velocity.y *= -damping * 1.0;
            if wall.bottom {
                self.position.y = self.radius;
            } else if wall.top {
                self.position.y = SRC_HEIGHT as f32 - self.radius;
            }
        }
    }

    pub fn check_ball_square_collision() {
        // for ball1 in &mut ball_objects {
        //     for square in &mut square_objects {
        //         let (collided, side_index, ball_pos) = check_ball_square_collision(
        //             ball.position,
        //             ball.radius,
        //             square.position,
        //             square.size,
        //             square.rotation,
        //         );
        //         if collided {
        //             let normal = square.get_normal_relative_to(side_index);
        //
        //             ball.position = ball_pos;
        //
        //             ball.velocity -= 2.0 * ball.velocity.dot(normal) * normal;
        //             // ball.velocity *= 0.9; //Damping
        //
        //             normal_square = Vec3::new(ball.velocity.x, ball.velocity.y, 0.0);
        //
        //             square.color = Vec3::new(1.0, 0.5, 0.0);
        //             break;
        //         } else {
        //             square.color = Vec3::new(0.3, 0.8, 1.0);
        //         }
        //     }
        // }
    }

    pub fn check_ball_ball_collision(&mut self, ball2: &mut BallObject) {
        if !self.has_collision {
            return;
        }
        let damping = 0.85;
        let delta = ball2.position - self.position;
        let distance = delta.length();
        let mind_dist = self.radius + ball2.radius;

        if distance < mind_dist && distance > 0.0 {
            let total_mass = self.mass + ball2.mass;

            let overlap = mind_dist - distance;

            self.position -= delta.normalize() * overlap * (ball2.mass / total_mass);
            ball2.position -= delta.normalize() * overlap * (self.mass / total_mass);

            let rel_vel = ball2.velocity - self.velocity;
            let vel_along_normal = rel_vel.dot(delta.normalize());

            if vel_along_normal > 0.0 {
                return;
            }

            let restitution = 1.0_f32;
            let impulse_scalar = -(1.0 + restitution) * vel_along_normal / total_mass;
            let impulse = delta.normalize() * impulse_scalar * damping;
            self.velocity -= impulse * ball2.mass;
            ball2.velocity += impulse * self.mass;
        }
    }
}
