use glam::vec2;
use map_range::MapRange;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{score::Score, collision::{Rect, Side}, PongGame, HEIGHT, Ball, WIDTH};

pub enum Controller {
    Keyboard { up: VirtualKeyCode, down: VirtualKeyCode },
    Computer,
}

pub struct Player {
    pub score: Score,
    pub paddle: Paddle,
    pub controller: Controller,
}

impl Player {
    pub fn new(paddle: Paddle, controller: Controller, score: Score) -> Self { 
        Self { 
            score,
            paddle,
            controller
        } 
    }
    pub fn handle_input(&mut self, input: &WinitInputHelper, ball: &Ball, other_score: u32, dt: f32) {
        let mut vel = 0.0;

        match self.controller {
            Controller::Keyboard { up, down } => {
                if input.key_held(up) {
                    vel -= dt;
                }
                if input.key_held(down) {
                    vel += dt;
                }
                
                // move paddle dida and snap to grid if there's no input
                if vel != 0.0 {
                    self.paddle.moove(vel);
                } else {
                    self.paddle.rect.position.floor();
                }
            }
            Controller::Computer => {
                let offset = ball.rect.position.y - self.paddle.rect.position.y;
                let distance = (ball.rect.position.x - self.paddle.rect.position.x).map_range(0.0..WIDTH as f32, 0.0..0.5);
                let speed = ball.velocity.y * distance;
                let score = (other_score as f32).map_range(0.0..10.0, 0.5..1.0).min(1.0);
                let x = (offset + speed).signum() * score;
                self.paddle.moove(x * dt);
            }
        }
    }

    pub fn score(&mut self) {
        self.score.score();
        self.score.visible = false;
    }

    pub fn score_visibility_timer(&mut self) {
        if let Some(ref mut score_time) = self.score.scored_time {
            if score_time.elapsed().as_secs_f32() > 0.1 {
                self.score.scored_time = None;
                self.score.visible = true;
            }
        }
    }
}

pub struct Paddle {
    pub rect: Rect,
    pub speed: f32,
}

impl Paddle {
    pub fn new(rect: Rect) -> Self { 
        Self { 
            rect, 
            speed: 100.0
        } 
    }

    pub fn moove(&mut self, delta: f32) {
        let x = self.rect.position.x;
        self.rect.position.y += delta * self.speed;

        let mini = 2.0;
        if (self.rect.position.y + self.rect.size.y as f32) < mini {
            self.rect.position.y = -(self.rect.size.y as f32) + mini
        }

        if self.rect.position.y > (HEIGHT as f32) - mini {
            self.rect.position.y = HEIGHT as f32 - mini;
        }
    }
}