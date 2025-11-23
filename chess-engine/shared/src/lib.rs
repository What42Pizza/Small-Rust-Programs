#![allow(unused)]
#![warn(unused_must_use)]

#![feature(coroutines)]
#![feature(coroutine_trait)]



pub mod get_moves;
pub use get_moves::*;



use std::{ops::{Coroutine, CoroutineState}, pin::Pin};



pub type Board = [u8; 32];

pub fn default_board() -> Board {
	let mut board = [0; 32];
	
	// default start:
	set_piece(&mut board, 0, 0, Piece::WhiteRook);
	set_piece(&mut board, 1, 0, Piece::WhiteKnight);
	set_piece(&mut board, 2, 0, Piece::WhiteBishop);
	set_piece(&mut board, 3, 0, Piece::WhiteQueen);
	set_piece(&mut board, 4, 0, Piece::WhiteKing);
	set_piece(&mut board, 5, 0, Piece::WhiteBishop);
	set_piece(&mut board, 6, 0, Piece::WhiteKnight);
	set_piece(&mut board, 7, 0, Piece::WhiteRook);
	set_piece(&mut board, 0, 1, Piece::WhitePawn);
	set_piece(&mut board, 1, 1, Piece::WhitePawn);
	set_piece(&mut board, 2, 1, Piece::WhitePawn);
	set_piece(&mut board, 3, 1, Piece::WhitePawn);
	set_piece(&mut board, 4, 1, Piece::WhitePawn);
	set_piece(&mut board, 5, 1, Piece::WhitePawn);
	set_piece(&mut board, 6, 1, Piece::WhitePawn);
	set_piece(&mut board, 7, 1, Piece::WhitePawn);
	set_piece(&mut board, 0, 6, Piece::BlackPawn);
	set_piece(&mut board, 1, 6, Piece::BlackPawn);
	set_piece(&mut board, 2, 6, Piece::BlackPawn);
	set_piece(&mut board, 3, 6, Piece::BlackPawn);
	set_piece(&mut board, 4, 6, Piece::BlackPawn);
	set_piece(&mut board, 5, 6, Piece::BlackPawn);
	set_piece(&mut board, 6, 6, Piece::BlackPawn);
	set_piece(&mut board, 7, 6, Piece::BlackPawn);
	set_piece(&mut board, 0, 7, Piece::BlackRook);
	set_piece(&mut board, 1, 7, Piece::BlackKnight);
	set_piece(&mut board, 2, 7, Piece::BlackBishop);
	set_piece(&mut board, 3, 7, Piece::BlackQueen);
	set_piece(&mut board, 4, 7, Piece::BlackKing);
	set_piece(&mut board, 5, 7, Piece::BlackBishop);
	set_piece(&mut board, 6, 7, Piece::BlackKnight);
	set_piece(&mut board, 7, 7, Piece::BlackRook);
	
	// mid-game:
	//set_piece(&mut board, 3, 0, Piece::WhiteRook);
	//set_piece(&mut board, 4, 0, Piece::WhiteKing);
	//set_piece(&mut board, 7, 0, Piece::WhiteRook);
	//set_piece(&mut board, 0, 1, Piece::WhitePawn);
	//set_piece(&mut board, 1, 1, Piece::WhitePawn);
	//set_piece(&mut board, 5, 1, Piece::WhitePawn);
	//set_piece(&mut board, 6, 1, Piece::WhitePawn);
	//set_piece(&mut board, 7, 1, Piece::WhitePawn);
	//set_piece(&mut board, 2, 2, Piece::WhiteKnight);
	//set_piece(&mut board, 3, 2, Piece::WhiteBishop);
	//set_piece(&mut board, 2, 4, Piece::WhitePawn);
	//set_piece(&mut board, 4, 4, Piece::WhitePawn);
	//set_piece(&mut board, 1, 3, Piece::BlackPawn);
	//set_piece(&mut board, 0, 5, Piece::BlackPawn);
	//set_piece(&mut board, 4, 5, Piece::BlackPawn);
	//set_piece(&mut board, 5, 6, Piece::BlackPawn);
	//set_piece(&mut board, 6, 6, Piece::BlackPawn);
	//set_piece(&mut board, 7, 6, Piece::BlackPawn);
	//set_piece(&mut board, 0, 7, Piece::BlackRook);
	//set_piece(&mut board, 2, 7, Piece::BlackBishop);
	//set_piece(&mut board, 4, 7, Piece::BlackKing);
	//set_piece(&mut board, 6, 7, Piece::BlackRook);
	
	// testing:
	//set_piece(&mut board, 0, 0, Piece::WhiteRook);
	//set_piece(&mut board, 2, 0, Piece::WhiteBishop);
	//set_piece(&mut board, 3, 0, Piece::WhiteQueen);
	//set_piece(&mut board, 4, 0, Piece::WhiteKing);
	//set_piece(&mut board, 5, 0, Piece::WhiteBishop);
	//set_piece(&mut board, 7, 0, Piece::WhiteRook);
	//set_piece(&mut board, 0, 1, Piece::WhitePawn);
	//set_piece(&mut board, 1, 1, Piece::WhitePawn);
	//set_piece(&mut board, 2, 1, Piece::WhitePawn);
	//set_piece(&mut board, 3, 1, Piece::WhitePawn);
	//set_piece(&mut board, 4, 3, Piece::WhitePawn);
	//set_piece(&mut board, 5, 1, Piece::WhitePawn);
	//set_piece(&mut board, 6, 1, Piece::WhitePawn);
	//set_piece(&mut board, 7, 1, Piece::WhitePawn);
	//set_piece(&mut board, 5, 2, Piece::WhiteKnight);
	//set_piece(&mut board, 2, 2, Piece::WhiteKnight);
	//set_piece(&mut board, 0, 6, Piece::BlackPawn);
	//set_piece(&mut board, 1, 6, Piece::BlackPawn);
	//set_piece(&mut board, 2, 5, Piece::BlackPawn);
	//set_piece(&mut board, 3, 6, Piece::BlackPawn);
	//set_piece(&mut board, 4, 6, Piece::BlackPawn);
	//set_piece(&mut board, 5, 6, Piece::BlackPawn);
	//set_piece(&mut board, 6, 6, Piece::BlackPawn);
	//set_piece(&mut board, 7, 6, Piece::BlackPawn);
	//set_piece(&mut board, 0, 7, Piece::BlackRook);
	//set_piece(&mut board, 1, 7, Piece::BlackKnight);
	//set_piece(&mut board, 2, 7, Piece::BlackBishop);
	//set_piece(&mut board, 0, 4, Piece::BlackQueen);
	//set_piece(&mut board, 4, 7, Piece::BlackKing);
	//set_piece(&mut board, 5, 7, Piece::BlackBishop);
	//set_piece(&mut board, 7, 7, Piece::BlackRook);
	//set_piece(&mut board, 5, 5, Piece::BlackKnight);
	
	board
}



