#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Lighting {
    None = 0,

    // Deferred
    DefFull = 1,
    DefFixed = 2,
    DefRough = 3,
    DefFullButNoGi = 4,

    // Forward
    ForMajor = 11,
    ForFixed = 12,
    ForRough = 13,
}

pub struct RenderUnit {
    pub color: bool,
    pub depth: bool,
    pub depth_test: bool,

    /// Forward: ignore. If None, then mask.
    pub id: Option<u32>,
    /// 0(No lighting), 1(full), 2(fixed), 3(arbitrary lighting).
    /// forward : 0(major), 1(no), 2(arbitrary)
    ///  If None, then mask.
    pub lighting: Option<Lighting>,
}
