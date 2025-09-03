use shared::*;
use std::sync::Mutex;
use rayon::ThreadPool;



pub fn init() {
	init_attack_squares();
}



pub fn get_move(board: Board, game_flags: u8, time_remaining: Option<usize>, thread_pool: &ThreadPool) -> (u8, u8, u8, u8, MoveType) {
	
	let time_remaining = time_remaining.unwrap_or(20 * 1000);
	let search_depth = match time_remaining {
		x if x < 15 * 1000 => 4,
		x if x < 60 * 1000 => 5,
		_ => 6
	};
	
	static mut OUTPUTS: [(f32, u8, u8, MoveType); 64] = [(-100000000.0, 0, 0, MoveType::Normal); 64];
	thread_pool.scope(|s| {
		#[allow(static_mut_refs)]
		for (i, output) in unsafe {OUTPUTS.iter_mut().enumerate()} {
			s.spawn(move |_s| {
				*output = try_black_move(board, time_remaining, i as u8, game_flags, search_depth);
			});
		}
	});
	
	let (mut from_i, mut best_move) = (0, (-100000000.0, 0, 0, MoveType::Normal));
	#[allow(static_mut_refs)]
	for (i, output) in unsafe {OUTPUTS.iter().enumerate()} {
		if output.0 > best_move.0 {
			(from_i, best_move) = (i as u8, *output);
		}
	}
	
	let (from_x, from_y) = (from_i % 8, from_i / 8);
	(from_x, from_y, best_move.1, best_move.2, best_move.3)
}



static LOCK: Mutex<()> = Mutex::new(());

fn try_black_move(board: Board, ending_millis: usize, index: u8, game_flags: u8, depth: u8) -> (f32, u8, u8, MoveType) {
	let depth = depth - 1;
	let (x, y) = (index % 8, index / 8);
	let piece = get_piece(&board, x, y);
	let mut best_move = (-100000000.0, 0, 0, MoveType::Normal); // searching for the best move for the engine
	let mut alpha = -100000000.0;
	let beta = 100000000.0;
	for (move_x, move_y, move_type) in get_black_moves(&board, piece, x, y, game_flags) {
		let mut new_board = board;
		let mut new_game_flags = game_flags;
		perform_move(&mut new_board, &mut new_game_flags, piece, x, y, move_x, move_y, move_type);
		let move_score = try_white_moves(new_board, ending_millis, new_game_flags, alpha, beta, depth);
		if move_score > best_move.0 { // if found a better move for the engine
			best_move = (move_score, move_x, move_y, move_type);
		}
		alpha = alpha.max(move_score);
	}
	best_move
}

fn try_black_moves(board: Board, ending_millis: usize, game_flags: u8, mut alpha: f32, beta: f32, depth: u8) -> f32 {
	let depth = depth - 1;
	let mut score: f32 = -100000000.0; // searching for the best move for black
	for x in 0..4 {
		for y in 0..8 {
			let x = x * 2;
			let (piece1, piece2) = get_doubled_pieces(&board, x, y);
			if piece1.is_black() {for (move_x, move_y, move_type) in get_black_moves(&board, piece1, x, y, game_flags) {
				let mut new_board = board;
				let mut new_game_flags = game_flags;
				perform_move(&mut new_board, &mut new_game_flags, piece1, x, y, move_x, move_y, move_type);
				let move_score = if depth == 0 {
					get_board_score(&new_board, false)
				} else {
					try_white_moves(new_board, ending_millis, new_game_flags, alpha, beta, depth)
				};
				score = score.max(move_score);
				alpha = alpha.max(move_score);
				if alpha >= beta {return score;}
			}}
			let x = x + 1;
			if piece2.is_black() {for (move_x, move_y, move_type) in get_black_moves(&board, piece2, x, y, game_flags) {
				let mut new_board = board;
				let mut new_game_flags = game_flags;
				perform_move(&mut new_board, &mut new_game_flags, piece2, x, y, move_x, move_y, move_type);
				let move_score = if depth == 0 {
					get_board_score(&new_board, false)
				} else {
					try_white_moves(new_board, ending_millis, new_game_flags, alpha, beta, depth)
				};
				score = score.max(move_score);
				alpha = alpha.max(move_score);
				if alpha >= beta {return score;}
			}}
		}
	}
	score
}

