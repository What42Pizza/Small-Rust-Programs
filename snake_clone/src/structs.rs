use sdl2::{render::Texture, rect::Point};

use crate::fns;



pub struct GameData<'a> {
    pub textures: GameTextures<'a>,
    pub frame_count: u32, // overflows after ~10,000 hours at 120 fps
    pub zoom: u32,
    pub map_size: Point,
    pub time_between_moves: f64,
    pub exit: bool,

    pub pressed_keys: [bool; 4],
    pub map_contents: Vec<MapTile>,
    pub snake: Snake,
    pub apple_pos: Option<Point>,

}



pub struct GameTextures<'a> {
    pub ground: Texture<'a>,
    pub snake: Texture<'a>,
    pub apple: Texture<'a>,
}



pub struct Snake {
    pub positions: Vec<Point>,
    pub head_pos: Point,
    pub time_til_next_move: f64,
    pub direction: Direction,
    pub just_ate: bool,
}

impl Snake {
    pub fn new (x: u32, y: u32) -> Self {
        let head_pos = Point::new(x as i32, y as i32);
        Snake {
            positions: vec![head_pos],
            head_pos,
            time_til_next_move: 2.,
            direction: Direction::Right,
            just_ate: false,
        }
    }
}



#[derive(is_enum_variant)]
pub enum MapTile {
    Snake,
    Apple,
    None,
}

impl MapTile {
    pub fn get_texture<'a> (&self, textures: &'a GameTextures) -> &Texture<'a> {
        match self {
            MapTile::None => &textures.ground,
            MapTile::Snake => &textures.snake,
            MapTile::Apple => &textures.apple,
        }
    }
}



#[derive(Copy, is_enum_variant)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Clone for Direction {
    fn clone(&self) -> Direction {
        match self {
            Direction::Up => Direction::Up,
            Direction::Down => Direction::Down,
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
        }
    }
}





pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {

    pub fn new (x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
    pub fn new_u32 (x: u32, y: u32) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
        }
    }

    pub fn from_point (point: &Point) -> Self {
        Self {
            x: point.x as f64,
            y: point.y as f64,
        }
    }
    pub fn to_point (&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }

    pub fn add (mut self, other: &Vec2) -> Self {
        self.x += other.x;
        self.y += other.y;
        self
    }
    pub fn add_u32 (mut self, other_x: u32, other_y: u32) -> Self {
        self.x += other_x as f64;
        self.y += other_y as f64;
        self
    }
    pub fn add_f64 (mut self, other_x: f64, other_y: f64) -> Self {
        self.x += other_x;
        self.y += other_y;
        self
    }

    pub fn sub (mut self, other: &Vec2) -> Self {
        self.x -= other.x;
        self.y -= other.y;
        self
    }
    pub fn sub_u32 (mut self, other_x: u32, other_y: u32) -> Self {
        self.x -= other_x as f64;
        self.y -= other_y as f64;
        self
    }
    pub fn sub_f64 (mut self, other_x: f64, other_y: f64) -> Self {
        self.x -= other_x;
        self.y -= other_y;
        self
    }

    pub fn mult (mut self, other: &Vec2) -> Self {
        self.x *= other.x;
        self.y *= other.y;
        self
    }
    pub fn mult_u32 (mut self, other_x: u32, other_y: u32) -> Self {
        self.x *= other_x as f64;
        self.y *= other_y as f64;
        self
    }
    pub fn mult_f64 (mut self, other_x: f64, other_y: f64) -> Self {
        self.x *= other_x;
        self.y *= other_y;
        self
    }

    pub fn div (mut self, other: &Vec2) -> Self {
        self.x /= other.x;
        self.y /= other.y;
        self
    }
    pub fn div_u32 (mut self, other_x: u32, other_y: u32) -> Self {
        self.x /= other_x as f64;
        self.y /= other_y as f64;
        self
    }
    pub fn div_f64 (mut self, other_x: f64, other_y: f64) -> Self {
        self.x /= other_x;
        self.y /= other_y;
        self
    }

    pub fn modulo (mut self, other: &Vec2) -> Self {
        self.x %= other.x;
        self.y %= other.y;
        self
    }
    pub fn modulo_u32 (mut self, other_x: u32, other_y: u32) -> Self {
        self.x %= other_x as f64;
        self.y %= other_y as f64;
        self
    }
    pub fn modulo_f64 (mut self, other_x: f64, other_y: f64) -> Self {
        self.x %= other_x;
        self.y %= other_y;
        self
    }

}
