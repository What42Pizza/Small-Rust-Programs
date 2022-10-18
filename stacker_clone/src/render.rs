use sdl2::{render::WindowCanvas, rect::Rect, pixels::Color};

use crate::{structs::*};



pub fn render(canvas: &mut WindowCanvas, game_data: &mut GameData) -> Result<(), String> {

    canvas.set_draw_color(Color::RGB(255, 0, 255));
    canvas.clear();

    {
        for y in 0..game_data.grid_size.y {
            for x in 0..game_data.grid_size.x {
                let current_tile = get_tile(x, y, game_data);
                let current_texture = if current_tile {&game_data.textures.filled} else {&game_data.textures.empty};
                canvas.copy(current_texture, None, Rect::new((x * game_data.settings.zoom) as i32, (y * game_data.settings.zoom) as i32, game_data.settings.zoom, game_data.settings.zoom))?;
            }
        }
    }

    // finish
    canvas.present();
    game_data.frame_count += 1;
    Ok(())

}



pub fn get_tile (x: u32, y: u32, game_data: &GameData) -> bool {
    if game_data.grid[(x + y * game_data.grid_size.x) as usize] {return true;}

    match game_data.mode {

        GameMode::Moving | GameMode::Waiting => {
            let paddle = &game_data.paddle;
            y == paddle.y && x >= paddle.x && x < paddle.x + paddle.length
        }

        GameMode::Flashing => {
            let flashing_paddle = &game_data.flashing_paddle;
            if flashing_paddle.is_none() {return false;}
            let flashing_paddle = flashing_paddle.as_ref().unwrap();
            if !(y == flashing_paddle.y && x >= flashing_paddle.x && x < flashing_paddle.x + flashing_paddle.length) {return false;}
            let flashing_speed = game_data.settings.flashing_speed;
            let flashing_time_left = game_data.wait_time_left % flashing_speed;
            flashing_time_left > 0. && flashing_time_left < flashing_speed / 2.
        }

        GameMode::Won => false,
        GameMode::Lost => false,

    }

}
