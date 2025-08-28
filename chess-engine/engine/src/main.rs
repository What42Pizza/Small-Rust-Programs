// !!!!!!!!!! NOTE: THIS FILE IS OLD AND ONLY USED FOR DEBUG WHILE THE FRONTEND IS BEING MADE !!!!!!!!!! //



#![allow(unused)]
#![warn(unused_must_use)]

#![feature(coroutines)]
#![feature(coroutine_trait)]

use colored::{Color, Colorize};
use crossterm::{cursor, execute, terminal};
use shared::*;
pub use std::{time::Instant, io::{Write, Stdout}, thread, time::Duration};
pub use smart_read::prelude::*;



pub mod engine;
//pub mod utils;
//pub use utils::*;

//use crate::*;
use std::{ops::{Coroutine, CoroutineState}, pin::Pin};

const BACKGROUND_COLOR_1: Color = Color::TrueColor { r: 95, g: 95, b: 95 };
const BACKGROUND_COLOR_2: Color = Color::TrueColor { r: 159, g: 159, b: 159 };
const BLACK_COLOR: Color = Color::TrueColor { r: 0, g: 0, b: 0 };
const WHITE_COLOR: Color = Color::TrueColor { r: 255, g: 255, b: 255 };



#[macro_export]
macro_rules! output {
	($stdout:expr, $line:expr $(, $format:expr)+) => {
		execute!($stdout, crossterm::cursor::MoveTo(0, $line), crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine))?;
		write!($stdout $(, $format)+)?;
		$stdout.flush()?;
	};
}

pub fn time_to_string(time: usize) -> String {
	let time = time / 1000;
	let minutes = time / 60;
	let seconds = time % 60;
	if minutes == 0 {
		format!("0:{seconds:02}")
	} else {
		format!("{minutes}:{seconds:02}")
	}
}

pub fn pluralize(value: usize, singular: &str, plural: &str) -> String {
	if value == 1 {format!("{value} {singular}")} else {format!("{value} {plural}")}
}



pub struct CoroutineIter<G> (pub G);

impl<G, Y, R> Iterator for CoroutineIter<G>
where
	G: Coroutine<Yield = Y, Return = R> + Unpin,
{
	type Item = Y;
	
	fn next(&mut self) -> Option<Self::Item> {
		match Pin::new(&mut self.0).resume(()) {
			CoroutineState::Yielded(val) => Some(val),
			CoroutineState::Complete(_) => None,
		}
	}
}



trait PieceFns {
	fn print_self(&self, color: Color, out: &mut Stdout) -> Result<(), Box<dyn std::error::Error>>;
}

impl PieceFns for Piece {
	fn print_self(&self, background_color: Color, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
		match self {
			Self::None => write!(stdout, "{}", "  ".on_color(background_color))?,
			Self::BlackPawn    => write!(stdout, "{}", "Pw".color(BLACK_COLOR).on_color(background_color))?,
			Self::BlackKnight  => write!(stdout, "{}", "Kn".color(BLACK_COLOR).on_color(background_color))?,
			Self::BlackBishop  => write!(stdout, "{}", "Bp".color(BLACK_COLOR).on_color(background_color))?,
			Self::BlackRook    => write!(stdout, "{}", "Rk".color(BLACK_COLOR).on_color(background_color))?,
			Self::BlackQueen   => write!(stdout, "{}", "Qn".color(BLACK_COLOR).on_color(background_color))?,
			Self::BlackKing    => write!(stdout, "{}", "Ki".color(BLACK_COLOR).on_color(background_color))?,
			Self::WhitePawn   => write!(stdout, "{}", "Pw".color(WHITE_COLOR).on_color(background_color))?,
			Self::WhiteKnight => write!(stdout, "{}", "Kn".color(WHITE_COLOR).on_color(background_color))?,
			Self::WhiteBishop => write!(stdout, "{}", "Bp".color(WHITE_COLOR).on_color(background_color))?,
			Self::WhiteRook   => write!(stdout, "{}", "Rk".color(WHITE_COLOR).on_color(background_color))?,
			Self::WhiteQueen  => write!(stdout, "{}", "Qn".color(WHITE_COLOR).on_color(background_color))?,
			Self::WhiteKing   => write!(stdout, "{}", "Ki".color(WHITE_COLOR).on_color(background_color))?,
		}
		Ok(())
	}
}

pub fn print_board(board: &Board, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
	execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
	stdout.flush()?;
	print_board_here(board, stdout)
}
	
