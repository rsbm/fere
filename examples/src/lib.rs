use fere::prelude::{
    fere_resources::{Mesh, Texture},
    *,
};
use rand::prelude::*;
use rops::*;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;

pub fn draw_grid(
    color: IVec4,
    xcolor: IVec4,
    ycolor: IVec4,
    count: usize,
    interval: f32,
    width: f32,
    z_offset: f32,
) -> RenderOp {
    let count = count as i32;
    let z = z_offset;
    let max = interval * count as f32;
    let mut ops_list = Vec::new();
    for c in -count..=count {
        let i = interval * c as f32;
        if c == 0 {
            ops_list.push(rops::DrawLine {
                pos1: Vec3::new(0.0, 0.0, z),
                pos2: Vec3::new(max, 0.0, z),
                color: xcolor,
                width,
            });
            ops_list.push(rops::DrawLine {
                pos1: Vec3::new(0.0, 0.0, z),
                pos2: Vec3::new(0.0, max, z),
                color: ycolor,
                width,
            });
            ops_list.push(rops::DrawLine {
                pos1: Vec3::new(0.0, -max, z),
                pos2: Vec3::new(0.0, 0.0, z),
                color,
                width,
            });
            ops_list.push(rops::DrawLine {
                pos1: Vec3::new(-max, 0.0, z),
                pos2: Vec3::new(0.0, 0.0, z),
                color,
                width,
            });
        } else {
            ops_list.push(rops::DrawLine {
                pos1: Vec3::new(i, -max, z),
                pos2: Vec3::new(i, max, z),
                color,
                width,
            });
            ops_list.push(rops::DrawLine {
                pos1: Vec3::new(-max, i, z),
                pos2: Vec3::new(max, i, z),
                color,
                width,
            });
        }
    }
    RenderOp::Multiple(ops_list.into_iter().map(RenderOp::DrawLine).collect())
}

pub fn gen_color() -> IVec3 {
    let mut color = Vec3::new(0.0, 0.0, 0.0);
    let mut rng = thread_rng();

    for i in 0..3 {
        color[i] = rng.gen_range(0.01..1.0);
    }
    let max = color
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let color = color * (1.0 / max) * 255.0;
    IVec3::new(color.x as i32, color.y as i32, color.z as i32)
}

pub fn read_mesh(name: &str) -> Arc<Mesh> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("resources/meshes/{}", name));
    let file = File::open(path).unwrap();
    let mesh_data = fere_resources::mesh::obj::import_single(name, BufReader::new(file)).unwrap();
    let mut mesh = Mesh::new(None, mesh_data);
    mesh.buffer();
    Arc::new(mesh)
}

pub fn read_texture(name: &str) -> Arc<Texture> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("resources/textures/{}", name));
    let texture_data = fere_resources::texture::import(name, {
        let file = File::open(path).unwrap();
        BufReader::new(file)
    })
    .unwrap();
    let mut texture = Texture::new(None, texture_data);
    texture.buffer();
    Arc::new(texture)
}

pub fn calc_ori_for_cuboid(bpos: Vec3, size: Vec3, rotate: f32) -> Ori {
    let x_angle = rotate.to_radians();
    let y_angle = (rotate + 90.0).to_radians();
    Ori::new(
        Vec3::new(bpos.x, bpos.y, bpos.z + size.z / 2.0),
        size,
        Vec3::new(x_angle.cos(), x_angle.sin(), 0.0),
        Vec3::new(y_angle.cos(), y_angle.sin(), 0.0),
    )
}
