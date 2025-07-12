// started 22/07/09
// updated 25/05/23



use std::error::Error;
use rand::{self, Rng};

mod input;
mod errors;
mod helpers;



#[derive(Clone)]
struct Tile {
	is_bomb: bool,
	bomb_count: u32,
	is_covered: bool,
	is_flagged: bool,
	is_selected: bool
}

impl Tile {
	
	fn new (is_bomb: bool) -> Self {
		Tile {
			is_bomb,
			bomb_count: 0,
			is_covered: true,
			is_flagged: false,
			is_selected: false,
		}
	}
	
	fn to_char (&self, show_bombs: bool) -> char {
		if show_bombs && self.is_bomb {return '*';}
		if self.is_selected {return 'X';}
		if self.is_flagged {return 'O';}
		if self.is_covered {return '#';}
		if self.bomb_count == 0 {return ' ';}
		return char::from_u32(self.bomb_count + 48).unwrap();
	}
	
}



struct Field {
	width: usize,
	height: usize,
    tiles: Vec<Vec<Tile>>,
	is_exploded: bool,
}

impl Field {
	
	fn new (width: usize, height: usize) -> Field {
		let column = vec![Tile::new(false); height];
		let tiles = vec![column; width];
		Field {
			width: width,
			height: height,
			tiles: tiles,
			is_exploded: false,
		}
	}
	
	fn print (&self, show_bombs: bool) {
		
		let highest_x_len = helpers::usize_len(self.width );
		let highest_y_len = helpers::usize_len(self.height);
		
		let mut first_line = String::from("");
		for i in 0..self.width {
			let i_as_string = (i+1).to_string();
			first_line += &helpers::pad_string(i_as_string, highest_x_len + 1);
		}
		println!("{} {}", " ".repeat(highest_y_len), first_line);
		
		for y in 0..self.height {
			let mut line: String = String::from("");
			for x in 0..self.width {
				let tile_as_string = self.tiles[x][y].to_char(show_bombs).to_string();
				line += &helpers::pad_string(tile_as_string, highest_x_len + 1);
			}
			let y_as_string = helpers::pad_string((y+1).to_string(), highest_y_len);
			println!("{} {}", y_as_string, line);
		}
	}
	
	fn is_finished (&self) -> bool {
		
		if self.is_exploded {return true;}
		
		for y in 0..self.height {
			for x in 0..self.width {
				let current_tile = &self.tiles[x][y];
				if current_tile.is_bomb {continue;}
				if current_tile.is_covered && !current_tile.is_flagged {return false;}
			}
		}
		
		return true;
		
	}
	
}










fn main() -> Result<(), Box<dyn Error>> {
	
	println!("field width:");
	let width = input::get_int()?;
	println!("field height:");
	let height = input::get_int()?;
	println!("Bomb percent:  (recommended amount: 10%)");
	let bomb_percent = input::get_int_bounded (5, 20)? as f32 / 100.;
	let mut field = generate_field (width, height, bomb_percent);
	
	loop {
		let continue_game = make_move(&mut field)?;
		if !continue_game {break;}
	}
	
	Ok(())
	
}





fn generate_field (width: usize, height: usize, bomb_percent: f32) -> Field {
    let mut rng = rand::rng();
	let mut field = Field::new(width, height);
	
	for y in 0..height {
		for x in 0..width {
			if rng.random::<f32>() < bomb_percent {
				field.tiles[x][y].is_bomb = true;
			}
		}
	}
	
	for y in 0..height {
		for x in 0..width {
			field.tiles[x][y].bomb_count = get_bomb_count (&field, x, y);
		}
	}
	
	field
}



fn get_bomb_count (field: &Field, x: usize, y: usize) -> u32 {
	
	let left_edge = x == 0;
	let right_edge = x == field.width - 1;
	let top_edge = y == 0;
	let bottom_edge = y == field.height - 1;
	
	let mut count = 0;
	if !top_edge    && !left_edge  && field.tiles[x-1][y-1].is_bomb {count += 1;}
	if !top_edge    &&                field.tiles[x  ][y-1].is_bomb {count += 1;}
	if !top_edge    && !right_edge && field.tiles[x+1][y-1].is_bomb {count += 1;}
	if                 !left_edge  && field.tiles[x-1][y  ].is_bomb {count += 1;}
	if                 !right_edge && field.tiles[x+1][y  ].is_bomb {count += 1;}
	if !bottom_edge && !left_edge  && field.tiles[x-1][y+1].is_bomb {count += 1;}
	if !bottom_edge &&                field.tiles[x  ][y+1].is_bomb {count += 1;}
	if !bottom_edge && !right_edge && field.tiles[x+1][y+1].is_bomb {count += 1;}
	
	count
	
}



