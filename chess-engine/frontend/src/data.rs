use crate::*;



pub struct AppData {
	
	pub settings: AppSettings,
	pub resources_path: PathBuf,
	pub should_close: bool,
	
	pub board: Board,
	
}



pub struct Textures<'a> {
	pub black_pawn: Texture<'a>,
	pub black_knight: Texture<'a>,
	pub black_bishop: Texture<'a>,
	pub black_rook: Texture<'a>,
	pub black_queen: Texture<'a>,
	pub black_king: Texture<'a>,
	pub white_pawn: Texture<'a>,
	pub white_knight: Texture<'a>,
	pub white_bishop: Texture<'a>,
	pub white_rook: Texture<'a>,
	pub white_queen: Texture<'a>,
	pub white_king: Texture<'a>,
}



pub struct AppSettings {
	
	pub last_modified_time: SystemTime,
	
	pub font_name: String,
	
	pub background_color: Color,
	pub top_bar_color: Color,
	pub board_color_dark: Color,
	pub board_color_light: Color,
	pub board_trim_color: Color,
	
}
