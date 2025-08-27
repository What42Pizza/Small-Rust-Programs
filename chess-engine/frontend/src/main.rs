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



pub use shared::*;
pub use std::{collections::HashMap, result::Result::{self as StdResult, Ok as StdOk, Err as StdErr}};
pub use sdl3::{render::{Canvas, FRect}, video::Window, event::Event, keyboard::Mod, render::{Texture, TextureCreator}, video::WindowContext, pixels::Color};
pub use anyhow::*;
pub use easy_sdl3_text as sdl3_text;
pub use ab_glyph::FontRef;



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
	let font = FontRef::try_from_slice(include_bytes!("resources/Inter_24pt-Regular.ttf"))?;
	let mut text_cache = sdl3_text::TextCache::new(&font);
	
	let mut data = AppData {
		should_close: false,
		board: default_board(),
		debug_color: Color::RGB(0, 0, 0),
	};
	
	while !data.should_close {
		
		for event in event_pump.poll_iter() { handle_event(&mut data, event)?; }
		
		draw(&mut data, &mut canvas, &texture_creator, &mut text_cache)?;
		
		//println!("{:?}", data.debug_color);
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
			
			if keycode == Some(sdl3::keyboard::Keycode::Q) {data.debug_color.r += 1;}
			if keycode == Some(sdl3::keyboard::Keycode::W) {data.debug_color.g += 1;}
			if keycode == Some(sdl3::keyboard::Keycode::E) {data.debug_color.b += 1;}
			if keycode == Some(sdl3::keyboard::Keycode::A) {data.debug_color.r -= 1;}
			if keycode == Some(sdl3::keyboard::Keycode::S) {data.debug_color.g -= 1;}
			if keycode == Some(sdl3::keyboard::Keycode::D) {data.debug_color.b -= 1;}
			
		}
		
		_ => {}
	}
	Ok(())
}
