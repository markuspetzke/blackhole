use rusttype::{Font, Scale, point};

pub struct TextRenderer {
    font: Font<'static>,
    shader_program: u32,
    vao: u32,
    vbo: u32,
}

impl TextRenderer {
    pub fn new(shader_program: u32) -> Self {
        let font_data = include_bytes!("../fonts/Roboto-Regular.ttf"); // TTF rein
        let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

        let (vao, vbo) = unsafe {
            let mut vao = 0;
            let mut vbo = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (6 * 4 * std::mem::size_of::<f32>()) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            (vao, vbo)
        };

        Self {
            font,
            shader_program,
            vao,
            vbo,
        }
    }

    pub fn draw(&self, text: &str, x: f32, y: f32, scale: f32, ortho: &glam::Mat4) {
        let scale = Scale::uniform(scale);
        let v_metrics = self.font.v_metrics(scale);

        let glyphs: Vec<_> = self
            .font
            .layout(text, scale, point(0.0, v_metrics.ascent))
            .collect();

        let width = glyphs
            .iter()
            .filter_map(|g| g.pixel_bounding_box())
            .map(|bb| bb.max.x)
            .max()
            .unwrap_or(0) as usize;

        let height = (v_metrics.ascent - v_metrics.descent).ceil() as usize;

        if width == 0 || height == 0 {
            return;
        }

        let mut buffer = vec![0u8; width * height];
        for glyph in &glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let gx = gx as i32 + bb.min.x;
                    let gy = gy as i32 + bb.min.y;
                    if gx >= 0 && gy >= 0 && (gx as usize) < width && (gy as usize) < height {
                        let gy_flipped = height - 1 - gy as usize;
                        buffer[gy_flipped * width + gx as usize] = (v * 255.0) as u8;
                    }
                });
            }
        }

        unsafe {
            let mut texture = 0u32;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                width as i32,
                height as i32,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                buffer.as_ptr() as *const _,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::UseProgram(self.shader_program);
            let model = glam::Mat4::from_translation(glam::Vec3::new(x, y, 0.0))
                * glam::Mat4::from_scale(glam::Vec3::new(width as f32, height as f32, 1.0));

            let mvp = *ortho * model;

            let mvp_loc = gl::GetUniformLocation(self.shader_program, c"mvp".as_ptr() as *const _);
            gl::UniformMatrix4fv(mvp_loc, 1, gl::FALSE, mvp.as_ref().as_ptr());

            let color_loc =
                gl::GetUniformLocation(self.shader_program, c"textColor".as_ptr() as *const _);
            gl::Uniform3f(color_loc, 1.0, 1.0, 1.0);

            let vertices: [f32; 24] = [
                0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
                1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
            ];

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
            );
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::DeleteTextures(1, &texture);
            gl::BindVertexArray(0);
        }
    }
}