fn make_move (field: &mut Field) -> Result<bool, Box<dyn Error>> {
	
	let (x_pos, y_pos, action) = choose_tile (field)?;
	
	match &*action {
		
		"uncover" => {
			uncover_and_propagate_tile (field, x_pos, y_pos);
		},
		
		"flag" => {
			flag_tile (field, x_pos, y_pos);
		}
		
		"unflag" => {
			unflag_tile (field, x_pos, y_pos);
		}
		
		_ => {unreachable!();}
	}
	
	if !field.is_finished() {
		return Ok(true);
	}
	
	if field.is_exploded {
		print!("\n\n\n");
		field.print (true);
		println!("Game over.");
	} else {
		print!("\n\n\n");
		field.print (true);
		println!("You finished!");
	}
	
	Ok(false)
	
}



fn choose_tile (field: &mut Field) -> Result<(usize, usize, String), Box<dyn Error>> {
	loop {
		
		print!("\n\n\n");
		field.print (false);
		
		println!("\nX position:");
		let x_pos = input::get_int_bounded(1, field.width)? - 1;
		println!("Y position:");
		let y_pos = input::get_int_bounded(1, field.height)? - 1;
		
		field.tiles[x_pos][y_pos].is_selected = true;
		field.print (false);
		field.tiles[x_pos][y_pos].is_selected = false;
		
		println!("\nWhat would you like to do at this location?");
		let action = input::get_string_bounded(vec!["uncover", "flag", "unflag", "choose other spot"])?;
		
		match &*action {
			"uncover" => {return Ok((x_pos, y_pos, action));},
			"flag" => {return Ok((x_pos, y_pos, action));},
			"unflag" => {return Ok((x_pos, y_pos, action));}
			"choose other spot" => {continue;},
			_ => {unreachable!();},
		}
		
	}
}



fn uncover_and_propagate_tile (field: &mut Field, x_pos: usize, y_pos: usize) {
	
	let tile = &field.tiles[x_pos][y_pos];
	
	if tile.is_flagged {
		println!("\n\nFlagged tiles cannot be uncovered.");
		return;
	}
	
	uncover_tile (field, x_pos, y_pos);
	
	let mut positions_to_propagate = vec![(x_pos, y_pos)];
	loop {
		if positions_to_propagate.len() == 0 {break;}
		let (current_x, current_y) = positions_to_propagate.pop().unwrap();
		let current_tile = &field.tiles[current_x][current_y];
		if current_tile.is_bomb || current_tile.bomb_count > 0 {continue;}
		expose_surrounding_tiles (field, current_x, current_y, &mut positions_to_propagate);
	}
	
}



fn uncover_tile (field: &mut Field, x_pos: usize, y_pos: usize) {
	
	let tile = &mut field.tiles[x_pos][y_pos];
	
	if tile.is_bomb {
		field.is_exploded = true;
		return;
	}
	
	tile.is_covered = false;
	
}



fn expose_surrounding_tiles (field: &mut Field, current_x: usize, current_y: usize, positions_to_propagate: &mut Vec<(usize, usize)>) {
	
	let left_edge = current_x == 0;
	let right_edge = current_x == field.width - 1;
	let top_edge = current_y == 0;
	let bottom_edge = current_y == field.height - 1;
	
	if !top_edge    && !left_edge  {expose_tile (field, current_x-1, current_y-1, positions_to_propagate);}
	if !top_edge                   {expose_tile (field, current_x  , current_y-1, positions_to_propagate);}
	if !top_edge    && !right_edge {expose_tile (field, current_x+1, current_y-1, positions_to_propagate);}
	if                 !left_edge  {expose_tile (field, current_x-1, current_y  , positions_to_propagate);}
	if                 !right_edge {expose_tile (field, current_x+1, current_y  , positions_to_propagate);}
	if !bottom_edge && !left_edge  {expose_tile (field, current_x-1, current_y+1, positions_to_propagate);}
	if !bottom_edge                {expose_tile (field, current_x  , current_y+1, positions_to_propagate);}
	if !bottom_edge && !right_edge {expose_tile (field, current_x+1, current_y+1, positions_to_propagate);}
	
}

fn expose_tile (field: &mut Field, current_x: usize, current_y: usize, positions_to_propagate: &mut Vec<(usize, usize)>) {
	
	let tile = &mut field.tiles[current_x][current_y];
	
	if !tile.is_covered {return;}
	
	tile.is_covered = false;
	
	if tile.bomb_count == 0 {positions_to_propagate.push((current_x, current_y));}
	
}



fn flag_tile (field: &mut Field, x_pos: usize, y_pos: usize) {
	
	let tile = &mut field.tiles[x_pos][y_pos];
	
	if !tile.is_covered {
		println!("\n\nThis tile is uncovered, no need to flag it.");
		return;
	}
	
	tile.is_flagged = true;
	
}



fn unflag_tile (field: &mut Field, x_pos: usize, y_pos: usize) {
	
	let tile = &mut field.tiles[x_pos][y_pos];
	
	if !tile.is_covered {
		println!("\n\nThis tile is uncovered, no need to unflag it.");
		return;
	}
	
	tile.is_flagged = false;
	
}
