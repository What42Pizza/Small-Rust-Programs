use sdl2::{render::Texture};



pub struct GameData<'a> {
    pub textures: GameTextures<'a>,
    pub frame_count: u32, // overflows after ~10,000 hours at 120 fps
    pub exit: bool,

    pub grid: Vec<bool>,
    pub grid_size: UPoint,
    pub mode: GameMode,

    pub paddle: Paddle,
    pub flashing_paddle: Option<FlashingPaddle>,
    pub padder_dir: i32,
    pub time_til_move: f64,

    pub wait_time_left: f64,
    pub printed_end_message: bool,

    pub space_pressed: bool,
    
    pub settings: GameSettings,

}



pub enum GameMode {
    Moving,
    Waiting,
    Flashing,
    Won,
    Lost,
}



pub struct GameTextures<'a> {
    pub empty: Texture<'a>,
    pub filled: Texture<'a>,
}



pub struct GameSettings {

    pub time_between_moves: f64,
    pub starting_length: u32,
    pub flashing_time: f64,
    pub flashing_speed: f64,

    pub zoom: u32,
    pub window_width: u32,
    pub window_height: u32,
    pub grid_width: u32,
    pub grid_height: u32,

}



pub struct Paddle {
    pub x: u32,
    pub y: u32,
    pub length: u32,
    pub dir: i32,
}

impl Paddle {
    pub fn new (x: u32, y: u32, length: u32, dir: i32) -> Self {
        Self {
            x,
            y,
            length,
            dir,
        }
    }
}



pub struct FlashingPaddle {
    pub x: u32,
    pub y: u32,
    pub length: u32,
    pub time_left: f64,
    pub flashing_speed: f64,
}

impl FlashingPaddle {
    pub fn from_paddle (paddle: &Paddle, game_data: &GameData) -> Self {
        Self {
            x: paddle.x,
            y: paddle.y,
            length: paddle.length,
            time_left: game_data.settings.flashing_time,
            flashing_speed: game_data.settings.flashing_speed,
        }
    }
}





pub struct UPoint {
    pub x: u32,
    pub y: u32,
}

impl UPoint {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }
}
