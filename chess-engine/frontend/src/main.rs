#![allow(unused)]
#![warn(unused_must_use)]

#![feature(duration_constants)]



pub mod events;
pub use events::*;
pub mod draw;
pub use draw::*;
pub mod data;
pub use data::*;
pub mod utils;
use sdl3::EventPump;
pub use utils::*;



pub use shared::*;
use std::fs;
pub use std::{collections::HashMap, result::Result::{self as StdResult, Ok as StdOk, Err as StdErr}, path::{Path, PathBuf}, time::{Instant, Duration, SystemTime}};
pub use sdl3::{render::{Canvas, FRect}, video::Window, event::Event, keyboard::Mod, render::{Texture, TextureCreator}, video::WindowContext, pixels::{Color, PixelFormat}, sys::pixels::SDL_PixelFormat, mouse::MouseState};
use image::{EncodableLayout, ImageReader};
pub use anyhow::*;
pub use easy_sdl3_text as sdl3_text;
pub use ab_glyph::FontVec;
pub use easy_configuration_format as ecf;



fn main() {
	let result = main_result();
	if let Err(err) = result {
		show_fatal_error(err.to_string());
	}
}



fn main_result() -> Result<()> {
	
	let resources_path = get_resources_path()?;
	let settings = load_settings(&resources_path.join("settings.txt"))?;
	
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
	let font_data = fs::read(resources_path.join(&settings.font_name))?;
	let font = FontVec::try_from_vec(font_data)?;
	let mut text_cache = sdl3_text::TextCache::new(font);
	let textures = load_textures(&resources_path, &texture_creator)?;
	
	let mut data = AppData {
		
		// basics
		settings,
		resources_path,
		should_close: false,
		last_update_time: Instant::now(),
		
		// window elements
		window_size: (0.0, 0.0),
		mouse_state: event_pump.mouse_state(),
		prev_mouse_state: event_pump.mouse_state(),
		new_game_button_rect: FRect::ZERO,
		new_game_button_down: false,
		
		// game data
		board: default_board(),
		state: State::NotPlaying,
		
	};
	update_window_elements(&mut data, &canvas, &event_pump)?;
	
	while !data.should_close {
		
		reload_settings_if_needed(&mut data, &mut text_cache)?;
		
		for event in event_pump.poll_iter() { handle_event(&mut data, event)?; }
		update(&mut data, &event_pump);
		
		update_window_elements(&mut data, &canvas, &event_pump)?;
		draw(&mut data, &mut canvas, &texture_creator, &mut text_cache, &textures)?;
		
	}
	
	Ok(())
}



pub fn update(data: &mut AppData, event_pump: &EventPump) {
	
	data.mouse_state = event_pump.mouse_state();
	
	let new_update_time = Instant::now();
	let dt = new_update_time.duration_since(data.last_update_time);
	data.last_update_time = new_update_time;
	
	// tick time remainings
	if let State::Playing { time_remainings, time_per_move, turn } = &mut data.state {
		if let Some((player_time, engine_time)) = time_remainings {
			match turn {
				TurnData::PlayersTurn (_) => *player_time = player_time.saturating_sub(dt),
				TurnData::EnginesTurn => *engine_time = engine_time.saturating_sub(dt)
			}
			if player_time.is_zero() { data.state = State::GameEnded (GameEndedState::EngineWon); }
			else if engine_time.is_zero() { data.state = State::GameEnded (GameEndedState::PlayerWon); }
		}
	}
	
	// let go of pieces
	if !data.mouse_state.left() && let State::Playing { turn: TurnData::PlayersTurn (players_turn_state), .. } = &mut data.state {
		if let PlayersTurnState::HoldingPiece { x, y, piece } = *players_turn_state {
			set_piece(&mut data.board, x, y, piece);
			*players_turn_state = PlayersTurnState::NotHoldingPiece;
		}
	}
	
}



