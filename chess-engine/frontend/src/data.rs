use crate::*;



pub static THREAD_POOL: LazyLock<ThreadPool> = LazyLock::new(|| rayon::ThreadPoolBuilder::new().num_threads(16).build().unwrap());

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
	pub game_flags: u8, // flags: 0: can castle with player left rook, 1: can castle with player right rook, 2: can castle with engine left rook, 3: can castle with engine right rook, 4: can en passant, 5-7: en passant file
	pub state: State,
	pub engine_move: Arc<Mutex<Option<(u8, u8, u8, u8, MoveType)>>>,
	pub ring_selectors: Option<(u8, u8, u8, u8)>,
	
}



#[derive(Debug)]
pub enum State {
	NotPlaying,
	Playing {
		time_remainings: Option<(Duration, Duration)>,
		time_per_move: Option<Duration>,
		turn: TurnState,
	},
	GameEnded (GameEndedState),
}

#[derive(Debug, PartialEq)]
pub enum TurnState {
	PlayersTurn (PlayersTurnState),
	EnginesTurn,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PlayersTurnState {
	NotHoldingPiece,
	HoldingPiece {x: u8, y: u8, piece: Piece},
}

#[derive(Debug, Copy, Clone)]
pub enum GameEndedState {
	PlayerWon,
	EngineWon,
	NeitherWon,
}



pub struct Textures<'a> {
	pub ring: Texture<'a>,
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
