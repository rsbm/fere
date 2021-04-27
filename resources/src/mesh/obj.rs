#![allow(clippy::single_char_pattern)]

use super::MeshData;
use fere_common::*;
use std::{io::BufRead, str::Split};

fn parse_vec2(iter: &mut Split<&str>) -> Result<Vec2, ()> {
    let f1 = iter.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let f2 = iter.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    Ok(Vec2::new(f1, f2))
}

fn parse_vec3(iter: &mut Split<&str>) -> Result<Vec3, ()> {
    let f1 = iter.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let f2 = iter.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let f3 = iter.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    Ok(Vec3::new(f1, f2, f3))
}

pub fn import_single<T: BufRead>(base_name: &str, source: T) -> Result<MeshData, String> {
    let x = import(base_name, source)?;
    Ok(MeshData::merge(x))
}

pub fn import<T: BufRead>(base_name: &str, source: T) -> Result<Vec<MeshData>, String> {
    // We create multiple MeshData per OBJ object, and per material
    let mut result = Vec::<MeshData>::new();
    let mut current_object = (
        Vec::<Vec3>::new(),
        Vec::<Vec2>::new(),
        Vec::<Vec3>::new(),
        "".to_owned(),
    );
    let mut current_material = None;

    let mut offset_pos = 0;
    let mut offset_tex = 0;
    let mut offset_normal = 0;

    for (i, line) in source.lines().enumerate() {
        let line = line.map_err(|_| "Failed to read OBJ file".to_owned())?;
        let mut line_iter = line.split(" ");
        let errmsg = |m: &str| format!("Failed to read OBJ file at {}: {} ({})", i, m, line);
        let command = line_iter.next().ok_or_else(|| errmsg("Invalid command"))?;

        match command {
            // Meta
            "#" => continue,
            "mtllib" => continue,
            "o" => {
                if current_material.is_some() {
                    result.push(current_material.take().unwrap());
                }
                let name = line_iter
                    .next()
                    .ok_or_else(|| errmsg("No name after 'o'"))?;
                offset_pos += current_object.0.len();
                offset_tex += current_object.1.len();
                offset_normal += current_object.2.len();

                current_object = (
                    Vec::<Vec3>::new(),
                    Vec::<Vec2>::new(),
                    Vec::<Vec3>::new(),
                    name.to_owned(),
                );
            }
            "s" => continue,
            "usemtl" => {
                if current_material.is_some() {
                    result.push(current_material.take().unwrap());
                }
                let name = line_iter
                    .next()
                    .ok_or_else(|| errmsg("No name after 'usemtl'"))?;
                current_material = Some(MeshData {
                    name: base_name.to_string() + "_" + &current_object.3 + "_" + name,
                    pos: Vec::new(),
                    normal: Vec::new(),
                    uv: Vec::new(),
                    tan: Vec::new(),
                    minmax: None,
                })
            }

            // Data
            "v" => current_object
                .0
                .push(parse_vec3(&mut line_iter).map_err(|_| errmsg("Invalid Vec3"))?),
            "vt" => current_object
                .1
                .push(parse_vec2(&mut line_iter).map_err(|_| errmsg("Invalid Vec2"))?),
            "vn" => current_object
                .2
                .push(parse_vec3(&mut line_iter).map_err(|_| errmsg("Invalid Vec3"))?),
            "f" => {
                let mut vertices = Vec::<(usize, Option<usize>, usize)>::new();
                for vertex in line_iter {
                    let mut vertex = vertex.split('/');
                    let vpos = vertex
                        .next()
                        .ok_or_else(|| errmsg("Invalid vertex in 'f'"))?
                        .parse::<usize>()
                        .map_err(|_| errmsg("Invalid vertex in 'f'"))?;
                    let vuv = {
                        let raw = vertex
                            .next()
                            .ok_or_else(|| errmsg("Invalid vertex in 'f'"))?;
                        if raw.is_empty() {
                            None
                        } else {
                            Some(
                                raw.parse::<usize>()
                                    .map_err(|_| errmsg("Invalid vertex in 'f'"))?,
                            )
                        }
                    };
                    let vnormal = vertex
                        .next()
                        .ok_or_else(|| errmsg("Invalid vertex in 'f'"))?
                        .parse::<usize>()
                        .map_err(|_| errmsg("Invalid vertex in 'f'"))?;

                    vertices.push((vpos, vuv, vnormal));
                }

                if vertices.len() < 3 {
                    return Err(errmsg("Less than 3 vertices in a single 'f'"));
                }

                for i in 0..(vertices.len() - 2) {
                    let mut set1 = |index: usize| -> Result<(), String> {
                        current_material.as_mut().unwrap().pos.push(
                            *current_object
                                .0
                                .get(vertices[index].0 - 1 - offset_pos)
                                .ok_or_else(|| errmsg("Vertex out of index"))?,
                        );
                        if vertices[index].1.is_some() {
                            current_material.as_mut().unwrap().uv.push(
                                *current_object
                                    .1
                                    .get(vertices[index].1.unwrap() - 1 - offset_tex)
                                    .ok_or_else(|| errmsg("UV out of index"))?,
                            );
                        }
                        current_material.as_mut().unwrap().normal.push(
                            *current_object
                                .2
                                .get(vertices[index].2 - 1 - offset_normal)
                                .ok_or_else(|| errmsg("Normal out of index"))?,
                        );
                        Ok(())
                    };

                    set1(0)?;
                    set1(i + 1)?;
                    set1(i + 2)?;
                }
                let material = current_material.as_ref().unwrap();
                let len = material.pos.len();
                assert_eq!(len, material.normal.len());
                if !current_object.1.is_empty() {
                    assert_eq!(len, material.uv.len());
                }
            }
            "g" => continue,
            "l" => continue,
            "" => continue,
            _ => panic!("OBJ file contains unsupported command: [{}]", line),
        }
    }
    // Don't forget the last one!
    result.push(current_material.take().unwrap());

    Ok(result)
}

#[cfg(test)]
mod tests {
    /*
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn read_obj() {
        let f = File::open(tpf_config::translate_path("$N/meshes/mattest.obj"))
            .unwrap();
        let f = BufReader::new(f);
        let _result = match import("test", f) {
            Ok(x) => x,
            Err(_e) => {
                panic!();
            }
        };
    }
    */
}
