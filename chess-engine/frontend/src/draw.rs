use crate::*;
use sdl3::{pixels::Color, render::FRect};



pub fn draw<'a>(data: &mut AppData, canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, text_cache: &mut TextCache<'a>, font: &FontRef) -> Result<()> {
	canvas.set_draw_color(Color::RGB(245, 245, 245));
	canvas.clear();
	let (width, height) = canvas.output_size()?;
	let (width, height) = (width as f32, height as f32);
	
	canvas.set_draw_color(Color::RGB(241, 219, 203));
	canvas.fill_rect(FRect::new(0.0, 0.0, width, height * 0.08))?;
	
	render_text("This is a lot of text! I have 1, 2, and 3? this... is (1 - 3) - thing[4]", 15.0, 50, 150, Color::RGB(30, 30, 30), Color::RGB(245, 245, 245), canvas, texture_creator, text_cache, font)?;
	
	canvas.present();
	Ok(())
}
