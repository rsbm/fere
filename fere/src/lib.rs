mod frame;
mod graphics;
pub mod ops;
mod renderer;

use crate::graphics::gi::probe_volume::ProbeVolumeSuite;
use crate::graphics::graphics::{Graphics, GraphicsConfig};
use fere_common::geo::SixDir;
use fere_common::vec::IteratorVec4;
use fere_common::*;
use frame::{Frame, FrameConfig};
use frame::{OpQueueReceiver, OpQueueSender};
use ops::ChamberIndex;
use renderer::{RenderEnd, Renderer, RendererParams};
use serde::Deserialize;
use thiserror::Error;

pub mod prelude {
    pub use crate::frame::{Frame, FrameConfig};
    pub use crate::{ops as rops, renderer::Renderer, ChamberConfig, Error, Fere, FereConfigs};
    pub use fere_common::{self, *};
    pub use fere_resources;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid chamber accesses")]
    InvalidChamberAccess,
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
                self.current_probe.0.x,
                self.current_probe.0.y,
                self.current_probe.0.z,
                self.current_probe.1 as i32,
            ),
        };
        self.current_probe = if let Some(x) = i.next() {
            (IVec3::new(x.w, x.z, x.y), x.x as i8)
        } else {
            (IVec3::new(0, 0, 0), 0)
        };
    }
}

pub struct Chamber {
    config: ChamberConfig,
    state: ChamberState,
}

/// Configurations for a Fere Instance, required only once for the initial creation.
#[derive(Clone, Debug, Deserialize)]
pub struct FereConfigs {
    pub resolution: IVec2,

    pub shadow_resolution: usize,
    pub probe_resolution: usize,
    pub max_major_lights: usize,

    pub max_chamber_num: usize,

    pub pv_scale: f32,
}

/// The Fere instance.
pub struct Fere {
    graphics: Option<Graphics>,
    configs: FereConfigs,
    chambers: Vec<Option<Chamber>>,
}

impl Fere {
    pub fn new(configs: FereConfigs) -> Self {
        Self {
            graphics: Some(Graphics::new(GraphicsConfig {
                resolution: configs.resolution,
                shadow_resolution: configs.shadow_resolution,
                probe_resolution: configs.probe_resolution,
                max_major_lights: configs.max_major_lights,
            })),
            chambers: (0..configs.max_chamber_num).map(|_| None).collect(),
            configs,
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

    pub fn new_frame(&mut self, config: FrameConfig) -> (Frame, Renderer) {
        let (send, recv): (OpQueueSender, OpQueueReceiver) = crossbeam::channel::unbounded();
        (
            Frame::new(config, send),
            Renderer::new(
                self.graphics.take().unwrap(),
                recv,
                Default::default(),
                self.configs.clone(),
                self.chambers
                    .iter_mut()
                    .map(|x| match x.take() {
                        Some(x) => Some(x),
                        None => None,
                    })
                    .collect(),
            ),
        )
    }

    pub fn end_frame(&mut self, mut render_end: RenderEnd) {
        render_end
            .logs
            .sort_by(|x, y| x.timestamp.cmp(&y.timestamp));
        for log in render_end.logs.iter() {
            println!("JRE: {}", log.to_string());
        }
        self.chambers = render_end.chambers;
        assert!(
            self.graphics.replace(render_end.graphics).is_none(),
            "end_frame() called without new_frame()"
        );
    }
}
