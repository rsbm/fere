mod graphics;

use fere_common::geo::SixDir;
use fere_common::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid chamber accesses")]
    InvalidChamberAccess,
}

pub type ChamberIndex = u32;

#[derive(Clone, Debug)]
pub struct ChamberConfig {
    pub bpos: Vec3,
    pub size: Vec3,
}

pub struct ChamberState {
    current_probe: (IVec3, SixDir),
}
