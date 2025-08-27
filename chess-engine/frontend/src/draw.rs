use crate::*;



pub fn draw<'a, Font: sdl3_text::ThreadSafeFont>(data: &mut AppData, canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, text_cache: &mut sdl3_text::TextCache<'a, Font>) -> Result<()> {
	canvas.set_draw_color(Color::RGB(246, 223, 178));
	canvas.clear();
	let (width, height) = canvas.output_size()?;
	let (width, height) = (width as f32, height as f32);
	
	canvas.set_draw_color(Color::RGB(254, 234, 192));
	canvas.fill_rect(FRect::new(0.0, 0.0, width, height * 0.08))?;
	
	sdl3_text::render_text_subpixel("This is a lot of text! I have 1, 2, and 3? this... is (1 - 3) - thing[4]", 30, 50, 150, Color::RGB(30, 30, 30), Color::RGB(246, 223, 178), canvas, texture_creator, text_cache)?;
	
	canvas.present();
	Ok(())
}
