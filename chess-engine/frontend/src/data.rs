use crate::*;



pub struct AppData {
	
	pub settings: AppSettings,
	pub resources_path: PathBuf,
	pub should_close: bool,
	
	pub board: Board,
	
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