fn try_white_moves(board: Board, ending_millis: usize, game_flags: u8, alpha: f32, mut beta: f32, depth: u8) -> f32 {
	let depth = depth - 1;
	let mut score: f32 = 100000000.0; // searching for the best move for white
	for x in 0..4 {
		for y in 0..8 {
			let x = x * 2;
			let (piece1, piece2) = get_doubled_pieces(&board, x, y);
			if piece1.is_white() {for (move_x, move_y, move_type) in get_white_moves(&board, piece1, x, y, game_flags) {
				let mut new_board = board;
				let mut new_game_flags = game_flags;
				perform_move(&mut new_board, &mut new_game_flags, piece1, x, y, move_x, move_y, move_type);
				let move_score = if depth == 0 {
					get_board_score(&new_board, true)
				} else {
					try_black_moves(new_board, ending_millis, new_game_flags, alpha, beta, depth)
				};
				score = score.min(move_score);
				beta = beta.min(move_score);
				if beta <= alpha {return score;}
			}}
			let x = x + 1;
			if piece2.is_white() {for (move_x, move_y, move_type) in get_white_moves(&board, piece2, x, y, game_flags) {
				let mut new_board = board;
				let mut new_game_flags = game_flags;
				perform_move(&mut new_board, &mut new_game_flags, piece2, x, y, move_x, move_y, move_type);
				let move_score = if depth == 0 {
					get_board_score(&new_board, true)
				} else {
					try_black_moves(new_board, ending_millis, new_game_flags, alpha, beta, depth)
				};
				score = score.min(move_score);
				beta = beta.min(move_score);
				if beta <= alpha {return score;}
			}}
		}
	}
	score
}



static mut BLACK_PAWN_ATTACK_SQUARES: [u64; 64] = [0; 64];
static mut WHITE_PAWN_ATTACK_SQUARES: [u64; 64] = [0; 64];
static mut KNIGHT_ATTACK_SQUARES: [u64; 64] = [0; 64];
static mut BISHOP_ATTACK_SQUARES: [[u64; 64]; 13] = [[0; 64]; 13];
static mut BISHOP_ATTACK_BLOCKERS: [[u64; 64]; 13] = [[0; 64]; 13];
static mut ROOK_ATTACK_SQUARES: [[u64; 64]; 14] = [[0; 64]; 14];
static mut ROOK_ATTACK_BLOCKERS: [[u64; 64]; 14] = [[0; 64]; 14];
static mut QUEEN_ATTACK_SQUARES: [[u64; 64]; 27] = [[0; 64]; 27];
static mut QUEEN_ATTACK_BLOCKERS: [[u64; 64]; 27] = [[0; 64]; 27];
static mut KING_ATTACK_SQUARES: [u64; 64] = [0; 64];

