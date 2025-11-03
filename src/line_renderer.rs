use glam::{Mat4, Vec3};

pub struct LineRenderer {
    vao: u32,
    vbo: u32,
}

impl LineRenderer {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (6 * std::mem::size_of::<f32>()) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

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

        LineRenderer { vao, vbo }
    }

    pub fn draw_line(
        &self,
        start: Vec3,
        end: Vec3,
        color: Vec3,
        shader_program: u32,
        projection: &Mat4,
    ) {
        unsafe {
            gl::UseProgram(shader_program);

            let transform = *projection;

            let transform_name = std::ffi::CString::new("transform").unwrap();
            let transform_loc = gl::GetUniformLocation(shader_program, transform_name.as_ptr());
            gl::UniformMatrix4fv(
                transform_loc,
                1,
                gl::FALSE,
                &transform as *const Mat4 as *const f32,
            );

            let color_name = std::ffi::CString::new("objectColor").unwrap();
            let color_loc = gl::GetUniformLocation(shader_program, color_name.as_ptr());
            gl::Uniform3f(color_loc, color.x, color.y, color.z);

            let vertices: [f32; 6] = [start.x, start.y, start.z, end.x, end.y, end.z];

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                std::mem::size_of_val(&vertices) as isize,
                vertices.as_ptr() as *const std::ffi::c_void,
            );

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::LINES, 0, 2);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_vector(
        &self,
        origin: Vec3,
        direction: Vec3,
        length: f32,
        color: Vec3,
        shader_program: u32,
        projection: &Mat4,
    ) {
        let end = origin + direction.normalize() * length;
        self.draw_line(origin, end, color, shader_program, projection);

        self.draw_arrow_head(origin, end, color, shader_program, projection);
    }

    fn draw_arrow_head(
        &self,
        start: Vec3,
        end: Vec3,
        color: Vec3,
        shader_program: u32,
        projection: &Mat4,
    ) {
        let direction = (end - start).normalize();
        let arrow_size = 10.0;

        let perp1 = Vec3::new(-direction.y, direction.x, 0.0).normalize() * arrow_size;
        let perp2 = Vec3::new(direction.y, -direction.x, 0.0).normalize() * arrow_size;

        let arrow_back = end - direction * arrow_size;

        self.draw_line(end, arrow_back + perp1, color, shader_program, projection);
        self.draw_line(end, arrow_back + perp2, color, shader_program, projection);
    }
}
