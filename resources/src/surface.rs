use super::{Mesh, Texture};
use fere_common::*;
use std::{fmt::Debug, sync::Arc};

pub fn no_normal_map<T>() -> TexVar<T, IVec3> {
    TexVar::U(IVec3::new(0, 0, 0))
}

#[derive(Debug)]
pub enum TexVar<T, U> {
    T(T),
    U(U),
}

impl<T: Clone, U: Clone> Clone for TexVar<T, U> {
    fn clone(&self) -> Self {
        match self {
            TexVar::T(x) => TexVar::T(x.clone()),
            TexVar::U(x) => TexVar::U(x.clone()),
        }
    }
}

impl<T: Clone, U: Clone> TexVar<T, U> {
    pub fn to_instance(&self) -> TexVar<T, U> {
        match self {
            TexVar::T(x) => TexVar::T(x.clone()),
            TexVar::U(x) => TexVar::U(x.clone()),
        }
    }
}

pub trait TypePack {
    type TexturePointer1: Debug;
    type TexturePointer3: Debug;
    type VideoPointer: Debug;
    type MeshPointer: Debug;
    type NamePointer: Debug;
}

#[derive(Debug)]
pub struct TypePackSource;
impl TypePack for TypePackSource {
    type TexturePointer1 = TexVar<Arc<Texture>, u8>;
    type TexturePointer3 = TexVar<Arc<Texture>, IVec3>;
    type VideoPointer = Arc<u8>;
    type MeshPointer = Arc<Mesh>;
    type NamePointer = Arc<String>;
}

#[derive(Debug)]
pub struct TypePackInstance;
impl TypePack for TypePackInstance {
    type TexturePointer1 = TexVar<Arc<Texture>, u8>;
    type TexturePointer3 = TexVar<Arc<Texture>, IVec3>;
    type VideoPointer = Arc<u8>;
    type MeshPointer = Arc<Mesh>;
    type NamePointer = Arc<String>;
}

#[derive(Clone)]
pub struct GeneralSimple {
    pub basecolor: <TypePackInstance as TypePack>::TexturePointer3,
    pub emission: <TypePackInstance as TypePack>::TexturePointer3,
    pub emission_intensity: <TypePackInstance as TypePack>::TexturePointer1,
}

