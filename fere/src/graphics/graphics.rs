pub mod material;
mod pass;
mod prgs;
mod probe;
pub mod texture_internal;

use super::{
    glmanager::{
        shader::{self, Shader},
        GlManager,
    },
    render_unit::RenderUnit,
};
use crate::graphics::glmanager::light::*;
use crate::graphics::resources::Meshes;
use fere_common::*;
use fere_resources::*;
use gl::types::GLuint;
use serde::{Deserialize, Serialize};
use texture_internal::{FrameBuffer, TextureInternal3D};

pub fn deferred_mode(color: bool, depth: bool, index: bool) {
    unsafe {
        gl::DepthMask(depth as u8);
        if !color && index {
            gl::ColorMask(0, 0, 0, 0);
            gl::ColorMaski(6, 1, 0, 0, 0);
        } else if color && !index {
            gl::ColorMask(1, 1, 1, 1);
            gl::ColorMaski(6, 0, 0, 0, 0);
        } else if color && index {
            gl::ColorMask(1, 1, 1, 1);
        } else {
            gl::ColorMask(0, 0, 0, 0);
        }
    }
}

#[derive(Debug)]
pub struct TextureFetcher {
    _framebuffer: GLuint,
    _color_attachment: GLuint,
}

impl TextureFetcher {
    pub fn fetch(&self) -> u64 {
        0
    }

