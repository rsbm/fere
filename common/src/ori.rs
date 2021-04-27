use crate::*;
use serde::Serialize;

/// An orientation of a 3D object.
#[derive(Serialize)]
pub struct Ori {
    pos: Vec3,
    size: Vec3,
    xdir: Vec3,
    ydir: Vec3,

    // calculated from the given propety
    #[serde(skip_serializing)]
    zdir: Vec3,
    #[serde(skip_serializing)]
    trans: Mat4,
}

impl Ori {
    pub fn new(pos: Vec3, size: Vec3, xdir: Vec3, ydir: Vec3) -> Self {
        let trans = glm::translate(&Mat4::identity(), &pos);
        let trans = trans
            * crate::geo::rotation_between(
                &Vec3::new(1.0, 0.0, 0.0),
                &Vec3::new(0.0, 1.0, 0.0),
                &xdir,
                &ydir,
            );
        let trans = glm::scale(&trans, &size);
        Self {
            pos,
            size,
            xdir,
            ydir,
            zdir: glm::cross(&xdir, &ydir),
            trans,
        }
    }

    pub fn with_rotation(pos: Vec3, size: Vec3, rotate_axis: Vec3, rotate_angle: f32) -> Self {
        let trans = glm::translate(&Mat4::identity(), &pos);
        let trans = glm::rotate(&trans, rotate_angle, &rotate_axis);
        let trans = glm::scale(&trans, &size);

        let rotate = glm::rotate(&Mat4::identity(), rotate_angle, &rotate_axis);
        let xdir = rotate * Vec4::new(1.0, 0.0, 0.0, 1.0);
        let ydir = rotate * Vec4::new(0.0, 1.0, 0.0, 1.0);
        let xdir = Vec3::new(xdir.x, xdir.y, xdir.z);
        let ydir = Vec3::new(ydir.x, ydir.y, ydir.z);

        Self {
            pos,
            size,
            xdir,
            ydir,
            zdir: glm::cross(&xdir, &ydir),
            trans,
        }
    }

    pub fn identity() -> Self {
        Self {
            pos: Vec3::from_element(0.0),
            size: Vec3::from_element(0.0),
            xdir: Vec3::new(1.0, 0.0, 0.0),
            ydir: Vec3::new(0.0, 1.0, 0.0),
            zdir: Vec3::new(0.0, 0.0, 1.0),
            trans: Mat4::identity(),
        }
    }

    pub fn trans(&self) -> &Mat4 {
        &self.trans
    }
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn size(&self) -> &Vec3 {
        &self.size
    }
    pub fn xdir(&self) -> &Vec3 {
        &self.xdir
    }
    pub fn ydir(&self) -> &Vec3 {
        &self.ydir
    }
    pub fn zdir(&self) -> &Vec3 {
        &self.zdir
    }
}
