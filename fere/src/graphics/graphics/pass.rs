use super::super::glmanager::shader::Shader;
use super::{
    deferred_mode,
    texture_internal::{FrameBuffer, InternalTexType, TextureInternal2D, TextureInternal3D},
};
use fere_common::*;
use gl::types::GLenum;

// Rendering Pipeline
// TODO

pub fn create_deferred(size: IVec2) -> FrameBuffer {
    let outputs = vec![
        TextureInternal2D::new(InternalTexType::Float3, size), // pos
        TextureInternal2D::new(InternalTexType::HalfFloat3, size), // normal
        TextureInternal2D::new(InternalTexType::Color, size),  // basecolor
        TextureInternal2D::new(InternalTexType::Material, size), // roughness
        TextureInternal2D::new(InternalTexType::Material, size), // metalness
        TextureInternal2D::new(InternalTexType::Float3, size), // emission
        TextureInternal2D::new(InternalTexType::Index, size),  // object_index
        TextureInternal2D::new(InternalTexType::Flag, size),   // lighting
    ];

    let depth = TextureInternal2D::new(InternalTexType::Depth, size);
    FrameBuffer::new(outputs, Some(depth))
}

pub fn create_final(size: IVec2) -> FrameBuffer {
    let outputs = vec![TextureInternal2D::new(InternalTexType::Float3, size)];
    FrameBuffer::new(outputs, None)
}

pub fn create_shadow(size: u32) -> FrameBuffer {
    let size = IVec2::new(size as i32, size as i32);

    let outputs = vec![];
    let depth = TextureInternal2D::new(InternalTexType::Depth, size);

    FrameBuffer::new(outputs, Some(depth))
}

pub fn create_probe(size: u32) -> FrameBuffer {
    let size = IVec2::new(size as i32, size as i32);
    let outputs = vec![
        TextureInternal2D::new(InternalTexType::Float3, size), // diffuse
        TextureInternal2D::new(InternalTexType::Float3, size), // emission
    ];

    let depth = TextureInternal2D::new(InternalTexType::Depth, size);
    FrameBuffer::new(outputs, Some(depth))
}

pub fn create_yuv(size: IVec2) -> FrameBuffer {
    let outputs = vec![
        TextureInternal2D::new(InternalTexType::Yuv, size),
        TextureInternal2D::new(InternalTexType::Yuv, size),
        TextureInternal2D::new(InternalTexType::Yuv, size),
    ];
    FrameBuffer::new(outputs, None)
}

pub fn bind_shadow(graphics: &super::Graphics, index: usize) {
    deferred_mode(false, true, false);
    graphics.pass_shadow[index].bind();
    graphics.pass_shadow[index].clear_depth();
    graphics.pass_shadow[index].clear_color_all();
    unsafe {
        gl::Disable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Disable(gl::MULTISAMPLE);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::FRONT); // Important!
        gl::Disable(gl::STENCIL_TEST);
    }
}

pub fn bind_probe(graphics: &super::Graphics) {
    deferred_mode(true, true, false);
    graphics.pass_probe.bind();
    graphics.pass_probe.clear_depth();
    graphics.pass_probe.clear_color_all();
    unsafe {
        gl::Disable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Disable(gl::MULTISAMPLE);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::Disable(gl::STENCIL_TEST);
    }
}

pub fn bind_deferred_pass1(graphics: &super::Graphics) {
    deferred_mode(true, true, true);
    graphics.pass_deferred1.bind();
    graphics.pass_deferred1.clear_depth();
    graphics.pass_deferred1.clear_color_all();
    unsafe {
        gl::Disable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Disable(gl::MULTISAMPLE);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::Disable(gl::STENCIL_TEST);
    }
}

pub fn bind_deferred_pass2(graphics: &super::Graphics, clear: bool) {
    graphics.pass_final.bind();
    if clear {
        graphics.pass_final.clear_depth();
        graphics.pass_final.clear_color_all();
    }

    // Various configurations will be done in each drawing of light
    // because each splits to two passes.

    unsafe {
        gl::Clear(gl::STENCIL_BUFFER_BIT);
        gl::Enable(gl::DEPTH_TEST);
        gl::Disable(gl::MULTISAMPLE);
        gl::Enable(gl::CULL_FACE);
    }
}

pub fn bind_gbuffer(graphics: &super::Graphics, program: &Shader, offset: usize) {
    let pass = &graphics.pass_deferred1;
    unsafe {
        for i in 0..8 {
            // Object index is replaced with shadow map for final shade
            if i == 6 {
                continue;
            }
            gl::ActiveTexture(gl::TEXTURE0 + i as GLenum + offset as GLenum);
            gl::BindTexture(gl::TEXTURE_2D, pass.outputs_get()[i].tex_get().raw_get());
            gl::Uniform1i(program.uloc_get_tex()[i + offset], (i + offset) as i32)
        }
    }
}

pub fn bind_probe_volume(
    _graphics: &super::Graphics,
    program: &Shader,
    offset: usize,
    illumination: &TextureInternal3D,
    depth: &TextureInternal3D,
) {
    let targets = [illumination, depth];
    for (i, tex) in targets.iter().enumerate() {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + i as GLenum + offset as GLenum);
            gl::BindTexture(gl::TEXTURE_3D, tex.tex_get().raw_get());
            gl::Uniform1i(program.uloc_get_tex()[i + offset], (i + offset) as i32)
        }
    }
}

pub fn render_final(graphics: &super::Graphics) {
    deferred_mode(true, false, false);

    unsafe {
        gl::Disable(gl::BLEND);
        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::MULTISAMPLE);
        gl::Disable(gl::CULL_FACE);
        gl::Disable(gl::STENCIL_TEST);

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        deferred_mode(true, true, false);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        let program = graphics.prgs.dr_3.bind();

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(
            gl::TEXTURE_2D,
            graphics.pass_final.outputs_get()[0].tex_get().raw_get(),
        );
        gl::Uniform1i(program.uloc_get_tex()[0], 0);

        graphics.fill_screen(program);
    }
}

pub fn render_yuv(graphics: &super::Graphics) {
    deferred_mode(true, false, false);
    graphics.pass_yuv.as_ref().unwrap().bind();
    graphics.pass_yuv.as_ref().unwrap().clear_color_all();
    unsafe {
        gl::ColorMask(1, 1, 1, 1);
        gl::Disable(gl::BLEND);
        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::MULTISAMPLE);
        gl::Disable(gl::CULL_FACE);
        gl::Disable(gl::STENCIL_TEST);

        let program = graphics.prgs.yuv.bind();

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(
            gl::TEXTURE_2D,
            graphics.pass_final.outputs_get()[0].tex_get().raw_get(),
        );
        gl::Uniform1i(program.uloc_get_tex()[0], 0);

        graphics.fill_screen(program);
    }
}

pub fn bind_forward(_graphics: &super::Graphics) {
    deferred_mode(true, true, false);
    //graphics.pass_final.bind(false);
    unsafe {
        gl::Enable(gl::BLEND);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Disable(gl::MULTISAMPLE);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::Disable(gl::STENCIL_TEST);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
}

pub fn bind_2d(graphics: &super::Graphics) {
    deferred_mode(true, false, false);
    graphics.pass_final.bind();
    unsafe {
        gl::Enable(gl::BLEND);
        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::MULTISAMPLE);
        gl::Disable(gl::CULL_FACE);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
}
