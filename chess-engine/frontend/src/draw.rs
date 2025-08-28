use crate::*;



pub fn draw<'a, Font: sdl3_text::ThreadSafeFont>(data: &mut AppData, canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, text_cache: &mut sdl3_text::TextCache<'a, Font>, textures: &Textures<'a>) -> Result<()> {
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
	let mut board_rect = FRect::new(screen_mid.0 - height * 0.26, screen_mid.1 - height * 0.26, height * 0.52, height * 0.52);
	canvas.set_draw_color(data.settings.board_trim_color);
	canvas.fill_rect(board_rect)?;
	for x in 0..8 {
		for y in 0..8 {
			let pos = get_slot_screen_rect(x, y, data.window_size);
			canvas.set_draw_color(if (x + y) % 2 == 1 {data.settings.board_color_dark} else {data.settings.board_color_light});
			canvas.fill_rect(pos)?;
			let piece = get_piece(&data.board, x, y, 123);
			if let Some(piece_texture) = get_texture_for_piece(piece, textures) {
				canvas.copy(piece_texture, None, pos)?;
			}
		}
	}
	
	canvas.present();
	Ok(())
}



fn get_texture_for_piece<'a, 'b>(piece: Piece, textures: &'b Textures<'a>) -> Option<&'b Texture<'a>> {
	Some(match piece {
		Piece::None => return None,
		Piece::BlackPawn => &textures.black_pawn,
		Piece::BlackKnight => &textures.black_knight,
		Piece::BlackBishop => &textures.black_bishop,
		Piece::BlackRook => &textures.black_rook,
		Piece::BlackQueen => &textures.black_queen,
		Piece::BlackKing => &textures.black_king,
		Piece::WhitePawn => &textures.white_pawn,
		Piece::WhiteKnight => &textures.white_knight,
		Piece::WhiteBishop => &textures.white_bishop,
		Piece::WhiteRook => &textures.white_rook,
		Piece::WhiteQueen => &textures.white_queen,
		Piece::WhiteKing => &textures.white_king,
	})
}



pub fn load_textures<'a>(resources_path: &Path, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Textures<'a>> {
	let textures_path = resources_path.join("textures");
	Ok(Textures {
		black_pawn: load_image_from_path(textures_path.join("black-pawn.png"), texture_creator)?,
		black_knight: load_image_from_path(textures_path.join("black-knight.png"), texture_creator)?,
		black_bishop: load_image_from_path(textures_path.join("black-bishop.png"), texture_creator)?,
		black_rook: load_image_from_path(textures_path.join("black-rook.png"), texture_creator)?,
		black_queen: load_image_from_path(textures_path.join("black-queen.png"), texture_creator)?,
		black_king: load_image_from_path(textures_path.join("black-king.png"), texture_creator)?,
		white_pawn: load_image_from_path(textures_path.join("white-pawn.png"), texture_creator)?,
		white_knight: load_image_from_path(textures_path.join("white-knight.png"), texture_creator)?,
		white_bishop: load_image_from_path(textures_path.join("white-bishop.png"), texture_creator)?,
		white_rook: load_image_from_path(textures_path.join("white-rook.png"), texture_creator)?,
		white_queen: load_image_from_path(textures_path.join("white-queen.png"), texture_creator)?,
		white_king: load_image_from_path(textures_path.join("white-king.png"), texture_creator)?,
	})
}

pub fn load_image_from_path<'a>(path: impl AsRef<Path>, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Texture<'a>> {
	let image = ImageReader::open(path)?.decode()?.to_rgba8();
	let (width, height) = image.dimensions();
	let mut texture = texture_creator.create_texture(
		Some(unsafe {PixelFormat::from_ll(SDL_PixelFormat::ABGR8888)}),
		sdl3::render::TextureAccess::Static,
		width,
		height,
	)?;
	texture.update(None, image.as_bytes(), width as usize * 4)?;
	Ok(texture)
}
