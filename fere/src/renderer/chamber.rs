use super::*;
use fere_resources::surface;
use fere_resources::Mesh;

#[derive(Debug)]
pub(crate) struct ChamberShadowObject {
    pub mesh: Arc<Mesh>,
    pub trans: Mat4,
}

pub(crate) struct ChamberEmissiveStaticObject {
    pub mesh: Arc<Mesh>,
    pub trans: Mat4,
    pub surface: surface::EmissiveStaticI,
}

pub(crate) struct ChamberContext {
    pub chamber: Chamber,

    pub major_lights: Vec<AddMajorLight>,
    pub ambient_lights: Vec<AddAmbientLight>,
    pub point_lights: Vec<AddPointLight>,

    pub shadow_objects: Vec<ChamberShadowObject>,
    pub emissive_static_objects: Vec<ChamberEmissiveStaticObject>,

    pub billboards: Vec<DrawBillboard>,

    pub shade_with_iv: Option<ShadeWithIv>,
}

impl ChamberContext {
    pub fn new(chamber: Chamber) -> Self {
        Self {
            chamber,
            major_lights: Default::default(),
            ambient_lights: Default::default(),
            point_lights: Default::default(),
            shadow_objects: Default::default(),
            emissive_static_objects: Default::default(),
            billboards: Default::default(),
            shade_with_iv: Default::default(),
        }
    }
}
