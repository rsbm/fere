use fere_common::*;
use gl::types::*;
use image::{io::Reader as IReader, DynamicImage};
use std::io::{BufRead, Seek};

#[derive(Debug)]
pub struct TextureData {
    pub name: String,
    pub data: Vec<u8>,
    pub size: IVec2,
    pub channel: u8,
}

pub fn import<T: BufRead + Seek>(name: &str, source: T) -> Result<TextureData, String> {
    let source = IReader::new(source).with_guessed_format().unwrap();
    let result = source.decode().map_err(|e| format!("{:?}", e))?;
    match result {
        DynamicImage::ImageRgb8(img) => {
            let size = IVec2::new(img.dimensions().0 as i32, img.dimensions().1 as i32);
            let data = img.into_raw();
            assert_eq!(data.len(), (size.x * size.y * 3) as usize);
            Ok(TextureData {
                name: name.to_owned(),
                data,
                size,
                channel: 3,
            })
        }
        DynamicImage::ImageRgba8(img) => {
            let size = IVec2::new(img.dimensions().0 as i32, img.dimensions().1 as i32);
            let data = img.into_raw();
            assert_eq!(data.len(), (size.x * size.y * 4) as usize);
            Ok(TextureData {
                name: name.to_owned(),
                data,
                size,
                channel: 4,
            })
        }
        _ => Err("Unsupported image format".to_owned()),
    }
}

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

    pub fn bind(&self) {
        debug_assert!(self.data.is_none(), "bind() on an unbufferd texure");
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.tex) }
    }

    pub fn bind_at(&self, index: u32) {
        debug_assert!(self.data.is_none(), "bind() on an unbufferd texure");
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
        }
    }

    pub fn bind_or_buffer(&mut self) {
        if self.data.is_some() {
            self.buffer();
        }
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.tex) }
    }

    pub fn get_raw(&self) -> GLuint {
        self.tex
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if self.data.is_none() {
            unsafe {
                gl::DeleteTextures(1, &self.tex);
            }
        }
    }
}
