use fere_resources::{mesh::obj, Mesh};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

fn read_mesh(name: &str) -> Mesh {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("resources/mesh/{}", name));
    let file = File::open(path).unwrap();
    let mesh_data = obj::import_single(name, BufReader::new(file)).unwrap();
    Mesh::new(None, mesh_data)
}

pub struct Meshes {
    pub square: Mesh,
    pub square_coarse: Mesh,
    pub sphere: Mesh,
    pub pyramid: Mesh,
    pub cube: Mesh,
}

impl Default for Meshes {
    fn default() -> Self {
        let mut x = Self {
            square: read_mesh("square.obj"),
            square_coarse: read_mesh("square_coarse.obj"),
            sphere: read_mesh("sphere_low.obj"),
            pyramid: read_mesh("pyramid.obj"),
            cube: read_mesh("cube.obj"),
        };
        x.square.buffer();
        x.square_coarse.buffer();
        x.sphere.buffer();
        x.pyramid.buffer();
        x.cube.buffer();
        x
    }
}
