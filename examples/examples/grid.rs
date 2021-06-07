use fere::prelude::*;
use fere_window::*;
struct Scene {
    renderer: Fere,
}

impl Program for Scene {
    fn new() -> Self {
        let mut renderer = Fere::new(
            serde_yaml::from_str(
                &std::fs::read_to_string("./examples/examples/basic_fere_configs.yml").unwrap(),
            )
            .unwrap(),
        );
        renderer
            .add_chamber(ChamberConfig {
                bpos: Vec3::zeros(),
                size: Vec3::new(50.0, 50.0, 50.0),
            })
            .unwrap();
        Scene { renderer }
    }

    fn update(&mut self) -> String {
        let cpos = Vec3::new(8.0, -10.0, 10.0);
        let screen_size = self.renderer.configs().resolution;
        let mut camera = SetCamera::new(
            cpos,
            Vec3::new(0.0, 0.0, 0.0),
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
        let render_thread = std::thread::spawn(|| render(frame));
        self.renderer.end_frame(renderer.render());
        render_thread.join().unwrap();
        "continue".to_owned()
    }
}

fn render(mut frame: Frame) {
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

    frame.push(rops::AddAmbientLight {
        color: Vec3::new(0.0, 0.0, 0.0),
        omni: false,
        chamber_index: 0,
    });

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
