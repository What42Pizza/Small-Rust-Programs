use crate::*;
use std::{array, mem::MaybeUninit, ops::{Coroutine, CoroutineState}, pin::Pin, sync::{atomic::AtomicUsize, Mutex}, thread::ScopedJoinHandle};
use rayon::ThreadPool;

mod engine_utils;
use engine_utils::*;



pub fn init() {
	init_attack_squares();
}



pub fn make_move(board: &mut Board, game_flags: &mut u8, time_remaining: Option<usize>, thread_pool: &ThreadPool) {
	
	let time_remaining = if let Some(time_remaining) = time_remaining {
		(time_remaining as f32).powf(0.7) as usize
	} else {20 * 1000};
	
	static mut OUTPUTS: [(f32, u8, u8, SpecialMove); 64] = [(-100000000.0, 0, 0, SpecialMove::None); 64];
	thread_pool.scope(|s| {
		for i in 0..64 {
			let board = *board;
			let game_flags = *game_flags;
			s.spawn(move |_s| {
				let output = try_self_move(board, time_remaining, i as u8, game_flags, 7);
				unsafe {
					OUTPUTS[i] = output;
				}
			});
		}
	});
	
	let (mut from_i, mut best_move) = (0, (-100000000.0, 0, 0, SpecialMove::None));
	#[allow(static_mut_refs)]
	for (i, output) in unsafe {OUTPUTS.iter().enumerate()} {
		if output.0 > best_move.0 {
			(from_i, best_move) = (i as u8, *output);
		}
	}
	
	let (x, y) = (from_i % 8, from_i / 8);
	let piece = get_piece(board, x, y, 66);
	perform_move(board, game_flags, piece, x, y, best_move.1, best_move.2, best_move.3);
	
	//unsafe {
	//	#[allow(static_mut_refs)]
	//	let count = COUNTER.load(std::sync::atomic::Ordering::Relaxed);
	//	panic!("{count}");
	//}
	
}



static LOCK: Mutex<()> = Mutex::new(());

fn try_self_move(board: Board, ending_millis: usize, index: u8, game_flags: u8, depth: u8) -> (f32, u8, u8, SpecialMove) {
	let depth = depth - 1;
	let (x, y) = (index % 8, index / 8);
	let piece = get_piece(&board, x, y, 67);
	let mut best_move = (-100000000.0, 0, 0, SpecialMove::None); // searching for the best move for the engine
	let mut alpha = -100000000.0;
	let mut beta = 100000000.0;
	for (move_x, move_y, move_type) in get_self_moves(&board, piece, x, y, game_flags) {
		let mut new_board = board;
		let mut new_game_flags = game_flags;
		perform_move(&mut new_board, &mut new_game_flags, piece, x, y, move_x, move_y, move_type);
		let move_score = try_other_moves(new_board, ending_millis, new_game_flags, alpha, beta, depth);
		if move_score > best_move.0 { // if found a better move for the engine
			best_move = (move_score, move_x, move_y, move_type);
		}
		alpha = alpha.max(move_score);
	}
	best_move
}

fn try_self_moves(board: Board, ending_millis: usize, game_flags: u8, mut alpha: f32, mut beta: f32, depth: u8) -> f32 {
	let depth = depth - 1;
	let mut score: f32 = -100000000.0; // searching for the best move for the engine
	//let mut move_count = 0;
	for x in 0..4 {
		for y in 0..8 {
			let x = x * 2;
			let (piece1, piece2) = get_doubled_pieces(&board, x, y);
			if piece1.is_self() {for (move_x, move_y, move_type) in get_self_moves(&board, piece1, x, y, game_flags) {
				//move_count += 1;
				let mut new_board = board;
				let mut new_game_flags = game_flags;
				perform_move(&mut new_board, &mut new_game_flags, piece1, x, y, move_x, move_y, move_type);
				let move_score = if depth == 0 {
					get_board_score(&new_board, false)
				} else {
					try_other_moves(new_board, ending_millis, new_game_flags, alpha, beta, depth)
				};
				score = score.max(move_score);
				alpha = alpha.max(move_score);
				if alpha >= beta {return score;}
			}}
			let x = x + 1;
			if piece2.is_self() {for (move_x, move_y, move_type) in get_self_moves(&board, piece2, x, y, game_flags) {
				//move_count += 1;
				let mut new_board = board;
				let mut new_game_flags = game_flags;
				perform_move(&mut new_board, &mut new_game_flags, piece2, x, y, move_x, move_y, move_type);
				let move_score = if depth == 0 {
					get_board_score(&new_board, false)
				} else {
					try_other_moves(new_board, ending_millis, new_game_flags, alpha, beta, depth)
				};
				score = score.max(move_score);
				alpha = alpha.max(move_score);
				if alpha >= beta {return score;}
			}}
		}
	}
	//if move_count > 0 {score} else {0.0}
	score
}

