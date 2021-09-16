use fere_resources::mesh::{MeshData, MeshDescription};
use gl::types::*;

#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    // If it's not from the particular file, then None
    pub path: Option<String>,
    pub size: usize,

    // CPU things - will be purged from memory after buffer
    data: Option<MeshData>,
    description: MeshDescription,

    // GPU things - will exist only after buffer
    vao: GLuint,
    vbo: GLuint,
}

impl Mesh {
    pub fn new(path: Option<String>, data: MeshData) -> Self {
        let description = data.create_description();
        Mesh {
            name: data.name.clone(),
            path,
            size: data.pos.len(),
            data: Some(data),
            vao: 0,
            vbo: 0,
            description,
        }
    }

    pub fn description(&self) -> &MeshDescription {
        &self.description
    }

    pub fn buffer(&mut self) {
        assert!(
            crate::is_main_thread(),
            "Mesh must be buffered in the main thread"
        );
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.size = self.data.as_ref().unwrap().pos.len();
            if self.size == 0 {
                return;
            }

            let data = self.data.take().unwrap();
            let f = 4; // size of f32
            let n = self.size as isize;

            gl::BufferData(
                gl::ARRAY_BUFFER,
                f * (n * 3 + n * 3 + n * 2 + n * 3),
                std::ptr::null(),
                gl::STATIC_DRAW,
            );
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                f * (n * 3),
                data.pos[0].as_ptr().cast(),
            );
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                f * (n * 3),
                f * (n * 3),
                data.normal[0].as_ptr().cast(),
            );

            if !data.uv.is_empty() {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    f * (n * 6),
                    f * (n * 2),
                    data.uv[0].as_ptr().cast(),
                );
            }
            if !data.tan.is_empty() {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    f * (n * 8),
                    f * (n * 3),
                    data.tan[0].as_ptr().cast(),
                );
            }

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null::<u8>().cast());
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
                ((f * 3 * n) as *const u8).cast(),
            );
            if !data.uv.is_empty() {
                gl::EnableVertexAttribArray(2);
                gl::VertexAttribPointer(
                    2,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    0,
                    ((f * 6 * n) as *const u8).cast(),
                );
            }
            if !data.tan.is_empty() {
                gl::EnableVertexAttribArray(3);
                gl::VertexAttribPointer(
                    3,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    0,
                    ((f * 8 * n) as *const u8).cast(),
                );
            }
        }
    }

    pub(crate) fn bind(&self) {
        debug_assert!(self.data.is_none(), "bind() on an unbufferd mesh");
        unsafe { gl::BindVertexArray(self.vao) }
    }

    pub(crate) fn draw(&self) {
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, self.size as i32) }
    }

    /// Treat this mesh as a line
    pub(crate) fn draw_line(&self) {
        unsafe { gl::DrawArrays(gl::LINES, 0, 2) }
    }

    pub(crate) fn draw_wireframe(&self) {
        unsafe {
            gl::Disable(gl::CULL_FACE);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::DrawArrays(gl::TRIANGLES, 0, self.size as i32);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::Enable(gl::CULL_FACE);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        assert!(
            crate::is_main_thread(),
            "Mesh must be dropped in the main thread"
        );
        if self.data.is_none() {
            unsafe {
                gl::DeleteVertexArrays(1, &self.vao);
                gl::DeleteBuffers(1, &self.vbo);
            }
        }
    }
}
