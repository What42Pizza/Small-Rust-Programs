use std::collections::HashSet;

use crate::*;
use sdl3::pixels::Color;



pub struct AppData {
	pub should_close: bool,
	pub board: Board,
}

// (char, size, foreground, background) -> (texture, x_offset, y_offset)
#[derive(Default)]
pub struct TextCache<'a> {
	pub map: HashMap<(char, usize, Color, Color), (Texture<'a>, f32, f32)>,
	pub set: HashSet<(char, usize, Color, Color)>,
}
