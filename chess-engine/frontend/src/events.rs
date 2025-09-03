use crate::*;



pub fn handle_event(data: &mut AppData, event: Event) -> Result<()> {
	match event {
		
		Event::Quit { timestamp: _ } => data.should_close = true,
		
		Event::KeyDown { timestamp: _, window_id: _, keycode, scancode: _, keymod, repeat: _, which: _, raw: _ } => {
			
			if keycode == Some(sdl3::keyboard::Keycode::W) && (keymod.contains(Mod::RCTRLMOD) || keymod.contains(Mod::LCTRLMOD)) {
				data.should_close = true;
			}
			
		}
		
		Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn, clicks: _, x, y } => {
			
			// new game button
			if data.new_game_button_rect.contains((x, y)) {
				data.new_game_button_down = true;
				new_game_button_pressed(data);
				return Ok(());
			}
			
			// dragging pieces
			if let Some((x, y)) = get_slot_from_screen_pos(x, y, data.window_size) {
				if let State::Playing { turn: TurnState::PlayersTurn (players_turn_state), .. } = &mut data.state {
					let piece = get_piece(&data.board, x, y);
					if piece.is_white() {
						set_piece(&mut data.board, x, y, Piece::None);
						*players_turn_state = PlayersTurnState::HoldingPiece { x, y, piece };
					}
				}
			}
			
		}
		
		_ => {}
	}
	Ok(())
}



pub fn new_game_button_pressed(data: &mut AppData) {
	data.state = State::new_playing(&data.settings);
	data.board = default_board();
	data.ring_selectors = None;
}
