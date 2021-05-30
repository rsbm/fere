use super::*;
use serde::Deserialize;

/// Configurations for a Fere Instance, required only once for the initial creation.
#[derive(Clone, Debug, Deserialize)]
pub struct FereConfigs {
    pub resolution: IVec2,

    pub shadow_resolution: usize,
    pub probe_resolution: usize,
    pub max_major_lights: usize,

    pub irradiance_volume: Option<IrradianceVolumeConfigs>,

    pub max_chamber_num: usize,

    pub pv_scale: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IrradianceVolumeConfigs {}
