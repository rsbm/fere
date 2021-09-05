use fere_common::*;
use fere_resources::texture::TextureData;
use gl::types::*;

use crate::graphics::graphics::texture_internal::InternalTexType;

#[derive(Debug)]
pub struct Texture {
    pub name: String,
    // If it's not from the particular file, then None
    pub path: Option<String>,
    pub size: IVec2,

    // CPU things - will be purged from memory after buffer
    data: Option<TextureData>,

    // GPU things - will exist only after buffer
    tex: GLuint,
}

impl Texture {
    pub fn new(path: Option<String>, data: TextureData) -> Self {
        Texture {
            name: data.name.clone(),
            path,
            size: data.size,
            data: Some(data),
            tex: 0,
        }
    }

    pub fn buffer(&mut self) {
        assert!(
            crate::is_main_thread(),
            "Mesh must be buffered in the main thread"
        );
        unsafe {
            gl::GenTextures(1, &mut self.tex);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);

            let data = self.data.take().unwrap();
            let format = match data.channel {
                1 => gl::RED,
                3 => gl::RGB,
                4 => gl::RGBA,
                _ => panic!("Invalid Image channel set"),
            };
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                self.size.x,
                self.size.y,
                0,
                format,
                gl::UNSIGNED_BYTE,
                data.data.as_ptr().cast(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }
    }

    pub(crate) fn bind(&self) {
        debug_assert!(self.data.is_none(), "bind() on an unbufferd texure");
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.tex) }
    }

    pub(crate) fn bind_at(&self, index: u32) {
        debug_assert!(self.data.is_none(), "bind() on an unbufferd texure");
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
        }
    }

    pub fn set_pixel(&mut self, _pos: IVec2, _color: IVec3) {
        unimplemented!()
    }

    pub fn write_buffer(&mut self, data: &[u8]) {
        assert_eq!(data.len(), (self.size.x * self.size.y * 3) as usize);
        let tex_param = InternalTexType::Color.tex_parameters();
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                tex_param.internal_format as i32,
                self.size.x,
                self.size.y,
                0,
                tex_param.format,
                tex_param.data_type,
                data.as_ptr().cast(),
            );
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        assert!(
            crate::is_main_thread(),
            "Mesh must be dropped in the main thread"
        );
        if self.data.is_none() {
            unsafe {
                gl::DeleteTextures(1, &self.tex);
            }
        }
    }
}
