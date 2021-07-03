mod configs;
mod frame;
mod graphics;
pub mod ops;
mod renderer;
mod video_record;

use crate::graphics::gi::probe_volume::ProbeVolumeSuite;
use crate::graphics::graphics::{Graphics, GraphicsConfig};
use configs::FereConfigs;
use fere_common::geo::SixDir;
use fere_common::vec::IteratorVec4;
use fere_common::*;
use frame::{Frame, FrameConfig};
use frame::{OpQueueReceiver, OpQueueSender};
use ops::ChamberIndex;
use renderer::{RenderEnd, Renderer, RendererParams};
use thiserror::Error;

pub mod prelude {
    pub use crate::configs::FereConfigs;
    pub use crate::frame::{Frame, FrameConfig};
    pub use crate::{ops as rops, renderer::Renderer, ChamberConfig, Error, Fere};
    pub use fere_common::{self, *};
    pub use fere_resources;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid chamber accesses")]
    InvalidChamberAccess,
    #[error("Recording error: {0}")]
    RecordingError(String),
}

#[derive(Clone, Debug)]
pub struct ChamberConfig {
    pub bpos: Vec3,
    pub size: Vec3,
}

pub struct ChamberState {
    current_probe: (IVec3, SixDir),
    probe_volume_suite: ProbeVolumeSuite,
}

impl ChamberState {
    fn set_next_probe(&mut self) {
        let num = self.probe_volume_suite.probe_volume().number();
        let mut i = IteratorVec4 {
            size: IVec4::new(6, num.x, num.y, num.z),
            cur: IVec4::new(
                self.current_probe.1 as i32,
                self.current_probe.0.x,
                self.current_probe.0.y,
                self.current_probe.0.z,
            ),
        };
        self.current_probe = if let Some(x) = i.next() {
            (IVec3::new(x.y, x.z, x.w), x.x as i8)
        } else {
            (IVec3::new(0, 0, 0), 0)
        };
    }
}

pub struct Chamber {
    config: ChamberConfig,
    state: ChamberState,
}

/// The Fere instance.
pub struct Fere {
    graphics: Option<Graphics>,
    configs: FereConfigs,
    chambers: Vec<Option<Chamber>>,

    recording_session: Option<video_record::VideoRecordingSession>,
}

impl Fere {
    pub fn new(configs: FereConfigs) -> Self {
        Self {
            graphics: Some(Graphics::new(GraphicsConfig {
                resolution: configs.resolution,
                shadow_resolution: configs.shadow_resolution,
                probe_resolution: configs.probe_resolution,
                max_major_lights: configs.max_major_lights,
                video_record: configs.video_record,
            })),
            chambers: (0..configs.max_chamber_num).map(|_| None).collect(),
            configs,
            recording_session: None,
        }
    }

    /// Returns internal graphics state.
    ///
    /// It won't be ever used by the most of users.
    pub fn graphics(&self) -> &Graphics {
        self.graphics.as_ref().unwrap()
    }

    pub fn configs(&self) -> &FereConfigs {
        &self.configs
    }

    /// Add a chamber.
    ///
    /// Returns error if it's not available to add a new chamber.
    pub fn add_chamber(&mut self, config: ChamberConfig) -> Result<ChamberIndex, Error> {
        let index = self
            .chambers
            .iter()
            .position(|x| x.is_none())
            .ok_or(Error::InvalidChamberAccess)?;
        let config_ = config.clone();
        self.chambers[index] = Some(Chamber {
            config,
            state: ChamberState {
                current_probe: (IVec3::new(0, 0, 0), 0),
                probe_volume_suite: ProbeVolumeSuite::new(
                    config_.size,
                    self.configs.pv_scale,
                    self.configs.probe_resolution,
                ),
            },
        });
        Ok(index as ChamberIndex)
    }

    /// Remove an existing chamber.
    ///
    /// Panics if there is no such chamber corresponding to the given index.
    pub fn remove_chamber(&mut self, index: ChamberIndex) {
        self.chambers
            .get_mut(index as usize)
            .ok_or(Error::InvalidChamberAccess)
            .unwrap()
            .take()
            .ok_or(Error::InvalidChamberAccess)
            .map(|_| ())
            .unwrap()
    }

    pub fn fetch_index_buffer(_pos: IVec2) -> u64 {
        0
    }

    pub fn new_frame(&mut self, config: FrameConfig) -> (Frame, Renderer) {
        let (send, recv): (OpQueueSender, OpQueueReceiver) = crossbeam::channel::unbounded();

        let renderer = Renderer::new(
            self.graphics.take().unwrap(),
            recv,
            create_renderer_param(&self.configs, &config),
            self.configs.clone(),
            self.chambers.iter_mut().map(|x| x.take()).collect(),
        );
        (Frame::new(config, send), renderer)
    }

    pub fn end_frame(&mut self, mut render_end: RenderEnd) {
        render_end
            .logs
            .sort_by(|x, y| x.timestamp.cmp(&y.timestamp));
        for log in render_end.logs.iter() {
            println!("Fere: {}", log.to_string());
        }
        self.chambers = render_end.chambers;

        if let Some(recording_session) = self.recording_session.as_mut() {
            render_end.graphics.render_yuv();
            recording_session.update_frame(&render_end.graphics)
        }
        assert!(
            self.graphics.replace(render_end.graphics).is_none(),
            "end_frame() called without new_frame()"
        );
    }

    pub fn start_recording(&mut self, port: u16) -> Result<(), Error> {
        if self.recording_session.is_some() {
            Err(Error::RecordingError(
                "start_recording() while there's already a session".to_owned(),
            ))
        } else {
            self.recording_session = Some(video_record::VideoRecordingSession::new(
                port,
                self.graphics().screen_size(),
                60,
            ));
            Ok(())
        }
    }

    pub fn end_recording(&mut self) -> Result<(), Error> {
        if let Some(recording_session) = self.recording_session.take() {
            recording_session.end();
            Ok(())
        } else {
            Err(Error::RecordingError(
                "end_recording() called without recording session".to_owned(),
            ))
        }
    }
}

fn create_renderer_param(configs: &FereConfigs, frame_configs: &FrameConfig) -> RendererParams {
    RendererParams {
        debug_lightvolume_outline: frame_configs.show_lightvolume_outline,
        enable_shadow: true,
        enable_irradiance_volume: configs.irradiance_volume.is_some(),
    }
}
