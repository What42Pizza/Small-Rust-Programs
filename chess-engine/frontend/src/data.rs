use crate::*;



pub struct AppData {
	
	// basics
	pub settings: AppSettings,
	pub resources_path: PathBuf,
	pub should_close: bool,
	pub last_update_time: Instant,
	
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



#[derive(Debug)]
pub enum State {
	NotPlaying,
	Playing {
		time_remainings: Option<(Duration, Duration)>,
		time_per_move: Option<Duration>,
		turn: TurnData,
	},
	GameEnded (GameEndedState),
}

#[derive(Debug)]
pub enum TurnData {
	PlayersTurn (PlayersTurnState),
	EnginesTurn,
}

#[derive(Debug, Copy, Clone)]
pub enum PlayersTurnState {
	NotHoldingPiece,
	HoldingPiece {x: u8, y: u8, piece: Piece},
}

#[derive(Debug)]
pub enum GameEndedState {
	PlayerWon,
	EngineWon,
	NeitherWon,
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
	
	pub total_time: u64,
	pub time_per_move: u64,
	
	pub font_name: String,
	
	pub background_color: Color,
	pub top_bar_color: Color,
	pub top_bar_buttons_color: Color,
	pub top_bar_buttons_darkened_color: Color,
	pub board_color_dark: Color,
	pub board_color_light: Color,
	pub board_trim_color: Color,
	
}
