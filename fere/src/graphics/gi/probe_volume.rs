#![allow(clippy::needless_range_loop)]

mod calculation;
mod suite;

use crate::SetCamera;
use fere_common::*;
use fere_common::{
    geo::{six_sides_dir, SixDir},
    vec::{GridAccessor3, IteratorVec2, IteratorVec3},
};
use rand::{rngs::StdRng, Rng};

pub use suite::ProbeVolumeSuite;

pub struct Probe {
    pub pos: Vec3,
    /// diffuse(rgb), illumination(rgb), depth
    pub sh: Vec<(Vec3, Vec3, f32)>,
}

pub struct ProbeVolume {
    fb_size: usize,
    /// geometry cache
    sh_cache: [Vec<Vec<f32>>; 6],
    /// number of paramters per probe
    param: usize,
    room_size: Vec3,
    min_gap: f32,

    // 'Texture' means calculated final sh coefficients that will be fed to the shader
    /// number of texel
    texture_size: IVec3,
    texture_diffuse: Vec<Vec3>,
    texture_illumination: Vec<Vec3>,
    texture_depth: Vec<f32>,

    probes: Vec<Probe>,

    cell_size: Vec3,
    number: IVec3,
    offset: Vec3,
}

impl ProbeVolume {
    pub fn probes(&self) -> &Vec<Probe> {
        &self.probes
    }

    pub fn fb_size(&self) -> usize {
        self.fb_size
    }

    pub fn room_size(&self) -> Vec3 {
        self.room_size
    }

    pub fn min_gap(&self) -> f32 {
        self.min_gap
    }

    pub fn camera(&self, index: IVec3, side: SixDir) -> SetCamera {
        let dir = six_sides_dir(side);
        let forward_dir = -glm::cross(&dir.0, &dir.1);
        let pos = self.probes[GridAccessor3(self.number).get(&index)].pos;
        SetCamera::new(
            pos,
            pos + forward_dir,
            dir.1,
            90.0_f32.to_radians(),
            1.0,
            0.2,
            200.0,
        )
    }

    pub fn params(&self) -> usize {
        self.param
    }

    pub fn cell_size(&self) -> Vec3 {
        self.cell_size
    }
    pub fn number(&self) -> IVec3 {
        self.number
    }

    /// Coord of probe (1,1,1) - Coord of left-bottom floor
    pub fn offset(&self) -> Vec3 {
        self.offset
    }

    pub fn texture_size(&self) -> IVec3 {
        self.texture_size
    }

    pub fn texture_diffuse(&self) -> &Vec<Vec3> {
        &self.texture_diffuse
    }

    pub fn texture_illumination(&self) -> &Vec<Vec3> {
        &self.texture_illumination
    }

    pub fn texture_depth(&self) -> &Vec<f32> {
        &self.texture_depth
    }

    pub fn new(room_size: Vec3, scale: f32, fb_size: usize) -> Self {
        let mut rng: StdRng = rand::SeedableRng::from_entropy();

        let param = 18;
        let sh_cache = calculation::calculate_sh_cache(fb_size, param);
        let min_gap = 4.0;
        let room_size_shrinked = room_size - Vec3::from_element(min_gap) * 2.0;

        let mut number = IVec3::from_element(0);
        let mut cell_size = Vec3::from_element(0.0);

        for i in 0..3 {
            number[i] = (room_size_shrinked[i] / scale).round() as i32;
            cell_size[i] = room_size_shrinked[i] / number[i] as f32;
            number[i] += 1;
        }

        let ga = GridAccessor3(number);
        let mut probes: Vec<Option<Probe>> = (0..ga.size()).into_iter().map(|_| None).collect();

        for i in IteratorVec3::new(number) {
            let i_f: Vec3 = nalgebra::convert(i);
            let mut sh = vec![(Vec3::from_element(0.0), Vec3::from_element(0.0), 0.0); param];

            let pos = i_f.component_mul(&cell_size) + Vec3::from_element(min_gap)
                - Vec3::new(room_size.x, room_size.y, 0.0) * 0.5;
            for c in 0..param {
                sh[c] = (
                    Vec3::new(
                        rng.gen_range(-1.0..1.0),
                        rng.gen_range(-1.0..1.0),
                        rng.gen_range(-1.0..1.0),
                    ),
                    Vec3::new(
                        rng.gen_range(-1.0..1.0),
                        rng.gen_range(-1.0..1.0),
                        rng.gen_range(-1.0..1.0),
                    ),
                    rng.gen_range(0.0..1.0),
                );
            }
            let probe = Probe { sh, pos };
            probes[ga.get(&i)] = Some(probe);
        }
        let probes: Vec<Probe> = probes.into_iter().map(|x| x.unwrap()).collect();

        let offset = probes[0].pos + Vec3::new(room_size.x, room_size.y, 0.0) * 0.5;

        // This size will be feeded to OpenGL texture actually. So not reversed
        let texture_size = IVec3::new(number.x + 2, number.y + 2, (number.z + 2) * param as i32);

        let tsize = (texture_size.x * texture_size.y * texture_size.z) as usize;
        let texture_diffuse = vec![Vec3::from_element(0.0); tsize];
        let texture_illumination = vec![Vec3::from_element(0.0); tsize];
        let texture_depth = vec![0.0; tsize];

        ProbeVolume {
            fb_size,
            sh_cache,
            param,
            room_size,
            min_gap,
            texture_size,
            texture_diffuse,
            texture_illumination,
            texture_depth,
            probes,
            cell_size,
            number,
            offset,
        }
    }

