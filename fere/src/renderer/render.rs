use super::*;
use chamber::*;

impl RenderContext {
    pub fn process_op(&mut self, op: RenderOp) -> Result<Option<RenderOp>, OpError> {
        match op {
            RenderOp::Multiple(_) => {
                panic!("`RenderOp::Multiple` on `process_op()`")
            }
            RenderOp::StartFrame(_) => {
                self.graphics.bind_deferred_pass1();
                Ok(None)
            }
            RenderOp::SetCamera(camera_info_) => {
                let programs = vec![
                    self.graphics.prgs.basic.as_ref(),
                    self.graphics.prgs.standard.as_ref(),
                    self.graphics.prgs.sh_visualize.as_ref(),
                    self.graphics.prgs.sh_visualize_single.as_ref(),
                    self.graphics.prgs.geo_visualize.as_ref(),
                    self.graphics.prgs.dr_2.as_ref(),
                    self.graphics.prgs.dr_2_irradiance.as_ref(),
                    self.graphics.prgs.dr_2_ambient.as_ref(),
                    self.graphics.prgs.dr_2_omni.as_ref(),
                ];
                for program in programs {
                    program.bind();
                    program.uniform_transformations(
                        camera_info_.projection_get(),
                        camera_info_.view_get(),
                    );
                }
                self.camera_info = Some(camera_info_);
                Ok(None)
            }
            RenderOp::DrawLine(DrawLine {
                pos1,
                pos2,
                color,
                width,
            }) => {
                let prg = self.graphics.prgs.standard.as_ref();
                prg.bind();
                let runit = RenderUnit {
                    color: true,
                    depth: true,
                    depth_test: true,
                    id: None,
                    lighting: Some(Lighting::DefFixed),
                };
                self.graphics.ru_set(prg, &runit);
                let arr = &self.graphics.meshes().line;
                arr.bind();
                bind_fixed_color(&prg, &color);
                prg.uniform_line(&pos1, &pos2, width);
                arr.draw_line();
                Ok(None)
            }
            RenderOp::DrawGeneral(DrawGeneral { object, surface }) => {
                self.get_chamber_ctx(object.chamber_index)?;

                let prg = self.graphics.prgs.standard.as_ref();
                prg.bind();
                let runit = RenderUnit {
                    color: true,
                    depth: true,
                    depth_test: true,
                    id: None,
                    lighting: Some(Lighting::DefFull),
                };
                self.graphics.ru_set(prg, &runit);

                prg.uniform_model(&object.trans, false);
                object.mesh.bind();
                bind_general(&prg, &surface);
                object.mesh.draw();

                if object.shadow {
                    self.get_mut_chamber_ctx(object.chamber_index)?
                        .shadow_objects
                        .push(ChamberShadowObject {
                            mesh: object.mesh,
                            trans: object.trans,
                        })
                }
                Ok(None)
            }
            RenderOp::DrawEmissiveStatic(DrawEmissiveStatic { object, surface }) => {
                self.get_chamber_ctx(object.chamber_index)?;

                let prg = self.graphics.prgs.standard.as_ref();
                prg.bind();
                let runit = RenderUnit {
                    color: true,
                    depth: true,
                    depth_test: true,
                    id: None,
                    lighting: Some(Lighting::DefFull),
                };
                self.graphics.ru_set(prg, &runit);

                prg.uniform_model(&object.trans, false);
                object.mesh.bind();
                bind_emissive_static(&prg, &surface, 0.0);
                object.mesh.draw();

                if object.shadow {
                    self.get_mut_chamber_ctx(object.chamber_index)?
                        .shadow_objects
                        .push(ChamberShadowObject {
                            mesh: Arc::clone(&object.mesh),
                            trans: object.trans,
                        })
                }
                self.get_mut_chamber_ctx(object.chamber_index)?
                    .emissive_static_objects
                    .push(ChamberEmissiveStaticObject {
                        mesh: object.mesh,
                        trans: object.trans,
                        surface: surface,
                    });
                Ok(None)
            }
            RenderOp::DrawWireFrame(DrawWireFrame {
                mesh,
                trans,
                color,
                width,
            }) => {
                let prg = self.graphics.prgs.standard.as_ref();
                prg.bind();
                let runit = RenderUnit {
                    color: true,
                    depth: true,
                    depth_test: true,
                    id: None,
                    lighting: Some(Lighting::DefFixed),
                };
                self.graphics.ru_set(prg, &runit);
                mesh.bind();
                prg.uniform_wireframe(&trans, &color, width);
                mesh.draw_wireframe();
                Ok(None)
            }
            RenderOp::AddMajorLight(x) => {
                self.get_chamber_ctx(x.chamber_index)?;

                let result =
                    if self.params.debug_lightvolume_outline {
                        let (light, _) = common::prepare_major_light(&x);
                        Some(RenderOp::DrawWireFrame(DrawWireFrame {
						mesh: Arc::clone(&self.graphics.meshes().pyramid),
						trans: crate::graphics::graphics::Graphics::get_transform_for_lightvolume_dir(&light),
						color: IVec4::new(0, 255, 255 ,255),
						width: 1.0,
					}))
                    } else {
                        None
                    };
                self.get_mut_chamber_ctx(x.chamber_index)?
                    .major_lights
                    .push(x);
                Ok(result)
            }
            RenderOp::AddAmbientLight(x) => {
                self.get_mut_chamber_ctx(x.chamber_index)?
                    .ambient_lights
                    .push(x);
                Ok(None)
            }
            RenderOp::AddPointLight(x) => {
                self.get_mut_chamber_ctx(x.chamber_index)?
                    .point_lights
                    .push(x);
                Ok(None)
            }
            RenderOp::ShadeWithIv(x) => {
                let chamber_index = x.chamber_index;
                if self
                    .get_mut_chamber_ctx(x.chamber_index)?
                    .shade_with_iv
                    .replace(x)
                    .is_some()
                {
                    Err(OpError::InvalidShade(format!(
                        "ShadeWithIv on chamber #{}",
                        chamber_index
                    )))
                } else {
                    Ok(None)
                }
            }
            RenderOp::VisualizeProbes(c) => {
                let chamber = self.get_chamber_ctx(c.chamber_index)?;
                let probes = chamber
                    .chamber
                    .state
                    .probe_volume_suite
                    .probe_volume()
                    .probes();

                let prg = self.graphics.prgs.sh_visualize_single.as_ref();
                prg.bind();
                let mesh = &self.graphics.meshes().sphere;
                mesh.bind();

                for probe in probes {
                    let sh: Vec<Vec3> = probe.sh.iter().map(|x| x.1).collect();
                    prg.uniform_sh(&sh);
                    prg.uniform_model_s(
                        &probe.pos,
                        &Mat4::identity(),
                        &Vec3::new(4.0, 4.0, 4.0),
                        false,
                    );
                    mesh.draw()
                }
                Ok(None)
            }
            RenderOp::DrawImage(x) => {
                self.draw_images.push(x);
                Ok(None)
            }
            RenderOp::DrawBillboard(x) => {
                self.draw_billboarsd.push(x);
                Ok(None)
            }
            RenderOp::ShowInternalTexture(x) => {
                self.show_internal_textures.push(x);
                Ok(None)
            }
            _ => unimplemented!(),
        }
    }
}
