extern crate glfw;
use glam::{Mat4, Vec3, Vec4};

use crate::{SRC_HEIGHT, SRC_WIDTH, collision::*, line_renderer::LineRenderer};

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
pub struct LightObject {
    pub position: Vec3,
    pub color: Color,
    pub radius: f32,
    vao: u32,
    vbo: u32,
    ebo: u32,
    vertex_count: i32,
}

impl LightObject {
    pub fn new(
        position: Vec3,
        // color: Color,
    ) -> Self {
        LightObject {
            position,
            radius: 10.,
            color: Color::new(255, 255, 0, 255),
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertex_count: 0,
        }
    }

    pub fn render(&mut self, shader_program: u32, projection: &Mat4) {
        self.mesh();
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
            // let world_pos_name = std::ffi::CString::new("worldPos").unwrap();
            // let world_pos_loc = gl::GetUniformLocation(shader_program, world_pos_name.as_ptr());
            // gl::Uniform2f(world_pos_loc, self.position.x, self.position.y);
            //
            // let light_name = std::ffi::CString::new("lightPos").unwrap();
            // let lightloc = gl::GetUniformLocation(shader_program, light_name.as_ptr());
            // gl::Uniform2f(lightloc, 600., 500.);
            //
            // let light_radius_name = std::ffi::CString::new("lightRadius").unwrap();
            // let light_radius_loc =
            //     gl::GetUniformLocation(shader_program, light_radius_name.as_ptr());
            // gl::Uniform1f(light_radius_loc, 40000.0);

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
}
