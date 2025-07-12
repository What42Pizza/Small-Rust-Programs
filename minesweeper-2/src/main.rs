#![allow(unused)]
#![warn(unused_must_use)]



use std::collections::{vec_deque, VecDeque};

use rand::{rng, Rng};
use smart_read::prelude::*;



mod input;



fn main() {
	
	let width = prompt!("Field width: "; 10..);
	let height = prompt!("Field height: "; 10..);
	let bomb_count = prompt!("Bomb count: "; [width * height / 10] UsizeInput);
	
	let (start_x, start_y) = prompt!("Choose a starting location"; input::StartingTileInput {width, height});
	
	// generate field where user has uncovered many tiles
	let mut tiles = vec![Tile::default(); width * height];
	loop {
		regenerate_field(&mut tiles, width, height, bomb_count);
	}
	
}



#[derive(Copy, Clone, Debug, Default)]
pub struct Tile {
	pub neighbor_count: u8,
	pub is_bomb: bool,
	pub is_covered: bool,
}



pub enum TileAction {
	Uncover,
	Flag,
	Unflag,
}

impl TileAction {
	fn from_str(input: &str) -> Option<Self> {
		match &*input.to_ascii_lowercase() {
			"uncover" | "u" => Some(Self::Uncover),
			"flag" | "f" => Some(Self::Flag),
			"unflag" => Some(Self::Unflag),
			_ => None,
		}
	}
}



pub fn regenerate_field(tiles: &mut [Tile], width: usize, height: usize, bomb_count: usize) {
	
	// clear field
	for i in 0 .. width * height {
		tiles[i] = Tile::default();
	}
	
	// create bombs
	for _ in 0..bomb_count {
		'inner: loop {
			let tile_index = rng().random_range(0 .. width * height);
			if tiles[tile_index].is_bomb {continue 'inner;}
			tiles[tile_index].is_bomb = true;
			break 'inner;
		}
	}
	
	// fill numbers
	for y in 0..height {
		for x in 0..width {
			let mut count = 0;
			let up = y.max(1) - 1;
			let down = y.min(height - 2) + 1;
			let left = x.max(1) - 1;
			let right = x.min(width - 2) + 1;
			for y in up..=down {
				for x in left..=right {
					if tiles[x + y * width].is_bomb {count += 1;}
				}
			}
			tiles[x + y * width].neighbor_count = count;
		}
	}
	
}



// return: uncovered_tile_count (0 = uncovered mine)
pub fn uncover_tile(x: usize, y: usize, tiles: &mut [Tile], width: usize) -> usize {
	if tiles[x + y * width].is_bomb {return 0;}
	let mut tiles_to_uncover = vec!((x, y));
	let mut uncovered_count = 0;
	while !tiles_to_uncover.is_empty() {
		uncovered_count += 1;
		let (x, y) = tiles_to_uncover.swap_remove(0);
		let i = x + y * width;
		if !tiles[i].is_covered {continue;}
		tiles[i].is_covered = false;
	}
	todo!()
}
