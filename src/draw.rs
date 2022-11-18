use glam::vec3;

use crate::player::Paddle;

use super::{Score, Ball};

pub trait Draw {
    fn draw(&self, brush: glam::Vec2) -> Option<glam::Vec3>;
}

impl Draw for Paddle {
    fn draw(&self, brush: glam::Vec2) -> Option<glam::Vec3> {
        if self.rect.contains(brush) {
            Some(vec3(1.0, 1.0, 1.0))
        } else {
            None
        }
    }
}

impl Draw for Ball {
    fn draw(&self, brush: glam::Vec2) -> Option<glam::Vec3> {
        if !self.visible { return None; }
        if self.rect.contains(brush) {
            Some(vec3(1.0, 1.0, 1.0))
        } else {
            None
        }
    }
}