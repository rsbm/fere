use fere::prelude::*;
use fere_window::*;
use parking_lot::RwLock;
use rand::prelude::*;
use rops::*;
use std::sync::Arc;
use std::time::Instant;
use surface::{no_normal_map, GeneralI, TexVar};

struct SceneState {
    /// radius, angle_speed, num, angle
    rings: Vec<(f32, f32, usize, f32)>,
    frame: u64,
}

impl SceneState {
    fn new() -> Self {
        let mut rings = Vec::new();
        let mut rng = thread_rng();
        for i in 0..100 {
            rings.push((
                10.0 + i as f32 * 3.0,
                rng.gen_range(-0.05..0.05),
                40 + i * 2,
                0.0,
            ));
        }
        Self { rings, frame: 0 }
    }

    fn update(&mut self) {
        for (_, angle_speed, _, angle) in self.rings.iter_mut() {
            *angle += *angle_speed
        }
        self.frame += 1;
    }
}

struct Scene {
    renderer: Fere,
    frame: usize,
    state: Arc<RwLock<SceneState>>,
    resources: Arc<Resources>,
    last_instant: Instant,
}

struct Resources {
    cube: Arc<Mesh>,
}

impl ProgramWithImgui for Scene {
    fn new() -> Self {
        let fere_config = FereConfigs {
            resolution: IVec2::new(2500, 1600),
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

        let state = Arc::new(RwLock::new(SceneState::new()));
        Scene {
            renderer,
            frame: 0,
            resources: Arc::new(Resources {
                cube: fere_examples::read_mesh("cube.obj"),
            }),
            state,
            last_instant: Instant::now(),
        }
    }

    fn update(&mut self, _imgui_ctx: ImgUiContext) -> String {
        let cpos = Vec3::new(0.0, 0.0, 600.0);
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

        self.state.write().update();
        let state = Arc::clone(&self.state);
        let resources = Arc::clone(&self.resources);

        let render_thread =
            std::thread::spawn(move || render(frame, &*state.read(), resources.as_ref()));
        self.renderer.end_frame(renderer.render());
        render_thread.join().unwrap();

        self.frame += 1;
        self.last_instant = Instant::now();

        "continue".to_owned()
    }
}

fn render(mut frame: Frame, state: &SceneState, resources: &Resources) {
    // Add lights
    frame.push(AddAmbientLight {
        color: Vec3::new(0.55, 0.55, 0.55),
        omni: true,
        chamber_index: 0,
    });

    let color = IVec4::new(255, 255, 255, 255);
    let xcolor = IVec4::new(255, 255, 0, 255);
    let ycolor = IVec4::new(0, 255, 255, 255);

    let count = 5;
    let interval = 10.0;
    let width = 1.0;
    let z_offset = 0.1;
    frame.push(fere_examples::draw_grid(
        color, xcolor, ycolor, count, interval, width, z_offset,
    ));

    for (radius, _, num, angle) in state.rings.iter() {
        for i in 0..*num {
            let surface = GeneralI {
                basecolor: TexVar::U(IVec3::new(255, 255, 255)),
                roughness: TexVar::U(50),
                metalness: TexVar::U(0),
                normal: no_normal_map(),
            };
            let a = 2.0 * std::f32::consts::PI / (*num as f32) * (i as f32) + angle;
            let ori = Ori::new(
                Vec3::new(a.cos() * radius, a.sin() * radius, 0.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(a.cos(), a.sin(), 0.0),
                Vec3::new(-a.sin(), a.cos(), 0.0),
            );

            frame.push(DrawGeneral {
                object: Object {
                    mesh: Arc::clone(&resources.cube),
                    shadow: true,
                    irradiance_volume: false,
                    trans: *ori.trans(),
                    chamber_index: 0,
                },
                surface,
            });
        }
    }

    frame.end();
}

fn main() {
    run_with_imgui::<Scene>(WindowConfig {
        screen_size: IVec2::new(2500, 1600),
        initial_window_pos: IVec2::new(0, 0),
        monitor_index: 2,
        title: "123123".into(),
    });
}
