use crate::*;
use sdl3::{pixels::Color, render::FRect};



pub fn draw(data: &mut AppData) -> Result<()> {
	data.canvas.set_draw_color(Color::RGB(225, 199, 178));
	data.canvas.clear();
	let (width, height) = data.canvas.output_size()?;
	let (width, height) = (width as f32, height as f32);
	
	data.canvas.set_draw_color(Color::RGB(241, 219, 203));
	data.canvas.fill_rect(FRect::new(0.0, 0.0, width, height as f32 * 0.08))?;
	
	data.canvas.present();
	Ok(())
}
