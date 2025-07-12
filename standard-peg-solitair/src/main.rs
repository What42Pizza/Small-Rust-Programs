#![allow(unused)]
#![warn(unused_must_use)]



use std::time::Instant;



fn main() {
	
	let mut board = [true; 7 * 7];
	board[3 + 3 * 7] = false;
	let mut move_list = [(0, 0, 0); 31];
	let mut checked_boards = vec![0; 1073741824]; // 2 ^ 33 / 8
	let mut board_i = 8589869055; // 2 ^ 33 - 1 - 65536
	
	let start = Instant::now();
	process_moves(&mut board, &mut checked_boards, &mut board_i, &mut move_list, 0);
	println!("Time taken: {:?}", start.elapsed());
	
}


	
static mut COUNT: usize = 0;
static mut HIGHEST_DEPTH: usize = 0;
static mut IS_FINISHED: bool = false;



const BOARD_BITS: [usize; 49] = [
	0000000000, 0000000000, 0000000001, 0000000002, 0000000004, 0000000000, 0000000000,
	0000000000, 0000000000, 0000000008, 0000000016, 0000000032, 0000000000, 0000000000,
	0000000064, 0000000128, 0000000256, 0000000512, 0000001024, 0000002048, 0000004096,
	0000008192, 0000016384, 0000032768, 0000065536, 0000131072, 0000262144, 0000524288,
	0001048576, 0002097152, 0004194304, 0008388608, 0016777216, 0033554432, 0067108864,
	0000000000, 0000000000, 0134217728, 0268435456, 0536870912, 0000000000, 0000000000,
	0000000000, 0000000000, 1073741824, 2147483648, 4294967296, 0000000000, 0000000000,
];

#[unroll::unroll_for_loops]
fn process_moves(board: &mut [bool; 49], checked_boards: &mut [u8], board_i: &mut usize, move_list: &mut [(u8, u8, u8); 31], move_count: usize) {
	
	unsafe {
		let boards_chunk = *checked_boards.get_unchecked(*board_i / 8);
		let chunk_i = (*board_i % 8) as u8;
		if boards_chunk & chunk_i > 0 {
			return;
		}
		let new_boards_chunk = boards_chunk ^ chunk_i;
		*checked_boards.get_unchecked_mut(*board_i / 8) = new_boards_chunk;
	}
	
	unsafe {
		COUNT += 1;
		HIGHEST_DEPTH = HIGHEST_DEPTH.max(move_count);
		if COUNT % 10000000 == 0 {
			println!("Moves checked: {} million (highest depth: {})", COUNT / 1000000, HIGHEST_DEPTH + 0);
		}
	}
	if move_count == 31 {
		unsafe {IS_FINISHED = true;}
		println!("Found valid solution:");
		return;
	}
	
	macro_rules! try_move {
		($x:expr, $y:expr, $i:expr, $dx:expr, $dy:expr) => {
			unsafe {
				let x1 = ($x as i8 + $dx) as u8;
				let y1 = ($y as i8 + $dy) as u8;
				let x2 = ($x as i8 + $dx * 2) as u8;
				let y2 = ($y as i8 + $dy * 2) as u8;
				let i1 = (x1 + y1 * 7) as u8;
				let i2 = (x2 + y2 * 7) as u8;
				if
					pos_is_valid(x2, y2)
					&& pos_is_valid(x1, y1)
					&& *board.get_unchecked(i1 as usize)
					&& *board.get_unchecked(i2 as usize)
				{
					*board.get_unchecked_mut($i as usize) = true;
					*board_i += *BOARD_BITS.get_unchecked($i as usize);
					*board.get_unchecked_mut(i1 as usize) = false;
					*board_i -= *BOARD_BITS.get_unchecked(i1 as usize);
					*board.get_unchecked_mut(i2 as usize) = false;
					*board_i -= *BOARD_BITS.get_unchecked(i2 as usize);
					process_moves(board, checked_boards, board_i, move_list, move_count + 1);
					*board.get_unchecked_mut($i as usize) = false;
					*board_i -= *BOARD_BITS.get_unchecked($i as usize);
					*board.get_unchecked_mut(i1 as usize) = true;
					*board_i += *BOARD_BITS.get_unchecked(i1 as usize);
					*board.get_unchecked_mut(i2 as usize) = true;
					*board_i += *BOARD_BITS.get_unchecked(i2 as usize);
					if IS_FINISHED {
						println!("Move {}: pos {}, {} dir {}, {}", move_count + 1, $x + 1, $y + 1, -$dx, -$dy);
						return;
					}
				}
			}
		};
	}
	
	for y_usize in 0..7 {
		for x_usize in 0..7 {
			'a: {
				let (x, y) = (x_usize as u8, y_usize as u8);
				if !pos_is_valid(x, y) {break 'a;}
				let i = (x + y * 7) as u8;
				unsafe {if *board.get_unchecked(i as usize) {break 'a;}}
				try_move!(x, y, i,  0i8, -1i8);
				try_move!(x, y, i,  0i8,  1i8);
				try_move!(x, y, i, -1i8,  0i8);
				try_move!(x, y, i,  1i8,  0i8);
			}
		}
	}
	
}



fn pos_is_valid(x: u8, y: u8) -> bool {
	if x >= 7 || y >= 7 {return false;}
	let extreme_x = x < 2 || x > 4;
	let extreme_y = y < 2 || y > 4;
	!(extreme_x && extreme_y)
}

/*
	# # #
	# # #
# # # # # # #
# # # O # # #
# # # # # # #
	# # #
	# # #
*/
