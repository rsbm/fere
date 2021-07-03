mod gl_state;
mod imgui_renderer;

use fere::prelude::fere_common::*;
use gl_state::GlState;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use serde::Deserialize;
use std::sync::Arc;
use std::time::Instant;

pub type ImgUiContext<'a> = Arc<imgui::Ui<'a>>;

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

pub trait ProgramWithImgui: Send + Sync + 'static {
    fn new() -> Self;
    fn update(&mut self, imgui_ctx: ImgUiContext) -> String;
}

pub fn run<T: Program>(config: WindowConfig) {
    let event_loop = EventLoop::new();
    let _context = glutin::ContextBuilder::new().with_vsync(false);

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
                x: monitor_positions[config.monitor_index].0 + config.initial_window_pos.x,
                y: monitor_positions[config.monitor_index].1 + config.initial_window_pos.y,
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

#[allow(clippy::branches_sharing_code)]
pub fn run_with_imgui<T: ProgramWithImgui>(config: WindowConfig) {
    let event_loop = EventLoop::new();
    let _context = glutin::ContextBuilder::new().with_vsync(false);

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
                x: monitor_positions[config.monitor_index].0 + config.initial_window_pos.x,
                y: monitor_positions[config.monitor_index].1 + config.initial_window_pos.y,
            },
        ));

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let window = main_window.window();
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
    }

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../../mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
    let imgui_ren = imgui_renderer::Renderer::new(&mut imgui);

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

    let mut gl_state: Option<GlState> = None;
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(_) => {
            imgui
                .io_mut()
                .update_delta_time(Instant::now().duration_since(last_frame));
            last_frame = Instant::now();
        }
        Event::MainEventsCleared => {
            platform
                .prepare_frame(imgui.io_mut(), main_window.as_ref())
                .expect("Failed to prepare frame");
        }
        Event::RedrawRequested(window_id) => {
            if window_id == main_window_id {
                unsafe {
                    main_window_gl = Some(main_window_gl.take().unwrap().make_current().unwrap());
                }

                // For the first time, backup the gl state for the imgui renderer.
                if gl_state.is_none() {
                    let ui = imgui.frame();
                    platform.prepare_render(&ui, &main_window);
                    imgui_ren.render(ui);

                    gl_state = Some(GlState::default());
                    gl_state.as_mut().unwrap().backup();
                } else {
                    let ui = Arc::new(imgui.frame());
                    let msg = program.update(Arc::clone(&ui));
                    unsafe {
                        gl::Clear(gl::DEPTH_BUFFER_BIT);
                    }
                    gl_state.as_ref().unwrap().load();

                    imgui_ren.render(
                        Arc::try_unwrap(ui)
                            .map_err(|_| "You must not keep the Imgui UI context after the frame")
                            .unwrap(),
                    );
                    gl_state.as_mut().unwrap().backup();

                    if msg.as_str() == "exit" {
                        *control_flow = ControlFlow::Exit
                    }
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
        event => {
            platform.handle_event(imgui.io_mut(), main_window.as_ref(), &event);
        }
    })
}
