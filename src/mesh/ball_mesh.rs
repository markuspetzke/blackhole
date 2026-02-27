#[derive(Clone, Copy)]
pub struct BallMesh {
    pub vao: u32,
    vbo: u32,
    ebo: u32,
    pub vertex_count: i32,
}

impl BallMesh {
    pub fn new(radius: f32) -> Self {
        let mut mesh = BallMesh {
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertex_count: 0,
        };
        mesh.build(radius);
        mesh
    }

    fn build(&mut self, radius: f32) {
        let segments = 32;
        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        vertices.extend_from_slice(&[0.0, 0.0, 0.0]);

        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;
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
