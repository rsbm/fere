use super::*;

impl RenderContext {
    fn render_shadow_world(&self, chamber_index: ChamberIndex) {
        let chamber = self.chamber_contexts[chamber_index as usize]
            .as_ref()
            .unwrap();
        let prg = self.graphics.prgs.shadow_1.bind();
        let runit = RenderUnit {
            color: false,
            depth: true,
            depth_test: true,
            id: None,
            lighting: None,
        };
        self.graphics.ru_set(prg, &runit);
        for object in &chamber.shadow_objects {
            prg.uniform_model(&object.trans, false);
            object.mesh.bind();
            object.mesh.draw();
        }
    }

    pub(super) fn shade(&mut self, chamber_index: ChamberIndex) {
        let chamber = if let Ok(x) = self.get_chamber_ctx(chamber_index) {
            x
        } else {
            return;
        };
        let camera = self
            .camera_info
            .as_ref()
            .expect("You must set camera first");

        // # Direct lighting phase
        self.graphics.bind_deferred_pass2(false);

        // - Omni-lights
        let omni_lights: Vec<&AddAmbientLight> = chamber
            .ambient_lights
            .iter()
            .filter(|light| light.omni)
            .collect();
        let prg = self.graphics.prgs.dr_2_omni.bind();
        self.graphics.bind_gbuffer(prg, 0);
        for omni_light in omni_lights {
            prg.uniform_ambient(&omni_light.color);
            self.graphics.draw_lightvolume_ambient(
                prg,
                &(chamber.chamber.config.bpos - Vec3::new(0.5, 0.5, 0.5)),
                &camera.pos,
                &(chamber.chamber.config.size + Vec3::new(1.0, 1.0, 1.0)),
            );
        }

        // - Raw ambient lights
        let omni_lights: Vec<&AddAmbientLight> = chamber
            .ambient_lights
            .iter()
            .filter(|light| !light.omni)
            .collect();
        let prg = self.graphics.prgs.dr_2_ambient.bind();
        self.graphics.bind_gbuffer(prg, 0);
        for omni_light in omni_lights {
            prg.uniform_ambient(&omni_light.color);
            self.graphics.draw_lightvolume_ambient(
                prg,
                &(chamber.chamber.config.bpos - Vec3::new(0.5, 0.5, 0.5)),
                &camera.pos,
                &(chamber.chamber.config.size + Vec3::new(1.0, 1.0, 1.0)),
            );
        }

        // - Major lights
        for major_light in &chamber.major_lights {
            // Step1: Draw the world again
            let (light, camera) = common::prepare_major_light(major_light);

            self.graphics.bind_shadow(0);
            let prg = self.graphics.prgs.shadow_1.bind();
            prg.uniform_transformations(camera.projection_get(), camera.view_get());
            self.render_shadow_world(chamber_index);

            self.graphics.bind_deferred_pass2(false);
            let prg = self.graphics.prgs.dr_2.bind();
            self.graphics.bind_shadow_map(prg, 0);
            self.graphics.draw_lightvolume_dir(prg, &light, camera.pos);
        }

        // Point lights
        let prg = self.graphics.prgs.dr_2.bind();
        for point_light in &chamber.point_lights {
            let light = LightUni {
                light: Light {
                    pos: Vec4::new(point_light.pos.x, point_light.pos.y, point_light.pos.z, 1.0),
                    color: point_light.color,
                    shadow: true,
                },
                radius: 200.0,
            };
            self.graphics
                .draw_lightvolume_uni(prg, &light, camera.pos, false);
        }

        // Irradiance volumes
        if let (Some(ShadeWithIv { weight, .. }), true) = (
            chamber.shade_with_iv.as_ref(),
            self.params.enable_irradiance_volume,
        ) {
            let prg = self.graphics.prgs.dr_2_irradiance.bind();
            self.graphics.bind_gbuffer(prg, 0);
            self.graphics.bind_probe_volume(
                prg,
                8,
                chamber
                    .chamber
                    .state
                    .probe_volume_suite
                    .get_illumination_texture(),
                chamber.chamber.state.probe_volume_suite.get_depth_texture(),
            );

            let pv = &chamber.chamber.state.probe_volume_suite.probe_volume();
            let nums_float: Vec3 = nalgebra::convert(pv.number());
            let volume_room = crate::graphics::glmanager::light::ProbeVolumeRoom {
                trans: Mat4::identity(),
                offset: pv.offset(),
                cell_size: pv.cell_size(),
                nums: pv.number(),
                room_size: chamber.chamber.config.size,
                padded_room_size: pv.cell_size().component_mul(&nums_float),
                params: pv.params(),
                weight: *weight,
            };
            prg.uniform_probe_volume(&volume_room);
            self.graphics.draw_lightvolume_ambient(
                prg,
                &(chamber.chamber.config.bpos - Vec3::new(0.5, 0.5, 0.5)),
                &camera.pos,
                &(chamber.chamber.config.size + Vec3::new(1.0, 1.0, 1.0)),
            );
        }
    }
}