fn init_attack_squares() {
	fn bit(x: u8, y: u8) -> u64 {
		1u64 << (x + y * 8)
	}
	unsafe {
		for x in 0..8 {
			for y in 0..8 {
				let i = (x + y * 8) as usize;
				// black pawns
				if y <= 6 {
					if x >= 1 {BLACK_PAWN_ATTACK_SQUARES[i] |= bit(x - 1, y + 1);}
					if x <= 6 {BLACK_PAWN_ATTACK_SQUARES[i] |= bit(x + 1, y + 1);}
				}
				// white pawns
				if y <= 6 {
					if x >= 1 {WHITE_PAWN_ATTACK_SQUARES[i] |= bit(x - 1, y + 1);}
					if x <= 6 {WHITE_PAWN_ATTACK_SQUARES[i] |= bit(x + 1, y + 1);}
				}
				// knights
				if x >= 1 && y <= 5 {KNIGHT_ATTACK_SQUARES[i] |= bit(x - 1, y + 2);}
				if x <= 6 && y <= 5 {KNIGHT_ATTACK_SQUARES[i] |= bit(x + 1, y + 2);}
				if x <= 5 && y <= 6 {KNIGHT_ATTACK_SQUARES[i] |= bit(x + 2, y + 1);}
				if x <= 5 && y >= 1 {KNIGHT_ATTACK_SQUARES[i] |= bit(x + 2, y - 1);}
				if x <= 6 && y >= 2 {KNIGHT_ATTACK_SQUARES[i] |= bit(x + 1, y - 2);}
				if x >= 1 && y >= 2 {KNIGHT_ATTACK_SQUARES[i] |= bit(x - 1, y - 2);}
				if x >= 2 && y >= 1 {KNIGHT_ATTACK_SQUARES[i] |= bit(x - 2, y - 1);}
				if x >= 2 && y <= 6 {KNIGHT_ATTACK_SQUARES[i] |= bit(x - 2, y + 1);}
				// bishops (and queens)
				let mut i2 = 0;
				let mut fill_bishops_and_queens = |x_dir: u8, y_dir: u8| {
					let mut curr_x = x;
					let mut curr_y = y;
					let mut blockers: u64 = 0;
					while curr_x > 0 && curr_x < 7 && curr_y > 0 && curr_y < 7 {
						curr_x = curr_x.wrapping_add(x_dir);
						curr_y = curr_y.wrapping_add(y_dir);
						let bit = bit(curr_x, curr_y);
						BISHOP_ATTACK_SQUARES[i2][i] |= bit;
						BISHOP_ATTACK_BLOCKERS[i2][i] = blockers;
						QUEEN_ATTACK_SQUARES[i2][i] |= bit;
						QUEEN_ATTACK_BLOCKERS[i2][i] = blockers;
						blockers |= bit;
						i2 += 1;
					}
				};
				fill_bishops_and_queens(1, 1);
				fill_bishops_and_queens(1, 255);
				fill_bishops_and_queens(255, 1);
				fill_bishops_and_queens(255, 255);
				// rooks (and queens)
				let mut i2 = 0;
				let mut fill_rooks_and_queens = |x_dir: u8, y_dir: u8| {
					let mut curr_x = x;
					let mut curr_y = y;
					let mut blockers: u64 = 0;
					while curr_x > 0 && curr_x < 7 && curr_y > 0 && curr_y < 7 {
						curr_x = curr_x.wrapping_add(x_dir);
						curr_y = curr_y.wrapping_add(y_dir);
						let bit = bit(curr_x, curr_y);
						ROOK_ATTACK_SQUARES[i2][i] |= bit;
						ROOK_ATTACK_BLOCKERS[i2][i] = blockers;
						QUEEN_ATTACK_SQUARES[i2 + 13][i] |= bit;
						QUEEN_ATTACK_BLOCKERS[i2 + 13][i] = blockers;
						blockers |= bit;
						i2 += 1;
					}
				};
				fill_rooks_and_queens(0, 1);
				fill_rooks_and_queens(1, 0);
				fill_rooks_and_queens(0, 255);
				fill_rooks_and_queens(255, 0);
				// kings
				let x_min = x.max(1) - 1;
				let y_min = y.max(1) - 1;
				let x_max = x.min(6) + 1;
				let y_max = y.min(6) + 1;
				for x in x_min..=x_max {
					for y in y_min..=y_max {
						KING_ATTACK_SQUARES[i] |= bit(x, y);
					}
				}
			}
		}
	}
}

static BLACK_PAWN_SCORES: [f32; 64] = [
	1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 ,
	1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 ,
	1.25, 1.25, 1.25, 1.25, 1.25, 1.25, 1.25, 1.25,
	1.15, 1.15, 1.15, 1.15, 1.15, 1.15, 1.15, 1.15,
	1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 
	1.05, 1.05, 1.05, 1.05, 1.05, 1.05, 1.05, 1.05,
	1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 ,
	0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 ,
];
static BLACK_KNIGHT_SCORES: [f32; 64] = [
	3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 ,
	3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05,
	3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 ,
	2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 ,
];
static BLACK_KING_SCORES: [f32; 64] = [
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4,
	1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0,
];
static WHITE_PAWN_SCORES: [f32; 64] = [
	0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 ,
	1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 ,
	1.05, 1.05, 1.05, 1.05, 1.05, 1.05, 1.05, 1.05,
	1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 
	1.15, 1.15, 1.15, 1.15, 1.15, 1.15, 1.15, 1.15,
	1.25, 1.25, 1.25, 1.25, 1.25, 1.25, 1.25, 1.25,
	1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 ,
	1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 ,
];
static WHITE_KNIGHT_SCORES: [f32; 64] = [
	2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 ,
	3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 ,
	3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05,
	3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 ,
];
static WHITE_KING_SCORES: [f32; 64] = [
	1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0,
	1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
];