pub fn print_board_here(board: &Board, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
	write!(stdout, "  ")?;
	for x in 0..32 {
		let background_color = if x % 8 < 4 {BACKGROUND_COLOR_1} else {BACKGROUND_COLOR_2};
		write!(stdout, "{}", "\u{2584}".color(background_color))?;
	}
	write!(stdout, "\n8 ")?;
	for x in 0..8 {
		let background_color = if x % 2 == 0 {BACKGROUND_COLOR_1} else {BACKGROUND_COLOR_2};
		write!(stdout, "{}", " ".on_color(background_color))?;
		get_piece(board, x, 7, 0).print_self(background_color, stdout)?;
		write!(stdout, "{}", " ".on_color(background_color))?;
	}
	write!(stdout, "\n  ")?;
	for x in 0..32 {
		let text = if x % 8 < 4 {"\u{2580}"} else {"\u{2584}"};
		write!(stdout, "{}", text.color(BACKGROUND_COLOR_1).on_color(BACKGROUND_COLOR_2))?;
	}
	write!(stdout, "\n7 ")?;
	for x in 0..8 {
		let background_color = if x % 2 == 0 {BACKGROUND_COLOR_2} else {BACKGROUND_COLOR_1};
		write!(stdout, "{}", " ".on_color(background_color))?;
		get_piece(board, x, 6, 1).print_self(background_color, stdout)?;
		write!(stdout, "{}", " ".on_color(background_color))?;
	}
	write!(stdout, "\n  ")?;
	for x in 0..32 {
		let text = if x % 8 < 4 {"\u{2584}"} else {"\u{2580}"};
		write!(stdout, "{}", text.color(BACKGROUND_COLOR_1).on_color(BACKGROUND_COLOR_2))?;
	}
	write!(stdout, "\n6 ")?;
	for x in 0..8 {
		let background_color = if x % 2 == 0 {BACKGROUND_COLOR_1} else {BACKGROUND_COLOR_2};
		write!(stdout, "{}", " ".on_color(background_color))?;
		get_piece(board, x, 5, 2).print_self(background_color, stdout)?;
		write!(stdout, "{}", " ".on_color(background_color))?;
	}
	write!(stdout, "\n  ")?;
	for x in 0..32 {
		let text = if x % 8 < 4 {"\u{2580}"} else {"\u{2584}"};
		write!(stdout, "{}", text.color(BACKGROUND_COLOR_1).on_color(BACKGROUND_COLOR_2))?;
	}
	write!(stdout, "\n5 ")?;
	for x in 0..8 {
		let background_color = if x % 2 == 0 {BACKGROUND_COLOR_2} else {BACKGROUND_COLOR_1};
		write!(stdout, "{}", " ".on_color(background_color))?;
		get_piece(board, x, 4, 3).print_self(background_color, stdout)?;
		write!(stdout, "{}", " ".on_color(background_color))?;
	}
	write!(stdout, "\n  ")?;
	for x in 0..32 {
		let text = if x % 8 < 4 {"\u{2584}"} else {"\u{2580}"};
		write!(stdout, "{}", text.color(BACKGROUND_COLOR_1).on_color(BACKGROUND_COLOR_2))?;
	}
	write!(stdout, "\n4 ")?;
	for x in 0..8 {
		let background_color = if x % 2 == 0 {BACKGROUND_COLOR_1} else {BACKGROUND_COLOR_2};
		write!(stdout, "{}", " ".on_color(background_color))?;
		get_piece(board, x, 3, 4).print_self(background_color, stdout)?;
		write!(stdout, "{}", " ".on_color(background_color))?;
	}
	write!(stdout, "\n  ")?;
	for x in 0..32 {
		let text = if x % 8 < 4 {"\u{2580}"} else {"\u{2584}"};
		write!(stdout, "{}", text.color(BACKGROUND_COLOR_1).on_color(BACKGROUND_COLOR_2))?;
	}
	write!(stdout, "\n3 ")?;
	for x in 0..8 {
		let background_color = if x % 2 == 0 {BACKGROUND_COLOR_2} else {BACKGROUND_COLOR_1};
		write!(stdout, "{}", " ".on_color(background_color))?;
		get_piece(board, x, 2, 5).print_self(background_color, stdout)?;
		write!(stdout, "{}", " ".on_color(background_color))?;
	}
	write!(stdout, "\n  ")?;
	for x in 0..32 {
		let text = if x % 8 < 4 {"\u{2584}"} else {"\u{2580}"};
		write!(stdout, "{}", text.color(BACKGROUND_COLOR_1).on_color(BACKGROUND_COLOR_2))?;
	}
	write!(stdout, "\n2 ")?;
	for x in 0..8 {
		let background_color = if x % 2 == 0 {BACKGROUND_COLOR_1} else {BACKGROUND_COLOR_2};
		write!(stdout, "{}", " ".on_color(background_color))?;
		get_piece(board, x, 1, 6).print_self(background_color, stdout)?;
		write!(stdout, "{}", " ".on_color(background_color))?;
	}
	write!(stdout, "\n  ")?;
	for x in 0..32 {
		let text = if x % 8 < 4 {"\u{2580}"} else {"\u{2584}"};
		write!(stdout, "{}", text.color(BACKGROUND_COLOR_1).on_color(BACKGROUND_COLOR_2))?;
	}
	write!(stdout, "\n1 ")?;
	for x in 0..8 {
		let background_color = if x % 2 == 0 {BACKGROUND_COLOR_2} else {BACKGROUND_COLOR_1};
		write!(stdout, "{}", " ".on_color(background_color))?;
		get_piece(board, x, 0, 7).print_self(background_color, stdout)?;
		write!(stdout, "{}", " ".on_color(background_color))?;
	}
	write!(stdout, "\n  ")?;
	for x in 0..32 {
		let background_color = if x % 8 < 4 {BACKGROUND_COLOR_2} else {BACKGROUND_COLOR_1};
		write!(stdout, "{}", "\u{2580}".color(background_color))?;
	}
	write!(stdout, "\n   a   b   c   d   e   f   g   h\n")?;
	stdout.flush()?;
	Ok(())
}





fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();
	
	engine::init();
	
	// pre-game input
	let starting_time = prompt!("Starting time: (set as 0 to disable) "; ["5m"]);
	let Some(starting_time_end) = starting_time.chars().last() else {unreachable!();};
	let mult = match starting_time_end {
		'm' => 1000 * 60,
		's' => 1000,
		_ => panic!("Unknown starting time units, please end the text with either 'm' (for minutes) or 's' (for seconds)"),
	};
	let Some(starting_time) = starting_time.strip_suffix(starting_time_end) else {unreachable!();};
	let starting_time = match starting_time.parse::<usize>() {
		Ok(v) => v * mult,
		Err(err) => panic!("Failed to parse starting time: {err}"),
	};
	
	let bonus_time = if starting_time > 0 {
		let bonus_time = prompt!("Time per move: "; ["5s"]);
		let Some(bonus_time_end) = bonus_time.chars().last() else {panic!("Cannot give empty time-per-move input");};
		let mult = match bonus_time_end {
			'm' => 1000 * 60,
			's' => 1000,
			_ => panic!("Unknown time-per-move units, please end the text with either 'm' (for minutes) or 's' (for seconds)"),
		};
		let Some(bonus_time) = bonus_time.strip_suffix(bonus_time_end) else {unreachable!();};
		let bonus_time = match bonus_time.parse::<usize>() {
			Ok(v) => v,
			Err(err) => panic!("Failed to parse time-per-move: {err}"),
		};
		bonus_time * mult
	} else {0};
	
	execute!(stdout, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::FromCursorDown))?;
	
	// game data
	let mut board = default_board();
	let mut player_won = false;
	let mut time_data = if starting_time > 0 {Some((starting_time, starting_time, bonus_time))} else {None};
	let prev_board_state = board;
	let mut game_flags = 0b00001111; // flags: 0: can castle with player left rook, 1: can castle with player right rook, 2: can castle with engine left rook, 3: can castle with engine right rook, 4: can en passant, 5-7: en passant file
	
	//print_board(&board, &mut stdout)?;
	//println!("{}", engine::get_board_score(&board));
	//panic!();
	
	let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(16).use_current_thread().build().unwrap();
	
	// main game loop
	'game_loop: loop {
		
		// display game
		print_board(&board, &mut stdout)?;
		if let Some((player_time, engine_time, _bonus_time)) = time_data {
			output!(stdout, 18, "Your time: {}", time_to_string(player_time));
			output!(stdout, 19, "Engine's time: {}", time_to_string(engine_time));
		}
		
		// process player move
		'player_move: loop {
			
			// get move
			output!(stdout, 20, "Enter your move: ");
			let start = Instant::now();
			let player_move = read!();
			if let Some((player_time, _engine_time, bonus_time)) = &mut time_data {
				let time_taken = start.elapsed().as_millis() as usize;
				if *player_time < time_taken {
					output!(stdout, 21, "You have lost on time");
					player_won = false;
					break 'game_loop;
				}
				*player_time -= time_taken;
				*player_time += *bonus_time + 2000;
			}
			if let Some((player_time, engine_time, _bonus_time)) = time_data {
				output!(stdout, 18, "Your time: {}", time_to_string(player_time));
				output!(stdout, 19, "Engine's time: {}", time_to_string(engine_time));
			}
			
			// process input
			let player_move_chars = player_move.chars().collect::<Vec<_>>();
			if player_move_chars.len() != 4 {output!(stdout, 21, "Invalid input, must always be four characters (ex: \"e2e4\")"); continue 'player_move;}
			let from_file = (player_move_chars[0] as u8).wrapping_sub(97);
			if from_file > 7 {output!(stdout, 21, "Invalid input, starting file must be 'a' through 'h'"); continue 'player_move;}
			let from_rank = (player_move_chars[1] as u8).wrapping_sub(49);
			if from_rank > 7 {output!(stdout, 21, "Invalid input, starting rank must be '1' through '8'"); continue 'player_move;}
			let to_file = (player_move_chars[2] as u8).wrapping_sub(97);
			if to_file > 7 {output!(stdout, 21, "Invalid input, ending file must be 'a' through 'h'"); continue 'player_move;}
			let to_rank = (player_move_chars[3] as u8).wrapping_sub(49);
			if to_rank > 7 {output!(stdout, 21, "Invalid input, ending rank must be '1' through '8'"); continue 'player_move;}
			if to_file == from_file && to_rank == from_rank {output!(stdout, 21, "Invalid input, cannot move piece to itself"); continue 'player_move;}
			let from_piece = get_piece(&board, from_file, from_rank, 68);
			if !from_piece.is_white() {output!(stdout, 21, "Invalid input, you do not have a piece at {}{}", player_move_chars[0], player_move_chars[1]); continue 'player_move;}
			let to_piece = get_piece(&board, to_file, to_rank, 69);
			if to_piece.is_white() {output!(stdout, 21, "Invalid input, you already have a piece at {}{}", player_move_chars[2], player_move_chars[3]); continue 'player_move;}
			
			//let is_slot_under_attack = |x: u8, y: u8| {
			//	// pawns
			//	if (x >= 1 && y <= 6 && get_piece(&board, x - 1, y + 1, 70) == Piece::BlackPawn)
			//	|| (x <= 6 && y <= 6 && get_piece(&board, x + 1, y + 1, 71) == Piece::BlackPawn)
			//	{return true;}
			//	// knights
			//	if (x >= 1 && y <= 5 && get_piece(&board, x - 1, y + 2, 72) == Piece::BlackKnight)
			//	|| (x <= 6 && y <= 5 && get_piece(&board, x + 1, y + 2, 73) == Piece::BlackKnight)
			//	|| (x <= 5 && y <= 6 && get_piece(&board, x + 2, y + 1, 74) == Piece::BlackKnight)
			//	|| (x <= 5 && y >= 1 && get_piece(&board, x + 2, y - 1, 75) == Piece::BlackKnight)
			//	|| (x <= 6 && y >= 2 && get_piece(&board, x + 1, y - 2, 76) == Piece::BlackKnight)
			//	|| (x >= 1 && y >= 2 && get_piece(&board, x - 1, y - 2, 77) == Piece::BlackKnight)
			//	|| (x >= 2 && y >= 1 && get_piece(&board, x - 2, y - 1, 78) == Piece::BlackKnight)
			//	|| (x >= 2 && y <= 6 && get_piece(&board, x - 2, y + 1, 79) == Piece::BlackKnight)
			//	{return true;}
			//	// bishops (and queens)
			//	let check_bishops_and_queens = |x_dir: u8, y_dir: u8| {
			//		let mut curr_x = x;
			//		let mut curr_y = y;
			//		while curr_x > 0 && curr_x < 7 && curr_y > 0 && curr_y < 7 {
			//			curr_x = curr_x.wrapping_add(x_dir);
			//			curr_y = curr_y.wrapping_add(y_dir);
			//			let piece = get_piece(&board, curr_x, curr_y, 80);
			//			if piece == Piece::BlackBishop || piece == Piece::BlackQueen {return true;}
			//			if piece.is_other() {break;}
			//		}
			//		false
			//	};
			//	if check_bishops_and_queens(1, 1) {return true;}
			//	if check_bishops_and_queens(1, 255) {return true;}
			//	if check_bishops_and_queens(255, 1) {return true;}
			//	if check_bishops_and_queens(255, 255) {return true;}
			//	// rooks (and queens)
			//	let check_rooks_and_queens = |x_dir: u8, y_dir: u8| {
			//		let mut curr_x = x;
			//		let mut curr_y = y;
			//		while curr_x > 0 && curr_x < 7 && curr_y > 0 && curr_y < 7 {
			//			curr_x = curr_x.wrapping_add(x_dir);
			//			curr_y = curr_y.wrapping_add(y_dir);
			//			let piece = get_piece(&board, curr_x, curr_y, 81);
			//			if piece == Piece::BlackRook || piece == Piece::BlackQueen {return true;}
			//			if piece.is_other() {break;}
			//		}
			//		false
			//	};
			//	if check_rooks_and_queens(0, 1) {return true;}
			//	if check_rooks_and_queens(1, 0) {return true;}
			//	if check_rooks_and_queens(0, 255) {return true;}
			//	if check_rooks_and_queens(255, 0) {return true;}
			//	// kings
			//	let x_min = x.max(1) - 1;
			//	let y_min = y.max(1) - 1;
			//	let x_max = x.min(6) + 1;
			//	let y_max = y.min(6) + 1;
			//	for x in x_min..=x_max {
			//		for y in y_min..=y_max {
			//			if get_piece(&board, x, y, 82) == Piece::BlackKing {return true;}
			//		}
			//	}
			//	false
			//};
			
			// check if move is valid
			let is_valid = 'is_valid: {match from_piece {
				Piece::WhitePawn => {
					if to_file == from_file && to_rank == from_rank + 1 && to_piece == Piece::None {break 'is_valid true;}; // move forward
					if to_file == from_file && from_rank == 1 && to_rank == 3 && get_piece(&board, from_file, 2, 83) == Piece::None && to_piece == Piece::None {break 'is_valid true;}; // starting move two forward
					if to_file.abs_diff(from_file) == 1 && to_rank == from_rank + 1 && to_piece.is_black() {break 'is_valid true;}; // capture
					if to_file.abs_diff(from_file) == 1 && to_rank == 6 && get_piece(&board, to_file, 5, 84) == Piece::BlackPawn && get_piece(&prev_board_state, to_file, 5, 85) == Piece::None {break 'is_valid true;}; // en passant
					false
				}
				Piece::WhiteKnight => {
					let len1 = to_file.abs_diff(from_file);
					let len2 = to_rank.abs_diff(from_rank);
					(len1 == 1 && len2 == 2) || (len2 == 1 && len1 == 2)
				}
				Piece::WhiteBishop => {
					let x_len = to_file.abs_diff(from_file);
					let y_len = to_rank.abs_diff(from_rank);
					if x_len != y_len {break 'is_valid false;}
					let x_dir = if to_file > from_file {1} else {255};
					let y_dir = if to_rank > from_rank {1} else {255};
					let mut x = from_file;
					let mut y = from_rank;
					for _ in 1..x_len {
						x = x.wrapping_add(x_dir);
						y = y.wrapping_add(y_dir);
						if get_piece(&board, x, y, 86) != Piece::None {break 'is_valid false;}
					}
					true
				}
				Piece::WhiteRook => {
					let x_len = to_file.abs_diff(from_file);
					let y_len = to_rank.abs_diff(from_rank);
					let can_move = if x_len == 0 {
						let y_dir = if to_rank > from_rank {1} else {255};
						let mut y = from_rank;
						for _ in 1..y_len {
							y = y.wrapping_add(y_dir);
							if get_piece(&board, from_file, y, 87) != Piece::None {break 'is_valid false;}
						}
						true
					} else if y_len == 0 {
						let x_dir = if to_file > from_file {1} else {255};
						let mut x = from_file;
						for _ in 1..x_len {
							x = x.wrapping_add(x_dir);
							if get_piece(&board, x, from_rank, 88) != Piece::None {break 'is_valid false;}
						}
						true
					} else {
						false
					};
					if can_move && from_rank == 0 {
						if from_rank == 0 {
							game_flags &= 0b11111110;
						} else if from_rank == 7 {
							game_flags &= 0b11111101;
						}
					}
					can_move
				}
				Piece::WhiteQueen => {
					let x_len = to_file.abs_diff(from_file);
					let y_len = to_rank.abs_diff(from_rank);
					if x_len == 0 {
						let y_dir = if to_rank > from_rank {1} else {255};
						let mut y = from_rank;
						for _ in 1..y_len {
							y = y.wrapping_add(y_dir);
							if get_piece(&board, from_file, y, 89) != Piece::None {break 'is_valid false;}
						}
						true
					} else if y_len == 0 {
						let x_dir = if to_file > from_file {1} else {255};
						let mut x = from_file;
						for _ in 1..x_len {
							x = x.wrapping_add(x_dir);
							if get_piece(&board, x, from_rank, 90) != Piece::None {break 'is_valid false;}
						}
						true
					} else if x_len == y_len {
						let x_dir = if to_file > from_file {1} else {255};
						let y_dir = if to_rank > from_rank {1} else {255};
						let mut x = from_file;
						let mut y = from_rank;
						for _ in 1..x_len {
							x = x.wrapping_add(x_dir);
							y = y.wrapping_add(y_dir);
							if get_piece(&board, x, y, 91) != Piece::None {break 'is_valid false;}
						}
						true
					} else {
						false
					}
				}
				Piece::WhiteKing => {
					if from_file == 4 && from_rank == 0 && to_file == 2 && to_rank == 0 && (game_flags & 0b00000001) > 0 {
						set_piece(&mut board, 0, 0, Piece::None);
						set_piece(&mut board, 3, 0, Piece::WhiteRook);
						game_flags &= 0b11111100;
						true
					} else if from_file == 4 && from_rank == 0 && to_file == 6 && to_rank == 0 && (game_flags & 0b00000010) > 0 {
						set_piece(&mut board, 7, 0, Piece::None);
						set_piece(&mut board, 5, 0, Piece::WhiteRook);
						game_flags &= 0b11111100;
						true
					} else {
						let x_len = to_file.abs_diff(from_file);
						let y_len = to_rank.abs_diff(from_rank);
						x_len < 2 && y_len < 2
					}
				}
				_ => unreachable!()
			}};
			if !is_valid {output!(stdout, 21, "Invalid input, move is not allowed by the piece's rules"); continue 'player_move;}
			
			// update board
			set_piece(&mut board, from_file, from_rank, Piece::None);
			set_piece(&mut board, to_file, to_rank, from_piece);
			
			// promotion
			if from_piece == Piece::WhitePawn && to_rank == 7 && to_piece != Piece::BlackKing {
				print_board(&board, &mut stdout)?;
				output!(stdout, 21, "Your pawn has reached the end of the board! What you want to promote it to? ");
				let new_piece = read!();
				match &*new_piece.to_ascii_lowercase() {
					"knight" => set_piece(&mut board, to_file, to_rank, Piece::WhiteKnight),
					"bishop" => set_piece(&mut board, to_file, to_rank, Piece::WhiteBishop),
					"rook" => set_piece(&mut board, to_file, to_rank, Piece::WhiteRook),
					"queen" => set_piece(&mut board, to_file, to_rank, Piece::WhiteQueen),
					_ => {output!(stdout, 22, "Invalid option, enter 'knight', 'bishop', 'rook' or 'queen'");}
				}
				execute!(stdout, cursor::MoveTo(0, 22), terminal::Clear(terminal::ClearType::CurrentLine))?;
			}
			
			print_board(&board, &mut stdout)?;
			
			execute!(stdout, cursor::MoveTo(0, 21), terminal::Clear(terminal::ClearType::CurrentLine))?;
			
			// win condition
			if to_piece == Piece::BlackKing {
				print_board(&board, &mut stdout)?;
				output!(stdout, 21, "You have captured your opponent's king!");
				player_won = true;
			}
			
			break 'player_move;
			
		}
		
		if player_won {break;}
		
		let start = Instant::now();
		engine::make_move(&mut board, &mut game_flags, time_data.map(|v| v.1), &thread_pool);
		print_board(&board, &mut stdout)?;
		if let Some((_player_time, engine_time, bonus_time)) = &mut time_data {
			let time_taken = start.elapsed().as_millis() as usize;
			if *engine_time < time_taken {
				output!(stdout, 21, "Engine has lost on time");
				player_won = true;
				break 'game_loop;
			}
			*engine_time -= time_taken;
			*engine_time += *bonus_time;
		}
		
	}
	
	if player_won {
		output!(stdout, 22, "Game Won");
	} else {
		output!(stdout, 22, "Game Lost");
	}
	execute!(stdout, cursor::MoveTo(0, 23))?;
	println!();
	
	Ok(())
}
