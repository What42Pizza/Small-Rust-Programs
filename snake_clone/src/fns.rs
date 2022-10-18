use std::error::Error;
use rand::Rng;
use sdl2::{render::Texture,
    rect::{Rect, Point}
};

use crate::structs::*;



pub fn get_texture_size (texture: &Texture) -> (u32, u32) {
    let query = texture.query();
    (query.width, query.height)
}



pub fn div_ceil (right: u32, left: u32) -> u32 {
    (right as f64 / left as f64).ceil() as u32
}



pub fn random_map_pos (map_size: Point) -> Point {
    let mut rng = rand::thread_rng();
    let random_x: f64 = rng.gen();
    let random_y: f64 = rng.gen();
    let x = (random_x * map_size.x as f64) as i32;
    let y = (random_y * map_size.y as f64) as i32;
    Point::new(x, y)
}



pub fn empty_map (map_width: u32, map_height: u32) -> Vec<MapTile> {
    let mut output = Vec::new();
    for i in 0..map_width * map_height {
        output.push(MapTile::None);
    }
    output
}



pub fn shift_point (origin: &Point, dir: &Direction) -> Point {
    match dir {
        Direction::Up => Point::new(origin.x, origin.y - 1),
        Direction::Down => Point::new(origin.x, origin.y + 1),
        Direction::Left => Point::new(origin.x - 1, origin.y),
        Direction::Right => Point::new(origin.x + 1, origin.y),
    }
}


pub fn point_is_in_map (point: &Point, map_size: &Point) -> bool {
    point.x >= 0 && point.x < map_size.x && point.y >= 0 && point.y < map_size.y
}



pub fn point_to_index (point: &Point, map_size: &Point) -> usize {
    (point.x + point.y * map_size.x) as usize
}



pub fn move_apple (game_data: &mut GameData) {

    if let Some(apple_pos) = game_data.apple_pos {
        game_data.map_contents[point_to_index(&apple_pos, &game_data.map_size)] = MapTile::Snake;
    }

    let mut new_apple_pos = Point::new(0, 0);
    loop {
        new_apple_pos = random_map_pos(game_data.map_size);
        if let MapTile::None = game_data.map_contents[point_to_index(&new_apple_pos, &game_data.map_size)] {break;}
    }
    game_data.apple_pos = Some(new_apple_pos);

    game_data.map_contents[point_to_index(&new_apple_pos, &game_data.map_size)] = MapTile::Apple;

}
