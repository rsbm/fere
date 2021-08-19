use super::*;

impl RenderContext {
    fn render_irradiance_world(&self, chamber_index: ChamberIndex) {
        let chamber = self.chamber_contexts[chamber_index as usize]
            .as_ref()
            .unwrap();
        let prg = self.graphics.prgs.standard_probe.bind();
        let runit = RenderUnit {
            color: true,
            depth: true,
            depth_test: true,
            id: None,
            lighting: None,
        };
        self.graphics.ru_set(prg, &runit);
        for object in &chamber.emissive_static_objects {
            bind_emissive_static(prg, &object.surface, 0.0);
            prg.uniform_model(&object.trans, false);
            object.mesh.bind();
            object.mesh.draw();
        }
    }

    pub fn update_probe(&mut self, chamber_index: ChamberIndex) {
        self.graphics.bind_probe();

        let chamber = self.chamber_contexts[chamber_index as usize]
            .as_ref()
            .unwrap();
        let probe = chamber.chamber.state.current_probe;
        let mut camera = chamber
            .chamber
            .state
            .probe_volume_suite
            .probe_volume()
            .camera(probe.0, probe.1);
        camera.trans();

        let prg = self.graphics.prgs.standard_probe.bind();
        prg.uniform_transformations(camera.projection_get(), camera.view_get());
        self.render_irradiance_world(chamber_index);

        let chamber = self.chamber_contexts[chamber_index as usize]
            .as_mut()
            .unwrap();
        unsafe {
            chamber
                .chamber
                .state
                .probe_volume_suite
                .write_buffer(&self.graphics, probe.1 as u8);
        }
        if probe.1 == 5 {
            chamber
                .chamber
                .state
                .probe_volume_suite
                .update_probe(probe.0);
        }
        chamber.chamber.state.set_next_probe();
    }
}
