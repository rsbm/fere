use fere_common::*;
use gl::types::{GLenum, GLuint};

pub enum InternalTexType {
    Float1,     //
    Float3,     // Position, Emission
    HalfFloat3, // Normal
    Color,      // Base Color
    Index,      // Object Index
    Flag,       // Lighting
    Material,   // Roughness, Metalness
    Depth,
    Yuv,
}

pub struct TexParam {
    pub internal_format: GLenum,
    pub format: GLenum,
    pub data_type: GLenum,
    pub num: usize,
    pub _size: usize,
}

impl TexParam {
    fn new(tuple: (GLenum, GLenum, GLenum, usize, usize)) -> Self {
        Self {
            internal_format: tuple.0,
            format: tuple.1,
            data_type: tuple.2,
            num: tuple.3,
            _size: tuple.4,
        }
    }
}

impl InternalTexType {
    /// (..., num, size)
    pub(crate) fn tex_parameters(&self) -> TexParam {
        TexParam::new(match self {
            InternalTexType::Float1 => (gl::RED, gl::RED, gl::FLOAT, 1, 4),
            InternalTexType::Float3 => (gl::RGB32F, gl::RGB, gl::FLOAT, 3, 4),
            InternalTexType::HalfFloat3 => (gl::RGB16F, gl::RGB, gl::HALF_FLOAT, 3, 2),
            InternalTexType::Color => (gl::RGB, gl::RGB, gl::UNSIGNED_BYTE, 3, 1),
            InternalTexType::Index => (gl::R32UI, gl::RED_INTEGER, gl::UNSIGNED_INT, 1, 4),
            InternalTexType::Flag => (gl::R8I, gl::RED_INTEGER, gl::UNSIGNED_BYTE, 1, 1),
            InternalTexType::Material => (gl::RED, gl::RED, gl::UNSIGNED_BYTE, 1, 1),
            InternalTexType::Depth => (gl::DEPTH_COMPONENT, gl::DEPTH_COMPONENT, gl::FLOAT, 1, 4),
            InternalTexType::Yuv => (gl::R8UI, gl::RED_INTEGER, gl::UNSIGNED_BYTE, 1, 1),
        })
    }
}

pub struct TextureInternal {
    raw: GLuint,
    tex_type: InternalTexType,
}

impl TextureInternal {
    pub fn raw_get(&self) -> GLuint {
        self.raw
    }
}

impl Drop for TextureInternal {
    fn drop(&mut self) {
        let buf = [self.raw];
        unsafe {
            gl::DeleteTextures(1, buf.as_ptr().cast());
        }
    }
}

pub struct TextureInternal2D {
    tex: TextureInternal,
    size: IVec2,
    tex_param: TexParam,
}

impl TextureInternal2D {
    pub fn new(tex_type: InternalTexType, size: IVec2) -> Self {
        let tex_param = tex_type.tex_parameters();
        let mut tex = TextureInternal { raw: 0, tex_type };
        unsafe {
            gl::GenTextures(1, &mut tex.raw);
            gl::BindTexture(gl::TEXTURE_2D, tex.raw);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                tex_param.internal_format as i32,
                size.x,
                size.y,
                0,
                tex_param.format,
                tex_param.data_type,
                std::ptr::null(),
            );
            Self::parameters_set();
        }
        TextureInternal2D {
            tex,
            size,
            tex_param,
        }
    }

    pub fn tex_param(&self) -> &TexParam {
        &self.tex_param
    }

    fn parameters_set() {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );
        }
    }

    #[allow(dead_code)]
    pub fn load(&self, buf: &[f32]) {
        assert!(buf.len() >= (self.size.x as usize) * (self.size.y as usize) * self.tex_param.num);
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            let parameter = self.tex.tex_type.tex_parameters();
            gl::BindTexture(gl::TEXTURE_2D, self.tex.raw);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                parameter.internal_format as i32,
                self.size.x,
                self.size.y,
                0,
                parameter.format,
                parameter.data_type,
                buf.as_ptr().cast(),
            );
        }
    }

    /*
    pub fn clear(&self) {
        let zero = [0, 0, 0, 0];
        let parameter = self.tex.tex_type.tex_parameters();
        unsafe {
            gl::ClearTexImage(
                self.tex.raw,
                0,
                parameter.format,
                parameter.data_type,
                zero.as_ptr().cast(),
            );
        }
    }
    */

    pub fn tex_get(&self) -> &TextureInternal {
        &self.tex
    }

    pub fn size_get(&self) -> IVec2 {
        self.size
    }
}

