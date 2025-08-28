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
			println!("{:?}", get_slot_from_screen_pos(x, y, data.window_size));
		}
		
		_ => {}
	}
	Ok(())
}
