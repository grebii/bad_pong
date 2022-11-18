use glam::{ivec2, IVec2};

use super::{uvec2, vec2, UVec2, Vec2};

pub struct CollissionResult {
    pub overlap: bool,
    pub distance: Vec2,
    pub normalized: Vec2,
}

pub enum Side {
    Left,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    BottomLeft,
    Bottom,
    Center,
}

#[derive(Default, Clone)]
pub struct Rect {
    pub position: Vec2,
    pub size: UVec2,
}

impl Rect {
    pub fn new(position: Vec2, size: UVec2) -> Self {
        Self {
            position,
            size,
        }
    }

    pub fn at_side(position: Vec2, size: UVec2, side: Side) -> Self {
        let mut a = Self::new(Vec2::default(), size);
        a.set_pos_at(position, side);
        a
    }

    pub fn collide(&self, other: &Self) -> CollissionResult {
        let xc = collide_1(self.position.x, other.position.x, self.size.x, other.size.x);
        let yc = collide_1(self.position.y, other.position.y, self.size.y, other.size.y);

        CollissionResult {
            overlap: xc.2 && yc.2,
            distance: vec2(xc.0, yc.0),
            normalized: vec2(xc.1, yc.1),
        }
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        (pos.x >= self.position.x.floor() && pos.x < self.position.x.floor() + self.size.x as f32) &&
        (pos.y >= self.position.y.floor() && pos.y < self.position.y.floor() + self.size.y as f32)
    }

    pub fn get_pos_at(&self, side: Side) -> Vec2 {
        let half_size = self.size.as_vec2() * 0.5;
        match side {
            Side::Left => vec2(self.position.x, self.position.y + half_size.y.floor()),
            Side::TopLeft => vec2(self.position.x, self.position.y),
            Side::Top => vec2(self.position.x + half_size.x.floor(), self.position.y),
            Side::TopRight => vec2(self.position.x + self.size.x as f32, self.position.y),
            Side::Right => vec2(
                self.position.x + self.size.x as f32,
                self.position.y + half_size.y.floor(),
            ),
            Side::BottomRight => vec2(
                self.position.x + self.size.x as f32,
                self.position.y + self.size.y as f32,
            ),
            Side::BottomLeft => vec2(self.position.x, self.position.y + self.size.y as f32),
            Side::Bottom => self.position + vec2(half_size.x, self.size.y as f32),
            Side::Center => vec2(self.position.x + half_size.x, self.position.y + half_size.y),
        }
    }

    pub fn set_size(mut self, size: UVec2) -> Self {
        self.size = size;
        self
    }

    pub fn set_size_centered(mut self, size: UVec2) -> Self {
        unimplemented!()
    }

    pub fn set_pos_at(&mut self, pos: Vec2, side: Side) {
        let half_size = self.size.as_vec2() * 0.5;
        self.position = match side {
            Side::Left => pos - vec2(0.0, half_size.y),
            Side::Right => pos - vec2(self.size.x as f32, half_size.y),
            Side::Center => pos - half_size,
            Side::Bottom => pos - vec2(half_size.x, self.size.y as f32),
            Side::BottomLeft => pos - vec2(0.0, self.size.y as f32),
            _ => unimplemented!(),
        };
    }
}

fn collide_1(a_pos: f32, b_pos: f32, a_size: u32, b_size: u32) -> (f32, f32, bool) {
    let a_half_size = a_size as f32 * 0.5;
    let a_center = a_pos.floor() + a_half_size.floor();

    let b_half_size = b_size as f32 * 0.5;
    let b_center = b_pos.floor() + b_half_size.floor();

    let distance = a_center - b_center;
    let normalized = distance / (a_half_size + b_half_size);

    (distance, normalized, normalized.abs() < 1.0)
}
