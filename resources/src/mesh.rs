pub mod obj;

use fere_common::*;
use gl::types::*;

#[derive(Debug, Default)]
pub struct MeshData {
    pub name: String,

    pub pos: Vec<Vec3>,
    pub normal: Vec<Vec3>,
    pub uv: Vec<Vec2>,
    pub tan: Vec<Vec3>,

    pub minmax: Option<(Vec3, Vec3)>,
}

impl MeshData {
    pub fn merge(meshes: Vec<MeshData>) -> Self {
        let init = MeshData {
            name: "".to_owned(),
            pos: Vec::new(),
            normal: Vec::new(),
            uv: Vec::new(),
            tan: Vec::new(),
            minmax: None,
        };
        meshes.into_iter().fold(init, |mut acc, mut x| {
            acc.name += &x.name;
            acc.pos.append(&mut x.pos);
            acc.normal.append(&mut x.normal);
            acc.uv.append(&mut x.uv);
            acc.tan.append(&mut x.tan);
            acc
        })
    }

    pub fn create_description(&self) -> MeshDescription {
        let mean_pos = self.pos.iter().sum::<Vec3>() / self.pos.len() as f32;
        MeshDescription { mean_pos }
    }
}

#[derive(Debug)]
pub struct MeshDescription {
    pub mean_pos: Vec3,
}
