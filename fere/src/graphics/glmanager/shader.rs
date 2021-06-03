#![allow(clippy::needless_range_loop)]

use super::light::*;
use fere_common::*;
use gl::types::*;
use heck::SnakeCase;
use std::ffi::CString;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

const UNIFORM_MAX: usize = 16;

#[derive(AsRefStr, EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum Uniform {
    Null,
    // Transformatinos
    Projection,
    View,
    Model,
    Model3,
    Inside,

    // Material properties. Used optionally instead of texture
    Basecolor,
    Roughness,
    Metalness,
    Emission,
    EmissionIntensity,
    Emission2,
    EmissionIntensity2,
    Transparent,

    BasecolorOn,
    RoughnessOn,
    MetalnessOn,
    EmissionOn,
    EmissionIntensityOn,
    Emission2On,
    EmissionIntensity2On,
    TransparentOn,

    // Visualizing single sh
    ShCoeff,

    // Emission is the only material which is natural to be dynamic,
    // thus we support blending
    EmissionBlend,
    EmissionBlendRate,

    // True then use normal mapping
    NormalMap,

    // Integer things
    ObjectIndex,
    Lighting,

    // Shading
    Ambient,
    Cpos,
    Shadow,

    // Non-trivial
    Color,

    Max_,
}

#[derive(AsRefStr, EnumIter, Debug)]
pub enum UniformPv {
    Null,
    Coeff, //raw
    Trans,
    Offset,
    CellSize,
    Nums,
    RoomSize,
    PaddedRoomSize,
    Params,
    Max_,
}

#[derive(AsRefStr, EnumIter, Debug)]
pub enum UniformLight {
    Null,

    Wpos,
    Color,
    Trans,
    Round,
    Smoothness, // 0~1, availiable only if GULightRound == true
    Max_,
}

/// It works with currently bound program
unsafe fn find_uniforms(prg: GLuint) -> (Vec<GLint>, Vec<GLint>, Vec<GLint>, Vec<Vec<GLint>>) {
    let mut uloc: Vec<GLint> = Vec::new();
    let mut uloc_pv: Vec<GLint> = Vec::new();
    let mut uloc_tex: Vec<GLint> = Vec::new();
    let mut uloc_multiple: Vec<Vec<GLint>> = Vec::new();

    for u in Uniform::iter() {
        let name = CString::new(format!("u_{}", u.as_ref().to_owned().to_snake_case())).unwrap();
        uloc.push(gl::GetUniformLocation(prg, name.as_ptr().cast()));
    }

    // TODO: take sh_coeff in the struct
    for u in UniformPv::iter() {
        let name = CString::new(format!("u_lv.{}", u.as_ref().to_owned().to_snake_case())).unwrap();
        uloc_pv.push(gl::GetUniformLocation(prg, name.as_ptr().cast()));
    }

    for i in 0..UNIFORM_MAX {
        uloc_multiple.push(Vec::new());
        for u in UniformLight::iter() {
            let s = CString::new(format!(
                "u_lights[{}].{}",
                i,
                u.as_ref().to_owned().to_snake_case()
            ))
            .unwrap();
            uloc_multiple[i].push(gl::GetUniformLocation(prg, s.as_ptr().cast()));
        }
    }

    for i in 0..UNIFORM_MAX {
        let name = CString::new(format!("u_tex{}", i)).unwrap();
        uloc_tex.push(gl::GetUniformLocation(prg, name.as_ptr().cast()));
    }

    (uloc, uloc_pv, uloc_tex, uloc_multiple)
}

pub struct Shader {
    name: String,
    raw: GLuint,
    uloc: Vec<GLint>,
    uloc_pv: Vec<GLint>,
    uloc_tex: Vec<GLint>,
    uloc_lights: Vec<Vec<GLint>>,
}

