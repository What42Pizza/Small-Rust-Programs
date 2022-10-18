use sdl2::{Sdl, pixels::Color,
    image::{self, LoadTexture, InitFlag},
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext}, rect::Point
};

use crate::{structs::*, fns};



pub fn init_sdl2 (width: u32, height: u32) -> (Sdl, Canvas<Window>) {
    
    let sdl_context = sdl2::init().expect("Failed to init sdl2.");
    let _image_context = image::init(InitFlag::PNG).expect("Failed to get image context.");
    let video_subsystem = sdl_context.video().expect("Failed to get video subsystem.");
    let window = video_subsystem.window("SDL2 Testing Window", width, height)
        .position_centered()
        .build()
        .expect("Failed to build window.");
    
    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Failed to create canvas.");

    canvas.set_draw_color(Color::RGB(255, 0, 255));
    canvas.clear();
    canvas.present();

    (sdl_context, canvas)

}





pub fn init_game_data (map_width: u32, map_height: u32, zoom: u32, time_between_moves: f64, texture_creator: &TextureCreator<WindowContext>) -> Result<GameData<'_>, String> {
    
    let map_size = Point::new(map_width as i32, map_height as i32);
    let textures = load_textures(texture_creator)?;

    let snake_x = map_width / 2;
    let snake_y = map_height / 2;

    let mut map_contents = fns::empty_map(map_width, map_height);
    map_contents[(snake_x + snake_y * map_width) as usize] = MapTile::Snake;

    let mut game_data = GameData {
        textures,
        frame_count: 0,
        zoom,
        map_size,
        time_between_moves,
        exit: false,

        pressed_keys: [false; 4],
        map_contents,
        snake: Snake::new(snake_x, snake_y),
        apple_pos: None,

    };

    fns::move_apple(&mut game_data);

    Ok(game_data)

}



pub fn load_textures (texture_creator: &TextureCreator<WindowContext>) -> Result<GameTextures<'_>, String> {
    Ok(GameTextures {
        ground: texture_creator.load_texture("assets/ground.png")?,
        snake: texture_creator.load_texture("assets/snake.png")?,
        apple: texture_creator.load_texture("assets/apple.png")?,
    })
}