pub fn update_window_elements(data: &mut AppData, canvas: &Canvas<Window>, event_pump: &EventPump) -> Result<()> {
	
	data.prev_mouse_state = data.mouse_state;
	let window_size = canvas.output_size()?;
	let (width, height) = (window_size.0 as f32, window_size.1 as f32);
	data.window_size = (width, height);
	
	let top_bar_rect = get_top_bar_rect(width, height);
	
	// new_game button
	let x = top_bar_rect.x + top_bar_rect.w * 0.01;
	let y = top_bar_rect.y + top_bar_rect.h * 0.15;
	let w = top_bar_rect.w * 0.14;
	let h = top_bar_rect.h * 0.7;
	data.new_game_button_rect = FRect::new(x, y, w, h);
	if !data.mouse_state.left() || !data.new_game_button_rect.contains(data.mouse_state.pos()) {data.new_game_button_down = false;}
	
	Ok(())
}



fn load_settings(settings_path: &Path) -> Result<AppSettings> {
	let meta = fs::metadata(settings_path)?;
	let last_modified_time = meta.modified()?;
	
	let settings = std::fs::read_to_string(settings_path)?;
	let (settings, did_update, errors) = ecf::File::from_str(settings, &[], &mut ());
	for error in errors {
		println!("Error while loading settings: {error}");
	}
	
	let total_time = settings.get_int("total time")? as u64;
	let time_per_move = settings.get_int("time per move")? as u64;
	
	let font_name = settings.get_str("font")?.to_string();
	
	let background_color = get_settings_color_rgb("background color", &settings)?;
	let top_bar_color = get_settings_color_rgb("top bar color", &settings)?;
	let top_bar_buttons_color = get_settings_color_rgb("top bar buttons color", &settings)?;
	let top_bar_buttons_darkened_color = get_settings_color_rgb("top bar buttons darkened color", &settings)?;
	let board_color_dark = get_settings_color_rgb("board color dark", &settings)?;
	let board_color_light = get_settings_color_rgb("board color light", &settings)?;
	let board_trim_color = get_settings_color_rgb("board trim color", &settings)?;
	
	Ok(AppSettings {
		
		last_modified_time,
		
		total_time,
		time_per_move,
		
		font_name,
		
		background_color,
		top_bar_color,
		top_bar_buttons_color,
		top_bar_buttons_darkened_color,
		board_color_dark,
		board_color_light,
		board_trim_color,
		
	})
}



fn get_resources_path() -> Result<PathBuf> {
	let mut path = std::env::current_exe()?;
	while !path.join("resources").exists() {
		let popped = path.pop();
		if !popped {return Err(Error::msg("Failed to find resources folder alongside executable or any of its parent directories"))}
	}
	Ok(path.join("resources"))
}

fn get_settings_color_rgb(color_name: impl Into<String>, settings: &ecf::File) -> Result<Color> {
	let color_name = color_name.into();
	let red   = settings.get_int(color_name.clone() + " red")?;
	let green = settings.get_int(color_name.clone() + " green")?;
	let blue  = settings.get_int(color_name.clone() + " blue")?;
	Ok(Color::RGB(red as u8, green as u8, blue as u8))
}

fn reload_settings_if_needed<'a>(data: &mut AppData, text_cache: &mut sdl3_text::TextCache<'a, FontVec>) -> Result<()> {
	
	let settings_path = data.resources_path.join("settings.txt");
	let meta = fs::metadata(&settings_path)?;
	let last_modified_time = meta.modified()?;
	if last_modified_time == data.settings.last_modified_time {return Ok(());}
	println!("Reloading settings...");
	
	let new_settings = load_settings(&settings_path)?;
	data.settings = new_settings;
	let font_data = fs::read(data.resources_path.join(&data.settings.font_name))?;
	let new_font = FontVec::try_from_vec(font_data)?;
	text_cache.switch_font(new_font);
	
	Ok(())
}