#[derive(Debug)]
pub struct General<T: TypePack> {
    pub basecolor: T::TexturePointer3,
    pub roughness: T::TexturePointer1,
    pub metalness: T::TexturePointer1,
    /// If no texture, value will be ignored (because mesh already has one)
    pub normal: T::TexturePointer3,
}
pub type GeneralS = General<TypePackSource>;
pub type GeneralI = General<TypePackInstance>;
impl GeneralS {
    pub fn to_instance(&self) -> GeneralI {
        GeneralI {
            basecolor: self.basecolor.to_instance(),
            roughness: self.roughness.to_instance(),
            metalness: self.metalness.to_instance(),
            normal: self.normal.to_instance(),
        }
    }
}
impl Clone for GeneralI {
    fn clone(&self) -> Self {
        GeneralI {
            basecolor: self.basecolor.clone(),
            roughness: self.roughness.clone(),
            metalness: self.metalness.clone(),
            normal: self.normal.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Transparent<T: TypePack> {
    pub general: General<T>,
    pub alpha: T::TexturePointer1,
}
pub type TransparentS = Transparent<TypePackSource>;
pub type TransparentI = Transparent<TypePackInstance>;
impl TransparentS {
    pub fn to_instance(&self) -> TransparentI {
        TransparentI {
            general: self.general.to_instance(),
            alpha: self.alpha.to_instance(),
        }
    }
}
impl Clone for TransparentI {
    fn clone(&self) -> Self {
        TransparentI {
            general: self.general.clone(),
            alpha: self.alpha.clone(),
        }
    }
}

pub const TIMEPOINT_NUMBER: usize = 4;
#[derive(Clone, Debug)]
pub struct Timepoint<T: TypePack> {
    pub emission: T::TexturePointer3,
    pub emission_intensity: T::TexturePointer1,
    /// smoothness to next timepoint: 0~1
    pub smoothness: f32,
}
pub type TimepointS = Timepoint<TypePackSource>;
pub type TimepointI = Timepoint<TypePackInstance>;

impl TimepointS {
    pub fn to_instance(&self) -> TimepointI {
        TimepointI {
            emission: self.emission.to_instance(),
            emission_intensity: self.emission_intensity.to_instance(),
            smoothness: self.smoothness,
        }
    }
}
impl Clone for TimepointI {
    fn clone(&self) -> Self {
        TimepointI {
            emission: self.emission.clone(),
            emission_intensity: self.emission_intensity.clone(),
            smoothness: self.smoothness,
        }
    }
}

#[derive(Debug)]
pub struct EmissiveStatic<T: TypePack> {
    pub general: General<T>,
    pub timepoints: [Timepoint<T>; TIMEPOINT_NUMBER],
}
pub type EmissiveStaticS = EmissiveStatic<TypePackSource>;
pub type EmissiveStaticI = EmissiveStatic<TypePackInstance>;
impl EmissiveStaticS {
    pub fn to_instance(&self) -> EmissiveStaticI {
        EmissiveStaticI {
            general: self.general.to_instance(),
            timepoints: [
                self.timepoints[0].to_instance(),
                self.timepoints[1].to_instance(),
                self.timepoints[2].to_instance(),
                self.timepoints[3].to_instance(),
            ],
        }
    }
}
impl EmissiveStaticI {
    pub fn with_uniform_timepoints(general: GeneralI, timepoint: TimepointI) -> Self {
        Self {
            general,
            timepoints: [
                timepoint.clone(),
                timepoint.clone(),
                timepoint.clone(),
                timepoint,
            ],
        }
    }
}
impl Clone for EmissiveStaticI {
    fn clone(&self) -> Self {
        EmissiveStaticI {
            general: self.general.clone(),
            timepoints: self.timepoints.clone(),
        }
    }
}

#[derive(Debug)]
pub enum EmissiveMaterial<T: TypePack> {
    Plain(T::TexturePointer3, T::TexturePointer1),
    Video(T::VideoPointer, u8),
    /// [(color, intensity, duration)] where duration is given in game frame.
    AnimatedUniform(Vec<(IVec3, u8, u32)>),
}
pub type EmissiveMaterialS = EmissiveMaterial<TypePackSource>;
pub type EmissiveMaterialI = EmissiveMaterial<TypePackInstance>;
impl EmissiveMaterialS {
    pub fn to_instance(&self) -> EmissiveMaterialI {
        match self {
            EmissiveMaterial::Plain(e, em) => {
                EmissiveMaterial::Plain(e.to_instance(), em.to_instance())
            }
            EmissiveMaterial::Video(_v, _em) => {
                unimplemented!()
            }
            EmissiveMaterial::AnimatedUniform(x) => EmissiveMaterial::AnimatedUniform(x.clone()),
        }
    }
}
impl Clone for EmissiveMaterialI {
    fn clone(&self) -> Self {
        match self {
            EmissiveMaterial::Plain(e, em) => EmissiveMaterial::Plain(e.clone(), em.clone()),
            EmissiveMaterial::Video(v, em) => EmissiveMaterial::Video(v.clone(), *em),
            EmissiveMaterial::AnimatedUniform(x) => EmissiveMaterial::AnimatedUniform(x.clone()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CurrentEmission {
    Material(usize),
    Arbitrary(IVec4),
}
#[derive(Clone, Debug)]
/// This material is special; it is same for both source and instance!
pub struct EmissiveDynamic {
    pub current: CurrentEmission,
    pub time: u32,
}

#[derive(Clone, Copy)]
pub enum SurfaceKind {
    General,
    Transparent,
    EmissiveStatic,
    EmissiveDynamic,
}
