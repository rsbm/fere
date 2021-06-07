use fere::prelude::*;
use fere_resources::surface::{no_normal_map, GeneralI, TexVar};
use fere_resources::Mesh;
use fere_util::ui::camera_control::CameraControl;
use fere_util::ui::input_manager::InputManager;
use fere_window::*;
use rops::*;
use std::sync::Arc;

#[derive(Debug, serde::Deserialize)]
struct SceneParams {
    room_size: Vec3,

    glowing_cuboid_bpos: Vec3,
    glowing_cuboid_size: Vec3,
    glowing_cuboid_rotate: f32,

    metal_sphere_pos: Vec3,
    metal_sphere_radius: f32,

    clay_cuboid_bpos: Vec3,
    clay_cuboid_size: Vec3,
    clay_cuboid_rotate: f32,

    ceramic_cuboid_bpos: Vec3,
    ceramic_cuboid_size: Vec3,
    ceramic_cuboid_rotate: f32,
}

struct Scene {
    renderer: Fere,
    input_manager: InputManager,
    camera_control: CameraControl,
    timer: u64,
    params: Arc<SceneParams>,
    resources: Arc<Resources>,
}

struct Resources {
    cube_map_coarse: Arc<Mesh>,
    cube: Arc<Mesh>,
    sphere: Arc<Mesh>,
}

impl ProgramWithImgui for Scene {
    fn new() -> Self {
        let mut renderer = Fere::new(
            serde_yaml::from_str(
                &std::fs::read_to_string("./examples/examples/basic_fere_configs.yml").unwrap(),
            )
            .unwrap(),
        );
        let params: SceneParams = serde_yaml::from_str(
            &std::fs::read_to_string("./examples/examples/scene1.yml").unwrap(),
        )
        .unwrap();
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
            show_lightvolume_outline: true,
        });

        let timer = self.timer;
        let scene_params = Arc::clone(&self.params);
        let resources = Arc::clone(&self.resources);

        let render_thread = std::thread::spawn(move || {
            render(frame, timer, scene_params.as_ref(), resources.as_ref())
        });
        self.renderer.end_frame(renderer.render());
        render_thread.join().unwrap();

        self.timer += 1;

        "continue".to_owned()
    }
}

fn render(mut frame: Frame, timer: u64, params: &SceneParams, resources: &Resources) {
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

    // Draw a metal sphere
    let mesh = Arc::clone(&resources.sphere);
    let ori = Ori::new(
        params.metal_sphere_pos,
        Vec3::from_element(params.metal_sphere_radius),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let surface = GeneralI {
        basecolor: TexVar::U(IVec3::new(200, 200, 255)),
        roughness: TexVar::U(20),
        metalness: TexVar::U(240),
        normal: no_normal_map(),
    };
    frame.push(DrawGeneral {
        object: Object {
            mesh: Arc::clone(&mesh),
            shadow: true,
            irradiance_volume: false,
            trans: *ori.trans(),
            chamber_index: 0,
        },
        surface,
    });

    // Draw a glowing cube
    let mesh = Arc::clone(&resources.cube);
    let ori = fere_examples::calc_ori_for_cuboid(
        params.glowing_cuboid_bpos,
        params.glowing_cuboid_size,
        params.glowing_cuboid_rotate,
    );
    let surface = GeneralI {
        basecolor: TexVar::U(IVec3::new(200, 200, 200)),
        roughness: TexVar::U(60),
        metalness: TexVar::U(20),
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

    // Draw a non-metal cube
    let mesh = Arc::clone(&resources.cube);
    let ori = fere_examples::calc_ori_for_cuboid(
        params.clay_cuboid_bpos,
        params.clay_cuboid_size,
        params.clay_cuboid_rotate,
    );
    let surface = GeneralI {
        basecolor: TexVar::U(IVec3::new(200, 150, 200)),
        roughness: TexVar::U(150),
        metalness: TexVar::U(0),
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

    // Add lights
    frame.push(AddAmbientLight {
        color: Vec3::new(0.05, 0.05, 0.05),
        omni: true,
        chamber_index: 0,
    });

    let r = timer as f32 * 0.01;
    let pos = Vec3::new(
        -params.room_size.x / 2.0 + 1.0,
        -params.room_size.y / 2.0 + 1.0,
        params.room_size.z - 1.0,
    );
    let look_pos = Vec3::new(r.cos() * 10.0, r.sin() * 10.0, (r * 0.7).sin() * 8.0 + 8.0);
    let look_dir = normalize(&(look_pos - pos));
    let xdir = normalize(&glm::cross(&look_dir, &Vec3::new(0.0, 0.0, 1.0)));
    let ydir = normalize(&glm::cross(&xdir, &look_dir));

    frame.push(AddMajorLight {
        pos,
        color: Vec3::new(1000.0, 0.0, 0.0),
        xdir,
        ydir,
        perspective: (40.0_f32).to_radians(),
        chamber_index: 0,
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
