use crate::*;



pub struct AppData {
	
	// basics
	pub settings: AppSettings,
	pub resources_path: PathBuf,
	pub should_close: bool,
	
	// window elements
	pub window_size: (f32, f32),
	pub mouse_state: MouseState,
	pub prev_mouse_state: MouseState,
	pub new_game_button_rect: FRect,
	pub new_game_button_down: bool,
	
	// game data
	pub board: Board,
	pub state: State,
	
}



pub enum State {
	NotPlaying,
	Playing {time_data: TimeData, turn: TurnData},
}

pub enum TurnData {
	PlayersTurn (PlayersTurnState),
	EnginesTurn,
}

pub enum PlayersTurnState {
	NotHoldingPiece,
	HoldingPiece {x: u8, y: u8, piece: Piece},
}



pub struct TimeData {
	pub player_time_remaining: Duration,
	pub engine_time_remaining: Duration,
	pub time_per_move: Duration
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
	pub top_bar_buttons_color: Color,
	pub top_bar_buttons_darkened_color: Color,
	pub board_color_dark: Color,
	pub board_color_light: Color,
	pub board_trim_color: Color,
	
}
