use super::*;
use fere_common::{vec::normalize_w, *};

impl RenderContext {
    pub(super) fn render_images(&mut self) {
        let prg = self.graphics.prgs.image.as_ref();
        prg.bind();
        let arr = &self.graphics.meshes().square;
        arr.bind();
        let runit = RenderUnit {
            color: true,
            depth: false,
            depth_test: false,
            id: None,
            lighting: None,
        };
        self.graphics.ru_set(prg, &runit);
        let screen_size: Vec2 = nalgebra::convert(self.graphics.screen_size());
        for image in &self.draw_images {
            image.texture.bind_at(0);
            let image_size: Vec2 = nalgebra::convert(image.texture.size);
            let pos = image.pos.component_div(&screen_size) - Vec2::new(0.5, 0.5);
            let size = image
                .size
                .component_mul(&image_size)
                .component_div(&screen_size)
                * 2.0;
            let trans = glm::translate(&Mat4::identity(), &Vec3::new(pos.x, pos.y, 0.0));
            let trans = glm::scale(&trans, &Vec3::new(size.x, size.y, 1.0));
            let trans = glm::rotate(&trans, image.rotation, &Vec3::new(0.0, 0.0, -1.0));
            prg.uniform_model(&trans, false);
            arr.draw();
        }

        for tex in &self.show_internal_textures {
            match tex.name.as_str() {
                "iv_illusion" => {
                },
                other => {
                    self.logs.push(FrameLog::new(format!("Invalid internal texture name: {}", other)));
                } 
            }
        }


        let runit = RenderUnit {
            color: true,
            depth: true,
            depth_test: true,
            id: None,
            lighting: None,
        };
        self.graphics.ru_set(prg, &runit);
        // TODO: filter by boundary
        let screen_size: Vec2 = nalgebra::convert(self.graphics.screen_size());
        let camera = self
            .camera_info
            .as_ref()
            .expect("Yout must set the camera first before using `DrawBillboard`");
        for billboard in &self.draw_billboarsd {
            billboard.texture.bind_at(0);
            let image_size: Vec2 = nalgebra::convert(billboard.texture.size);
            let pos = camera.projection_get()
                * camera.view_get()
                * Vec4::new(billboard.pos.x, billboard.pos.y, billboard.pos.z, 1.0);
            let pos = normalize_w(&pos);
            let size = billboard
                .size
                .component_mul(&image_size)
                .component_div(&screen_size)
                * 2.0;
            let trans = glm::translate(&Mat4::identity(), &Vec3::new(pos.x, pos.y, 0.0));
            let trans = glm::scale(&trans, &Vec3::new(size.x, size.y, 1.0));
            let trans = glm::rotate(&trans, billboard.rotation, &Vec3::new(0.0, 0.0, -1.0));
            prg.uniform_model(&trans, false);
            arr.draw();
        }
    }
}
