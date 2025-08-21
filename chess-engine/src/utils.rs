use crate::*;
use std::{ops::{Coroutine, CoroutineState}, pin::{pin, Pin}};



#[macro_export]
macro_rules! output {
	($stdout:expr, $line:expr $(, $format:expr)+) => {
		execute!($stdout, cursor::MoveTo(0, $line), terminal::Clear(terminal::ClearType::CurrentLine))?;
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



impl Piece {
	pub fn print_self(&self, background_color: Color, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
		match self {
			Self::None => write!(stdout, "{}", "  ".on_color(background_color))?,
			Self::SelfPawn    => write!(stdout, "{}", "Pw".color(BLACK_COLOR).on_color(background_color))?,
			Self::SelfKnight  => write!(stdout, "{}", "Kn".color(BLACK_COLOR).on_color(background_color))?,
			Self::SelfBishop  => write!(stdout, "{}", "Bp".color(BLACK_COLOR).on_color(background_color))?,
			Self::SelfRook    => write!(stdout, "{}", "Rk".color(BLACK_COLOR).on_color(background_color))?,
			Self::SelfQueen   => write!(stdout, "{}", "Qn".color(BLACK_COLOR).on_color(background_color))?,
			Self::SelfKing    => write!(stdout, "{}", "Ki".color(BLACK_COLOR).on_color(background_color))?,
			Self::OtherPawn   => write!(stdout, "{}", "Pw".color(WHITE_COLOR).on_color(background_color))?,
			Self::OtherKnight => write!(stdout, "{}", "Kn".color(WHITE_COLOR).on_color(background_color))?,
			Self::OtherBishop => write!(stdout, "{}", "Bp".color(WHITE_COLOR).on_color(background_color))?,
			Self::OtherRook   => write!(stdout, "{}", "Rk".color(WHITE_COLOR).on_color(background_color))?,
			Self::OtherQueen  => write!(stdout, "{}", "Qn".color(WHITE_COLOR).on_color(background_color))?,
			Self::OtherKing   => write!(stdout, "{}", "Ki".color(WHITE_COLOR).on_color(background_color))?,
		}
		Ok(())
	}
	pub fn is_other(self) -> bool {
		self as u8 & 0b00001000 > 0
	}
	pub fn is_self(self) -> bool {
		self as u8 > 0 && (self as u8) < 8
	}
	pub fn copy_owner(self, other: Self) -> Self {
		let mut byte = self as u8 & 0b111;
		byte |= other as u8 & 0b1000;
		unsafe {std::mem::transmute(byte)}
	}
}



pub fn set_piece(board: &mut Board, x: u8, y: u8, piece: Piece) {
	let index = x + y * 8;
	let mut byte = board[(index / 2) as usize];
	byte &= if index % 2 == 0 {0b11110000} else {0b00001111};
	let piece = if index % 2 == 0 {piece as u8} else {(piece as u8) << 4};
	byte |= piece;
	board[(index / 2) as usize] = byte;
}

pub fn get_piece(board: &Board, x: u8, y: u8, id: u8) -> Piece {
	let index = x + y * 8;
	let mut byte = board[(index / 2) as usize];
	byte &= if index % 2 == 0 {0b00001111} else {0b11110000};
	byte = if index % 2 == 1 {byte >> 4} else {byte};
	unsafe {std::mem::transmute(byte)}
}

pub fn get_doubled_pieces(board: &Board, x: u8, y: u8) -> (Piece, Piece) {
	let index = x + y * 8;
	let byte = board[(index / 2) as usize];
	let byte1 = byte & 0b00001111;
	let byte2 = (byte & 0b11110000) >> 4;
	unsafe {(std::mem::transmute(byte1), std::mem::transmute(byte2))}
}

pub fn default_board() -> Board {
	let mut board = [0; 32];
	
	// default start:
	//set_piece(&mut board, 0, 0, Piece::OtherRook);
	//set_piece(&mut board, 1, 0, Piece::OtherKnight);
	//set_piece(&mut board, 2, 0, Piece::OtherBishop);
	//set_piece(&mut board, 3, 0, Piece::OtherQueen);
	//set_piece(&mut board, 4, 0, Piece::OtherKing);
	//set_piece(&mut board, 5, 0, Piece::OtherBishop);
	//set_piece(&mut board, 6, 0, Piece::OtherKnight);
	//set_piece(&mut board, 7, 0, Piece::OtherRook);
	//set_piece(&mut board, 0, 1, Piece::OtherPawn);
	//set_piece(&mut board, 1, 1, Piece::OtherPawn);
	//set_piece(&mut board, 2, 1, Piece::OtherPawn);
	//set_piece(&mut board, 3, 1, Piece::OtherPawn);
	//set_piece(&mut board, 4, 1, Piece::OtherPawn);
	//set_piece(&mut board, 5, 1, Piece::OtherPawn);
	//set_piece(&mut board, 6, 1, Piece::OtherPawn);
	//set_piece(&mut board, 7, 1, Piece::OtherPawn);
	//set_piece(&mut board, 0, 6, Piece::SelfPawn);
	//set_piece(&mut board, 1, 6, Piece::SelfPawn);
	//set_piece(&mut board, 2, 6, Piece::SelfPawn);
	//set_piece(&mut board, 3, 6, Piece::SelfPawn);
	//set_piece(&mut board, 4, 6, Piece::SelfPawn);
	//set_piece(&mut board, 5, 6, Piece::SelfPawn);
	//set_piece(&mut board, 6, 6, Piece::SelfPawn);
	//set_piece(&mut board, 7, 6, Piece::SelfPawn);
	//set_piece(&mut board, 0, 7, Piece::SelfRook);
	//set_piece(&mut board, 1, 7, Piece::SelfKnight);
	//set_piece(&mut board, 2, 7, Piece::SelfBishop);
	//set_piece(&mut board, 3, 7, Piece::SelfQueen);
	//set_piece(&mut board, 4, 7, Piece::SelfKing);
	//set_piece(&mut board, 5, 7, Piece::SelfBishop);
	//set_piece(&mut board, 6, 7, Piece::SelfKnight);
	//set_piece(&mut board, 7, 7, Piece::SelfRook);
	
	// mid-game:
	//set_piece(&mut board, 3, 0, Piece::OtherRook);
	//set_piece(&mut board, 4, 0, Piece::OtherKing);
	//set_piece(&mut board, 7, 0, Piece::OtherRook);
	//set_piece(&mut board, 0, 1, Piece::OtherPawn);
	//set_piece(&mut board, 1, 1, Piece::OtherPawn);
	//set_piece(&mut board, 5, 1, Piece::OtherPawn);
	//set_piece(&mut board, 6, 1, Piece::OtherPawn);
	//set_piece(&mut board, 7, 1, Piece::OtherPawn);
	//set_piece(&mut board, 2, 2, Piece::OtherKnight);
	//set_piece(&mut board, 3, 2, Piece::OtherBishop);
	//set_piece(&mut board, 2, 4, Piece::OtherPawn);
	//set_piece(&mut board, 4, 4, Piece::OtherPawn);
	//set_piece(&mut board, 1, 3, Piece::SelfPawn);
	//set_piece(&mut board, 0, 5, Piece::SelfPawn);
	//set_piece(&mut board, 4, 5, Piece::SelfPawn);
	//set_piece(&mut board, 5, 6, Piece::SelfPawn);
	//set_piece(&mut board, 6, 6, Piece::SelfPawn);
	//set_piece(&mut board, 7, 6, Piece::SelfPawn);
	//set_piece(&mut board, 0, 7, Piece::SelfRook);
	//set_piece(&mut board, 2, 7, Piece::SelfBishop);
	//set_piece(&mut board, 4, 7, Piece::SelfKing);
	//set_piece(&mut board, 6, 7, Piece::SelfRook);
	
	// testing:
	set_piece(&mut board, 0, 0, Piece::OtherRook);
	set_piece(&mut board, 2, 0, Piece::OtherBishop);
	set_piece(&mut board, 3, 0, Piece::OtherQueen);
	set_piece(&mut board, 4, 0, Piece::OtherKing);
	set_piece(&mut board, 5, 0, Piece::OtherBishop);
	set_piece(&mut board, 7, 0, Piece::OtherRook);
	set_piece(&mut board, 0, 1, Piece::OtherPawn);
	set_piece(&mut board, 1, 1, Piece::OtherPawn);
	set_piece(&mut board, 2, 1, Piece::OtherPawn);
	set_piece(&mut board, 3, 1, Piece::OtherPawn);
	set_piece(&mut board, 4, 3, Piece::OtherPawn);
	set_piece(&mut board, 5, 1, Piece::OtherPawn);
	set_piece(&mut board, 6, 1, Piece::OtherPawn);
	set_piece(&mut board, 7, 1, Piece::OtherPawn);
	set_piece(&mut board, 5, 2, Piece::OtherKnight);
	set_piece(&mut board, 2, 2, Piece::OtherKnight);
	set_piece(&mut board, 0, 6, Piece::SelfPawn);
	set_piece(&mut board, 1, 6, Piece::SelfPawn);
	set_piece(&mut board, 2, 5, Piece::SelfPawn);
	set_piece(&mut board, 3, 6, Piece::SelfPawn);
	set_piece(&mut board, 4, 6, Piece::SelfPawn);
	set_piece(&mut board, 5, 6, Piece::SelfPawn);
	set_piece(&mut board, 6, 6, Piece::SelfPawn);
	set_piece(&mut board, 7, 6, Piece::SelfPawn);
	set_piece(&mut board, 0, 7, Piece::SelfRook);
	set_piece(&mut board, 1, 7, Piece::SelfKnight);
	set_piece(&mut board, 2, 7, Piece::SelfBishop);
	set_piece(&mut board, 0, 4, Piece::SelfQueen);
	set_piece(&mut board, 4, 7, Piece::SelfKing);
	set_piece(&mut board, 5, 7, Piece::SelfBishop);
	set_piece(&mut board, 7, 7, Piece::SelfRook);
	set_piece(&mut board, 5, 5, Piece::SelfKnight);
	
	board
}

pub fn print_board(board: &Board, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
	execute!(stdout, cursor::MoveTo(0, 0))?;
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
