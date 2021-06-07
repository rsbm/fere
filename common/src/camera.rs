use super::*;

#[derive(Clone, Debug)]
pub struct SetCamera {
    pub pos: Vec3,
    pub look: Vec3,
    pub up: Vec3,

    pub perspective: f32,
    pub ratio: f32,
    pub near: f32,
    pub far: f32,

    projection: Option<Mat4>,
    view: Option<Mat4>,
}

impl SetCamera {
    pub fn new(
        pos: Vec3,
        look: Vec3,
        up: Vec3,
        perspective: f32,
        ratio: f32,
        near: f32,
        far: f32,
    ) -> Self {
        SetCamera {
            pos,
            look,
            up,
            perspective,
            ratio,
            near,
            far,
            projection: None,
            view: None,
        }
    }

    pub fn projection_get(&self) -> &Mat4 {
        self.projection.as_ref().unwrap()
    }

    pub fn view_get(&self) -> &Mat4 {
        self.view.as_ref().unwrap()
    }

    pub fn trans(&mut self) {
        let mut up_temp = self.up.normalize();
        let look_vec = (self.look - self.pos).normalize();
        if dot(&up_temp, &look_vec) > 0.9999 {
            up_temp = Vec3::new(up_temp.x, up_temp.y, up_temp.z + 0.1).normalize();
        }

        let proj = Mat4::new_perspective(self.ratio, self.perspective, self.near, self.far);
        let view = glm::look_at(&self.pos, &self.look, &up_temp);

        self.view = Some(view);
        self.projection = Some(proj);
    }
}
