#![feature(array_chunks)]

use std::error::Error;

use pixels::{SurfaceTexture, Pixels};
use winit::{event_loop::EventLoop, window::WindowBuilder, dpi::{LogicalSize, LogicalPosition}, event::{Event, WindowEvent}};
use winit_input_helper::WinitInputHelper;

use bad_pong::*;



fn main() -> Result<(), Box<dyn Error>>{
    // create the winit event loop
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    // make the window
    let window = WindowBuilder::new()
            .with_visible(false)
            .with_title("pongerino")
            .with_inner_size(LogicalSize::new(WIDTH * V_SCALE, HEIGHT * V_SCALE))
            .with_resizable(false)
            .build(&event_loop)?;

    // create the pixel buffer
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    window.set_visible(true);

    // create the game
    let mut game = PongGame::new();

    // event loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => control_flow.set_exit(),
            Event::RedrawRequested(_) => {
                // draw the game to the pixel buffer
                game.view(&mut pixels, WIDTH as f32, HEIGHT as f32);

                // render to the window
                if pixels.render().is_err() {
                    control_flow.set_exit();
                }
            },
            _ => {}
        }
        input.update(&event);

        game.update(&input);
        window.request_redraw();
    });
}