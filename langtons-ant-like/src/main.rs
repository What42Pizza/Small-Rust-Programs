// Started:      24/01/18
// Last updated: 24/01/19



#![allow(unused)]



pub mod settings {
	use crate::prelude::*;
	
	pub const CAMERA_SPEED: f32 = 25.0;
	
	pub const USE_RANDOM_RULES: bool = true;
	pub const RANDOM_COLORS_COUNT: u8 = 4;
	pub const RANDOM_RULES_COUNT: usize = 6;
	pub const RANDOM_MOVE_AMOUNT: usize = 1;
	
	pub const DEFAULT_RULES: &[AntRule] = &[
		AntRule::new(1, (0, 1), 1),
		AntRule::new(0, (0, 1), 3),
	];
	
	pub const DEFAULT_CANVAS_SIZE: (usize, usize) = (1000, 1000);
	
	pub const TEXTURE_SIZE: usize = 32;
	pub const AFAP_STEP_COUNT: usize = 64;
	pub const AFAP_MILLIS_COUNT: usize = 10;
	
}



use prelude::*;
use notan::draw::{DrawConfig, CreateFont};



pub mod ant;
pub mod update;
pub mod render;
pub mod data_mod;
pub mod gui_mod;
pub mod gui_integration_mod;
pub mod utils;
pub mod custom_impls;



pub mod prelude {
	
	pub use crate::{ant, data_mod::general_data::*, utils::*, settings::*, gui_mod::{self, prelude::*}, custom_impls::*};
	pub use crate::gui_integration_mod::{*, init::*, update::*, render::*};
	pub use std::path::*;
	pub use std::{error::Error as StdError, result::Result as StdResult};
	
	pub use notan::prelude::*;
	pub use notan::draw::Font as RenderingFont;
	pub use ab_glyph::*;
	pub use ab_glyph::FontVec as PositioningFont;
	pub use anyhow::*;
	pub use derive_is_enum_variant::is_enum_variant;
	pub use array_init::array_init;
	
}



#[notan_main]
fn main() -> Result<(), String> {
	let win_config = WindowConfig::new()
		.set_resizable(true)
		.set_size(1280, 720)
		.set_vsync(true);
	
	notan::init_with(init_wrapper)
		.add_config(win_config)
		.add_config(DrawConfig)
		.update(update::update_wrapper)
		.draw(render::render_wrapper)
		.build()
}





pub fn init_wrapper(gfx: &mut Graphics) -> ProgramData {
	init(gfx).unwrap_or_else(|err| {
		panic!("Unable to load:\n{err}");
	})
}

pub fn init(gfx: &mut Graphics) -> Result<ProgramData> {
	
	
	
	// General Data
	
	// load textures
	let textures = Textures {};
	
	// load font
	const FONT_BYTES: &[u8] = include_bytes!("../assets/Ubuntu-B.ttf");
	let rendering_font = gfx.create_font(FONT_BYTES).unwrap();
	let positioning_font = PositioningFont::try_from_vec(FONT_BYTES.to_vec()).unwrap();
	
	// load gui
	let gui = init_gui(&textures)?;
	
	//fn print_gui(element: &GuiElement<CustomGuiData>) {
	//	println!("{}", element.name);
	//	for child in &element.children_by_layer {
	//		print_gui(child);
	//	}
	//}
	//print_gui(&gui);
	
	
	
	let colors = vec!(
		Color::BLACK,
		Color::RED,
		Color::YELLOW,
		Color::AQUA,
		Color::BLUE,
		Color::GREEN,
		Color::PURPLE,
		Color::BROWN,
		Color::ORANGE,
		Color::PINK,
		Color::MAGENTA,
		Color::NAVY,
		Color::TEAL,
		Color::SILVER,
		Color::WHITE,
	);
	
	let rules = if USE_RANDOM_RULES {
		let mut rules = Vec::with_capacity(RANDOM_RULES_COUNT);
		for _ in 0..RANDOM_RULES_COUNT {
			rules.push(AntRule::random(RANDOM_COLORS_COUNT, RANDOM_MOVE_AMOUNT));
		}
		rules
	} else {
		DEFAULT_RULES.to_vec()
	};
	
	
	
	Ok(ProgramData {
		
		exit: false,
		
		gui,
		textures,
		rendering_font,
		positioning_font,
		last_screen_size: gfx.size(),
		
		ant: Ant::new(DEFAULT_CANVAS_SIZE, rules),
		ant_controls: AntControls::default(),
		
		camera_pos: (0., 0.),
		camera_zoom: 1.,
		
		canvas: update::create_new_canvas(DEFAULT_CANVAS_SIZE, colors)?,
		
	})
}
