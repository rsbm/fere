use fere_resources::{mesh::obj, Mesh};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;

fn read_mesh(name: &str) -> Arc<Mesh> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("resources/mesh/{}", name));
    let file = File::open(path).unwrap();
    let mesh_data = obj::import_single(name, BufReader::new(file)).unwrap();
    let mut mesh = Mesh::new(None, mesh_data);
    mesh.buffer();
    Arc::new(mesh)
}

pub struct Meshes {
    pub square: Arc<Mesh>,
    pub square_coarse: Arc<Mesh>,
    pub sphere: Arc<Mesh>,
    pub pyramid: Arc<Mesh>,
    pub cube: Arc<Mesh>,
    pub line: Arc<Mesh>,
}

impl Default for Meshes {
    fn default() -> Self {
        Self {
            square: read_mesh("square.obj"),
            square_coarse: read_mesh("square_coarse.obj"),
            sphere: read_mesh("sphere_low.obj"),
            pyramid: read_mesh("pyramid.obj"),
            cube: read_mesh("cube.obj"),
            line: read_mesh("line.obj"),
        }
    }
}
