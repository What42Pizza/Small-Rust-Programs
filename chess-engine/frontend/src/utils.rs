use crate::*;



pub fn show_fatal_error(message: impl AsRef<str>) -> ! {
	let message = message.as_ref();
	rfd::MessageDialog::new()
		.set_title("Program Crashed")
		.set_description(message)
		.set_buttons(rfd::MessageButtons::Ok)
		.set_level(rfd::MessageLevel::Error)
		.show();
	panic!("{message}");
}



pub fn get_slot_screen_rect(x: u8, y: u8, window_size: (u32, u32)) -> FRect {
	let window_size = (window_size.0 as f32, window_size.1 as f32);
	let (start_x, start_y) = (window_size.0 / 2.0 - window_size.1 / 4.0, window_size.1 / 4.0);
	let slot_width = window_size.1 / 16.0;
	FRect::new(start_x + slot_width * x as f32, start_y + slot_width * y as f32, slot_width, slot_width)
}

pub fn get_slot_from_screen_pos(x: f32, y: f32, window_size: (u32, u32)) -> Option<(u8, u8)> {
	let window_size = (window_size.0 as f32, window_size.1 as f32);
	let (start_x, start_y) = (window_size.0 / 2.0 - window_size.1 / 4.0, window_size.1 / 4.0);
	if x < start_x || y < start_y {return None;}
	let slot_width = window_size.1 / 16.0;
	let slot_x = (x - start_x) / slot_width;
	if slot_x >= 8.0 {return None;}
	let slot_y = (y - start_y) / slot_width;
	if slot_y >= 8.0 {return None;}
	Some((slot_x as u8, slot_y as u8))
}