#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Piece {
	None = 0,
	BlackPawn = 1,
	BlackKnight = 2,
	BlackBishop = 3,
	BlackRook = 4,
	BlackQueen = 5,
	BlackKing = 6,
	WhitePawn = 9,
	WhiteKnight = 10,
	WhiteBishop = 11,
	WhiteRook = 12,
	WhiteQueen = 13,
	WhiteKing = 14,
}

impl Piece {
	pub fn is_white(self) -> bool {
		self as u8 & 0b00001000 > 0
	}
	pub fn is_black(self) -> bool {
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

pub fn get_piece(board: &Board, x: u8, y: u8) -> Piece {
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
	unsafe {(std::mem::transmute::<u8, Piece>(byte1), std::mem::transmute::<u8, Piece>(byte2))}
}



#[allow(clippy::too_many_arguments)]
pub fn perform_move(board: &mut Board, game_flags: &mut u8, piece: Piece, from_x: u8, from_y: u8, to_x: u8, to_y: u8, move_type: MoveType) {
	*game_flags &= 0b00001111; // reset en passant data
	set_piece(board, from_x, from_y, Piece::None);
	set_piece(board, to_x, to_y, piece);
	match move_type {
		MoveType::Normal => {}
		MoveType::EnPassant => {
			set_piece(board, to_x, from_y, Piece::None);
		}
		MoveType::CastleKingsSide => {
			set_piece(board, 7, to_y, Piece::None);
			set_piece(board, 5, to_y, Piece::BlackRook.copy_owner(piece));
			*game_flags &= if piece.is_white() {0b11111100} else {0b11110011};
		}
		MoveType::CastleQueensSide => {
			set_piece(board, 0, to_y, Piece::None);
			set_piece(board, 3, to_y, Piece::BlackRook.copy_owner(piece));
			*game_flags &= if piece.is_white() {0b11111100} else {0b11110011};
		}
		MoveType::PromoteKnight => {
			set_piece(board, to_x, to_y, Piece::WhiteKnight.copy_owner(piece));
		}
		MoveType::PromoteBishop => {
			set_piece(board, to_x, to_y, Piece::WhiteBishop.copy_owner(piece));
		}
		MoveType::PromoteRook => {
			set_piece(board, to_x, to_y, Piece::WhiteRook.copy_owner(piece));
		}
		MoveType::PromoteQueen => {
			set_piece(board, to_x, to_y, Piece::WhiteQueen.copy_owner(piece));
		}
	}
	if piece as u8 & 0b111 == Piece::BlackPawn as u8 && to_y.abs_diff(from_y) == 2 {
		*game_flags |= (to_x << 5) | 0b00010000; // allow en passant for next move
	}
	if piece == Piece::WhiteKing {
		*game_flags &= 0b11111100;
	}
	if piece == Piece::BlackKing {
		*game_flags &= 0b11110011;
	}
	match (from_x, from_y) {
		(0, 0) => *game_flags &= 0b11111110,
		(0, 7) => *game_flags &= 0b11111101,
		(7, 0) => *game_flags &= 0b11111011,
		(7, 7) => *game_flags &= 0b11110111,
		_ => {}
	}
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
