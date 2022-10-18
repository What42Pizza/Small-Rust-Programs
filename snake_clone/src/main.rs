// Started 10/13/22
// Last updated 10/14/22



// default rust
#![allow(unused)]
#![warn(unused_must_use)]

// clippy
#![allow(clippy::never_loop)]



#[macro_use]
extern crate derive_is_enum_variant;





const TIME_BETWEEN_MOVES: f64 = 0.3;

const ZOOM: u32 = 64;
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 704;
const MAP_WIDTH: u32 = WINDOW_WIDTH / ZOOM;
const MAP_HEIGHT: u32 = WINDOW_HEIGHT / ZOOM;



mod update;
mod render;
mod init;
mod structs;
mod fns;



use std::{time::Instant, error::Error};

use sdl2::rect::Point;



fn main() -> Result<(), Box<dyn Error>> {
    let mut last_update_instant = Instant::now();

    // sdl
    let (sdl_context, mut canvas) = init::init_sdl2 (WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event loop.");
    let texture_creator = canvas.texture_creator();

    let mut game_data = init::init_game_data(MAP_WIDTH, MAP_HEIGHT, ZOOM, TIME_BETWEEN_MOVES, &texture_creator)?;

    while !game_data.exit {

        let dt = last_update_instant.elapsed();
        last_update_instant = Instant::now();
        update::update(&mut game_data, &mut event_pump, &dt)?;

        render::render(&mut canvas, &mut game_data)?;

    }

    Ok(())

}
