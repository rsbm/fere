use super::*;
use crate::graphics::graphics::texture_internal::{InternalTexType, TextureInternal3D};
use crate::graphics::graphics::Graphics;
use fere_common::*;

pub struct ProbeVolumeSuite {
    probe_volume: ProbeVolume,

    /// The actual texture to use in shader for shading
    sh_texture_illumination: TextureInternal3D,
    sh_texture_depth: TextureInternal3D,

    buffer_diffuse: [Vec<Vec3>; 6],
    buffer_illumination: [Vec<Vec3>; 6],
    buffer_depth: [Vec<f32>; 6],

    resolution: usize,
}

fn init_buffer<T: Copy>(zero: T, size: usize) -> [Vec<T>; 6] {
    let x = vec![zero; size * size];
    [x.clone(), x.clone(), x.clone(), x.clone(), x.clone(), x]
}

impl ProbeVolumeSuite {
    pub fn new(room_size: Vec3, scale: f32, resolution: usize) -> Self {
        let probe_volume = ProbeVolume::new(room_size, scale, resolution);

        let sh_texture_illumination =
            TextureInternal3D::new(InternalTexType::Float3, probe_volume.texture_size());
        let sh_texture_depth =
            TextureInternal3D::new(InternalTexType::Float1, probe_volume.texture_size());

        let buffer_diffuse = init_buffer(Vec3::new(0.0, 0.0, 0.0), resolution);
        let buffer_illumination = init_buffer(Vec3::new(0.0, 0.0, 0.0), resolution);
        let buffer_depth = init_buffer(0.0, resolution);

        Self {
            probe_volume,
            sh_texture_illumination,
            sh_texture_depth,
            buffer_diffuse,
            buffer_illumination,
            buffer_depth,
            resolution,
        }
    }

    pub fn probe_volume(&self) -> &ProbeVolume {
        &self.probe_volume
    }

    /// Reads output buffer in RAM for calculation
    ///
    /// # Safety
    /// TODO
    pub unsafe fn write_buffer(&mut self, graphics: &Graphics, dir: u8) {
        graphics.probe_read_illumination(self.buffer_illumination[dir as usize].as_mut_ptr());
        graphics.probe_read_depth(self.buffer_depth[dir as usize].as_mut_ptr());
    }

    pub fn get_illumination_texture(&self) -> &TextureInternal3D {
        &self.sh_texture_illumination
    }

    pub fn get_depth_texture(&self) -> &TextureInternal3D {
        &self.sh_texture_depth
    }

    /// After calling `write_buffer()` for 6 times, calculate the sh and update the 3d texture.
    pub fn update_probe(&mut self, probe_index: IVec3) {
        self.probe_volume.update_probe(
            probe_index,
            self.resolution,
            &self.buffer_diffuse,
            &self.buffer_illumination,
            &self.buffer_depth,
        );
        self.probe_volume.update_texture();
        unsafe {
            self.sh_texture_illumination
                .load(self.probe_volume.texture_illumination().as_ptr().cast());
            self.sh_texture_depth
                .load(self.probe_volume.texture_depth().as_ptr().cast());
        }
    }
}
