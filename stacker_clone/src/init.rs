use sdl2::{Sdl, pixels::Color,
    image::{self, LoadTexture, InitFlag},
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext}
};

use crate::structs::*;



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





pub fn init_game_data (game_settings: GameSettings, texture_creator: &TextureCreator<WindowContext>) -> Result<GameData<'_>, String> {

    let textures = load_textures(texture_creator)?;

    Ok(GameData {
        textures,
        frame_count: 0,
        exit: false,

        grid: vec![false; (game_settings.grid_width * game_settings.grid_height) as usize],
        grid_size: UPoint::new(game_settings.grid_width, game_settings.grid_height),
        mode: GameMode::Waiting,

        paddle: Paddle::new(0, game_settings.grid_height - 1, game_settings.starting_length, 1),
        flashing_paddle: None,
        padder_dir: 1,
        time_til_move: 0.,

        wait_time_left: 2.,
        printed_end_message: false,

        space_pressed: false,

        settings: game_settings,

    })

}



pub fn load_textures (texture_creator: &TextureCreator<WindowContext>) -> Result<GameTextures<'_>, String> {
    Ok(GameTextures {
        empty: texture_creator.load_texture("assets/empty.png")?,
        filled: texture_creator.load_texture("assets/filled.png")?,
    })
}
