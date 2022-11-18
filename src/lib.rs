#![feature(array_chunks)]

// TODO: NEEDS CLEANUP

use glam::{uvec2, vec2, vec3, UVec2, Vec2, Vec3};
use pixels::{Pixels, SurfaceTexture};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

mod collision;
mod draw;
mod player;
mod score;

use collision::{Rect, Side};
use draw::Draw;
use player::{Controller, Paddle, Player};
use score::Score;

pub const WIDTH: u32 = 101;
pub const HEIGHT: u32 = 81;
pub const V_SCALE: u32 = 5;
const PADDLE_PADDING: u32 = 8;
const SCORE_PADDING: u32 = 20;
const BALL_SPEED: f32 = 60.0;
const BALL_SPEED_INCREASE: f32 = 5.0;

struct Ball {
    rect: Rect,
    velocity: Vec2,
    scored_time: Option<Instant>,
    visible: bool,
}
impl Ball {
    fn moove(&mut self, delta_time: f32, players: &mut [Player; 2], camera_shake: &mut CameraShake, canvas_clear: &mut bool) {

        if !self.visible { return; }

        // store magniture and direction of velocity for the future
        let magn = self.velocity.length();
        let dir = self.velocity / magn;

        // move the ball along velocity
        self.rect.position += self.velocity * delta_time;

        // check if we moved past the screen on the y axis
        if self.rect.position.y + self.rect.size.y as f32 >= HEIGHT as f32 {
            // snap it back if we do
            self.rect.position.y = (HEIGHT - self.rect.size.y) as f32;
            // also mirror velocity on the axis
            self.velocity.y *= -1.0;

            camera_shake.pos.y += 1.0 + (magn * 0.01);

        } else if self.rect.position.y < 0.0 {
            // snap it back if we do
            self.rect.position.y = 0.0;
            // also mirror velocity on the axis
            self.velocity.y *= -1.0;

            camera_shake.pos.y -= 1.0 + (magn * 0.01);
        }

        // TODO: simplify this to one function
        // check collission with player 1 paddle
        let collision = players[0].paddle.rect.collide(&self.rect);

        if collision.distance.x > 0.0 && collision.normalized.y.abs() < 1.0 {

            self.rect.position.x += collision.distance.x + 1.0;

            self.velocity.x *= -1.0;
            self.velocity.y = -collision.normalized.y * magn;

            self.velocity = self.velocity.normalize() * (magn + BALL_SPEED_INCREASE);

            camera_shake.pos.x -= 1.0 + (magn * 0.01);
        }
        // check collision with player 2 paddle
        let collision = players[1].paddle.rect.collide(&self.rect);

        if collision.distance.x < 0.0 && collision.normalized.y.abs() < 1.0 {

            self.rect.position.x += collision.distance.x - 1.0;

            self.velocity.x *= -1.0;
            self.velocity.y = -collision.normalized.y * magn;

            self.velocity = self.velocity.normalize() * (magn + BALL_SPEED_INCREASE);

            camera_shake.pos.x += 1.0 + (magn * 0.01);
        }


        // check if player one should score
        if self.rect.position.x > (WIDTH - PADDLE_PADDING) as f32 {
            // move ball back to center
            self.rect.set_pos_at(vec2(WIDTH as f32 * 0.5, HEIGHT as f32 * 0.5), Side::Center);

            // self.velocity = vec2(0.0, 0.0);
            self.scored_time = Some(Instant::now());
            self.visible = false;

            players[0].score();

            camera_shake.pos += self.velocity * 0.1;

            *canvas_clear = true;
        }

        // check if player two should score
        if self.rect.position.x < PADDLE_PADDING as f32 {
            // move ball back to center
            self.rect.set_pos_at(vec2(WIDTH as f32 * 0.5, HEIGHT as f32 * 0.5), Side::Center);

            // self.velocity = vec2(0.0, 0.0);
            self.scored_time = Some(Instant::now());
            self.visible = false;

            players[1].score();

            // camera_shake.pos.x += 10.0;
            camera_shake.pos += self.velocity * 0.1;

            *canvas_clear = true;
        }

        // TODO: repeat less code please
    }

    fn respawn_timer(&mut self) {
        if let Some(scored_time) = self.scored_time {
            if scored_time.elapsed().as_secs_f32() > 0.4 {
                self.scored_time = None;
                self.visible = true;

                let mut rng = thread_rng();

                let dir = vec2(rng.gen_range(-2.0..2.0), rng.gen_range(-1.0..1.0)).normalize();

                self.velocity = self.velocity.normalize() * BALL_SPEED;
                self.rect.position.y = rng.gen_range(0.0..(HEIGHT - self.rect.size.y) as f32);
            }
        }
    }
}

#[derive(Default)]
struct CameraShake {
    pos: Vec2,
    speed: f32,
}

impl CameraShake {
    fn update(&mut self, delta_seconds: f32) {
        self.pos *= (0.9 - (delta_seconds * 2.0));
    }
}

