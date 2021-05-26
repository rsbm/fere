use super::input_manager::{InputImage, KeyboardKey};
use super::keys;
use fere::prelude::*;

pub struct CameraControl {
    // configurations
    zoom_sen: f32,
    rotate_sen: f32,
    move_sen: f32,

    // internal states
    camera_radius: f32,
    camera_rotation: Vec2,

    // external state
    camera_pos: Vec3,
    camera_look: Vec3,
}

impl CameraControl {
    fn key(&mut self, k: KeyboardKey) {
        let mut d = match k {
            keys::W => Vec2::new(0.0, 1.0),
            keys::A => Vec2::new(-1.0, 0.0),
            keys::S => Vec2::new(0.0, -1.0),
            keys::D => Vec2::new(1.0, 0.0),
            _ => Vec2::new(0.0, 0.0),
        };

        let theta = self.camera_rotation.x - 0.5 * std::f32::consts::PI;
        d = Vec2::new(
            theta.cos() * d.x - theta.sin() * d.y,
            theta.sin() * d.x + theta.cos() * d.y,
        );
        d *= self.move_sen * self.camera_radius;
        self.camera_pos += Vec3::new(d.x, d.y, 0.0);
    }

    fn mouse(&mut self, im: &InputImage) {
        let zoom = if im.wheel_delta > 0.0 {
            -self.zoom_sen * im.wheel_delta * (self.camera_radius / (self.camera_radius + 2000.0))
        } else {
            -self.zoom_sen * im.wheel_delta * (self.camera_radius / (self.camera_radius + 1000.0))
        };

        if im.mouse_pressed.wheel {
            self.camera_rotation.x -= self.rotate_sen * im.mouse_pos_delta.x as f32;
            self.camera_rotation.y += self.rotate_sen * im.mouse_pos_delta.y as f32;
            self.camera_rotation.y = nalgebra::clamp(
                self.camera_rotation.y,
                -0.49 * std::f32::consts::PI,
                0.49 * std::f32::consts::PI,
            );
        }
        let ldir = Vec3::new(
            self.camera_rotation.x.cos() * self.camera_rotation.y.cos(),
            self.camera_rotation.x.sin() * self.camera_rotation.y.cos(),
            self.camera_rotation.y.sin(),
        )
        .normalize();

        self.camera_pos -= ldir * zoom;
        self.camera_radius = self.camera_pos.z;
        self.camera_look = self.camera_pos + ldir;
    }

    pub fn new(camera_pos: Vec3, camera_look: Vec3) -> Self {
        CameraControl {
            zoom_sen: 200.0,
            rotate_sen: 0.005,
            move_sen: 0.03,

            camera_rotation: Vec2::new(
                90.0 * std::f32::consts::PI / 180.0,
                -40.0 * std::f32::consts::PI / 180.0,
            ),
            camera_radius: 200.0,

            camera_pos,
            camera_look,
        }
    }

    pub fn update(&mut self, ii: &InputImage) {
        for k in &ii.key_pressed {
            self.key(*k);
        }
        self.mouse(ii);
    }

    pub fn get(&self) -> (Vec3, Vec3) {
        (self.camera_pos, self.camera_look)
    }
}
