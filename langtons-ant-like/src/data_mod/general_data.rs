use crate::prelude::*;
use notan::random::rand::thread_rng;



#[derive(AppState)]
pub struct ProgramData {
	pub exit: bool,
	
	pub gui: GuiElement<CustomGuiData>,
	pub textures: Textures,
	pub rendering_font: RenderingFont,
	pub positioning_font: PositioningFont,
	pub last_screen_size: (u32, u32),
	
	pub ant: Ant,
	pub ant_controls: AntControls,
	
	pub camera_pos: (f32, f32),
	pub camera_zoom: f32,
	
	pub canvas: Canvas,
}



pub struct Textures {}



pub struct Ant {
	pub is_active: bool,
	pub rules: Vec<AntRule>,
	pub pos: (usize, usize),
	pub dir: u8,
}

impl Ant {
	pub fn new(canvas_size: (usize, usize), rules: Vec<AntRule>) -> Self {
		Self {
			is_active: true,
			rules,
			pos: (canvas_size.0 / 2, canvas_size.1 / 2),
			dir: 0,
		}
	}
}

#[derive(Clone)]
pub struct AntRule {
	pub next_val: u8,
	pub move_vec: (isize, isize),
	pub dir_change: u8,
}

impl AntRule {
	pub const fn new(next_val: u8, move_vec: (isize, isize), dir_change: u8) -> Self {
		Self {
			next_val,
			move_vec,
			dir_change,
		}
	}
	pub fn random(color_count: u8, max_move_len: usize) -> Self {
		let max_move_len = max_move_len as isize;
		let mut rng = thread_rng();
		let next_val = rng.gen_range(0..color_count);
		let move_vec_x = rng.gen_range(-max_move_len..=max_move_len);
		let move_vec_y = rng.gen_range(-max_move_len..=max_move_len);
		let move_vec = (move_vec_x, move_vec_y);
		let dir_change = rng.gen_range(0..4);
		Self {
			next_val,
			move_vec,
			dir_change,
		}
	}
}



pub struct AntControls {
	pub is_running: bool,
	pub steps_per_frame: usize,
	pub as_fast_as_possible: bool,
}

impl Default for AntControls {
	fn default() -> Self {
		Self {
			is_running: true,
			steps_per_frame: 1,
			as_fast_as_possible: false,
		}
	}
}



pub struct Canvas {
	
	pub raw_data: Vec<u8>,
	pub texture_datas: Vec<u32>,
	
	pub textures: Vec<CanvasTexture>,
	
	pub raw_data_size: (usize, usize),
	pub textures_size: (usize, usize),
	
	pub colors: Vec<Color>,
	
}

impl Canvas {
	pub fn get_cell_index(&self, pos: (usize, usize)) -> usize {
		pos.0 + pos.1 * self.raw_data_size.0
	}
	pub fn get_texture_index(&self, pos: (usize, usize)) -> usize {
		pos.0 + pos.1 * self.textures_size.0
	}
	pub fn get_texture_index_of_cell(&self, pos: (usize, usize)) -> usize {
		self.get_texture_index((pos.0 / TEXTURE_SIZE, pos.1 / TEXTURE_SIZE))
	}
}



pub struct CanvasTexture {
	pub texture: Option<Texture>,
	pub is_dirty: bool,
}
