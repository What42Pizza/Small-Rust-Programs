#![allow(unused)]
#![warn(unused_must_use)]

#![feature(coroutines)]
#![feature(coroutine_trait)]



pub mod get_moves;
pub use get_moves::*;



use std::{ops::{Coroutine, CoroutineState}, pin::Pin};



pub type Board = [u8; 32];



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

pub fn get_piece(board: &Board, x: u8, y: u8, _id: u8) -> Piece {
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
