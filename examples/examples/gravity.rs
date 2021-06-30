use fere::prelude::{fere_resources::Texture, *};
use fere_window::*;
use parking_lot::RwLock;
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

struct Scene {
    renderer: Fere,
    world: Arc<RwLock<World>>,
    resources: Arc<Resources>,
}

struct Resources {
    triangle: Arc<Texture>,
}

#[derive(Clone, Debug)]
struct Particle {
    pos: Vec3,
    speed: Vec3,
    mass: f32,
}

struct World {
    particles: Vec<Particle>,
    time: f32,
}

fn acc(p: &Particle, p2: &Particle) -> Vec3 {
    if p.pos == p2.pos {
        return Vec3::zeros();
    }

    let r = length(&(p.pos - p2.pos)) * 0.8;
    let r = if r < 0.8 {
        2.203125 * r - 0.2
    } else {
        1.0 / (r * r)
    };
    normalize(&(p2.pos - p.pos)) * p2.mass * r * 1.0
}

impl World {
    fn new() -> Self {
        let mut particles = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..2000 {
            let mut pos = Vec3::new(0.0, 0.0, 0.0);
            for i in 0..2 {
                pos[i] = rng.gen_range((-100.0)..(100.0));
            }
            let speed = Vec3::new(
                -pos.y * rng.gen_range((0.0)..(10.0)),
                pos.x * rng.gen_range((0.0)..(10.0)),
                0.0,
            ) * 0.0005;
            particles.push(Particle {
                pos,
                speed,
                mass: 1.0,
            });
        }
        Self {
            particles,
            time: 0.0,
        }
    }

    fn update(&mut self) {
        self.particles = self
            .particles
            .par_iter()
            .map(|x| {
                let acc = self
                    .particles
                    .par_iter()
                    .map(|y| acc(x, y))
                    .reduce(Vec3::zeros, |a, b| a + b);
                let mut particle = x.clone();
                particle.speed += acc * 0.01;
                particle.pos += particle.speed * 1.0;
                particle
            })
            .collect();
        self.time += 0.01;
    }
}

impl Program for Scene {
    fn new() -> Self {
        let renderer = Fere::new(
            serde_yaml::from_str(
                &std::fs::read_to_string("./examples/examples/basic_fere_configs.yml").unwrap(),
            )
            .unwrap(),
        );

        let world = Arc::new(RwLock::new(World::new()));
        let world_ = Arc::clone(&world);
        std::thread::spawn(move || loop {
            world_.write().update();
            std::thread::sleep(std::time::Duration::from_millis(1))
        });

        Scene {
            renderer,
            resources: Arc::new(Resources {
                triangle: fere_examples::read_texture("triangle.png"),
            }),
            world,
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
        let world = Arc::clone(&self.world);

        let render_thread = std::thread::spawn(|| render(frame, resources, world));
        self.renderer.end_frame(renderer.render());
        render_thread.join().unwrap();
        "continue".to_owned()
    }
}

fn render(mut frame: Frame, resources: Arc<Resources>, world: Arc<RwLock<World>>) {
    let world = world.read();
    for p in &world.particles {
        frame.push(rops::DrawBillboard {
            texture: Arc::clone(&resources.triangle),
            depth_test: false,
            depth_write: false,
            pos: p.pos,
            size: Vec2::new(0.3, 0.3),
            rotation: f32::atan2(p.speed.y, p.speed.x) - (90.0 as f32).to_radians(),
            blend_mode: (),
            color: Vec4::new(1.0, 0.0, 0.0, 0.7),
        });
    }
    drop(world);
    frame.end();
}

fn main() {
    run::<Scene>(
        serde_yaml::from_str(
            &std::fs::read_to_string("./examples/examples/window_config.yml").unwrap(),
        )
        .unwrap(),
    );
}
