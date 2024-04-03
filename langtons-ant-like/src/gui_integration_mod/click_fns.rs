use crate::prelude::*;



pub fn add_click_fns(gui: &mut GuiElement<CustomGuiData>) {
	
	fn set_click_fn(element: &mut GuiElement<CustomGuiData>, click_fn: fn(&mut ProgramData) -> Result<()>) {
		element.custom_data.click_fn = Some(click_fn);
	}
	
	
	
	let control_panel = gui.child_mut("control_panel"); {
		
		let inner = control_panel.child_mut("inner"); {
			
			
			
			set_click_fn(inner.child_mut("reset_button"), reset);
			
			fn reset(program_data: &mut ProgramData) -> Result<()> {
				
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
				
				program_data.ant = Ant::new(DEFAULT_CANVAS_SIZE, rules);
				program_data.ant_controls = AntControls::default();
				program_data.canvas = crate::update::create_new_canvas(DEFAULT_CANVAS_SIZE, colors)?;
				
				Ok(())
			};
			
			
			
		}
		
	}
	
	
	
}
