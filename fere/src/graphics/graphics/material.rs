#![allow(dead_code)]

use crate::graphics::glmanager::shader::{Shader, Uniform};
use crate::resources::{surface::*, *};
use fere_common::*;
use std::sync::Arc;

/*
Texture mapping
0: Basecolor
1: Roughness
2: Metalness
3: Emission
4: EmissionIntensity
5: Normal
6: Emission2 or
7: EmissionIntensity2 or
*/

fn bind_texture1(
    prg: &Shader,
    tex: &TexVar<Arc<Texture>, u8>,
    tex_index: u32,
    on: Uniform,
    uni: Uniform,
) {
    unsafe {
        match tex {
            TexVar::T(x) => {
                gl::Uniform1i(prg.uloc_get(on), 0);
                gl::ActiveTexture(gl::TEXTURE0 + tex_index);
                x.bind();
            }
            TexVar::U(x) => {
                gl::Uniform1i(prg.uloc_get(on), 1);
                gl::Uniform1f(prg.uloc_get(uni), *x as f32 / 255.0);
            }
        }
    }
}

fn bind_texture3(
    prg: &Shader,
    tex: &TexVar<Arc<Texture>, IVec3>,
    tex_index: u32,
    on: Uniform,
    uni: Uniform,
) {
    unsafe {
        match tex {
            TexVar::T(x) => {
                gl::Uniform1i(prg.uloc_get(on), 0);
                gl::ActiveTexture(gl::TEXTURE0 + tex_index);
                x.bind();
            }
            TexVar::U(x) => {
                gl::Uniform1i(prg.uloc_get(on), 1);
                let bc: Vec3 = nalgebra::convert(*x);
                let bc = bc / 255.0;
                gl::Uniform3fv(prg.uloc_get(uni), 1, bc.as_ptr());
            }
        }
    }
}

fn bind_general_(prg: &Shader, surface: &GeneralI) {
    unsafe {
        bind_texture3(
            prg,
            &surface.basecolor,
            0,
            Uniform::BasecolorOn,
            Uniform::Basecolor,
        );
        bind_texture1(
            prg,
            &surface.roughness,
            1,
            Uniform::RoughnessOn,
            Uniform::Roughness,
        );
        bind_texture1(
            prg,
            &surface.metalness,
            2,
            Uniform::MetalnessOn,
            Uniform::Metalness,
        );
        match &surface.normal {
            TexVar::T(x) => {
                gl::Uniform1i(prg.uloc_get(Uniform::NormalMap), 0);
                gl::ActiveTexture(gl::TEXTURE5);
                x.bind();
            }
            TexVar::U(_) => {
                gl::Uniform1i(prg.uloc_get(Uniform::NormalMap), 1);
            }
        }
    }
}

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

pub fn bind_general(prg: &Shader, surface: &GeneralI) {
    bind_general_(prg, surface);
    bind_no_emission(prg);
    bind_no_transparent(prg);
}

pub fn bind_transparent(prg: &Shader, surface: &TransparentI) {
    bind_general_(prg, &surface.general);
    bind_no_emission(prg);

    bind_texture1(
        prg,
        &surface.alpha,
        6,
        Uniform::TransparentOn,
        Uniform::Transparent,
    )
}

fn bind_no_transparent(prg: &Shader) {
    unsafe {
        gl::Uniform1i(prg.uloc_get(Uniform::TransparentOn), 1);
        gl::Uniform1f(prg.uloc_get(Uniform::Transparent), 0.0);
    }
}

/// time: 0 to 1
pub fn bind_emissive_static(prg: &Shader, surface: &EmissiveStaticI, time: f64) {
    bind_general_(prg, &surface.general);
    bind_no_transparent(prg);

    let first = (time * TIMEPOINT_NUMBER as f64) as usize;
    let second = (first + 1) % TIMEPOINT_NUMBER;
    let blend = time * TIMEPOINT_NUMBER as f64 - first as f64;

    unsafe {
        gl::Uniform1i(prg.uloc_get(Uniform::EmissionBlend), 1);
        gl::Uniform1f(prg.uloc_get(Uniform::EmissionBlendRate), blend as f32);
    }

    bind_texture3(
        prg,
        &surface.timepoints[first].emission,
        3,
        Uniform::EmissionOn,
        Uniform::Emission,
    );
    bind_texture1(
        prg,
        &surface.timepoints[first].emission_intensity,
        4,
        Uniform::EmissionIntensityOn,
        Uniform::EmissionIntensity,
    );

    bind_texture3(
        prg,
        &surface.timepoints[second].emission,
        6,
        Uniform::Emission2On,
        Uniform::Emission2,
    );
    bind_texture1(
        prg,
        &surface.timepoints[second].emission_intensity,
        7,
        Uniform::EmissionIntensity2On,
        Uniform::EmissionIntensity2,
    );
}

fn bind_no_emission(prg: &Shader) {
    unsafe {
        let zero = Vec3::from_element(0.0);
        gl::Uniform1i(prg.uloc_get(Uniform::EmissionOn), 1);
        gl::Uniform1i(prg.uloc_get(Uniform::EmissionBlend), 0);
        gl::Uniform3fv(prg.uloc_get(Uniform::Emission), 1, zero.as_ptr());
    }
}

/// It must be called together with `bind_general()`
pub fn bind_emissive_dynamic(
    prg: &Shader,
    surface: &EmissiveDynamic,
    materials: &[EmissiveMaterialI],
) {
    let (emission, intensity) = match surface.current {
        CurrentEmission::Arbitrary(x) => (TexVar::U(x.xyz()), TexVar::U(x.w as u8)),
        CurrentEmission::Material(x) => match &materials[x] {
            EmissiveMaterial::Plain(emission, intensity) => (emission.clone(), intensity.clone()),
            _ => panic!(),
        },
    };
    unsafe {
        gl::Uniform1i(prg.uloc_get(Uniform::EmissionBlend), 0);
    }

    bind_texture3(prg, &emission, 3, Uniform::EmissionOn, Uniform::Emission);
    bind_texture1(
        prg,
        &intensity,
        4,
        Uniform::EmissionIntensityOn,
        Uniform::EmissionIntensity,
    );
}

pub fn bind_simple_general(prg: &Shader, surface: &GeneralSimple) {
    bind_texture3(
        prg,
        &surface.basecolor,
        0,
        Uniform::BasecolorOn,
        Uniform::Basecolor,
    );
    bind_texture3(
        prg,
        &surface.emission,
        1,
        Uniform::EmissionOn,
        Uniform::Emission,
    );
    bind_texture1(
        prg,
        &surface.emission_intensity,
        2,
        Uniform::EmissionIntensityOn,
        Uniform::EmissionIntensity,
    );
}
