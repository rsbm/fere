use fere::prelude::fere_common::*;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use serde::Deserialize;
use std::sync::Arc;
#[derive(Debug, Clone, Deserialize)]
pub struct WindowConfig {
    pub screen_size: IVec2,
    pub initial_window_pos: IVec2,
    pub monitor_index: usize,
    pub title: String,
}

pub trait Program: Send + Sync + 'static {
    fn new() -> Self;
    fn update(&mut self) -> String;
}

pub fn run<T: Program>(config: WindowConfig) {
    let event_loop = EventLoop::new();
    let _context = glutin::ContextBuilder::new().with_vsync(true);

    let main_window = WindowBuilder::new()
        .with_title(config.title)
        .with_resizable(false)
        .with_decorations(true)
        .with_inner_size(glutin::dpi::PhysicalSize::new(
            config.screen_size.x as f64,
            config.screen_size.y as f64,
        ));

    let main_window = glutin::ContextBuilder::new()
        .build_windowed(main_window, &event_loop)
        .unwrap();
    let main_window = unsafe { main_window.make_current().unwrap() };
    gl::load_with(|symbol| main_window.get_proc_address(symbol));
    let main_window_id = main_window.window().id();

    let monitor_positions: Vec<(i32, i32)> = main_window
        .window()
        .available_monitors()
        .map(|x| (x.position().x, x.position().y))
        .collect();
    main_window
        .window()
        .set_outer_position(winit::dpi::Position::Physical(
            winit::dpi::PhysicalPosition {
                x: monitor_positions[config.monitor_index].0,
                y: monitor_positions[config.monitor_index].1,
            },
        ));

    let (main_window_gl, main_window) = unsafe { main_window.split() };
    let mut main_window_gl = Some(main_window_gl);
    let main_window = Arc::new(main_window);
    let main_window_ = Arc::clone(&main_window);

    let _redraw = std::thread::spawn(move || loop {
        main_window_.request_redraw();
        std::thread::sleep(std::time::Duration::from_micros(
            (1.0 / 60.0 * 1000.0 * 1000.0) as u64,
        ));
    });

    let mut program = T::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) => {
            if window_id == main_window_id {
                unsafe {
                    main_window_gl = Some(main_window_gl.take().unwrap().make_current().unwrap());
                }
                let msg = program.update();
                if msg.as_str() == "exit" {
                    *control_flow = ControlFlow::Exit
                }
                main_window_gl.as_ref().unwrap().swap_buffers().unwrap();
            } else {
                unreachable!()
            }
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        Event::LoopDestroyed => {
            main_window_gl.take().unwrap();
        }
        _ => (),
    })
}
