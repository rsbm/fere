#![allow(clippy::many_single_char_names)]

use fere::prelude::{fere_resources::Texture, *};
use fere_window::*;
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

struct Scene {
    renderer: Fere,
    world: Option<Box<World>>,
    resources: Arc<Resources>,

    frame_count: usize,
}

struct Resources {
    triangle: Arc<Texture>,
}

#[derive(Clone, Debug)]
struct Particle {
    pos: Vec3,
    speed: Vec3,
    acc: Vec3,
    color: IVec3,
    size: f32,
}

struct World {
    particles: Vec<Particle>,
    time: usize,
}

fn gen_color(x: f32) -> IVec3 {
    let r = (x * 0.9 + 0.4).cos() + 1.0;
    let b = (x * 0.8 + 0.3).cos() + 1.0;
    let g = (x * 0.7 + 0.2).cos() + 1.0;
    let r = 128 + (127.0 * r * 0.5) as i32;
    let g = 128 + (127.0 * g * 0.5) as i32;
    let b = 128 + (127.0 * b * 0.5) as i32;
    IVec3::new(r, g, b)
}

fn cos(x: f32) -> f32 {
    x.cos()
}

fn posco(x: f32) -> f32 {
    1.0 + x.cos() * 0.5
}

impl World {
    fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            particles: Vec::new(),
            time: rng.gen_range(0..100),
        }
    }

    fn update(&mut self) {
        let params = [(1.0, 2.0), (3.0, 2.0), (4.0, 1.0), (2.0, 2.0)];

        if self.time % 2 == 0 {
            let t = self.time as f32 * 0.1;

            let sp_pos: Vec<_> = params
                .iter()
                .map(|(a, b)| {
                    let a = *a;
                    let b = *b;
                    let x = cos(cos(t * (0.4 + posco(a)) * 0.4) * (cos(a) + 1.1)) * 200.0
                        + cos(cos(t * 0.3 * cos(b))) * 10.0;
                    let y = cos(cos(t * (0.4 + posco(a)) * 0.4) * (cos(b) + 1.1)) * 200.0
                        + cos(cos(t * 0.3 * cos(a))) * 10.0;
                    let z = 0.0;
                    Vec3::new(x, y, z)
                })
                .collect();

            for (s, sp) in sp_pos.iter().enumerate() {
                for i in 0..30 {
                    let at = (i as f32) / 30.0;
                    let angle = (((t * 2.1 + cos(sp.x)) as i32 * 3 + (i as i32) * 5) as f32)
                        .to_radians()
                        + 3.0 * cos(params[s].0);
                    let speed = Vec3::new(angle.cos(), angle.sin(), 0.0) * (2.0 + at);
                    for j in 0..3 {
                        self.particles.push(Particle {
                            pos: *sp,
                            speed,
                            acc: Vec3::new(-speed.y, speed.x, 0.0) * (0.02 + (j as f32) * 0.01),
                            color: gen_color(t + at * 3.0),
                            size: 0.5,
                        });
                    }
                }
            }
        }
        self.particles.par_iter_mut().for_each(|x| {
            x.speed += x.acc;
            x.pos += x.speed;
        });

        self.particles.retain(|x| length(&x.pos) < 700.0);
        println!("{}", self.particles.len());
        self.time += 1;
    }
}

impl Program for Scene {
    fn new() -> Self {
        let mut config: FereConfigs = serde_yaml::from_str(
            &std::fs::read_to_string("./examples/examples/basic_fere_configs.yml").unwrap(),
        )
        .unwrap();
        config.video_record = true;
        let mut renderer = Fere::new(config);
        renderer.start_recording(5555).unwrap();
        let world = Some(Box::new(World::new()));
        Scene {
            renderer,
            resources: Arc::new(Resources {
                triangle: fere_examples::read_texture("triangle.png"),
            }),
            world,
            frame_count: 0,
        }
    }

    fn update(&mut self) -> String {
        let cpos = Vec3::new(0.0, 0.0, 300.0);
        let screen_size = self.renderer.configs().resolution;
        let mut camera = SetCamera::new(
            cpos,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            (90.0_f32).to_radians(),
            screen_size.x as f32 / screen_size.y as f32,
            0.1,
            1000.0,
        );
        camera.trans();

        let (frame, renderer) = self.renderer.new_frame(FrameConfig {
            camera,
            show_lightvolume_outline: false,
        });

        let resources = Arc::clone(&self.resources);
        self.world.as_mut().unwrap().update();
        let world = self.world.take().unwrap();

        let render_thread = std::thread::spawn(|| render(frame, resources, world));
        self.renderer.end_frame(renderer.render());
        self.world = Some(render_thread.join().unwrap());

        self.frame_count += 1;
        if self.frame_count == 200 {
            self.renderer.end_recording().unwrap();
            return "exit".to_owned();
        }

        "continue".to_owned()
    }
}

fn render(mut frame: Frame, resources: Arc<Resources>, world: Box<World>) -> Box<World> {
    for p in &world.particles {
        frame.push(rops::DrawBillboard {
            texture: Arc::clone(&resources.triangle),
            depth_test: false,
            depth_write: false,
            pos: p.pos,
            size: Vec2::from_element(p.size),
            rotation: f32::atan2(p.speed.y, p.speed.x) - (90.0_f32).to_radians(),
            blend_mode: (),
            color: Vec4::new(
                p.color.x as f32 / 255.0,
                p.color.y as f32 / 255.0,
                p.color.z as f32 / 255.0,
                0.5,
            ),
        });
    }
    frame.end();
    world
}

fn main() {
    run::<Scene>(
        serde_yaml::from_str(
            &std::fs::read_to_string("./examples/examples/window_config.yml").unwrap(),
        )
        .unwrap(),
    );
}
