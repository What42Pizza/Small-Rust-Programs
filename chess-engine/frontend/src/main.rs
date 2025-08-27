#![allow(unused)]
#![warn(unused_must_use)]

#![feature(duration_constants)]
#![feature(hash_set_entry)]
#![feature(array_windows)]



pub mod draw;
pub use draw::*;
pub mod data;
pub use data::*;
pub mod text;
pub use text::*;
pub mod utils;
pub use utils::*;
pub mod settings;
pub use settings::*;



pub use shared::*;
use std::{collections::HashMap, result::Result::{self as StdResult, Ok as StdOk, Err as StdErr}};
use ab_glyph::FontRef;
pub use sdl3::{render::Canvas, video::Window, event::Event, keyboard::Mod, render::{Texture, TextureCreator}, video::WindowContext};
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
	let window = video_subsystem
		.window("Chess Engine", 800, 600)
		.maximized()
		.position_centered()
		.resizable()
		.hidden()
		.build()?;
	let mut canvas = window.into_canvas();
	unsafe {
		sdl3::sys::render::SDL_SetRenderVSync(canvas.raw(), 1);
	}
	let mut event_pump = sdl_context.event_pump().unwrap();
	
	canvas.present();
	canvas.window_mut().show();
	
	let texture_creator = canvas.texture_creator();
	let mut text_cache = TextCache::default();
	let font = FontRef::try_from_slice(include_bytes!("resources/Inter_24pt-Regular.ttf"))?;
	
	let mut data = AppData {
		should_close: false,
		board: default_board(),
	};
	
	while !data.should_close {
		
		for event in event_pump.poll_iter() { handle_event(&mut data, event)?; }
		
		draw(&mut data, &mut canvas, &texture_creator, &mut text_cache, &font)?;
		
	}
	
	Ok(())
}



fn handle_event(data: &mut AppData, event: Event) -> Result<()> {
	match event {
		
		Event::Quit { timestamp: _ } => data.should_close = true,
		
		Event::KeyDown { timestamp: _, window_id: _, keycode, scancode: _, keymod, repeat: _, which: _, raw: _ } => {
			if keycode == Some(sdl3::keyboard::Keycode::W) && (keymod.contains(Mod::RCTRLMOD) || keymod.contains(Mod::LCTRLMOD)) {
				data.should_close = true;
			}
		}
		
		_ => {}
	}
	Ok(())
}
