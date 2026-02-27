extern crate glfw;
use crate::ball_mesh::BallMesh;
use glam::{Mat4, Vec3, Vec4};

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
    mesh: BallMesh,
}

impl LightObject {
    pub fn new(
        position: Vec3,
        // color: Color,
        radius: f32,
    ) -> Self {
        LightObject {
            position,
            radius,
            color: Color::new(255, 255, 0, 255),
            mesh: BallMesh::new(radius),
        }
    }

    pub fn render(&mut self, shader_program: u32, projection: &Mat4) {
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

            let ball_pos_name = std::ffi::CString::new("ballPos").unwrap();
            let ball_pos_loc = gl::GetUniformLocation(shader_program, ball_pos_name.as_ptr());
            gl::Uniform2f(ball_pos_loc, self.position.x, self.position.y);

            gl::BindVertexArray(self.mesh.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.mesh.vertex_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

            gl::BindVertexArray(self.mesh.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.mesh.vertex_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