impl Shader {
    pub fn new_empty() -> Self {
        Shader {
            name: "NULL".to_owned(),
            raw: 0,
            uloc: Default::default(),
            uloc_pv: Default::default(),
            uloc_tex: Default::default(),
            uloc_lights: Default::default(),
        }
    }

    pub fn new(name: String, vert: &str, vert_path: &str, frag: &str, frag_path: &str) -> Self {
        let program = super::compile::link_program(
            super::compile::compile_shader(vert_path, vert, gl::VERTEX_SHADER),
            super::compile::compile_shader(frag_path, frag, gl::FRAGMENT_SHADER),
        );

        unsafe {
            gl::UseProgram(program);
        }

        let (uloc, uloc_pv, uloc_tex, uloc_lights) = unsafe { find_uniforms(program) };

        // set all textures
        for i in 0..UNIFORM_MAX {
            unsafe {
                gl::Uniform1i(uloc_tex[i], i as i32);
            }
        }

        Shader {
            name,
            raw: program,
            uloc,
            uloc_pv,
            uloc_tex,
            uloc_lights,
        }
    }

    pub fn raw(&self) -> GLuint {
        self.raw
    }

    pub fn uloc_get(&self, x: Uniform) -> GLint {
        self.uloc[x as usize]
    }

    pub fn uloc_get_tex(&self) -> &Vec<GLint> {
        &self.uloc_tex
    }

    pub fn uniform_color(&self, color: &IVec4) {
        let u: GLint;
        u = self.uloc[Uniform::Color as usize];
        unsafe {
            let color: Vec4 = nalgebra::convert(*color);
            let color = color / 255.0;
            gl::Uniform4fv(u, 1, color.as_ptr());
        }
    }

    pub fn uniform_ambient(&self, color: &Vec3) {
        let u: GLint;
        u = self.uloc[Uniform::Ambient as usize];
        unsafe {
            gl::Uniform3fv(u, 1, color.as_ptr());
        }
    }

    pub fn uniform_line(&self, p1: &Vec3, p2: &Vec3, width: f32) {
        unsafe {
            gl::LineWidth(width);
            let trans = glm::translate(&Mat4::identity(), p1);
            let trans = glm::scale(&trans, &(p2 - p1));

            let u: GLint;
            u = self.uloc[Uniform::Model as usize];
            gl::UniformMatrix4fv(u, 1, gl::FALSE, trans.as_ptr());
        }
    }

    pub fn uniform_wireframe(&self, trans: &Mat4, color: &IVec4, width: f32) {
        unsafe {
            gl::LineWidth(width);

            self.uniform_color(color);

            let u: GLint;
            u = self.uloc[Uniform::Model as usize];
            gl::UniformMatrix4fv(u, 1, gl::FALSE, trans.as_ptr());
        }
    }

    pub fn uniform_transformations(&self, projection: &Mat4, view: &Mat4) {
        unsafe {
            let mut u: GLint;
            u = self.uloc[Uniform::Projection as usize];
            gl::UniformMatrix4fv(u, 1, gl::FALSE, projection.as_ptr());
            u = self.uloc[Uniform::View as usize];
            gl::UniformMatrix4fv(u, 1, gl::FALSE, view.as_ptr());
        }
    }

    pub fn uniform_model(&self, model_transform: &Mat4, inside: bool) {
        unsafe {
            let mut u: GLint;
            u = self.uloc[Uniform::Model as usize];
            gl::UniformMatrix4fv(u, 1, gl::FALSE, model_transform.as_ptr());
            u = self.uloc[Uniform::Model3 as usize];
            let mat3: Mat3 = model_transform.fixed_slice::<3, 3>(0, 0).into();
            gl::UniformMatrix3fv(u, 1, gl::FALSE, mat3.as_ptr());
            u = self.uloc[Uniform::Inside as usize];
            gl::Uniform1i(u, inside as i32);
        }
    }