pub struct PongGame {
    players: [Player; 2],
    ball: Ball,
    last_update: Instant,
    delta_time: f32,
    last_frame: Instant,
    frame_delay: f32,
    camera_shake: CameraShake,
    clear_canvas: bool,
}

impl Default for PongGame {
    fn default() -> Self {
        Self::new()
    }
}
impl PongGame {
    pub fn new() -> Self {
        Self {
            players: [
                Player::new(
                    Paddle::new(Rect::at_side(vec2(PADDLE_PADDING as f32, HEIGHT as f32 * 0.5), uvec2(2, 9), Side::Left)),
                    Controller::Keyboard {
                        up: VirtualKeyCode::W,
                        down: VirtualKeyCode::S,
                    },
                    Score::new(0, 2, uvec2(SCORE_PADDING, 2))
                ),
                Player::new(
                    Paddle::new(Rect::at_side(vec2((WIDTH - PADDLE_PADDING) as f32, HEIGHT as f32 * 0.5), uvec2(2, 9), Side::Right)),
                    Controller::Keyboard {
                        up: VirtualKeyCode::Up,
                        down: VirtualKeyCode::Down,
                    },
                    Score::new(0, 2, uvec2(WIDTH - SCORE_PADDING - 6, 2))
                ),
            ],
            ball: Ball {
                rect: Rect::at_side(vec2(WIDTH as f32 * 0.5, HEIGHT as f32 * 0.5), uvec2(2, 2), Side::Center),
                velocity: vec2(BALL_SPEED * 0.5, BALL_SPEED * 0.5),
                scored_time: Some(Instant::now()),
                visible: false,
            },
            last_update: Instant::now(),
            delta_time: 0.0,
            last_frame: Instant::now(),
            frame_delay: 0.02,
            camera_shake: CameraShake::default(),
            clear_canvas: true,
        }
    }

    // ---------------------------------
    // INPUT UPDATE DAIUWHD

    pub fn update(&mut self, input: &WinitInputHelper) {
        self.delta_time = self.last_update.elapsed().as_secs_f32();
        self.last_update = Instant::now();

        // input
        self.players[0].handle_input(input, self.delta_time);
        self.players[1].handle_input(input, self.delta_time);

        // move ball
        self.ball.moove(self.delta_time, &mut self.players, &mut self.camera_shake, &mut self.clear_canvas);

        // score timer
        self.players[0].score_visibility_timer();
        self.players[1].score_visibility_timer();

        // respawn ball timer
        self.ball.respawn_timer();
    }

    pub fn view(&mut self, pixels: &mut Pixels, width: f32, height: f32) {

        let frame_delta = self.last_frame.elapsed().as_secs_f32();

        // simulate slow computer
        if frame_delta >= self.frame_delay {
            self.last_frame = Instant::now();
        } else {
            return;
        }

        // clear the canvas
        if self.clear_canvas {
            for cell in pixels.get_frame_mut().array_chunks_mut() {
                *cell =  [(cell[0] as f32 * 0.05) as u8, (cell[1] as f32 * 0.2) as u8, (cell[2] as f32 * 0.1) as u8, 0];
            }
            self.clear_canvas = false;
        }

        for (i, cell) in pixels.get_frame_mut().array_chunks_mut().enumerate() {
            let x = (i as f32 % width) + self.camera_shake.pos.x.round();
            let y = (i as f32 / width) + self.camera_shake.pos.y.round();
            let _uv = vec2(x / width, y / height);

            let mut col = vec3(
                cell[0] as f32 / 255.0,
                cell[1] as f32 / 255.0,
                cell[2] as f32 / 255.0,
            );

            // fancy ghost effect
            col -= vec3(col.x.powf(6.0), col.y.powf(2.5), col.y.powf(4.0)) * vec3(0.1, 0.8, 0.7);
            // col -= vec3(col.x.powf(2.0), col.y.powf(2.0), col.y.powf(2.0)) * vec3(0.1, 0.8, 0.7);

            // draw player 1 paddle
            if let Some(c) = self.players[0].paddle.draw(vec2(x, y)) {
                col = c;
            }
            // draw player 2 paddle
            if let Some(c) = self.players[1].paddle.draw(vec2(x, y)) {
                col = c;
            }
            // draw player 1 score
            if let Some(c) = self.players[0].score.draw(vec2(x, y)) {
                col = c;
            }
            // draw player 2 score
            if let Some(c) = self.players[1].score.draw(vec2(x, y)) {
                col = c;
            }
            // draw ball
            if let Some(c) = self.ball.draw(vec2(x, y)) {
                col = c;
            }
            // draw line in the center
            if x as u32 == WIDTH / 2 {
                if y as u32 % 4 == 2 {
                    col = vec3(1.0, 1.0, 1.0);
                }
            }

            // convert and apply color to buffer
            *cell = [
                (col.x * 255.0) as u8,
                (col.y * 255.0) as u8,
                (col.z * 255.0) as u8,
                255,
            ]
        }
        // update camera shake after drawing the frame
        // this looks nicer 
        self.camera_shake.update(frame_delta);
    }
}
