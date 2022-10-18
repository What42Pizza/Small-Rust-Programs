// Started 10/15/22
// Last updated 10/18/22



// default rust
#![allow(unused)]
#![warn(unused_must_use)]

// clippy
#![allow(clippy::never_loop)]



extern crate derive_is_enum_variant;





const TIME_BETWEEN_MOVES: f64 = 0.1;
const STARTING_LENGTH: u32 = 4;
const FLASHING_TIME: f64 = 2.;
const FLASHING_SPEED: f64 = 0.75;

const ZOOM: u32 = 64;
const WINDOW_WIDTH: u32 = 448;
const WINDOW_HEIGHT: u32 = 960;
const GRID_WIDTH: u32 = WINDOW_WIDTH / ZOOM;
const GRID_HEIGHT: u32 = WINDOW_HEIGHT / ZOOM;

const GAME_SETTINGS: GameSettings = GameSettings {

    time_between_moves: TIME_BETWEEN_MOVES,
    starting_length: STARTING_LENGTH,
    flashing_time: FLASHING_TIME,
    flashing_speed: FLASHING_SPEED,

    zoom: ZOOM,
    window_width: WINDOW_WIDTH,
    window_height: WINDOW_HEIGHT,
    grid_width: GRID_WIDTH,
    grid_height: GRID_HEIGHT,
    
};



mod update;
mod render;
mod init;
mod structs;



use std::{time::{Instant, Duration}, error::Error};

use structs::*;



fn main() -> Result<(), Box<dyn Error>> {
    let mut last_update_instant = Instant::now();

    // sdl
    let (sdl_context, mut canvas) = init::init_sdl2(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event loop.");
    let texture_creator = canvas.texture_creator();

    let mut game_data = init::init_game_data(GAME_SETTINGS, &texture_creator)?;

    let mut frame_count = 0;
    let mut last_print_time = Instant::now();
    while !game_data.exit {

        let dt = last_update_instant.elapsed();
        last_update_instant = Instant::now();
        update::update(&mut game_data, &mut event_pump, &dt)?;

        render::render(&mut canvas, &mut game_data)?;

        frame_count += 1;
        let elapsed_time = last_print_time.elapsed().as_secs_f64();
        if elapsed_time > 1. {
            println!("FPS: {}", frame_count);
            last_print_time = last_print_time.checked_add(Duration::from_secs(1)).unwrap();
            frame_count = 0;
        }
    }

    Ok(())

}
