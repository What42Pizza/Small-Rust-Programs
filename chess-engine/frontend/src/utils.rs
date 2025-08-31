use crate::*;



impl State {
	pub fn new_playing(settings: &AppSettings) -> Self {
		let total_time = Duration::from_secs(settings.total_time);
		let time_per_move = Duration::from_secs(settings.time_per_move);
		Self::Playing {
			time_remainings: if total_time.is_zero() {None} else {Some((total_time, total_time))},
			time_per_move: if time_per_move.is_zero() {None} else {Some(time_per_move)},
			turn: TurnData::PlayersTurn (PlayersTurnState::NotHoldingPiece),
		}
	}
}



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



pub fn get_slot_screen_rect(x: u8, y: u8, window_size: (f32, f32)) -> FRect {
	let (start_x, start_y) = (window_size.0 / 2.0 - window_size.1 / 4.0, window_size.1 / 4.0);
	let slot_width = window_size.1 / 16.0;
	FRect::new(start_x + slot_width * x as f32, start_y + slot_width * y as f32, slot_width, slot_width)
}

pub fn get_slot_from_screen_pos(x: f32, y: f32, window_size: (f32, f32)) -> Option<(u8, u8)> {
	let (start_x, start_y) = (window_size.0 / 2.0 - window_size.1 / 4.0, window_size.1 / 4.0);
	if x < start_x || y < start_y {return None;}
	let slot_width = window_size.1 / 16.0;
	let slot_x = (x - start_x) / slot_width;
	if slot_x >= 8.0 {return None;}
	let slot_y = (y - start_y) / slot_width;
	if slot_y >= 8.0 {return None;}
	Some((slot_x as u8, slot_y as u8))
}



pub fn format_min_sec(dur: Duration) -> String {
	let secs = dur.as_secs();
	format!("{}:{:02}", secs / 60, secs % 60)
}



pub trait FRectFns {
	const ZERO: Self;
	fn center(&self) -> (f32, f32);
	fn contains(&self, point: (f32, f32)) -> bool;
}

impl FRectFns for FRect {
	const ZERO: Self = Self { x: 0.0, y: 0.0, w: 0.0, h: 0.0 };
	fn center(&self) -> (f32, f32) {
		(self.x + self.w * 0.5, self.y + self.h * 0.5)
	}
	fn contains(&self, point: (f32, f32)) -> bool {
		point.0 >= self.x && point.0 <= self.x + self.w && point.1 >= self.y && point.1 <= self.y + self.h
	}
}

pub trait MouseStateFns {
	fn pos(&self) -> (f32, f32);
}

impl MouseStateFns for MouseState {
	fn pos(&self) -> (f32, f32) {
		(self.x(), self.y())
	}
}
