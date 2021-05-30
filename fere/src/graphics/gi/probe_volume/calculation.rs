use super::{Probe, ProbeVolume};
use fere_common::{
    geo::{six_sides, SixDir},
    vec::GridAccessor3,
    *,
};

pub fn calculate_sh_cache(fb_size: usize, param: usize) -> [Vec<Vec<f32>>; 6] {
    let mut result: [Vec<Vec<f32>>; 6] = Default::default();
    for i in 0..6 {
        result[i].resize(param, Default::default());
        for v in result[i].iter_mut() {
            v.resize(fb_size * fb_size, 0.0);
        }
        let coeff = &mut result[i];
        let mut count = 0;

        for x in 0..fb_size {
            for y in 0..fb_size {
                let q = Vec4::new(
                    x as f32 + 0.5 - fb_size as f32 / 2.0,
                    y as f32 + 0.5 - fb_size as f32 / 2.0,
                    -(fb_size as f32) / 2.0,
                    1.0,
                );
                let p = normalize(&(six_sides(i as SixDir) * q).xyz());
                let mut table = Mat3::from_element(0.0);
                for m in 0..3 {
                    for n in 0..3 {
                        *table.get_mut((m, n)).unwrap() = p[m] * p[n];
                    }
                }

                // l = 0
                coeff[0][count] = 1.0;
                // l = 1
                coeff[1][count] = p[1];
                coeff[2][count] = p[2];
                coeff[3][count] = p[0];
                // l = 2
                coeff[4][count] = table[(0, 1)];
                coeff[5][count] = table[(1, 2)];
                coeff[6][count] = 3.0 * table[(2, 2)] - 1.0;
                coeff[7][count] = table[(0, 2)];
                coeff[8][count] = table[(0, 0)] - table[(1, 1)];
                // l = 4
                coeff[9][count] = table[(0, 1)] * (table[(0, 0)] - table[(1, 1)]);
                coeff[10][count] = table[(1, 2)] * (3.0 * table[(0, 0)] - table[(1, 1)]);
                coeff[11][count] = table[(0, 1)] * (7.0 * table[(2, 2)] - 1.0);
                coeff[12][count] = table[(1, 2)] * (7.0 * table[(2, 2)] - 3.0);
                coeff[13][count] = table[(2, 2)] * (35.0 * table[(2, 2)] - 30.0) + 3.0;
                coeff[14][count] = table[(0, 2)] * (7.0 * table[(2, 2)] - 3.0);
                coeff[15][count] = (table[(0, 0)] - table[(1, 1)]) * (7.0 * table[(2, 2)] - 1.0);
                coeff[16][count] = table[(0, 2)] * (table[(0, 0)] - 3.0 * table[(1, 1)]);
                coeff[17][count] = table[(0, 0)] * (table[(0, 0)] - 3.0 * table[(1, 1)])
                    - table[(1, 1)] * (3.0 * table[(0, 0)] - table[(1, 1)]);
                /*
                for h in 1..18 {
                    coeff[h][count] = 0.0;
                }*/

                count += 1;
            }
        }
    }
    result
}

trait Accessor {
    fn acc(x: &mut (Vec3, Vec3, f32), dim: usize) -> &mut f32;
}

struct AccDiffuse;
impl Accessor for AccDiffuse {
    fn acc(x: &mut (Vec3, Vec3, f32), dim: usize) -> &mut f32 {
        &mut x.0[dim]
    }
}
struct AccIllum;
impl Accessor for AccIllum {
    fn acc(x: &mut (Vec3, Vec3, f32), dim: usize) -> &mut f32 {
        &mut x.1[dim]
    }
}
struct AccDepth;
impl Accessor for AccDepth {
    fn acc(x: &mut (Vec3, Vec3, f32), _dim: usize) -> &mut f32 {
        &mut x.2
    }
}

