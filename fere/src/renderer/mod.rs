mod chamber;
mod common;
mod images;
mod probe;
mod render;
mod shading;

use super::*;
use crate::frame::OpQueueReceiver;
use crate::graphics::glmanager::light::{Light, LightDir, LightUni};
use crate::graphics::glmanager::shader::Shader;
use crate::graphics::graphics::{
    material::{bind_emissive_static, bind_fixed_color, bind_general},
    texture_internal::{InternalTexType, TextureInternal2D, TextureInternal3D},
    Graphics,
};
use crate::graphics::render_unit::{Lighting, RenderUnit};
use crate::ops::*;
use chamber::ChamberContext;
use fere_common::*;
use fere_resources::Mesh;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FrameLog {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub log: String,
}

impl FrameLog {
    pub fn from_err(err: impl std::error::Error) -> Self {
        Self {
            timestamp: chrono::Utc::now(),
            log: err.to_string(),
        }
    }

    pub fn new(log: String) -> Self {
        Self {
            timestamp: chrono::Utc::now(),
            log,
        }
    }
}

impl std::fmt::Display for FrameLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.timestamp, self.log)
    }
}

#[derive(Error, Debug)]
pub enum OpError {
    #[error("Invalid chamber accesses: {0}")]
    InvalidChamberAccess(ChamberIndex),
    #[error("Duplicated operations that must be unique: {0}")]
    InvalidShade(String),
    #[error("{0}")]
    Other(String),
}

struct RenderContext {
    graphics: Graphics,
    params: RendererParams,
    _fere_configs: FereConfigs,
    logs: Vec<FrameLog>,
    camera_info: Option<SetCamera>,
    chamber_contexts: Vec<Option<ChamberContext>>,

    draw_images: Vec<DrawImage>,
    draw_billboarsd: Vec<DrawBillboard>,
    show_internal_textures: Vec<ShowInternalTexture>,
}

impl RenderContext {
    fn get_mut_chamber_ctx(
        &mut self,
        chamber_index: ChamberIndex,
    ) -> Result<&mut ChamberContext, OpError> {
        self.chamber_contexts
            .get_mut(chamber_index as usize)
            .ok_or(OpError::InvalidChamberAccess(chamber_index))?
            .as_mut()
            .ok_or(OpError::InvalidChamberAccess(chamber_index))
    }

    fn get_chamber_ctx(&self, chamber_index: ChamberIndex) -> Result<&ChamberContext, OpError> {
        self.chamber_contexts
            .get(chamber_index as usize)
            .ok_or(OpError::InvalidChamberAccess(chamber_index))?
            .as_ref()
            .ok_or(OpError::InvalidChamberAccess(chamber_index))
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RendererParams {
    pub debug_lightvolume_outline: bool,
    pub enable_shadow: bool,
    pub enable_irradiance_volume: bool,
}

impl Default for RendererParams {
    fn default() -> Self {
        Self {
            debug_lightvolume_outline: true,
            enable_irradiance_volume: false,
            enable_shadow: false,
        }
    }
}

pub struct Renderer {
    object_queue: OpQueueReceiver,
    graphics: Option<Graphics>,
    params: RendererParams,
    fere_configs: FereConfigs,
    chambers: Vec<Option<Chamber>>,
    /// Other logs captured during op processing
    logs: Vec<FrameLog>,
}

pub struct RenderEnd {
    pub(crate) graphics: Graphics,
    pub(crate) logs: Vec<FrameLog>,
    pub(crate) chambers: Vec<Option<Chamber>>,
}

impl Renderer {
    pub fn new(
        graphics: Graphics,
        object_queue: OpQueueReceiver,
        params: RendererParams,
        fere_configs: FereConfigs,
        chambers: Vec<Option<Chamber>>,
    ) -> Self {
        Self {
            graphics: Some(graphics),
            object_queue,
            params,
            fere_configs,
            chambers,
            logs: Vec::new(),
        }
    }

    fn create_renderer(&mut self) -> RenderContext {
        RenderContext {
            graphics: self.graphics.take().unwrap(),
            params: self.params.clone(),
            _fere_configs: self.fere_configs.clone(),

            camera_info: Default::default(),
            logs: Default::default(),
            chamber_contexts: self
                .chambers
                .iter_mut()
                .map(|x| match x.take() {
                    Some(x) => Some(ChamberContext::new(x)),
                    None => None,
                })
                .collect(),
            draw_images: Default::default(),
            draw_billboarsd: Default::default(),
            show_internal_textures: Default::default(),
        }
    }

    pub fn render(mut self) -> RenderEnd {
        let mut ctx = self.create_renderer();
        loop {
            match self.object_queue.recv().unwrap() {
                RenderOp::EndFrame(_) => {
                    ctx.graphics.bind_deferred_pass2(true);
                    // FIXME: Update only one.
                    if self.params.enable_irradiance_volume {
                        for i in 0..self.fere_configs.max_chamber_num {
                            if ctx.chamber_contexts[i].is_some() {
                                ctx.update_probe(i as u32);
                            }
                        }
                    }

                    for i in 0..self.fere_configs.max_chamber_num {
                        if ctx.chamber_contexts[i].is_some() {
                            ctx.shade(i as u32);
                        }
                    }

                    ctx.graphics.bind_2d();
                    ctx.render_images();
                    ctx.graphics.render_final();

                    self.logs.append(&mut ctx.logs);
                    return RenderEnd {
                        graphics: ctx.graphics,
                        logs: self.logs,
                        chambers: ctx
                            .chamber_contexts
                            .into_iter()
                            .map(|x| x.map(|x| x.chamber))
                            .collect(),
                    };
                }
                op => {
                    let mut op_list = if let RenderOp::Multiple(v) = op {
                        v
                    } else {
                        vec![op]
                    };

                    while let Some(op) = op_list.pop() {
                        match ctx.process_op(op) {
                            Ok(Some(op)) => match op {
                                RenderOp::Multiple(mut v) => {
                                    op_list.append(&mut v);
                                }
                                RenderOp::EndFrame(_) => {
                                    panic!("`RenderOp::EndFrame` must not be emitted")
                                }
                                x => op_list.push(x),
                            },
                            Err(err) => self.logs.push(FrameLog::from_err(err)),
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}
