use fere::prelude::*;
use fere_resources::surface::{no_normal_map, EmissiveStaticI, GeneralI, TexVar, TimepointI};
use fere_resources::Mesh;
use fere_util::ui::camera_control::CameraControl;
use fere_util::ui::input_manager::InputManager;
use fere_window::*;
use rand::prelude::*;
use rops::*;
use std::sync::Arc;

#[derive(Debug, serde::Deserialize)]
struct SceneParams {
    room_size: Vec3,
    glowing_spheres_n: usize,

    fere_configs: FereConfigs,
}

struct SceneState {
    /// Pos, color, intensity, radius
    spheres: Vec<(Vec3, IVec3, u8, f32)>,
}

impl SceneState {
    fn new(params: &SceneParams) -> Self {
        let mut rng = thread_rng();
        let mut spheres = Vec::new();

        let (min_radius, max_radius) = (3.0, 10.0);
        for _ in 0..params.glowing_spheres_n {
            let mut pos = Vec3::new(0.0, 0.0, 0.0);
            for i in 0..3 {
                pos[i] = rng.gen_range((-params.room_size[i] / 2.0)..(params.room_size[i] / 2.0));
            }
            pos[2] += params.room_size[2] * 0.5;
            spheres.push((
                pos,
                fere_examples::gen_color(),
                rng.gen_range(20..80),
                rng.gen_range(min_radius..max_radius),
            ));
        }
        /*
        spheres = vec![
            (Vec3::new(0.0, 0.0, params.room_size.z/2.0), IVec3::new(255, 0, 0), 50, 10.0)
        ];
        */
        Self { spheres }
    }
}

struct Scene {
    renderer: Fere,
    input_manager: InputManager,
    camera_control: CameraControl,
    timer: u64,
    resources: Arc<Resources>,
    params: Arc<SceneParams>,
    state: Arc<SceneState>,
}

struct Resources {
    cube_map_coarse: Arc<Mesh>,
    cube: Arc<Mesh>,
    sphere: Arc<Mesh>,
}

impl ProgramWithImgui for Scene {
    fn new() -> Self {
        let params: SceneParams = serde_yaml::from_str(
            &std::fs::read_to_string("./examples/examples/scene2.yml").unwrap(),
        )
        .unwrap();
        let mut renderer = Fere::new(params.fere_configs.clone());

        renderer
            .add_chamber(ChamberConfig {
                bpos: Vec3::zeros(),
                size: params.room_size,
            })
            .unwrap();

        let screen_size = renderer.configs().resolution;
        let camera_control =
            CameraControl::new(Vec3::new(0.0, -50.0, 50.0), Vec3::new(0.0, 0.0, 0.0));

        Scene {
            renderer,
            input_manager: InputManager::new(screen_size),
            camera_control,
            timer: 0,
            state: Arc::new(SceneState::new(&params)),
            params: Arc::new(params),
            resources: Arc::new(Resources {
                cube_map_coarse: fere_examples::read_mesh("cube_map_coarse.obj"),
                cube: fere_examples::read_mesh("cube.obj"),
                sphere: fere_examples::read_mesh("sphere2.obj"),
            }),
        }
    }

    fn update(&mut self, imgui_ctx: ImgUiContext) -> String {
        let screen_size = self.renderer.configs().resolution;

        self.input_manager.update(imgui_ctx.as_ref());
        self.camera_control
            .update(&self.input_manager.get_input_image());
        let mut camera = SetCamera::new(
            self.camera_control.get().0,
            self.camera_control.get().1,
            Vec3::new(0.0, 0.0, 1.0),
            (60.0_f32).to_radians(),
            screen_size.x as f32 / screen_size.y as f32,
            0.1,
            1000.0,
        );
        camera.trans();

        let (frame, renderer) = self.renderer.new_frame(FrameConfig {
            camera,
            show_lightvolume_outline: false,
        });

        let timer = self.timer;
        let scene_params = Arc::clone(&self.params);
        let scene_state = Arc::clone(&self.state);
        let resources = Arc::clone(&self.resources);

        let render_thread = std::thread::spawn(move || {
            render(
                frame,
                timer,
                scene_params.as_ref(),
                scene_state.as_ref(),
                resources.as_ref(),
            )
        });
        self.renderer.end_frame(renderer.render());
        render_thread.join().unwrap();

        self.timer += 1;

        "continue".to_owned()
    }
}

