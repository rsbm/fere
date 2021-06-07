use super::*;
use crate::ops::*;

pub(crate) type OpQueueSender = crossbeam::channel::Sender<RenderOp>;
pub(crate) type OpQueueReceiver = crossbeam::channel::Receiver<RenderOp>;

/// Configurations for a single frame.
///
/// You must provide this for the every frames you're rendering. (e.g 60 times in a sec)
pub struct FrameConfig {
    pub camera: SetCamera,

    /// For a debug purpose, show the outlines of light volumes.
    pub show_lightvolume_outline: bool,
}

pub struct Frame {
    _config: FrameConfig,
    object_queue: OpQueueSender,
}

impl Frame {
    pub fn new(config: FrameConfig, object_queue: OpQueueSender) -> Self {
        object_queue
            .send(RenderOp::StartFrame(InternalOp::do_not_call_this()))
            .unwrap();
        object_queue
            .send(RenderOp::SetCamera(config.camera.clone()))
            .unwrap();
        Self {
            _config: config,
            object_queue,
        }
    }

    pub fn end(self) {
        self.object_queue
            .send(RenderOp::EndFrame(InternalOp::do_not_call_this()))
            .unwrap();
    }

    pub fn push(&mut self, op: impl Into<RenderOp>) {
        let op = op.into();
        match op {
            RenderOp::Multiple(ops_list) => {
                for op in ops_list {
                    self.object_queue.send(op).unwrap();
                }
            }
            _ => {
                self.object_queue.send(op).unwrap();
            }
        }
    }
}