    pub fn update_probe(
        &mut self,
        index: IVec3,
        size: usize,
        cube_diffuse: &[Vec<Vec3>; 6],
        cube_illumination: &[Vec<Vec3>; 6],
        cube_depth: &[Vec<f32>; 6],
    ) {
        let a1: f32 = cube_diffuse
            .iter()
            .map(|x| -> f32 { x.iter().map(|y| (y.x + y.y + y.z) as f32).sum() })
            .sum();
        let a2: f32 = cube_illumination
            .iter()
            .map(|x| -> f32 { x.iter().map(|y| (y.x + y.y + y.z) as f32).sum() })
            .sum();
        let a3: f32 = cube_depth
            .iter()
            .map(|x| -> f32 { x.iter().map(|y| *y as f32).sum() })
            .sum();

        println!("{}, {}, {}", a1, a2, a3);

        calculation::update_probe(
            self,
            index,
            size,
            cube_diffuse,
            cube_illumination,
            cube_depth,
        )
    }

    fn get_coordinate(&self, loc: &IVec3, index: usize, _dim: usize) -> usize {
        // we consdier the padding
        let number_expand = self.number + IVec3::new(2, 2, 2);
        let ga = GridAccessor3(IVec3::new(
            number_expand.z,
            number_expand.y,
            number_expand.x,
        ));
        ga.get(&(IVec3::new(loc.z, loc.y, loc.x) + IVec3::new(1, 1, 1))) + index * ga.size()
    }

    /// TODO: use unchecked array access
    pub fn update_texture(&mut self) {
        let ga = GridAccessor3(self.number);
        for i in IteratorVec3::new(self.number) {
            for t in 0..self.param {
                let coord = self.get_coordinate(&i, t, 3);
                self.texture_diffuse[coord] = self.probes[ga.get(&i)].sh[t].0;

                self.texture_illumination[coord] = self.probes[ga.get(&i)].sh[t].1;

                let coord = self.get_coordinate(&i, t, 1);
                self.texture_depth[coord] = self.probes[ga.get(&i)].sh[t].2;
            }
        }

        // Fill paddings

        let (nx, ny, nz) = (self.number.x, self.number.y, self.number.z);
        let (nx_, ny_, nz_) = (nx - 1, ny - 1, nz - 1);

        let setter = |pv: &mut Self, p: IVec3, q: IVec3, t: usize| {
            let p = pv.get_coordinate(&p, t, 3);
            let q = pv.get_coordinate(&q, t, 3);
            pv.texture_diffuse[p] = pv.texture_diffuse[q];
            pv.texture_illumination[p] = pv.texture_illumination[q];
            pv.texture_depth[p] = pv.texture_depth[q];
        };

        let planes = [
            (0, 1, -1, 0),
            (1, 2, -1, 0),
            (0, 2, -1, 0),
            (0, 1, nz, nz_),
            (1, 2, nx, nx_),
            (0, 2, ny, ny_),
        ];
        for p in planes.iter() {
            let (d0, d1, padding_, target_) = *p;
            for i in IteratorVec2::new(IVec2::new(
                *self.number.get(d0).unwrap(),
                *self.number.get(d1).unwrap(),
            )) {
                for t in 0..self.param {
                    let mut padding = IVec3::from_element(padding_);
                    *padding.get_mut(d0).unwrap() = i.x;
                    *padding.get_mut(d1).unwrap() = i.y;
                    let mut target = IVec3::from_element(target_);
                    *target.get_mut(d0).unwrap() = i.x;
                    *target.get_mut(d1).unwrap() = i.y;
                    setter(self, padding, target, t);
                }
            }
        }

        let u = -123; // unused
        let edge = [
            (0, IVec3::new(u, -1, -1), IVec3::new(u, 0, 0)),
            (0, IVec3::new(u, ny, -1), IVec3::new(u, ny_, 0)),
            (0, IVec3::new(u, -1, nz), IVec3::new(u, 0, nz_)),
            (0, IVec3::new(u, ny, nz), IVec3::new(u, ny_, nz_)),
            (1, IVec3::new(-1, u, -1), IVec3::new(0, u, 0)),
            (1, IVec3::new(nx, u, -1), IVec3::new(nx_, u, 0)),
            (1, IVec3::new(-1, u, nz), IVec3::new(0, u, nz_)),
            (1, IVec3::new(nx, u, nz), IVec3::new(nx_, u, nz_)),
            (2, IVec3::new(-1, -1, u), IVec3::new(0, 0, u)),
            (2, IVec3::new(-1, ny, u), IVec3::new(0, ny_, u)),
            (2, IVec3::new(nx, -1, u), IVec3::new(nx_, 0, u)),
            (2, IVec3::new(nx, ny, u), IVec3::new(nx_, ny_, u)),
        ];

        for e in edge.iter() {
            let (axis, mut padding, mut target) = *e;
            for i in 0..self.number[axis] {
                for t in 0..self.param {
                    *padding.get_mut(axis).unwrap() = i;
                    *target.get_mut(axis).unwrap() = i;
                    setter(self, padding, target, t);
                }
            }
        }

        for t in 0..self.param {
            setter(self, IVec3::new(-1, -1, -1), IVec3::new(0, 0, 0), t);
            setter(self, IVec3::new(-1, -1, nz), IVec3::new(0, 0, nz_), t);
            setter(self, IVec3::new(-1, ny, -1), IVec3::new(0, ny_, 0), t);
            setter(self, IVec3::new(-1, ny, nz), IVec3::new(0, ny_, nz_), t);
            setter(self, IVec3::new(nx, -1, -1), IVec3::new(nx_, 0, 0), t);
            setter(self, IVec3::new(nx, -1, nz), IVec3::new(nx_, 0, nz_), t);
            setter(self, IVec3::new(nx, ny, -1), IVec3::new(nx_, ny_, 0), t);
            setter(self, IVec3::new(nx, ny, nz), IVec3::new(nx_, ny_, nz_), t);
        }
    }
}
