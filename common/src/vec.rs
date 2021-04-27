use crate::*;

pub fn normalize_w(vec: &Vec4) -> Vec3 {
    Vec3::new(vec.x / vec.w, vec.y / vec.w, vec.z / vec.w)
}

pub struct IteratorVec2 {
    pub size: IVec2,
    pub cur: IVec2,
}

pub struct IteratorVec3 {
    pub size: IVec3,
    pub cur: IVec3,
}

pub struct IteratorVec4 {
    pub size: IVec4,
    pub cur: IVec4,
}

impl IteratorVec2 {
    pub fn new(size: IVec2) -> Self {
        Self {
            size,
            cur: IVec2::new(-1, 0),
        }
    }
}
impl IteratorVec3 {
    pub fn new(size: IVec3) -> Self {
        Self {
            size,
            cur: IVec3::new(-1, 0, 0),
        }
    }
}

impl IteratorVec4 {
    pub fn new(size: IVec4) -> Self {
        Self {
            size,
            cur: IVec4::new(-1, 0, 0, 0),
        }
    }
}

impl Iterator for IteratorVec2 {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let mut up = false;
        for i in 0..2 {
            if self.cur[i] == self.size[i] - 1 {
                up = true;
                self.cur[i] = 0;
            } else {
                up = false;
                self.cur[i] += 1;
                break;
            }
        }
        if up {
            None
        } else {
            Some(self.cur)
        }
    }
}

impl Iterator for IteratorVec3 {
    type Item = IVec3;

    fn next(&mut self) -> Option<Self::Item> {
        let mut up = false;
        for i in 0..3 {
            if self.cur[i] == self.size[i] - 1 {
                up = true;
                self.cur[i] = 0;
            } else {
                up = false;
                self.cur[i] += 1;
                break;
            }
        }
        if up {
            None
        } else {
            Some(self.cur)
        }
    }
}

impl Iterator for IteratorVec4 {
    type Item = IVec4;

    fn next(&mut self) -> Option<Self::Item> {
        let mut up = false;
        for i in 0..4 {
            if self.cur[i] == self.size[i] - 1 {
                up = true;
                self.cur[i] = 0;
            } else {
                up = false;
                self.cur[i] += 1;
                break;
            }
        }
        if up {
            None
        } else {
            Some(self.cur)
        }
    }
}

pub struct GridAccessor2(pub IVec2);
impl GridAccessor2 {
    pub fn get(&self, p: &IVec2) -> usize {
        if p.x < 0 || p.y < 0 || p.x >= self.0.x || p.y > self.0.y {
            debug_assert!(false, "invalid grid access");
            0
        } else {
            (p.x * self.0.y + p.y) as usize
        }
    }

    pub fn size(&self) -> usize {
        (self.0.x * self.0.y) as usize
    }
}

pub struct GridAccessor3(pub IVec3);
impl GridAccessor3 {
    pub fn get(&self, p: &IVec3) -> usize {
        if p.x < 0 || p.y < 0 || p.z < 0 || p.x >= self.0.x || p.y > self.0.y || p.z >= self.0.z {
            debug_assert!(false, "invalid grid access");
            0
        } else {
            (p.x * self.0.y * self.0.z + p.y * self.0.z + p.z) as usize
        }
    }

    pub fn size(&self) -> usize {
        (self.0.x * self.0.y * self.0.z) as usize
    }
}
