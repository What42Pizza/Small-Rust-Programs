use crate::*;



pub fn draw<'a, Font: sdl3_text::ThreadSafeFont>(data: &AppData, canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, text_cache: &mut sdl3_text::TextCache<'a, Font>, textures: &Textures<'a>) -> Result<()> {
	canvas.set_draw_color(data.settings.background_color);
	canvas.clear();
	let (width, height) = (data.window_size.0, data.window_size.1);
	let screen_mid = (width * 0.5, height * 0.5);
	
	// top bar
	canvas.set_draw_color(data.settings.top_bar_color);
	canvas.fill_rect(get_top_bar_rect(width, height))?;
	
	let button_color = if data.new_game_button_down {data.settings.top_bar_buttons_darkened_color} else {data.settings.top_bar_buttons_color};
	canvas.set_draw_color(button_color);
	canvas.fill_rect(data.new_game_button_rect)?;
	let new_game_pos = data.new_game_button_rect.center();
	let mut render_text_settings = sdl3_text::TextRenderingSettings::new_subpixel((data.new_game_button_rect.h * 0.7) as u32, sdl3_text::HAlign::Center, sdl3_text::VAlign::Center, Color::RGB(30, 30, 30), button_color, canvas, texture_creator, text_cache);
	sdl3_text::render_text_subpixel("New Game", new_game_pos.0 as i32, new_game_pos.1 as i32, &mut render_text_settings)?;
	
	// chess board
	let board_rect = FRect::new(screen_mid.0 - height * 0.26, screen_mid.1 - height * 0.26, height * 0.52, height * 0.52);
	canvas.set_draw_color(data.settings.board_trim_color);
	canvas.fill_rect(board_rect)?;
	for x in 0..8 {
		for y in 0..8 {
			let pos = get_slot_screen_rect(x, y, data.window_size);
			canvas.set_draw_color(if (x + y) % 2 == 1 {data.settings.board_color_dark} else {data.settings.board_color_light});
			canvas.fill_rect(pos)?;
			let piece = get_piece(&data.board, x, y);
			if let Some(piece_texture) = get_texture_for_piece(piece, textures) {
				canvas.copy(piece_texture, None, pos)?;
			}
		}
	}
	if let Some((x1, y1, x2, y2)) = data.ring_selectors {
		let pos1 = get_slot_screen_rect(x1, y1, data.window_size);
		let pos2 = get_slot_screen_rect(x2, y2, data.window_size);
		canvas.copy(&textures.gray_ring, None, pos1)?;
		canvas.copy(&textures.gray_ring, None, pos2)?;
	}
	if let Some((x1, y1, x2, y2)) = data.check_indicators {
		let pos1 = get_slot_screen_rect(x1, y1, data.window_size);
		let pos2 = get_slot_screen_rect(x2, y2, data.window_size);
		canvas.copy(&textures.red_ring, None, pos1)?;
		canvas.copy(&textures.red_ring, None, pos2)?;
	}
	
	// game data text
	let size = (height * 0.05) as u32;
	let x = screen_mid.0 + height * 0.25 + width * 0.025;
	let mut y = height * 0.25;
	let mut render_text_settings = sdl3_text::TextRenderingSettings::new_subpixel(size, sdl3_text::HAlign::Left, sdl3_text::VAlign::Top, Color::RGB(30, 30, 30), data.settings.background_color, canvas, texture_creator, text_cache);
	if let State::Playing { time_remainings: Some((player_time, engine_time)), time_per_move: _, turn: _ } = &data.state {
		sdl3_text::render_text_subpixel(format!("Your time left: {}", format_min_sec(*player_time)), x as i32, y as i32, &mut render_text_settings)?;
		y += size as f32 * 1.1;
		sdl3_text::render_text_subpixel(format!("Engine's time left: {}", format_min_sec(*engine_time)), x as i32, y as i32, &mut render_text_settings)?;
		y += size as f32 * 1.1;
	}
	if let State::Playing { time_remainings: _, time_per_move: Some(time_per_move), turn: _ } = &data.state {
		render_text_settings.size *= 0.7;
		sdl3_text::render_text_subpixel(format!("Time per move: {}", format_min_sec(*time_per_move)), x as i32, y as i32, &mut render_text_settings)?;
	}
	render_text_settings.v_align = sdl3_text::VAlign::Bottom;
	let y = height * 0.75;
	if let State::Playing { turn, .. } = &data.state {
		let is_players_turn = *turn != TurnState::EnginesTurn;
		sdl3_text::render_text_subpixel(if is_players_turn {"Your Turn"} else {"Engine's turn"}, x as i32, y as i32, &mut render_text_settings)?;
	}
	if let State::GameEnded (ended_state) = data.state {
		let mut render_text_settings = sdl3_text::TextRenderingSettings::new_subpixel(size, sdl3_text::HAlign::Center, sdl3_text::VAlign::Top, Color::RGB(30, 30, 30), data.settings.background_color, canvas, texture_creator, text_cache);
		match ended_state {
			GameEndedState::PlayerWon => sdl3_text::render_text_subpixel("You have won", screen_mid.0 as i32, (height * 0.77) as i32, &mut render_text_settings)?,
			GameEndedState::EngineWon => sdl3_text::render_text_subpixel("You have lost", screen_mid.0 as i32, (height * 0.77) as i32, &mut render_text_settings)?,
			GameEndedState::NeitherWon => sdl3_text::render_text_subpixel("You have tied", screen_mid.0 as i32, (height * 0.77) as i32, &mut render_text_settings)?,
		}
	}
	
	// held piece
	if let State::Playing { turn: TurnState::PlayersTurn (PlayersTurnState::HoldingPiece { piece, .. }), .. } = &data.state {
		let slot_width = height / 16.0;
		let texture = get_texture_for_piece(*piece, textures).expect("Cannot hold no Piece::None");
		let dst = FRect::new(data.mouse_state.x() - slot_width * 0.5, data.mouse_state.y() - slot_width * 0.5, slot_width, slot_width);
		canvas.copy(texture, None, dst)?;
	}
	
	canvas.present();
	Ok(())
}



pub fn get_top_bar_rect(width: f32, height: f32) -> FRect {
	FRect::new(0.0, 0.0, width, height * 0.08)
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
		gray_ring: load_image_from_path(textures_path.join("gray ring.png"), texture_creator)?,
		red_ring: load_image_from_path(textures_path.join("red ring.png"), texture_creator)?,
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
