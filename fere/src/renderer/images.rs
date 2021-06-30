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
            let pos = image.pos.component_div(&screen_size) - Vec2::new(1.0, 1.0);
            let size = image
                .size
                .component_mul(&image_size)
                .component_div(&screen_size)
                * 2.0;
            let trans = glm::translate(&Mat4::identity(), &Vec3::new(pos.x, pos.y, 0.0));
            let trans = glm::scale(&trans, &Vec3::new(size.x, -size.y, 1.0));
            let trans = glm::rotate(&trans, image.rotation, &Vec3::new(0.0, 0.0, -1.0));
            prg.uniform_color_f(&image.color);
            prg.uniform_model(&trans, false);
            arr.draw();
        }

        let prg = self.graphics.prgs.debug_3vec.as_ref();
        prg.bind();
        let arr = &self.graphics.meshes().square;
        arr.bind();
        for tex in &self.show_internal_textures {
            let (tex_size, prg) = match tex.name.as_str() {
                "normal" => {
                    let (tex_raw, tex_size) = self.graphics.get_gbuffer_normal();
                    prg.uniform_texture(0, tex_raw);
                    (tex_size, prg)
                }
                "iv_illuminatiion" => {
                    let (tex_raw, tex_size) = self.graphics.get_irradiance_volume_tex();
                    prg.uniform_texture(0, tex_raw);
                    (tex_size, prg)
                }
                other => {
                    self.logs.push(FrameLog::new(format!(
                        "Invalid internal texture name: {}",
                        other
                    )));
                    continue;
                }
            };
            let image_size: Vec2 = nalgebra::convert(tex_size);
            let pos = tex.pos.component_div(&screen_size) - Vec2::new(1.0, 1.0);
            let size = tex
                .size
                .component_mul(&image_size)
                .component_div(&screen_size)
                * 2.0;
            let trans = glm::translate(&Mat4::identity(), &Vec3::new(pos.x, pos.y, 0.0));
            let trans = glm::scale(&trans, &Vec3::new(size.x, size.y, 1.0));
            let trans = glm::translate(&trans, &Vec3::new(0.5, 0.5, 0.0));
            prg.uniform_model(&trans, false);
            arr.draw();
        }

        let prg = self.graphics.prgs.image.as_ref();
        prg.bind();
        // FIXME: depth and depth_test are parameters of the rop
        let runit = RenderUnit {
            color: true,
            depth: false,
            depth_test: false,
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
            let trans = glm::scale(&trans, &Vec3::new(size.x, -size.y, 1.0));
            let trans = glm::rotate(&trans, billboard.rotation, &Vec3::new(0.0, 0.0, -1.0));
            prg.uniform_color_f(&billboard.color);
            prg.uniform_model(&trans, false);
            arr.draw();
        }
    }
}