fn try_other_moves(board: Board, ending_millis: usize, game_flags: u8, mut alpha: f32, mut beta: f32, depth: u8) -> f32 {
	let depth = depth - 1;
	let mut score: f32 = 100000000.0; // searching for the best move for the player
	//let mut move_count = 0;
	for x in 0..4 {
		for y in 0..8 {
			let x = x * 2;
			let (piece1, piece2) = get_doubled_pieces(&board, x, y);
			if piece1.is_other() {for (move_x, move_y, move_type) in get_other_moves(&board, piece1, x, y, game_flags) {
				//move_count += 1;
				let mut new_board = board;
				let mut new_game_flags = game_flags;
				perform_move(&mut new_board, &mut new_game_flags, piece1, x, y, move_x, move_y, move_type);
				let move_score = if depth == 0 {
					get_board_score(&new_board, true)
				} else {
					try_self_moves(new_board, ending_millis, new_game_flags, alpha, beta, depth)
				};
				score = score.min(move_score);
				beta = beta.min(move_score);
				if beta <= alpha {return score;}
			}}
			let x = x + 1;
			if piece2.is_other() {for (move_x, move_y, move_type) in get_other_moves(&board, piece2, x, y, game_flags) {
				//move_count += 1;
				let mut new_board = board;
				let mut new_game_flags = game_flags;
				perform_move(&mut new_board, &mut new_game_flags, piece2, x, y, move_x, move_y, move_type);
				let move_score = if depth == 0 {
					get_board_score(&new_board, true)
				} else {
					try_self_moves(new_board, ending_millis, new_game_flags, alpha, beta, depth)
				};
				score = score.min(move_score);
				beta = beta.min(move_score);
				if beta <= alpha {return score;}
			}}
		}
	}
	//if move_count > 0 {score} else {0.0}
	score
}

fn perform_move(board: &mut Board, game_flags: &mut u8, piece: Piece, from_x: u8, from_y: u8, to_x: u8, to_y: u8, move_type: SpecialMove) {
	*game_flags &= 0b00001111; // reset en passant data
	set_piece(board, from_x, from_y, Piece::None);
	set_piece(board, to_x, to_y, piece);
	match move_type {
		SpecialMove::None => {}
		SpecialMove::EnPassant => {
			set_piece(board, to_x, from_y, Piece::None);
		}
		SpecialMove::CastleKingsSide => {
			set_piece(board, 7, to_y, Piece::None);
			set_piece(board, 5, to_y, Piece::SelfRook.copy_owner(piece));
			*game_flags &= if piece.is_other() {0b11111100} else {0b11110011};
		}
		SpecialMove::CastleQueensSide => {
			set_piece(board, 0, to_y, Piece::None);
			set_piece(board, 3, to_y, Piece::SelfRook.copy_owner(piece));
			*game_flags &= if piece.is_other() {0b11111100} else {0b11110011};
		}
		SpecialMove::PromoteKnight => {
			set_piece(board, to_x, to_y, Piece::OtherKnight.copy_owner(piece));
		}
		SpecialMove::PromoteBishop => {
			set_piece(board, to_x, to_y, Piece::OtherBishop.copy_owner(piece));
		}
		SpecialMove::PromoteRook => {
			set_piece(board, to_x, to_y, Piece::OtherRook.copy_owner(piece));
		}
		SpecialMove::PromoteQueen => {
			set_piece(board, to_x, to_y, Piece::OtherQueen.copy_owner(piece));
		}
	}
	if piece as u8 & 0b111 == Piece::SelfPawn as u8 && to_y.abs_diff(from_y) == 2 {
		*game_flags |= (to_x << 5) | 0b00010000; // allow en passant for next move
	}
}