pub fn get_board_score(board: &Board, black_moves_next: bool) -> f32 {
	
	// get filled squares
	let mut filled_squares: u64 = 0;
	for (i, byte) in board.iter().copied().enumerate() {
		if byte & 0b00001111 > 0 {filled_squares |= 1 << (i * 2);}
		if byte & 0b11110000 > 0 {filled_squares |= 1 << (i * 2 + 1);}
	}
	
	// get attacked squares
	let mut attacked_squares: u64 = 0;
	if black_moves_next {
		for x in 0..4 {
			for y in 0..8 {
				let (piece1, piece2) = get_doubled_pieces(board, x * 2, y);
				let i = (x * 2 + y * 8) as usize;
				match piece1 {
					Piece::BlackPawn => unsafe {attacked_squares |= BLACK_PAWN_ATTACK_SQUARES[i];}
					Piece::BlackKnight => unsafe {attacked_squares |= KNIGHT_ATTACK_SQUARES[i];}
					Piece::BlackBishop => unsafe {
						for i2 in 0..13 {
							if filled_squares & BISHOP_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= BISHOP_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::BlackRook => unsafe {
						for i2 in 0..14 {
							if filled_squares & ROOK_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= ROOK_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::BlackQueen => unsafe {
						for i2 in 0..27 {
							if filled_squares & QUEEN_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= QUEEN_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::BlackKing => unsafe {attacked_squares |= KING_ATTACK_SQUARES[i];}
					_ => {}
				}
				let i = i + 1;
				match piece2 {
					Piece::BlackPawn => unsafe {attacked_squares |= BLACK_PAWN_ATTACK_SQUARES[i];}
					Piece::BlackKnight => unsafe {attacked_squares |= KNIGHT_ATTACK_SQUARES[i];}
					Piece::BlackBishop => unsafe {
						for i2 in 0..13 {
							if filled_squares & BISHOP_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= BISHOP_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::BlackRook => unsafe {
						for i2 in 0..14 {
							if filled_squares & ROOK_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= ROOK_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::BlackQueen => unsafe {
						for i2 in 0..27 {
							if filled_squares & QUEEN_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= QUEEN_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::BlackKing => unsafe {attacked_squares |= KING_ATTACK_SQUARES[i];}
					_ => {}
				}
			}
		}
	} else {
		for x in 0..4 {
			for y in 0..8 {
				let (piece1, piece2) = get_doubled_pieces(board, x * 2, y);
				let i = (x * 2 + y * 8) as usize;
				match piece1 {
					Piece::WhitePawn => unsafe {attacked_squares |= WHITE_PAWN_ATTACK_SQUARES[i];}
					Piece::WhiteKnight => unsafe {attacked_squares |= KNIGHT_ATTACK_SQUARES[i];}
					Piece::WhiteBishop => unsafe {
						for i2 in 0..13 {
							if filled_squares & BISHOP_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= BISHOP_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::WhiteRook => unsafe {
						for i2 in 0..14 {
							if filled_squares & ROOK_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= ROOK_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::WhiteQueen => unsafe {
						for i2 in 0..27 {
							if filled_squares & QUEEN_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= QUEEN_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::WhiteKing => unsafe {attacked_squares |= KING_ATTACK_SQUARES[i];}
					_ => {}
				}
				let i = i + 1;
				match piece2 {
					Piece::WhitePawn => unsafe {attacked_squares |= WHITE_PAWN_ATTACK_SQUARES[i];}
					Piece::WhiteKnight => unsafe {attacked_squares |= KNIGHT_ATTACK_SQUARES[i];}
					Piece::WhiteBishop => unsafe {
						for i2 in 0..13 {
							if filled_squares & BISHOP_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= BISHOP_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::WhiteRook => unsafe {
						for i2 in 0..14 {
							if filled_squares & ROOK_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= ROOK_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::WhiteQueen => unsafe {
						for i2 in 0..27 {
							if filled_squares & QUEEN_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= QUEEN_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::WhiteKing => unsafe {attacked_squares |= KING_ATTACK_SQUARES[i];}
					_ => {}
				}
			}
		}
	}
	//static LOCK: Mutex<()> = Mutex::new(());
	//let lock = LOCK.lock();
	//for y in (0..8).rev() {
	//	for x in 0..8 {
	//		let i = x + y * 8;
	//		let bit = attacked_squares & (1 << (i)) > 0;
	//		print!("{}", if bit {1} else {0});
	//	}
	//	println!();
	//}
	//drop(lock);
	//panic!();
	
	let mut black_score = 0.001;
	let mut white_score = 0.001;
	let mut black_has_king = false;
	let mut white_has_king = false;
	for x in 0..4 {
		for y in 0..8 {
			let (piece1, piece2) = get_doubled_pieces(board, x * 2, y);
			let i = (x * 2 + y * 8) as usize;
			let is_under_attack = attacked_squares & (1 << (x * 2 + y * 8)) > 0;
			match piece1 {
				Piece::BlackPawn   if !(is_under_attack && !black_moves_next) => black_score += BLACK_PAWN_SCORES[i + 1],
				Piece::BlackKnight if !(is_under_attack && !black_moves_next) => black_score += BLACK_KNIGHT_SCORES[i + 1],
				Piece::BlackBishop if !(is_under_attack && !black_moves_next) => black_score += 3.3,
				Piece::BlackRook   if !(is_under_attack && !black_moves_next) => black_score += 5.5,
				Piece::BlackQueen  if !(is_under_attack && !black_moves_next) => black_score += 9.9,
				Piece::BlackKing   if !(is_under_attack && !black_moves_next) => {black_score += BLACK_KING_SCORES[i + 1]; black_has_king = true;}
				Piece::WhitePawn   if !(is_under_attack &&  black_moves_next)=> white_score += WHITE_PAWN_SCORES[i + 1],
				Piece::WhiteKnight if !(is_under_attack &&  black_moves_next)=> white_score += WHITE_KNIGHT_SCORES[i + 1],
				Piece::WhiteBishop if !(is_under_attack &&  black_moves_next)=> white_score += 3.3,
				Piece::WhiteRook   if !(is_under_attack &&  black_moves_next)=> white_score += 5.5,
				Piece::WhiteQueen  if !(is_under_attack &&  black_moves_next)=> white_score += 9.9,
				Piece::WhiteKing   if !(is_under_attack &&  black_moves_next)=> {white_score += WHITE_KING_SCORES[i]; white_has_king = true;}
				_ => {}
			}
			match piece2 {
				Piece::BlackPawn   if !(is_under_attack && !black_moves_next) => black_score += BLACK_PAWN_SCORES[i + 1],
				Piece::BlackKnight if !(is_under_attack && !black_moves_next) => black_score += BLACK_KNIGHT_SCORES[i + 1],
				Piece::BlackBishop if !(is_under_attack && !black_moves_next) => black_score += 3.3,
				Piece::BlackRook   if !(is_under_attack && !black_moves_next) => black_score += 5.5,
				Piece::BlackQueen  if !(is_under_attack && !black_moves_next) => black_score += 9.9,
				Piece::BlackKing   if !(is_under_attack && !black_moves_next) => {black_score += BLACK_KING_SCORES[i + 1]; black_has_king = true;}
				Piece::WhitePawn   if !(is_under_attack &&  black_moves_next)=> white_score += WHITE_PAWN_SCORES[i + 1],
				Piece::WhiteKnight if !(is_under_attack &&  black_moves_next)=> white_score += WHITE_KNIGHT_SCORES[i + 1],
				Piece::WhiteBishop if !(is_under_attack &&  black_moves_next)=> white_score += 3.3,
				Piece::WhiteRook   if !(is_under_attack &&  black_moves_next)=> white_score += 5.5,
				Piece::WhiteQueen  if !(is_under_attack &&  black_moves_next)=> white_score += 9.9,
				Piece::WhiteKing   if !(is_under_attack &&  black_moves_next)=> {white_score += WHITE_KING_SCORES[i]; white_has_king = true;}
				_ => {}
			}
		}
	}
	
	if !black_has_king {
		-1000000.0
	} else if !white_has_king {
		1000000.0
	} else {
		black_score / white_score
	}
}