/// TODO: replace various subscribtion [] to unchecked get
/// TODO: Parallel execution
#[allow(clippy::approx_constant)]
fn update_sub<T: Accessor>(
    probe: &mut Probe,
    dc: &[f32; 4],
    max_depth: f32,
    param: usize,
    sh_cache: &[Vec<Vec<f32>>; 6],
    size: usize,
    dim: usize,
    buffer: &[*const f32],
) {
    for i in 0..6 {
        let mut count = 0;
        let coeff = &sh_cache[i];

        if dim == 1 {
            for x in 0..size {
                for _ in 0..size {
                    for d in 0..dim {
                        let mut value = unsafe { *buffer[i].add(count + d) };
                        // TODO: calculate this in shader?
                        value = -(dc[3] * value - dc[1]) / (dc[0] - dc[2] * value + 0.001);
                        value *= x as f32 / size as f32;
                        value = max_depth.min(value);
                        value /= max_depth;

                        let sh = &mut probe.sh;
                        for c in 0..param {
                            *T::acc(&mut sh[c], d) += value * coeff[c][count / dim];
                        }
                    }
                    count += dim;
                }
            }
        } else {
            for _ in 0..size {
                for _ in 0..size {
                    for d in 0..dim {
                        let value = unsafe { *buffer[i].add(count + d) };
                        let sh = &mut probe.sh;
                        for c in 0..param {
                            *T::acc(&mut sh[c], d) += value * coeff[c][count / dim];
                        }
                    }
                    count += dim;
                }
            }
        }
    }

    let k = (size * size * 6) as f64 / (2.0 * std::f64::consts::PI.powi(2)) * 20.0;
    let k = k as f32;
    for d in 0..dim {
        let sh = &mut probe.sh;
        let mut c;
        let mut a;

        a = 3.141593;
        c = 0.282095;
        *T::acc(&mut sh[0], d) *= c * a / k;

        a = 2.094395;
        c = 0.488603;
        *T::acc(&mut sh[1], d) *= c * a / k;
        *T::acc(&mut sh[2], d) *= c * a / k;
        *T::acc(&mut sh[3], d) *= c * a / k;

        a = 0.785298;
        c = 1.092548;
        *T::acc(&mut sh[4], d) *= c * a / k;
        *T::acc(&mut sh[5], d) *= c * a / k;
        c = 0.315392;
        *T::acc(&mut sh[6], d) *= c * a / k;
        c = 1.092548;
        *T::acc(&mut sh[7], d) *= c * a / k;
        c = 0.546274;
        *T::acc(&mut sh[8], d) *= c * a / k;

        a = -0.130900;
        c = 2.503342;
        *T::acc(&mut sh[9], d) *= c * a / k;
        c = 1.77013;
        *T::acc(&mut sh[10], d) *= c * a / k;
        c = 0.946174;
        *T::acc(&mut sh[11], d) *= c * a / k;
        c = 0.669046;
        *T::acc(&mut sh[12], d) *= c * a / k;
        c = 0.105785;
        *T::acc(&mut sh[13], d) *= c * a / k;
        c = 0.669046;
        *T::acc(&mut sh[14], d) *= c * a / k;
        c = 0.473087;
        *T::acc(&mut sh[15], d) *= c * a / k;
        c = 1.77013;
        *T::acc(&mut sh[16], d) *= c * a / k;
        c = 0.625835;
        *T::acc(&mut sh[17], d) *= c * a / k;
    }
}

pub fn update_probe(
    volume: &mut ProbeVolume,
    index: IVec3,
    size: usize,
    cube_diffuse: &[Vec<Vec3>; 6],
    cube_illumination: &[Vec<Vec3>; 6],
    cube_depth: &[Vec<f32>; 6],
) {
    let ga = GridAccessor3(volume.number);
    let probe = &mut volume.probes[ga.get(&index)];

    for p in 0..volume.param {
        probe.sh[p] = (Vec3::from_element(0.0), Vec3::from_element(0.0), 0.0);
    }

    // TODO: sync this
    let iproj = Mat4::new_perspective(90.0_f32.to_radians(), 1.0, 0.5, 200.0);
    //for simplicity, we assume x = 0 and y = 0 for the original coord (afterh view trans)
    let dc = [iproj[(2, 2)], iproj[(3, 2)], iproj[(2, 3)], iproj[(3, 3)]];

    let max_depth = 100.0;

    let mut cube_diffuse_unwrap: Vec<*const f32> = Vec::new();
    let mut cube_illumination_unwrap: Vec<*const f32> = Vec::new();
    let mut cube_depth_unwrap: Vec<*const f32> = Vec::new();

    for i in 0..6 {
        cube_diffuse_unwrap.push(cube_diffuse[i][0].as_ptr());
        cube_illumination_unwrap.push(cube_illumination[i][0].as_ptr());
        cube_depth_unwrap.push(cube_depth[i].as_ptr());
    }

    update_sub::<AccDiffuse>(
        probe,
        &dc,
        max_depth,
        volume.param,
        &volume.sh_cache,
        size,
        3,
        &cube_diffuse_unwrap,
    );
    update_sub::<AccIllum>(
        probe,
        &dc,
        max_depth,
        volume.param,
        &volume.sh_cache,
        size,
        3,
        &cube_illumination_unwrap,
    );
    update_sub::<AccDepth>(
        probe,
        &dc,
        max_depth,
        volume.param,
        &volume.sh_cache,
        size,
        1,
        &cube_depth_unwrap,
    );

    let _q: Vec<f32> = probe.sh.iter().map(|x| x.0.x).collect();
    //println!("{:?}", q);
}
