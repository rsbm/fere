use fere_common::*;
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