    pub fn uniform_model_s(&self, pos: &Vec3, rotate: &Mat4, scale: &Vec3, inside: bool) {
        let trans = glm::translate(&glm::identity(), pos);
        let trans = trans * rotate;
        let trans = glm::scale(&trans, scale);
        self.uniform_model(&trans, inside);
    }

    pub fn uniform_light(&self, light: &Light, i: usize) {
        unsafe {
            let mut u: GLint;
            u = self.uloc[Uniform::Shadow as usize];
            gl::Uniform1i(u, light.shadow as i32);
            u = self.uloc_lights[i][UniformLight::Wpos as usize];
            gl::Uniform4fv(u, 1, light.pos.as_ptr());
            u = self.uloc_lights[i][UniformLight::Color as usize];
            gl::Uniform3fv(u, 1, light.color.as_ptr());
        }
    }

    pub fn uniform_light_dir(&self, light: &LightDir, i: usize) {
        self.uniform_light(&light.light, i);
        unsafe {
            let mut u: GLint;
            u = self.uloc_lights[i][UniformLight::Trans as usize];
            gl::UniformMatrix4fv(u, 1, gl::FALSE, light.trans.as_ptr());
            u = self.uloc_lights[i][UniformLight::Round as usize];
            gl::Uniform1i(u, light.round as i32);
            u = self.uloc_lights[i][UniformLight::Smoothness as usize];
            gl::Uniform1f(u, light.smoothnes);
        }
    }

    pub fn uniform_camera(&self, cpos: &Vec3) {
        unsafe {
            let u = self.uloc[Uniform::Cpos as usize];
            gl::Uniform3fv(u, 1, cpos.as_ptr());
        }
    }

    pub fn uniform_probe_volume(&self, pv: &ProbeVolumeRoom) {
        unsafe {
            let u = self.uloc_pv[UniformPv::Trans as usize];
            gl::UniformMatrix4fv(u, 1, gl::FALSE, pv.trans.as_ptr());
            let u = self.uloc_pv[UniformPv::Offset as usize];
            gl::Uniform3fv(u, 1, pv.offset.as_ptr());
            let u = self.uloc_pv[UniformPv::CellSize as usize];
            gl::Uniform3fv(u, 1, pv.cell_size.as_ptr());
            let u = self.uloc_pv[UniformPv::Nums as usize];
            gl::Uniform3iv(u, 1, pv.nums.as_ptr());
            let u = self.uloc_pv[UniformPv::RoomSize as usize];
            gl::Uniform3fv(u, 1, pv.room_size.as_ptr());
            let u = self.uloc_pv[UniformPv::PaddedRoomSize as usize];
            gl::Uniform3fv(u, 1, pv.padded_room_size.as_ptr());
            let u = self.uloc_pv[UniformPv::Params as usize];
            gl::Uniform1i(u, pv.params as i32);
        }
    }

    pub fn uniform_texture(&self, index: usize, tex: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index as GLenum);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::Uniform1i(self.uloc_get_tex()[index], index as i32)
        }
    }

    pub fn bind(&self) -> &Self {
        debug_assert_ne!(self.name, "NULL");
        unsafe {
            gl::UseProgram(self.raw);
        }
        self
    }
}

// various uniform functions
impl Shader {
    pub fn uniform_mat4(&self, loc: GLint, mat: &Mat4) {
        unsafe {
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat.as_ptr());
        }
    }

    pub fn uniform_vec3(&self, loc: GLint, x: &Vec3) {
        unsafe {
            gl::Uniform3fv(loc, 1, x.as_ptr());
        }
    }

    pub fn uniform_vec4(&self, loc: GLint, x: &Vec4) {
        unsafe {
            gl::Uniform4fv(loc, 1, x.as_ptr());
        }
    }

    pub fn uniform_sh(&self, sh: &[Vec3]) {
        unsafe {
            gl::Uniform3fv(
                self.uloc[Uniform::ShCoeff as usize],
                sh.len() as i32,
                sh.as_ptr().cast(),
            );
        }
    }
}
