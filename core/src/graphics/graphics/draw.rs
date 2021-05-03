use super::material::*;

use super::glmanager::shader::Shader;
use tpf_math::types::*;
use tpf_package::resources::{surface::*, Model3DInstance};

pub fn bind_fixed_color(prg: &Shader, color: &IVec4) {
    let surface = TransparentI {
        general: GeneralI {
            basecolor: TexVar::U(color.xyz()),
            roughness: TexVar::U(0),
            metalness: TexVar::U(0),
            normal: no_normal_map(),
        },
        alpha: TexVar::U(0),
    };

    bind_general(prg, &surface.general);
    bind_transparent(prg, &surface);
}

pub fn draw_world(_model: &Model3DInstance) {
    //for mesh in model.
}
