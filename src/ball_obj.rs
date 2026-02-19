extern crate glfw;
use glam::{Mat4, Vec3};

use crate::{SRC_HEIGHT, SRC_WIDTH, collision::*};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_vec(self) -> Vec3 {
        Vec3::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
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
    pub collision: bool,
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
        collision: bool,
    ) -> Self {
        let mut square = BallObject {
            position,
            velocity,
            radius,
            color,
            mass,
            collision: true,
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertex_count: 0,
        };

        square.mesh();

        square
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position.x += self.velocity.x * delta_time;
        self.position.y += self.velocity.y * delta_time;
    }

    pub fn gravity_update(&mut self, another_ball: &BallObject, delta_time: f32) {
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

    pub fn render(&self, shader_program: u32, projection: &Mat4) {
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
            gl::Uniform3f(
                colorloc,
                self.color.to_vec().x,
                self.color.to_vec().y,
                self.color.to_vec().z,
            );

            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.vertex_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
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
}
