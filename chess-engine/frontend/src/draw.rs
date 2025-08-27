use crate::*;



pub fn draw<'a, Font: sdl3_text::ThreadSafeFont>(data: &mut AppData, canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, text_cache: &mut sdl3_text::TextCache<'a, Font>) -> Result<()> {
	canvas.set_draw_color(data.settings.background_color);
	canvas.clear();
	let (width, height) = canvas.output_size()?;
	let (width, height) = (width as f32, height as f32);
	let screen_mid = (width * 0.5, height * 0.5);
	
	//sdl3_text::render_text_subpixel("This is a lot of text! I have 1, 2, and 3? this... is (1 - 3) - thing[4]", 30, 50, 150, Color::RGB(30, 30, 30), Color::RGB(246, 223, 178), canvas, texture_creator, text_cache)?;
	
	// top bar
	canvas.set_draw_color(data.settings.top_bar_color);
	canvas.fill_rect(FRect::new(0.0, 0.0, width, height * 0.08))?;
	
	// chess board
	let mut board_rect = FRect::new(screen_mid.0 - height * 0.25, screen_mid.1 - height * 0.25, height * 0.5, height * 0.5);
	canvas.set_draw_color(data.settings.board_trim_color);
	canvas.fill_rect(board_rect)?;
	board_rect.x += height * 0.01;
	board_rect.y += height * 0.01;
	board_rect.w -= height * 0.02;
	board_rect.h -= height * 0.02;
	for x in 0..8 {
		for y in 0..8 {
			let pos = get_piece_pos(x, y, board_rect, width, height, screen_mid);
			canvas.set_draw_color(if (x + y) % 2 == 1 {data.settings.board_color_dark} else {data.settings.board_color_light});
			canvas.fill_rect(pos)?;
		}
	}
	
	canvas.present();
	Ok(())
}



fn get_piece_pos(x: u8, y: u8, board_rect: FRect, width: f32, height: f32, screen_mid: (f32, f32)) -> FRect {
	let slot_width = board_rect.w / 8.0;
	FRect::new(board_rect.x + slot_width * x as f32, board_rect.y + slot_width * y as f32, slot_width, slot_width)
}
