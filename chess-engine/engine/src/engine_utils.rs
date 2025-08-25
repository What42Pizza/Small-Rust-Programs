use shared::*;
use std::ops::Coroutine;



#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub(super) enum SpecialMove {
	None,
	EnPassant,
	CastleKingsSide,
	CastleQueensSide,
	PromoteKnight,
	PromoteBishop,
	PromoteRook,
	PromoteQueen,
}

// NOTE: this assumes that the engine cannot have any pawns on the first rank
pub(super) fn get_self_moves(board: &Board, piece: Piece, x: u8, y: u8, game_flags: u8) -> CoroutineIter<impl Coroutine<Yield = (u8, u8, SpecialMove), Return = ()>> {
	CoroutineIter (#[coroutine] move || {
		match piece {
			
			Piece::SelfPawn => {
				if get_piece(board, x, y - 1, 8) == Piece::None {
					if y == 6 && get_piece(board, x, 4, 9) == Piece::None {
						yield (x, y - 1, SpecialMove::None);
						yield (x, y - 2, SpecialMove::None);
					} else if y == 1 {
						yield (x, y - 1, SpecialMove::PromoteKnight);
						yield (x, y - 1, SpecialMove::PromoteBishop);
						yield (x, y - 1, SpecialMove::PromoteRook);
						yield (x, y - 1, SpecialMove::PromoteQueen);
					} else {
						yield (x, y - 1, SpecialMove::None);
					}
				}
				if x >= 1 && get_piece(board, x - 1, y - 1, 10).is_other() {
					if y == 1 {
						yield (x - 1, y - 1, SpecialMove::PromoteKnight);
						yield (x - 1, y - 1, SpecialMove::PromoteBishop);
						yield (x - 1, y - 1, SpecialMove::PromoteRook);
						yield (x - 1, y - 1, SpecialMove::PromoteQueen);
					} else {
						yield (x - 1, y - 1, SpecialMove::None);
					}
				}
				if x <= 6 && get_piece(board, x + 1, y - 1, 11).is_other() {
					if y == 1 {
						yield (x + 1, y - 1, SpecialMove::PromoteKnight);
						yield (x + 1, y - 1, SpecialMove::PromoteBishop);
						yield (x + 1, y - 1, SpecialMove::PromoteRook);
						yield (x + 1, y - 1, SpecialMove::PromoteQueen);
					} else {
						yield (x + 1, y - 1, SpecialMove::None);
					}
				}
				if game_flags & 0b00010000 > 0 && y == 3 {
					if x >= 1 && ((game_flags & 0b11100000) >> 5) == x - 1 {
						yield (x - 1, y - 1, SpecialMove::EnPassant);
					}
					if x <= 6 && ((game_flags & 0b11100000) >> 5) == x + 1 {
						yield (x + 1, y - 1, SpecialMove::EnPassant);
					}
				}
			}
			
			Piece::SelfKnight => {
				if x >= 1 && y <= 5 && !get_piece(board, x - 1, y + 2, 12).is_self() {yield (x - 1, y + 2, SpecialMove::None);}
				if x <= 6 && y <= 5 && !get_piece(board, x + 1, y + 2, 13).is_self() {yield (x + 1, y + 2, SpecialMove::None);}
				if x <= 5 && y <= 6 && !get_piece(board, x + 2, y + 1, 14).is_self() {yield (x + 2, y + 1, SpecialMove::None);}
				if x <= 5 && y >= 1 && !get_piece(board, x + 2, y - 1, 15).is_self() {yield (x + 2, y - 1, SpecialMove::None);}
				if x <= 6 && y >= 2 && !get_piece(board, x + 1, y - 2, 16).is_self() {yield (x + 1, y - 2, SpecialMove::None);}
				if x >= 1 && y >= 2 && !get_piece(board, x - 1, y - 2, 17).is_self() {yield (x - 1, y - 2, SpecialMove::None);}
				if x >= 2 && y >= 1 && !get_piece(board, x - 2, y - 1, 18).is_self() {yield (x - 2, y - 1, SpecialMove::None);}
				if x >= 2 && y <= 6 && !get_piece(board, x - 2, y + 1, 19).is_self() {yield (x - 2, y + 1, SpecialMove::None);}
			}
			
			Piece::SelfBishop => {
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y <= 6 {
					curr_x += 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y, 20);
					if piece.is_other() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y >= 1 {
					curr_x += 1;
					curr_y -= 1;
					let piece = get_piece(board, curr_x, curr_y, 21);
					if piece.is_other() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x >= 1 && curr_y >= 1 {
					curr_x -= 1;
					curr_y -= 1;
					let piece = get_piece(board, curr_x, curr_y, 22);
					if piece.is_other() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x >= 1 && curr_y <= 6 {
					curr_x -= 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y, 23);
					if piece.is_other() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
			}
			
			Piece::SelfRook => {
				let mut curr_x = x;
				while curr_x <= 6 {
					curr_x += 1;
					let piece = get_piece(board, curr_x, y, 24);
					if piece.is_other() {
						yield (curr_x, y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				while curr_x >= 1 {
					curr_x -= 1;
					let piece = get_piece(board, curr_x, y, 25);
					if piece.is_other() {
						yield (curr_x, y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_y = y;
				while curr_y <= 6 {
					curr_y += 1;
					let piece = get_piece(board, x, curr_y, 26);
					if piece.is_other() {
						yield (x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_y = y;
				while curr_y >= 1 {
					curr_y -= 1;
					let piece = get_piece(board, x, curr_y, 27);
					if piece.is_other() {
						yield (x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
			}
			
			Piece::SelfQueen => {
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y <= 6 {
					curr_x += 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y, 28);
					if piece.is_other() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y >= 1 {
					curr_x += 1;
					curr_y -= 1;
					let piece = get_piece(board, curr_x, curr_y, 29);
					if piece.is_other() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x >= 1 && curr_y >= 1 {
					curr_x -= 1;
					curr_y -= 1;
					let piece = get_piece(board, curr_x, curr_y, 30);
					if piece.is_other() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x >= 1 && curr_y <= 6 {
					curr_x -= 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y, 31);
					if piece.is_other() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				while curr_x <= 6 {
					curr_x += 1;
					let piece = get_piece(board, curr_x, y, 32);
					if piece.is_other() {
						yield (curr_x, y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				while curr_x >= 1 {
					curr_x -= 1;
					let piece = get_piece(board, curr_x, y, 33);
					if piece.is_other() {
						yield (curr_x, y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_y = y;
				while curr_y <= 6 {
					curr_y += 1;
					let piece = get_piece(board, x, curr_y, 34);
					if piece.is_other() {
						yield (x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_y = y;
				while curr_y >= 1 {
					curr_y -= 1;
					let piece = get_piece(board, x, curr_y, 35);
					if piece.is_other() {
						yield (x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
			}
			
			Piece::SelfKing => {
				let x_min = x.max(1) - 1;
				let y_min = y.max(1) - 1;
				let x_max = x.min(6) + 1;
				let y_max = y.min(6) + 1;
				for x in x_min..=x_max {
					for y in y_min..=y_max {
						if !get_piece(board, x, y, 36).is_self() {
							yield (x, y, SpecialMove::None);
						}
					}
				}
				if game_flags & 0b00000100 > 0 {
					yield (2, 7, SpecialMove::CastleQueensSide);
				}
				if game_flags & 0b00001000 > 0 {
					yield (6, 7, SpecialMove::CastleKingsSide);
				}
			}
			
			_ => {}
		}
	})
}



// NOTE: this assumes that the player cannot have any pawns on the eighth rank
pub(super) fn get_other_moves(board: &Board, piece: Piece, x: u8, y: u8, game_flags: u8) -> CoroutineIter<impl Coroutine<Yield = (u8, u8, SpecialMove), Return = ()>> {
	CoroutineIter (#[coroutine] move || {
		match piece {
			
			Piece::OtherPawn => {
				if get_piece(board, x, y + 1, 37) == Piece::None {
					if y == 1 && get_piece(board, x, 3, 38) == Piece::None {
						yield (x, y + 1, SpecialMove::None);
						yield (x, y + 2, SpecialMove::None);
					} else if y == 6 {
						yield (x, y + 1, SpecialMove::PromoteKnight);
						yield (x, y + 1, SpecialMove::PromoteBishop);
						yield (x, y + 1, SpecialMove::PromoteRook);
						yield (x, y + 1, SpecialMove::PromoteQueen);
					} else {
						yield (x, y + 1, SpecialMove::None);
					}
				}
				if x >= 1 && get_piece(board, x - 1, y + 1, 39).is_self() {
					if y == 6 {
						yield (x - 1, y + 1, SpecialMove::PromoteKnight);
						yield (x - 1, y + 1, SpecialMove::PromoteBishop);
						yield (x - 1, y + 1, SpecialMove::PromoteRook);
						yield (x - 1, y + 1, SpecialMove::PromoteQueen);
					} else {
						yield (x - 1, y + 1, SpecialMove::None);
					}
				}
				if x <= 6 && get_piece(board, x + 1, y + 1, 40).is_self() {
					if y == 6 {
						yield (x + 1, y + 1, SpecialMove::PromoteKnight);
						yield (x + 1, y + 1, SpecialMove::PromoteBishop);
						yield (x + 1, y + 1, SpecialMove::PromoteRook);
						yield (x + 1, y + 1, SpecialMove::PromoteQueen);
					} else {
						yield (x + 1, y + 1, SpecialMove::None);
					}
				}
				if game_flags & 0b00010000 > 0 && y == 4 {
					if x >= 1 && ((game_flags & 0b11100000) >> 5) == x - 1 {
						yield (x - 1, y + 1, SpecialMove::EnPassant);
					}
					if x <= 6 && ((game_flags & 0b11100000) >> 5) == x + 1 {
						yield (x + 1, y + 1, SpecialMove::EnPassant);
					}
				}
			}
			
			Piece::OtherKnight => {
				if x >= 1 && y <= 5 && !get_piece(board, x - 1, y + 2, 41).is_other() {yield (x - 1, y + 2, SpecialMove::None);}
				if x <= 6 && y <= 5 && !get_piece(board, x + 1, y + 2, 42).is_other() {yield (x + 1, y + 2, SpecialMove::None);}
				if x <= 5 && y <= 6 && !get_piece(board, x + 2, y + 1, 43).is_other() {yield (x + 2, y + 1, SpecialMove::None);}
				if x <= 5 && y >= 1 && !get_piece(board, x + 2, y - 1, 44).is_other() {yield (x + 2, y - 1, SpecialMove::None);}
				if x <= 6 && y >= 2 && !get_piece(board, x + 1, y - 2, 45).is_other() {yield (x + 1, y - 2, SpecialMove::None);}
				if x >= 1 && y >= 2 && !get_piece(board, x - 1, y - 2, 46).is_other() {yield (x - 1, y - 2, SpecialMove::None);}
				if x >= 2 && y >= 1 && !get_piece(board, x - 2, y - 1, 47).is_other() {yield (x - 2, y - 1, SpecialMove::None);}
				if x >= 2 && y <= 6 && !get_piece(board, x - 2, y + 1, 48).is_other() {yield (x - 2, y + 1, SpecialMove::None);}
			}
			
			Piece::OtherBishop => {
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y <= 6 {
					curr_x += 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y, 49);
					if piece.is_self() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y >= 1 {
					curr_x += 1;
					curr_y -= 1;
					let piece = get_piece(board, curr_x, curr_y, 50);
					if piece.is_self() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x >= 1 && curr_y >= 1 {
					curr_x -= 1;
					curr_y -= 1;
					let piece = get_piece(board, curr_x, curr_y, 51);
					if piece.is_self() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x >= 1 && curr_y <= 6 {
					curr_x -= 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y, 52);
					if piece.is_self() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
			}
			
			Piece::OtherRook => {
				let mut curr_x = x;
				while curr_x <= 6 {
					curr_x += 1;
					let piece = get_piece(board, curr_x, y, 53);
					if piece.is_self() {
						yield (curr_x, y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				while curr_x >= 1 {
					curr_x -= 1;
					let piece = get_piece(board, curr_x, y, 54);
					if piece.is_self() {
						yield (curr_x, y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_y = y;
				while curr_y <= 6 {
					curr_y += 1;
					let piece = get_piece(board, x, curr_y, 55);
					if piece.is_self() {
						yield (x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_y = y;
				while curr_y >= 1 {
					curr_y -= 1;
					let piece = get_piece(board, x, curr_y, 56);
					if piece.is_self() {
						yield (x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
			}
			
			Piece::OtherQueen => {
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y <= 6 {
					curr_x += 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y, 57);
					if piece.is_self() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y >= 1 {
					curr_x += 1;
					curr_y -= 1;
					let piece = get_piece(board, curr_x, curr_y, 58);
					if piece.is_self() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x >= 1 && curr_y >= 1 {
					curr_x -= 1;
					curr_y -= 1;
					let piece = get_piece(board, curr_x, curr_y, 59);
					if piece.is_self() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x >= 1 && curr_y <= 6 {
					curr_x -= 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y, 60);
					if piece.is_self() {
						yield (curr_x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				while curr_x <= 6 {
					curr_x += 1;
					let piece = get_piece(board, curr_x, y, 61);
					if piece.is_self() {
						yield (curr_x, y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_x = x;
				while curr_x >= 1 {
					curr_x -= 1;
					let piece = get_piece(board, curr_x, y, 62);
					if piece.is_self() {
						yield (curr_x, y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (curr_x, y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_y = y;
				while curr_y <= 6 {
					curr_y += 1;
					let piece = get_piece(board, x, curr_y, 63);
					if piece.is_self() {
						yield (x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
				let mut curr_y = y;
				while curr_y >= 1 {
					curr_y -= 1;
					let piece = get_piece(board, x, curr_y, 64);
					if piece.is_self() {
						yield (x, curr_y, SpecialMove::None);
						break;
					}
					if piece == Piece::None {
						yield (x, curr_y, SpecialMove::None);
					} else {
						break;
					}
				}
			}
			
			Piece::OtherKing => {
				let x_min = x.max(1) - 1;
				let y_min = y.max(1) - 1;
				let x_max = x.min(6) + 1;
				let y_max = y.min(6) + 1;
				for x in x_min..=x_max {
					for y in y_min..=y_max {
						if !get_piece(board, x, y, 65).is_other() {
							yield (x, y, SpecialMove::None);
						}
					}
				}
				if game_flags & 0b00000001 > 0 {
					yield (2, 0, SpecialMove::CastleQueensSide);
				}
				if game_flags & 0b00000010 > 0 {
					yield (6, 0, SpecialMove::CastleKingsSide);
				}
			}
			
			_ => {}
		}
	})
}