    pub fn noop() -> Self {
        Self {
            _framebuffer: 0,
            _color_attachment: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GraphicsConfig {
    pub resolution: IVec2,
    pub shadow_resolution: usize,
    pub probe_resolution: usize,
    pub max_major_lights: usize,
}

pub struct Graphics {
    gl_manager: GlManager,

    screen_size: IVec2,

    // passes
    pass_deferred1: FrameBuffer,
    pass_final: FrameBuffer,
    pass_shadow: Vec<FrameBuffer>,
    pass_probe: FrameBuffer,

    // useful meshes
    meshes: Meshes,

    pub prgs: prgs::Programs,
}

impl Graphics {
    pub fn new(config: GraphicsConfig) -> Self {
        let gl_manager = GlManager::new("".to_string());
        let screen_size = config.resolution;
        let max_major_lights = config.max_major_lights;

        let pass_deferred1 = pass::create_deferred(screen_size);
        let pass_final = pass::create_final(screen_size);

        let pass_shadow = (0..max_major_lights)
            .map(|_| pass::create_shadow(config.shadow_resolution as u32))
            .collect::<Vec<_>>();
        let pass_probe = pass::create_probe(config.probe_resolution as u32);

        let meshes = Meshes::default();

        let prgs = prgs::Programs::new(&gl_manager);

        Graphics {
            gl_manager,
            screen_size,
            pass_deferred1,
            pass_final,
            pass_shadow,
            pass_probe,
            meshes,
            prgs,
        }
    }

    pub fn meshes(&self) -> &Meshes {
        &self.meshes
    }

    /// Returns (FrameBuffer, ColorAttachment Index) for the UI.
    ///
    /// You can fetch object index from the buffer.
    pub fn get_object_index_fetcher(&self) -> TextureFetcher {
        TextureFetcher {
            _framebuffer: self.pass_deferred1.raw_get(),
            _color_attachment: 6,
        }
    }

    pub fn screen_size(&self) -> IVec2 {
        self.screen_size
    }

    pub fn ru_set(&self, program: &shader::Shader, unit: &RenderUnit) {
        deferred_mode(unit.color, unit.depth, unit.id.is_some());
        unsafe {
            if unit.depth_test {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }
            if let Some(id) = unit.id {
                let u = program.uloc_get(shader::Uniform::ObjectIndex);
                gl::Uniform1ui(u, id);
            }
            if let Some(lighting) = unit.lighting.as_ref() {
                let u = program.uloc_get(shader::Uniform::Lighting);
                gl::Uniform1i(u, *lighting as i32);
            }
        }
    }

    pub fn get_gl(&self) -> &GlManager {
        &self.gl_manager
    }

    /// We use stencil buffer trick to
    /// 1. handle the case where the camera enters inside the volume
    /// 2. reject those fragments BEHIND the volume, but not affected by volume
    ///
    /// https://kayru.org/articles/deferred-stencil/
    fn draw_lighvolume_common(&self, mesh: &Mesh) {
        mesh.bind();
        deferred_mode(false, false, false);
        unsafe {
            gl::CullFace(gl::BACK);
            gl::DepthFunc(gl::LEQUAL);
            gl::Disable(gl::STENCIL_TEST);

            gl::StencilFunc(gl::ALWAYS, 0, 0);
            gl::StencilOp(gl::KEEP, gl::KEEP, gl::INCR);

            gl::Disable(gl::BLEND);
        }
        mesh.draw();

        deferred_mode(true, false, false);
        unsafe {
            gl::CullFace(gl::FRONT);
            gl::DepthFunc(gl::GEQUAL);
            gl::Disable(gl::STENCIL_TEST);

            gl::StencilFunc(gl::NOTEQUAL, 0, 0xFF);
            gl::StencilOp(gl::ZERO, gl::ZERO, gl::ZERO);

            gl::Enable(gl::BLEND);
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::ONE, gl::ONE);
        }
        mesh.draw();
    }

    pub fn draw_lightvolume_uni(
        &self,
        program: &Shader,
        light: &LightUni,
        cpos: Vec3,
        _range: bool,
    ) {
        program.uniform_light(&light.light, 0);
        program.uniform_camera(&cpos);

        let radius = light.radius;

        program.uniform_model_s(
            &glm::vec4_to_vec3(&light.light.pos),
            &glm::identity(),
            &Vec3::from_element(radius),
            false,
        );
        self.draw_lighvolume_common(&self.meshes.sphere);
    }

    pub fn get_transform_for_lightvolume_dir(light: &LightDir) -> Mat4 {
        let mut trans = glm::translate(&Mat4::identity(), &glm::vec4_to_vec3(&light.light.pos));

        trans *= fere_common::geo::rotation_between(
            &Vec3::new(1.0, 0.0, 0.0),
            &Vec3::new(0.0, -1.0, 0.0),
            &light.xdir,
            &light.ydir,
        );
        trans = glm::scale(&trans, &Vec3::from_element(light.radius));
        trans = glm::scale(
            &trans,
            &Vec3::new((light.angle / 2.0).tan(), (light.angle / 2.0).tan(), 1.0),
        );
        trans = glm::translate(&trans, &Vec3::new(0.0, 0.0, 1.0));
        trans = glm::rotate(&trans, (180.0_f32).to_radians(), &Vec3::new(1.0, 0.0, 0.0));
        trans
    }

    pub fn draw_lightvolume_dir(&self, program: &Shader, light: &LightDir, cpos: Vec3) {
        program.uniform_light_dir(&light, 0);
        program.uniform_camera(&cpos);

        let trans = Self::get_transform_for_lightvolume_dir(light);
        program.uniform_model(&trans, false);
        self.draw_lighvolume_common(&self.meshes.pyramid);
    }

    pub fn draw_lightvolume_ambient(
        &self,
        program: &Shader,
        cbpos: &Vec3,
        cpos: &Vec3,
        size: &Vec3,
    ) {
        self.meshes.cube.bind();

        let trans = glm::translate(&glm::identity(), cbpos);
        let trans = glm::scale(&trans, size);
        let trans = glm::translate(&trans, &Vec3::new(0.0, 0.0, 0.5));
        program.uniform_model(&trans, false);
        program.uniform_camera(&cpos);
        self.draw_lighvolume_common(&self.meshes.cube);
    }

    pub fn fill_screen(&self, prg: &Shader) {
        prg.uniform_transformations(&Mat4::new_scaling(2.0), &Mat4::identity());
        self.meshes.square.bind();
        prg.uniform_model(&Mat4::identity(), false);
        self.meshes.square.draw();
    }

    pub fn bind_deferred_pass1(&self) {
        pass::bind_deferred_pass1(self)
    }

    pub fn bind_deferred_pass2(&self, clear: bool) {
        pass::bind_deferred_pass2(self, clear)
    }

    pub fn bind_shadow(&self, index: usize) {
        pass::bind_shadow(self, index);
    }

    pub fn bind_probe(&self) {
        pass::bind_probe(self)
    }

    pub fn bind_shadow_map(&self, program: &Shader, index: usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE6);
            gl::BindTexture(
                gl::TEXTURE_2D,
                self.pass_shadow[index].depth_get().tex_get().raw_get(),
            );
            gl::Uniform1i(program.uloc_get_tex()[6], 6)
        }
    }

    pub fn bind_gbuffer(&self, program: &Shader, offset: usize) {
        pass::bind_gbuffer(self, program, offset)
    }

    pub fn bind_probe_volume(
        &self,
        program: &Shader,
        offset: usize,
        diffuse: &TextureInternal3D,
        illumination: &TextureInternal3D,
        depth: &TextureInternal3D,
    ) {
        pass::bind_probe_volume(self, program, offset, diffuse, illumination, depth)
    }

    pub fn render_final(&self) {
        pass::render_final(self)
    }

    pub fn bind_forward(&self) {
        pass::bind_forward(self);
    }

    pub fn bind_2d(&self) {
        pass::bind_2d(self)
    }
}
