#![allow(unused)]
#![warn(unused_must_use)]

#![feature(coroutine_trait)]



use std::{io::Stdout, ops::{Coroutine, CoroutineState}, pin::Pin};



pub type Board = [u8; 32];



#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Piece {
	None = 0,
	SelfPawn = 1,
	SelfKnight = 2,
	SelfBishop = 3,
	SelfRook = 4,
	SelfQueen = 5,
	SelfKing = 6,
	OtherPawn = 9,
	OtherKnight = 10,
	OtherBishop = 11,
	OtherRook = 12,
	OtherQueen = 13,
	OtherKing = 14,
}



impl Piece {
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
	unsafe {(std::mem::transmute::<u8, Piece>(byte1), std::mem::transmute::<u8, Piece>(byte2))}
}

pub fn default_board() -> Board {
	let mut board = [0; 32];
	
	// default start:
	set_piece(&mut board, 0, 0, Piece::OtherRook);
	set_piece(&mut board, 1, 0, Piece::OtherKnight);
	set_piece(&mut board, 2, 0, Piece::OtherBishop);
	set_piece(&mut board, 3, 0, Piece::OtherQueen);
	set_piece(&mut board, 4, 0, Piece::OtherKing);
	set_piece(&mut board, 5, 0, Piece::OtherBishop);
	set_piece(&mut board, 6, 0, Piece::OtherKnight);
	set_piece(&mut board, 7, 0, Piece::OtherRook);
	set_piece(&mut board, 0, 1, Piece::OtherPawn);
	set_piece(&mut board, 1, 1, Piece::OtherPawn);
	set_piece(&mut board, 2, 1, Piece::OtherPawn);
	set_piece(&mut board, 3, 1, Piece::OtherPawn);
	set_piece(&mut board, 4, 1, Piece::OtherPawn);
	set_piece(&mut board, 5, 1, Piece::OtherPawn);
	set_piece(&mut board, 6, 1, Piece::OtherPawn);
	set_piece(&mut board, 7, 1, Piece::OtherPawn);
	set_piece(&mut board, 0, 6, Piece::SelfPawn);
	set_piece(&mut board, 1, 6, Piece::SelfPawn);
	set_piece(&mut board, 2, 6, Piece::SelfPawn);
	set_piece(&mut board, 3, 6, Piece::SelfPawn);
	set_piece(&mut board, 4, 6, Piece::SelfPawn);
	set_piece(&mut board, 5, 6, Piece::SelfPawn);
	set_piece(&mut board, 6, 6, Piece::SelfPawn);
	set_piece(&mut board, 7, 6, Piece::SelfPawn);
	set_piece(&mut board, 0, 7, Piece::SelfRook);
	set_piece(&mut board, 1, 7, Piece::SelfKnight);
	set_piece(&mut board, 2, 7, Piece::SelfBishop);
	set_piece(&mut board, 3, 7, Piece::SelfQueen);
	set_piece(&mut board, 4, 7, Piece::SelfKing);
	set_piece(&mut board, 5, 7, Piece::SelfBishop);
	set_piece(&mut board, 6, 7, Piece::SelfKnight);
	set_piece(&mut board, 7, 7, Piece::SelfRook);
	
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
	//set_piece(&mut board, 0, 0, Piece::OtherRook);
	//set_piece(&mut board, 2, 0, Piece::OtherBishop);
	//set_piece(&mut board, 3, 0, Piece::OtherQueen);
	//set_piece(&mut board, 4, 0, Piece::OtherKing);
	//set_piece(&mut board, 5, 0, Piece::OtherBishop);
	//set_piece(&mut board, 7, 0, Piece::OtherRook);
	//set_piece(&mut board, 0, 1, Piece::OtherPawn);
	//set_piece(&mut board, 1, 1, Piece::OtherPawn);
	//set_piece(&mut board, 2, 1, Piece::OtherPawn);
	//set_piece(&mut board, 3, 1, Piece::OtherPawn);
	//set_piece(&mut board, 4, 3, Piece::OtherPawn);
	//set_piece(&mut board, 5, 1, Piece::OtherPawn);
	//set_piece(&mut board, 6, 1, Piece::OtherPawn);
	//set_piece(&mut board, 7, 1, Piece::OtherPawn);
	//set_piece(&mut board, 5, 2, Piece::OtherKnight);
	//set_piece(&mut board, 2, 2, Piece::OtherKnight);
	//set_piece(&mut board, 0, 6, Piece::SelfPawn);
	//set_piece(&mut board, 1, 6, Piece::SelfPawn);
	//set_piece(&mut board, 2, 5, Piece::SelfPawn);
	//set_piece(&mut board, 3, 6, Piece::SelfPawn);
	//set_piece(&mut board, 4, 6, Piece::SelfPawn);
	//set_piece(&mut board, 5, 6, Piece::SelfPawn);
	//set_piece(&mut board, 6, 6, Piece::SelfPawn);
	//set_piece(&mut board, 7, 6, Piece::SelfPawn);
	//set_piece(&mut board, 0, 7, Piece::SelfRook);
	//set_piece(&mut board, 1, 7, Piece::SelfKnight);
	//set_piece(&mut board, 2, 7, Piece::SelfBishop);
	//set_piece(&mut board, 0, 4, Piece::SelfQueen);
	//set_piece(&mut board, 4, 7, Piece::SelfKing);
	//set_piece(&mut board, 5, 7, Piece::SelfBishop);
	//set_piece(&mut board, 7, 7, Piece::SelfRook);
	//set_piece(&mut board, 5, 5, Piece::SelfKnight);
	
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
