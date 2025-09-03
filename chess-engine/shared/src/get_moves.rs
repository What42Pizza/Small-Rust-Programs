use crate::*;



#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum SpecialMove {
	None,
	EnPassant,
	CastleKingsSide,
	CastleQueensSide,
	PromoteKnight,
	PromoteBishop,
	PromoteRook,
	PromoteQueen,
}

// NOTE: this assumes that black cannot have any pawns on the first rank
pub fn get_black_moves(board: &Board, piece: Piece, x: u8, y: u8, game_flags: u8) -> CoroutineIter<impl Coroutine<Yield = (u8, u8, SpecialMove), Return = ()>> {
	CoroutineIter (#[coroutine] move || {
		match piece {
			
			Piece::BlackPawn => {
				if get_piece(board, x, y - 1) == Piece::None {
					if y == 6 && get_piece(board, x, 4) == Piece::None {
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
				if x >= 1 && get_piece(board, x - 1, y - 1).is_white() {
					if y == 1 {
						yield (x - 1, y - 1, SpecialMove::PromoteKnight);
						yield (x - 1, y - 1, SpecialMove::PromoteBishop);
						yield (x - 1, y - 1, SpecialMove::PromoteRook);
						yield (x - 1, y - 1, SpecialMove::PromoteQueen);
					} else {
						yield (x - 1, y - 1, SpecialMove::None);
					}
				}
				if x <= 6 && get_piece(board, x + 1, y - 1).is_white() {
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
			
			Piece::BlackKnight => {
				if x >= 1 && y <= 5 && !get_piece(board, x - 1, y + 2).is_black() {yield (x - 1, y + 2, SpecialMove::None);}
				if x <= 6 && y <= 5 && !get_piece(board, x + 1, y + 2).is_black() {yield (x + 1, y + 2, SpecialMove::None);}
				if x <= 5 && y <= 6 && !get_piece(board, x + 2, y + 1).is_black() {yield (x + 2, y + 1, SpecialMove::None);}
				if x <= 5 && y >= 1 && !get_piece(board, x + 2, y - 1).is_black() {yield (x + 2, y - 1, SpecialMove::None);}
				if x <= 6 && y >= 2 && !get_piece(board, x + 1, y - 2).is_black() {yield (x + 1, y - 2, SpecialMove::None);}
				if x >= 1 && y >= 2 && !get_piece(board, x - 1, y - 2).is_black() {yield (x - 1, y - 2, SpecialMove::None);}
				if x >= 2 && y >= 1 && !get_piece(board, x - 2, y - 1).is_black() {yield (x - 2, y - 1, SpecialMove::None);}
				if x >= 2 && y <= 6 && !get_piece(board, x - 2, y + 1).is_black() {yield (x - 2, y + 1, SpecialMove::None);}
			}
			
			Piece::BlackBishop => {
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y <= 6 {
					curr_x += 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_white() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_white() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_white() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_white() {
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
			
			Piece::BlackRook => {
				let mut curr_x = x;
				while curr_x <= 6 {
					curr_x += 1;
					let piece = get_piece(board, curr_x, y);
					if piece.is_white() {
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
					let piece = get_piece(board, curr_x, y);
					if piece.is_white() {
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
					let piece = get_piece(board, x, curr_y);
					if piece.is_white() {
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
					let piece = get_piece(board, x, curr_y);
					if piece.is_white() {
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
			
			Piece::BlackQueen => {
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y <= 6 {
					curr_x += 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_white() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_white() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_white() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_white() {
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
					let piece = get_piece(board, curr_x, y);
					if piece.is_white() {
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
					let piece = get_piece(board, curr_x, y);
					if piece.is_white() {
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
					let piece = get_piece(board, x, curr_y);
					if piece.is_white() {
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
					let piece = get_piece(board, x, curr_y);
					if piece.is_white() {
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
			
			Piece::BlackKing => {
				let x_min = x.max(1) - 1;
				let y_min = y.max(1) - 1;
				let x_max = x.min(6) + 1;
				let y_max = y.min(6) + 1;
				for x in x_min..=x_max {
					for y in y_min..=y_max {
						if !get_piece(board, x, y).is_black() {
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



// NOTE: this assumes that white cannot have any pawns on the eighth rank
pub fn get_white_moves(board: &Board, piece: Piece, x: u8, y: u8, game_flags: u8) -> CoroutineIter<impl Coroutine<Yield = (u8, u8, SpecialMove), Return = ()>> {
	CoroutineIter (#[coroutine] move || {
		match piece {
			
			Piece::WhitePawn => {
				if get_piece(board, x, y + 1) == Piece::None {
					if y == 1 && get_piece(board, x, 3) == Piece::None {
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
				if x >= 1 && get_piece(board, x - 1, y + 1).is_black() {
					if y == 6 {
						yield (x - 1, y + 1, SpecialMove::PromoteKnight);
						yield (x - 1, y + 1, SpecialMove::PromoteBishop);
						yield (x - 1, y + 1, SpecialMove::PromoteRook);
						yield (x - 1, y + 1, SpecialMove::PromoteQueen);
					} else {
						yield (x - 1, y + 1, SpecialMove::None);
					}
				}
				if x <= 6 && get_piece(board, x + 1, y + 1).is_black() {
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
			
			Piece::WhiteKnight => {
				if x >= 1 && y <= 5 && !get_piece(board, x - 1, y + 2).is_white() {yield (x - 1, y + 2, SpecialMove::None);}
				if x <= 6 && y <= 5 && !get_piece(board, x + 1, y + 2).is_white() {yield (x + 1, y + 2, SpecialMove::None);}
				if x <= 5 && y <= 6 && !get_piece(board, x + 2, y + 1).is_white() {yield (x + 2, y + 1, SpecialMove::None);}
				if x <= 5 && y >= 1 && !get_piece(board, x + 2, y - 1).is_white() {yield (x + 2, y - 1, SpecialMove::None);}
				if x <= 6 && y >= 2 && !get_piece(board, x + 1, y - 2).is_white() {yield (x + 1, y - 2, SpecialMove::None);}
				if x >= 1 && y >= 2 && !get_piece(board, x - 1, y - 2).is_white() {yield (x - 1, y - 2, SpecialMove::None);}
				if x >= 2 && y >= 1 && !get_piece(board, x - 2, y - 1).is_white() {yield (x - 2, y - 1, SpecialMove::None);}
				if x >= 2 && y <= 6 && !get_piece(board, x - 2, y + 1).is_white() {yield (x - 2, y + 1, SpecialMove::None);}
			}
			
			Piece::WhiteBishop => {
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y <= 6 {
					curr_x += 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_black() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_black() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_black() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_black() {
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
			
			Piece::WhiteRook => {
				let mut curr_x = x;
				while curr_x <= 6 {
					curr_x += 1;
					let piece = get_piece(board, curr_x, y);
					if piece.is_black() {
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
					let piece = get_piece(board, curr_x, y);
					if piece.is_black() {
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
					let piece = get_piece(board, x, curr_y);
					if piece.is_black() {
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
					let piece = get_piece(board, x, curr_y);
					if piece.is_black() {
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
			
			Piece::WhiteQueen => {
				let mut curr_x = x;
				let mut curr_y = y;
				while curr_x <= 6 && curr_y <= 6 {
					curr_x += 1;
					curr_y += 1;
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_black() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_black() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_black() {
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
					let piece = get_piece(board, curr_x, curr_y);
					if piece.is_black() {
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
					let piece = get_piece(board, curr_x, y);
					if piece.is_black() {
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
					let piece = get_piece(board, curr_x, y);
					if piece.is_black() {
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
					let piece = get_piece(board, x, curr_y);
					if piece.is_black() {
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
					let piece = get_piece(board, x, curr_y);
					if piece.is_black() {
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
			
			Piece::WhiteKing => {
				let x_min = x.max(1) - 1;
				let y_min = y.max(1) - 1;
				let x_max = x.min(6) + 1;
				let y_max = y.min(6) + 1;
				for x in x_min..=x_max {
					for y in y_min..=y_max {
						if !get_piece(board, x, y).is_white() {
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
