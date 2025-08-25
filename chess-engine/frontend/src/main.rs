#![allow(unused)]
#![warn(unused_must_use)]

#![feature(duration_constants)]



pub mod draw;
pub use draw::*;
pub mod data;
pub use data::*;
pub mod utils;
pub use utils::*;
pub mod settings;
pub use settings::*;



use std::{ffi::{c_str, CStr}, time::{Duration, Instant}};

pub use shared::*;
pub use sdl3::{render::Canvas, video::Window, event::Event};
pub use anyhow::*;



fn main() {
	let result = main_result();
	if let Err(err) = result {
		show_fatal_error(err.to_string());
	}
}



fn main_result() -> Result<()> {
	
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let mut window = video_subsystem
		.window("Chess Engine", 800, 600)
		.maximized()
		.position_centered()
		.resizable()
		.hidden()
		.build()?;
	let mut canvas = window.into_canvas();
	unsafe {
		let result = sdl3::sys::render::SDL_SetRenderVSync(canvas.raw(), 1);
	}
	let mut event_pump = sdl_context.event_pump().unwrap();
	
	canvas.present();
	canvas.window_mut().show();
	
	let mut data = AppData {
		canvas,
		board: default_board(),
	};
	
	'main_loop: loop {
		
		for event in event_pump.poll_iter() { match event {
			Event::Quit { timestamp: _ } => break 'main_loop,
			_ => {}
		}}
		
		draw(&mut data)?;
		
	}
	
	Ok(())
}
