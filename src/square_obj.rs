extern crate glfw;
use glam::{Mat4, Vec3};

pub struct SquareObject {
    pub position: Vec3,
    pub size: f32,
    pub rotation: Vec3,
    pub color: Vec3,
    vao: u32,
    vbo: u32,
    ebo: u32,
    vertex_count: i32,
}

impl SquareObject {
    pub fn new(position: Vec3, size: f32, color: Vec3) -> Self {
        let mut square = SquareObject {
            position,
            size,
            rotation: Vec3::ZERO,
            color,
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertex_count: 0,
        };

        square.mesh();

        square
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
            gl::Uniform3f(colorloc, self.color.x, self.color.y, self.color.z);

            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    fn mesh(&mut self) {
        let half = self.size / 2.0;
        let vertices: Vec<f32> = vec![
            half, half, 0.0, // oben rechts
            half, -half, 0.0, // unten rechts
            -half, -half, 0.0, // unten links
            -half, half, 0.0, // oben links
        ];

        let indices: Vec<u32> = vec![0, 1, 3, 1, 2, 3];
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
