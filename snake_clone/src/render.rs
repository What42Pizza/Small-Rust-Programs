use sdl2::{render::WindowCanvas, rect::Rect, pixels::Color};

use crate::{structs::*, fns};



pub fn render(canvas: &mut WindowCanvas, game_data: &mut GameData) -> Result<(), String> {
    let textures = &game_data.textures;
    let (width, height) = canvas.output_size()?;

    canvas.set_draw_color(Color::RGB(255, 0, 255));
    canvas.clear();

    {
        let map_width = game_data.map_size.x;
        for y in 0..game_data.map_size.y {
            for x in 0..game_data.map_size.x {
                let current_tile = &game_data.map_contents[(x + y * map_width) as usize];
                let current_texture = current_tile.get_texture(&game_data.textures);
                canvas.copy(current_texture, None, Rect::new((x as u32 * game_data.zoom) as i32, (y as u32 * game_data.zoom) as i32, game_data.zoom, game_data.zoom))?;
            }
        }
    }

    // finish
    canvas.present();
    game_data.frame_count += 1;
    Ok(())

}
