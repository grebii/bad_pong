use std::{cell::RefCell, time::Instant};

use crate::{Player, Ball};

use crate::pong::camera::Camera;
use crate::pong::time::Time;

pub struct PongGame {
    players: [Player; 2],
    ball: Ball,
    time: RefCell<Time>,
    camera: Camera
}

impl PongGame {

}




mod time {
    use std::{time::Instant};
    pub struct Time {
        last_update: Instant,
        last_frame: Instant,
        update_delta: f32,
        frame_delta: f32,
    }
    
    impl Time {
        fn update_delta(&mut self) -> f32 {
            let dt = self.last_update.elapsed().as_secs_f32();
            self.update_delta = dt;
            dt
        }
        fn frame_delta(&mut self) -> f32 {
            let dt = self.last_frame.elapsed().as_secs_f32();
            self.frame_delta = dt;
            dt
        }
    }
}

mod camera {
    pub struct Camera;
} 

mod player {}