fn render(
    mut frame: Frame,
    _timer: u64,
    params: &SceneParams,
    state: &SceneState,
    resources: &Resources,
) {
    let color = IVec4::new(255, 255, 255, 255);
    let xcolor = IVec4::new(255, 255, 0, 255);
    let ycolor = IVec4::new(0, 255, 255, 255);

    let count = 10;
    let interval = 1.0;
    let width = 2.0;
    let z_offset = 0.001;

    frame.push(fere_examples::draw_grid(
        color, xcolor, ycolor, count, interval, width, z_offset,
    ));

    // Draw a chamber
    let mesh = Arc::clone(&resources.cube_map_coarse);
    let ori = Ori::new(
        Vec3::new(0.0, 0.0, params.room_size.z / 2.0),
        params.room_size / 2.0,
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let surface = GeneralI {
        basecolor: TexVar::U(IVec3::new(255, 255, 255)),
        roughness: TexVar::U(0),
        metalness: TexVar::U(0),
        normal: no_normal_map(),
    };
    frame.push(DrawGeneral {
        object: Object {
            mesh,
            shadow: false,
            irradiance_volume: false,
            trans: *ori.trans(),
            chamber_index: 0,
        },
        surface,
    });

    // Draw a wall
    let mesh = Arc::clone(&resources.cube);
    let ori = fere_examples::calc_ori_for_cuboid(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 50.0, 50.0),
        0.0,
    );
    let surface = GeneralI {
        basecolor: TexVar::U(IVec3::new(200, 150, 200)),
        roughness: TexVar::U(30),
        metalness: TexVar::U(200),
        normal: no_normal_map(),
    };
    frame.push(DrawGeneral {
        object: Object {
            mesh,
            shadow: true,
            irradiance_volume: false,
            trans: *ori.trans(),
            chamber_index: 0,
        },
        surface,
    });

    // Draw spheres
    let mesh = Arc::clone(&resources.sphere);
    for (pos, color, intensity, radius) in state.spheres.iter() {
        let ori = Ori::new(
            *pos,
            Vec3::from_element(*radius),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let surface = EmissiveStaticI::with_uniform_timepoints(
            GeneralI {
                basecolor: TexVar::U(IVec3::new(200, 200, 255)),
                roughness: TexVar::U(20),
                metalness: TexVar::U(240),
                normal: no_normal_map(),
            },
            TimepointI {
                emission: TexVar::U(*color),
                emission_intensity: TexVar::U(*intensity),
                smoothness: 1.0,
            },
        );
        frame.push(DrawEmissiveStatic {
            object: Object {
                mesh: Arc::clone(&mesh),
                shadow: true,
                irradiance_volume: false,
                trans: *ori.trans(),
                chamber_index: 0,
            },
            surface,
            point_light: Some(1.0),
        });
    }

    frame.push(AddAmbientLight {
        color: Vec3::new(0.05, 0.05, 0.05),
        omni: true,
        chamber_index: 0,
    });

    frame.push(VisualizeProbes { chamber_index: 0 });
    frame.push(ShadeWithIv {
        chamber_index: 0,
        weight: 0.0,
    });
    frame.push(ShowInternalTexture {
        name: "iv_illuminatiion".to_owned(),
        pos: Vec2::new(0.0, 0.0),
        size: Vec2::new(2.0, 2.0),
    });

    frame.end();
}

fn main() {
    run_with_imgui::<Scene>(
        serde_yaml::from_str(
            &std::fs::read_to_string("./examples/examples/window_config.yml").unwrap(),
        )
        .unwrap(),
    );
}
