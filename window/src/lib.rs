use fere::prelude::fere_common::*;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct WindowConfig {
    pub screen_size: IVec2,
    pub initial_window_pos: IVec2,
    pub monitor_index: usize,
    pub title: String,
}

pub fn run(config: WindowConfig) {
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
}