pub struct TextureInternal3D {
    tex: TextureInternal,
    size: IVec3,
    tex_param: TexParam,
}

impl TextureInternal3D {
    pub fn new(tex_type: InternalTexType, size: IVec3) -> Self {
        let tex_param = tex_type.tex_parameters();
        let mut tex = TextureInternal { raw: 0, tex_type };
        unsafe {
            gl::GenTextures(1, &mut tex.raw);
            gl::BindTexture(gl::TEXTURE_3D, tex.raw);
            gl::TexImage3D(
                gl::TEXTURE_3D,
                0,
                tex_param.internal_format as i32,
                size.x,
                size.y,
                size.z,
                0,
                tex_param.format,
                tex_param.data_type,
                std::ptr::null(),
            );
            Self::parameters_set();
        }
        TextureInternal3D {
            tex,
            size,
            tex_param,
        }
    }

    fn parameters_set() {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(
                gl::TEXTURE_3D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_3D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );
        }
    }

    /// Does some shit
    ///
    /// # Safety
    /// TODO
    pub unsafe fn load(&mut self, buf: *const f32) {
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        gl::BindTexture(gl::TEXTURE_3D, self.tex.raw);
        gl::TexImage3D(
            gl::TEXTURE_3D,
            0,
            self.tex_param.internal_format as i32,
            self.size.x,
            self.size.y,
            self.size.z,
            0,
            self.tex_param.format,
            self.tex_param.data_type,
            buf.cast(),
        );
    }

    pub fn tex_get(&self) -> &TextureInternal {
        &self.tex
    }

    pub fn size_get(&self) -> IVec3 {
        self.size
    }
}

pub struct FrameBuffer {
    raw: GLuint,
    outputs: Vec<TextureInternal2D>,
    depth: Option<TextureInternal2D>,
}

impl FrameBuffer {
    pub fn new(outputs: Vec<TextureInternal2D>, depth: Option<TextureInternal2D>) -> Self {
        debug_assert!(
            outputs.len() <= 8,
            "Too many texture attached to a framebuffer"
        );

        let mut raw = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut raw);
            gl::BindFramebuffer(gl::FRAMEBUFFER, raw);

            debug_assert!(
				outputs.iter().all(|x| x.size
					== outputs
						.get(0)
						.expect("Framebuffer with empty outputs")
						.size),
				"Framebuffer with not uniform size of outputs"
			);

            for (i, out) in outputs.iter().enumerate() {
                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl::COLOR_ATTACHMENT0 + i as GLenum,
                    gl::TEXTURE_2D,
                    out.tex.raw,
                    0,
                );
            }

            if let Some(x) = &depth {
                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl::DEPTH_ATTACHMENT,
                    gl::TEXTURE_2D,
                    x.tex.raw,
                    0,
                );
            }

            let buf: Vec<GLenum> = (0..8)
                .into_iter()
                .map(|x| gl::COLOR_ATTACHMENT0 + x as GLenum)
                .collect();
            gl::DrawBuffers(outputs.len() as i32, buf.as_ptr().cast());
            debug_assert_eq!(
                gl::CheckFramebufferStatus(gl::FRAMEBUFFER),
                gl::FRAMEBUFFER_COMPLETE,
                "Failed to create a Framebuffer"
            )
        }
        FrameBuffer {
            raw,
            outputs,
            depth,
        }
    }

    pub fn depth_get(&self) -> &TextureInternal2D {
        self.depth.as_ref().unwrap()
    }

    pub fn outputs_get(&self) -> &Vec<TextureInternal2D> {
        &self.outputs
    }

    pub fn raw_get(&self) -> GLuint {
        self.raw
    }

    pub fn bind(&self) {
        let size = if self.outputs.is_empty() {
            self.depth.as_ref().unwrap().size
        } else {
            self.outputs[0].size
        };
        unsafe {
            gl::Viewport(0, 0, size.x, size.y);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.raw);
        }
    }

    /// Call this after `bind()`
    pub fn clear_color_all(&self) {
        for i in 0..self.outputs.len() {
            self.clear_color(i)
        }
    }

    /// Call this after `bind()`
    pub fn clear_color(&self, index: usize) {
        unsafe {
            gl::ColorMask(0, 0, 0, 0);
            gl::ColorMaski(index as u32, 1, 1, 1, 1);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    /// Call this after `bind()`
    pub fn clear_depth(&self) {
        unsafe {
            gl::DepthMask(1);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        let buf = [self.raw];
        unsafe {
            gl::DeleteFramebuffers(1, buf.as_ptr().cast());
        }
    }
}
