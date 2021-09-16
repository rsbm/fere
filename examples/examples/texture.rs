use crate::fere_resources::texture::TextureData;
use fere::prelude::*;
use fere_window::*;
use parking_lot::RwLock;
use rops::*;
use std::sync::Arc;
use std::time::Instant;

struct SceneState {
    t: f64,
}

impl SceneState {
    fn new() -> Self {
        Self { t: 0.0 }
    }

    fn update(&mut self) {
        self.t += 0.01;
    }
}

struct Scene {
    renderer: Fere,
    state: Arc<RwLock<SceneState>>,
    resources: Arc<Resources>,
    last_instant: Instant,
}

struct Resources {
    texture: Arc<Texture>,
}

impl ProgramWithImgui for Scene {
    fn new() -> Self {
        let fere_config = FereConfigs {
            resolution: IVec2::new(1200, 1200),
            shadow_resolution: 512,
            probe_resolution: 256,
            max_major_lights: 16,
            video_record: true,
            irradiance_volume: None,
            max_chamber_num: 1,
            pv_scale: 100.0,
        };
        let mut renderer = Fere::new(fere_config);
        renderer
            .add_chamber(ChamberConfig {
                bpos: Vec3::zeros(),
                size: Vec3::new(500.0, 500.0, 500.0),
            })
            .unwrap();

        let size = IVec2::new(1200, 1200);
        let tex_data = TextureData {
            name: "".to_owned(),
            data: vec![255; (3 * size.x * size.y) as usize],
            size,
            channel: 3,
        };
        let mut tex = Texture::new(None, tex_data);
        tex.buffer();

        let state = Arc::new(RwLock::new(SceneState::new()));
        Scene {
            renderer,
            resources: Arc::new(Resources {
                texture: Arc::new(tex),
            }),
            state,
            last_instant: Instant::now(),
        }
    }

    fn update(&mut self, _imgui_ctx: ImgUiContext) -> String {
        let cpos = Vec3::new(0.0, 0.0, 1200.0);
        let screen_size = self.renderer.configs().resolution;
        let mut camera = SetCamera::new(
            cpos,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            (70.0_f32).to_radians(),
            screen_size.x as f32 / screen_size.y as f32,
            0.1,
            1000.0,
        );
        camera.trans();

        let (frame, renderer) = self.renderer.new_frame(FrameConfig {
            camera,
            show_lightvolume_outline: false,
        });

        let t = self.state.read().t;
        self.state.write().update();
        let state = Arc::clone(&self.state);
        let resources = Arc::clone(&self.resources);
        let mut buffer =
            vec![0; (resources.texture.size.x * resources.texture.size.y * 3) as usize];
        let mut count = 0;
        for i in 0..resources.texture.size.x {
            for j in 0..resources.texture.size.y {
                let x = i as f64 / resources.texture.size.x as f64;
                let y = j as f64 / resources.texture.size.y as f64;

                buffer[count] = ((x + t).sin() * 127.0 + 127.0) as u8;
                buffer[count + 1] = ((y + t * 0.9).sin() * 127.0 + 127.0) as u8;
                buffer[count + 2] = ((x * y * 0.1 + t * 0.8).sin() * 127.0 + 127.0) as u8;
                count += 3;
            }
        }
        resources.texture.write_buffer(&buffer);

        let render_thread =
            std::thread::spawn(move || render(frame, &*state.read(), resources.as_ref()));
        self.renderer.end_frame(renderer.render());
        render_thread.join().unwrap();

        self.last_instant = Instant::now();

        "continue".to_owned()
    }
}

fn render(mut frame: Frame, _state: &SceneState, resources: &Resources) {
    // Add lights
    frame.push(DrawImage {
        texture: Arc::clone(&resources.texture),
        pos: Vec2::new(600.0, 600.0),
        size: Vec2::new(1.0, 1.0),
        rotation: 0.0,
        blend_mode: (),
        color: Vec4::new(1.0, 1.0, 1.0, 1.0),
    });
    frame.end();
}

fn main() {
    run_with_imgui::<Scene>(WindowConfig {
        screen_size: IVec2::new(1200, 1200),
        initial_window_pos: IVec2::new(0, 0),
        monitor_index: 2,
        title: "123123".into(),
    });
}
