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
pub use utils::*;



pub use shared::*;
use std::thread;
pub use std::{collections::HashMap, result::Result::{self as StdResult, Ok as StdOk, Err as StdErr}, path::{Path, PathBuf}, time::{Instant, Duration, SystemTime}, sync::{LazyLock, Arc, Mutex}, fs::{self, File}, io::Read};
pub use sdl3::{render::{Canvas, FRect}, video::Window, event::Event, keyboard::Mod, render::{Texture, TextureCreator}, video::WindowContext, pixels::{Color, PixelFormat}, sys::pixels::SDL_PixelFormat, mouse::MouseState, EventPump};
pub use image::{EncodableLayout, ImageReader};
use rodio::{buffer::SamplesBuffer, Decoder, OutputStreamBuilder, Sink, Source, OutputStream};
pub use rayon::ThreadPool;
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
	
	engine::init();
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
	
	let audio_stream = OutputStreamBuilder::open_default_stream()?;
	let decoder = Decoder::new(File::open(resources_path.join("audio/ui pop.mp3"))?)?;
	let ui_pop_audio = SamplesBuffer::new(decoder.channels(), decoder.sample_rate(), decoder.collect::<Vec<_>>());
	
	let mut data = AppData {
		
		// basics
		settings,
		resources_path,
		should_close: false,
		last_update_time: Instant::now(),
		
		// audio
		audio_stream,
		ui_pop_audio,
		
		// window elements
		window_size: (0.0, 0.0),
		mouse_state: event_pump.mouse_state(),
		prev_mouse_state: event_pump.mouse_state(),
		new_game_button_rect: FRect::ZERO,
		new_game_button_down: false,
		
		// game data
		board: default_board(),
		game_flags: 0b00001111,
		state: State::NotPlaying,
		engine_move: Arc::new(Mutex::new(None)),
		ring_selectors: None,
		
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
				TurnState::PlayersTurn (_) => *player_time = player_time.saturating_sub(dt),
				TurnState::EnginesTurn => *engine_time = engine_time.saturating_sub(dt)
			}
			if player_time.is_zero() { data.state = State::GameEnded (GameEndedState::EngineWon); }
			else if engine_time.is_zero() { data.state = State::GameEnded (GameEndedState::PlayerWon); }
		}
	}
	
	// if playing
	if let State::Playing { time_remainings, time_per_move, turn } = &mut data.state {
		// if players turn
		if let TurnState::PlayersTurn (players_turn_state) = turn {
			// if letting go of piece
			if !data.mouse_state.left() && let PlayersTurnState::HoldingPiece { x: from_x, y: from_y, piece } = *players_turn_state {
				// if dropping onto board
				if let Some((to_x, to_y)) = get_slot_from_screen_pos(data.mouse_state.x(), data.mouse_state.y(), data.window_size) {
					let valid_move = get_white_moves(&data.board, piece, from_x, from_y, data.game_flags).find(|m| m.0 == to_x && m.1 == to_y);
					if let Some((_, _, move_type)) = valid_move {
						perform_move(&mut data.board, &mut data.game_flags, piece, from_x, from_y, to_x, to_y, move_type);
						if let Some((player_time, _engine_time)) = time_remainings && let Some(time_per_move) = time_per_move {
							*player_time += *time_per_move;
						}
						*turn = TurnState::EnginesTurn;
						let (board, game_flags, time_remaining) = (data.board, data.game_flags, time_remainings.map(|(_, v)| v.as_millis() as usize));
						let engine_move = data.engine_move.clone();
						data.ring_selectors = Some((from_x, from_y, to_x, to_y));
						rayon::spawn(move || {
							let new_engine_move = engine::get_move(board, game_flags, time_remaining, &THREAD_POOL);
							*engine_move.lock().unwrap() = Some(new_engine_move);
						});
						play_sound(&data, &data.ui_pop_audio);
					} else {
						set_piece(&mut data.board, from_x, from_y, piece);
						*players_turn_state = PlayersTurnState::NotHoldingPiece;
					}
				} else {
					set_piece(&mut data.board, from_x, from_y, piece);
					*players_turn_state = PlayersTurnState::NotHoldingPiece;
				}
			}
		}
	}
	
	// if waiting for engine
	if let State::Playing { turn, time_remainings, time_per_move } = &mut data.state {
		let mut engine_move = data.engine_move.lock().unwrap();
		if let Some((from_x, from_y, to_x, to_y, move_type)) = *engine_move {
			let piece = get_piece(&data.board, from_x, from_y);
			perform_move(&mut data.board, &mut data.game_flags, piece, from_x, from_y, to_x, to_y, move_type);
			*engine_move = None;
			*turn = TurnState::PlayersTurn (PlayersTurnState::NotHoldingPiece);
			data.ring_selectors = Some((from_x, from_y, to_x, to_y));
			if let (Some((_player_time, engine_time)), Some(time_per_move)) = (time_remainings, time_per_move) {
				*engine_time += *time_per_move;
			}
			play_sound(&data, &data.ui_pop_audio);
		}
	}
	
	// check for king captures
	if let State::Playing { turn, .. } = &data.state {
		let mut player_has_king = false;
		let mut engine_has_king = false;
		for x in 0..4 {
			for y in 0..8 {
				let x = x * 2;
				let (piece1, piece2) = get_doubled_pieces(&data.board, x, y);
				match piece1 {
					Piece::WhiteKing => player_has_king = true,
					Piece::BlackKing => engine_has_king = true,
					_ => {}
				}
				match piece2 {
					Piece::WhiteKing => player_has_king = true,
					Piece::BlackKing => engine_has_king = true,
					_ => {}
				}
			}
		}
		if let TurnState::PlayersTurn (PlayersTurnState::HoldingPiece { x, y, piece }) = turn {
			player_has_king |= *piece == Piece::WhiteKing;
		}
		if !player_has_king { data.state = State::GameEnded (GameEndedState::EngineWon); }
		if !engine_has_king { data.state = State::GameEnded (GameEndedState::PlayerWon); }
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

fn play_sound(data: &AppData, sound: &SamplesBuffer) {
	let sink = Sink::connect_new(data.audio_stream.mixer());
	sink.append(sound.clone());
	rayon::spawn(move || {
		while !sink.empty() {thread::sleep(Duration::SECOND);}
	});
}