static mut SELF_PAWN_ATTACK_SQUARES: [u64; 64] = [0; 64];
static mut OTHER_PAWN_ATTACK_SQUARES: [u64; 64] = [0; 64];
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
				// self pawns
				if y <= 6 {
					if x >= 1 {SELF_PAWN_ATTACK_SQUARES[i] |= bit(x - 1, y + 1);}
					if x <= 6 {SELF_PAWN_ATTACK_SQUARES[i] |= bit(x + 1, y + 1);}
				}
				// other pawns
				if y <= 6 {
					if x >= 1 {OTHER_PAWN_ATTACK_SQUARES[i] |= bit(x - 1, y + 1);}
					if x <= 6 {OTHER_PAWN_ATTACK_SQUARES[i] |= bit(x + 1, y + 1);}
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

static SELF_PAWN_SCORES: [f32; 64] = [
	1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 ,
	1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 ,
	1.25, 1.25, 1.25, 1.25, 1.25, 1.25, 1.25, 1.25,
	1.15, 1.15, 1.15, 1.15, 1.15, 1.15, 1.15, 1.15,
	1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 
	1.05, 1.05, 1.05, 1.05, 1.05, 1.05, 1.05, 1.05,
	1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 ,
	0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 ,
];
static SELF_KNIGHT_SCORES: [f32; 64] = [
	3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 ,
	3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05,
	3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 ,
	2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 ,
];
static SELF_KING_SCORES: [f32; 64] = [
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4,
	1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0,
];
static OTHER_PAWN_SCORES: [f32; 64] = [
	0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 , 0.9 ,
	1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 ,
	1.05, 1.05, 1.05, 1.05, 1.05, 1.05, 1.05, 1.05,
	1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 1.1 , 
	1.15, 1.15, 1.15, 1.15, 1.15, 1.15, 1.15, 1.15,
	1.25, 1.25, 1.25, 1.25, 1.25, 1.25, 1.25, 1.25,
	1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 , 1.5 ,
	1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 , 1.0 ,
];
static OTHER_KNIGHT_SCORES: [f32; 64] = [
	2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 , 2.9 ,
	3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 ,
	3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.1 , 3.05,
	3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05, 3.05,
	3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 , 3.0 ,
];
static OTHER_KING_SCORES: [f32; 64] = [
	1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0, 1000000.0 - 0.0,
	1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4, 1000000.0 - 0.4,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
	1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6, 1000000.0 - 0.6,
];

pub fn get_board_score(board: &Board, engine_moves_next: bool) -> f32 {
	let mut filled_squares: u64 = 0;
	for (i, byte) in board.iter().copied().enumerate() {
		if byte & 0b00001111 > 0 {filled_squares |= 1 << (i * 2);}
		if byte & 0b11110000 > 0 {filled_squares |= 1 << (i * 2 + 1);}
	}
	let mut attacked_squares: u64 = 0;
	
	if engine_moves_next {
		for x in 0..4 {
			for y in 0..8 {
				let (piece1, piece2) = get_doubled_pieces(&board, x * 2, y);
				let i = (x * 2 + y * 8) as usize;
				match piece1 {
					Piece::SelfPawn => unsafe {attacked_squares |= SELF_PAWN_ATTACK_SQUARES[i];}
					Piece::SelfKnight => unsafe {attacked_squares |= KNIGHT_ATTACK_SQUARES[i];}
					Piece::SelfBishop => unsafe {
						for i2 in 0..13 {
							if filled_squares & BISHOP_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= BISHOP_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::SelfRook => unsafe {
						for i2 in 0..14 {
							if filled_squares & ROOK_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= ROOK_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::SelfQueen => unsafe {
						for i2 in 0..27 {
							if filled_squares & QUEEN_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= QUEEN_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::SelfKing => unsafe {attacked_squares |= KING_ATTACK_SQUARES[i];}
					_ => {}
				}
				let i = i + 1;
				match piece2 {
					Piece::SelfPawn => unsafe {attacked_squares |= SELF_PAWN_ATTACK_SQUARES[i];}
					Piece::SelfKnight => unsafe {attacked_squares |= KNIGHT_ATTACK_SQUARES[i];}
					Piece::SelfBishop => unsafe {
						for i2 in 0..13 {
							if filled_squares & BISHOP_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= BISHOP_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::SelfRook => unsafe {
						for i2 in 0..14 {
							if filled_squares & ROOK_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= ROOK_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::SelfQueen => unsafe {
						for i2 in 0..27 {
							if filled_squares & QUEEN_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= QUEEN_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::SelfKing => unsafe {attacked_squares |= KING_ATTACK_SQUARES[i];}
					_ => {}
				}
			}
		}
	} else {
		for x in 0..4 {
			for y in 0..8 {
				let (piece1, piece2) = get_doubled_pieces(&board, x * 2, y);
				let i = (x * 2 + y * 8) as usize;
				match piece1 {
					Piece::OtherPawn => unsafe {attacked_squares |= OTHER_PAWN_ATTACK_SQUARES[i];}
					Piece::OtherKnight => unsafe {attacked_squares |= KNIGHT_ATTACK_SQUARES[i];}
					Piece::OtherBishop => unsafe {
						for i2 in 0..13 {
							if filled_squares & BISHOP_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= BISHOP_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::OtherRook => unsafe {
						for i2 in 0..14 {
							if filled_squares & ROOK_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= ROOK_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::OtherQueen => unsafe {
						for i2 in 0..27 {
							if filled_squares & QUEEN_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= QUEEN_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::OtherKing => unsafe {attacked_squares |= KING_ATTACK_SQUARES[i];}
					_ => {}
				}
				let i = i + 1;
				match piece2 {
					Piece::OtherPawn => unsafe {attacked_squares |= OTHER_PAWN_ATTACK_SQUARES[i];}
					Piece::OtherKnight => unsafe {attacked_squares |= KNIGHT_ATTACK_SQUARES[i];}
					Piece::OtherBishop => unsafe {
						for i2 in 0..13 {
							if filled_squares & BISHOP_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= BISHOP_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::OtherRook => unsafe {
						for i2 in 0..14 {
							if filled_squares & ROOK_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= ROOK_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::OtherQueen => unsafe {
						for i2 in 0..27 {
							if filled_squares & QUEEN_ATTACK_BLOCKERS[i2][i] == 0 {attacked_squares |= QUEEN_ATTACK_SQUARES[i2][i];}
						}
					}
					Piece::OtherKing => unsafe {attacked_squares |= KING_ATTACK_SQUARES[i];}
					_ => {}
				}
			}
		}
	}
	//for y in (0..8).rev() {
	//	for x in 0..8 {
	//		let i = x + y * 8;
	//		let bit = attacked_squares & (1 << (i)) > 0;
	//		print!("{}", if bit {1} else {0});
	//	}
	//	println!();
	//}
	
	let mut engine_score = 0.001;
	let mut player_score = 0.001;
	let mut engine_has_king = false;
	let mut player_has_king = false;
	for x in 0..4 {
		for y in 0..8 {
			let (piece1, piece2) = get_doubled_pieces(&board, x * 2, y);
			let i = (x * 2 + y * 8) as usize;
			let is_under_attack = attacked_squares & (1 << (x * 2 + y * 8)) > 0;
			match piece1 {
				Piece::SelfPawn   if !is_under_attack => engine_score += SELF_PAWN_SCORES[i],
				Piece::SelfKnight if !is_under_attack => engine_score += SELF_KNIGHT_SCORES[i],
				Piece::SelfBishop if !is_under_attack => engine_score += 3.3,
				Piece::SelfRook   if !is_under_attack => engine_score += 5.5,
				Piece::SelfQueen  if !is_under_attack => engine_score += 9.9,
				Piece::SelfKing   if !is_under_attack => {engine_score += SELF_KING_SCORES[i]; engine_has_king = true;}
				Piece::OtherPawn   => player_score += OTHER_PAWN_SCORES[i],
				Piece::OtherKnight => player_score += OTHER_KNIGHT_SCORES[i],
				Piece::OtherBishop => player_score += 3.3,
				Piece::OtherRook   => player_score += 5.5,
				Piece::OtherQueen  => player_score += 9.9,
				Piece::OtherKing   => {player_score += OTHER_KING_SCORES[i]; player_has_king = true;}
				_ => {}
			}
			match piece2 {
				Piece::SelfPawn   if !is_under_attack => engine_score += SELF_PAWN_SCORES[i + 1],
				Piece::SelfKnight if !is_under_attack => engine_score += SELF_KNIGHT_SCORES[i + 1],
				Piece::SelfBishop if !is_under_attack => engine_score += 3.3,
				Piece::SelfRook   if !is_under_attack => engine_score += 5.5,
				Piece::SelfQueen  if !is_under_attack => engine_score += 9.9,
				Piece::SelfKing   if !is_under_attack => {engine_score += SELF_KING_SCORES[i + 1]; engine_has_king = true;}
				Piece::OtherPawn   => player_score += OTHER_PAWN_SCORES[i + 1],
				Piece::OtherKnight => player_score += OTHER_KNIGHT_SCORES[i + 1],
				Piece::OtherBishop => player_score += 3.3,
				Piece::OtherRook   => player_score += 5.5,
				Piece::OtherQueen  => player_score += 9.9,
				Piece::OtherKing   => {player_score += OTHER_KING_SCORES[i]; player_has_king = true;}
				_ => {}
			}
		}
	}
	
	//unsafe {
	//	#[allow(static_mut_refs)]
	//	COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
	//}
	
	if !engine_has_king {
		-1000000.0
	} else if !player_has_king {
		1000000.0
	} else {
		engine_score / player_score
	}
}

//static mut COUNTER: AtomicUsize = AtomicUsize::new(0);
