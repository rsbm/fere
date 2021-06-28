use fere_common::*;

pub struct Light {
    pub pos: Vec4,
    pub color: Vec3,
    pub shadow: bool,
}

pub struct LightUni {
    pub light: Light,
    pub radius: f32,
}

pub struct LightDir {
    pub light: Light,
    pub radius: f32,
    pub angle: f32,
    pub xdir: Vec3,
    pub ydir: Vec3,

    pub round: bool,
    pub smoothnes: f32,
    pub trans: Mat4,
}

/// Room parameter
pub struct ProbeVolumeRoom {
    /// World -> Chamber transformation
    pub trans: Mat4,

    /// Chamber coordinate of probe (1,1,1) where (0,0,0) is the padded one
    pub offset: Vec3,

    pub cell_size: Vec3,

    /// Including padding
    pub nums: IVec3,

    pub room_size: Vec3,

    /// cell_size * nums
    pub padded_room_size: Vec3,

    /// sh order
    pub params: usize,

    /// [0, 1]
    pub weight: f32,
}
