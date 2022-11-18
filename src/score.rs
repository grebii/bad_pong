use std::time::Instant;

use glam::{ivec2, Vec3, vec3, UVec2, uvec2, Vec2};
use pixels::Pixels;

use super::{Draw};

const DIGIT_MAP: [[u8; 15]; 10] = [
    [
        1, 1, 1,
        1, 0, 1,
        1, 0, 1,
        1, 0, 1,
        1, 1, 1,
    ],
    [
        1, 1, 0,
        0, 1, 0,
        0, 1, 0,
        0, 1, 0,
        0, 1, 0,
    ],
    [
        1, 1, 1,
        0, 0, 1,
        1, 1, 1,
        1, 0, 0,
        1, 1, 1,
    ],
    [
        1, 1, 1,
        0, 0, 1,
        0, 1, 1,
        0, 0, 1,
        1, 1, 1,
    ],
    [
        1, 0, 1,
        1, 0, 1,
        1, 1, 1,
        0, 0, 1,
        0, 0, 1,
    ],
    [
        1, 1, 1,
        1, 0, 0,
        1, 1, 1,
        0, 0, 1,
        1, 1, 1,
    ],
    [
        1, 1, 1,
        1, 0, 0,
        1, 1, 1,
        1, 0, 1,
        1, 1, 1,
    ],
    [
        1, 1, 1,
        0, 0, 1,
        0, 0, 1,
        0, 0, 1,
        0, 0, 1,
    ],
    [
        1, 1, 1,
        1, 0, 1,
        1, 1, 1,
        1, 0, 1,
        1, 1, 1,
    ],
    [
        1, 1, 1,
        1, 0, 1,
        1, 1, 1,
        0, 0, 1,
        0, 0, 1,
    ],
];

const DIGIT_WIDTH: u32 = 3;
const DIGIT_HEIGHT: u32 = 5;

pub struct Score {
    pub score: u32,
    block_size: u32,
    position: UVec2,
    pub visible: bool,
    pub scored_time: Option<Instant>,
}

impl Score {
    pub fn new(score: u32, block_size: u32, position: UVec2) -> Self { 
        Self { score, block_size, position, visible: true, scored_time: None } 
    }

    pub fn score(&mut self) {
        self.scored_time = Some(Instant::now());
        self.score += 1;
    }
}

impl Draw for Score {
    fn draw(&self, brush: Vec2) -> Option<Vec3> {
        if !self.visible { return None; }

        let size = self.block_size as i32;
        let scaled_width = DIGIT_WIDTH as i32 * size;
        let scaled_height = DIGIT_HEIGHT as i32 * size;

        if (brush.y - self.position.y as f32) < 0.0 || (brush.y - self.position.y as f32) > scaled_height as f32 {
            return None;
        }

        let mut visible = false;
        let mut a = 0.0;

        for (i, digit) in self.score.to_string().chars().enumerate() {
            let offset = (scaled_width + 1) * i as i32;
            let local_pos = brush.as_ivec2() - (self.position.as_ivec2() + ivec2(offset, 0));

            if local_pos.x >= 0 && local_pos.x < scaled_width {
                let digit = digit.to_digit(10).unwrap();

                let index = (local_pos.y / size) * DIGIT_WIDTH as i32 + (local_pos.x / size);
                visible = DIGIT_MAP[digit as usize][index as usize] == 1;
                a = index as f32 / 15.0;
            }
        }

        if visible {
            // vec2(1.0, 1.0)
            // vec2(a, 0.0)
            Some(vec3(1.0, 1.0, 1.0))
        } else {
            None
        }
    }
}