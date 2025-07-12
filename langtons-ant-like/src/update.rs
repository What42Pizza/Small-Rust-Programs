use crate::prelude::*;



pub fn update_wrapper(app: &mut App, program_data: &mut ProgramData) {
	update(app, program_data).unwrap_or_else(|err| {
		panic!("Unable to load:\n{err}");
	})
}

pub fn update(app: &mut App, program_data: &mut ProgramData) -> Result<()> {
	let dt = app.system_timer.delta_f32();
	
	
	
	let mouse_pos = app.mouse.position().to_i32();
	let last_screen_size = program_data.last_screen_size;
	if app.mouse.left_was_pressed() {
		let hovered_elements = gui_mod::gui_utils::get_hovered_elements(&program_data.gui, mouse_pos, last_screen_size);
		for element in hovered_elements {
			if let Some(click_fn) = element.custom_data.click_fn {
				let result = click_fn(program_data);
				if let Err(err) = result {
					panic!("Error while processing button press: {err}");
				}
				break;
			}
		}
	}
	
	
	
	process_input(app, program_data, dt)?;
	
	update_gui(app, program_data);
	
	
	
	if program_data.ant_controls.is_running {
		ant::update(program_data);
	}
	
	
	
	if program_data.exit {
		app.exit();
	}
	Ok(())
}





pub fn process_input(app: &mut App, program_data: &mut ProgramData, dt: f32) -> Result<()> {
	if app.keyboard.ctrl() {
		
		if app.keyboard.was_pressed(KeyCode::R) {
			println!("Reloading gui...");
			program_data.gui = init_gui(&program_data.textures)?;
			println!("Done");
		}
		
		if app.keyboard.was_pressed(KeyCode::W) {
			program_data.exit = true;
		}
		
	} else {
		
		if app.keyboard.is_down(KeyCode::W) {
			program_data.camera_pos.1 -= CAMERA_SPEED / program_data.camera_zoom * dt * 10.;
		}
		if app.keyboard.is_down(KeyCode::A) {
			program_data.camera_pos.0 -= CAMERA_SPEED / program_data.camera_zoom * dt * 10.;
		}
		if app.keyboard.is_down(KeyCode::S) {
			program_data.camera_pos.1 += CAMERA_SPEED / program_data.camera_zoom * dt * 10.;
		}
		if app.keyboard.is_down(KeyCode::D) {
			program_data.camera_pos.0 += CAMERA_SPEED / program_data.camera_zoom * dt * 10.;
		}
		
		if app.keyboard.was_pressed(KeyCode::Space) {
			program_data.ant_controls.is_running = !program_data.ant_controls.is_running;
		}
		if app.keyboard.was_pressed(KeyCode::Right) {
			program_data.ant_controls.steps_per_frame += 1;
		}
		if app.keyboard.was_pressed(KeyCode::Left) {
			program_data.ant_controls.steps_per_frame -= 1;
		}
		if app.keyboard.was_pressed(KeyCode::Up) {
			program_data.ant_controls.steps_per_frame *= 2;
		}
		if app.keyboard.was_pressed(KeyCode::Down) {
			program_data.ant_controls.steps_per_frame /= 2;
		}
		if app.keyboard.was_pressed(KeyCode::Return) {
			program_data.ant_controls.as_fast_as_possible = !program_data.ant_controls.as_fast_as_possible;
		}
		
		let scroll_amount = app.mouse.wheel_delta.y;
		if scroll_amount != 0. {
			let old_mouse_canvas_pos = (app.mouse.x / program_data.camera_zoom, app.mouse.y / program_data.camera_zoom);
			let old_mouse_canvas_pos = (old_mouse_canvas_pos.0 + program_data.camera_pos.0, old_mouse_canvas_pos.1 + program_data.camera_pos.1);
			program_data.camera_zoom *= 1.0 + scroll_amount * 0.0015;
			let new_mouse_canvas_pos = (app.mouse.x / program_data.camera_zoom, app.mouse.y / program_data.camera_zoom);
			let new_mouse_canvas_pos = (new_mouse_canvas_pos.0 + program_data.camera_pos.0, new_mouse_canvas_pos.1 + program_data.camera_pos.1);
			let mouse_zoom_delta = (new_mouse_canvas_pos.0 - old_mouse_canvas_pos.0, new_mouse_canvas_pos.1 - old_mouse_canvas_pos.1);
			program_data.camera_pos.0 -= mouse_zoom_delta.0;
			program_data.camera_pos.1 -= mouse_zoom_delta.1;
		}
		
	}
	Ok(())
}





pub fn update_canvas_pixel(pos: (usize, usize), new_val: u8, canvas: &mut Canvas) {
	
	let index = canvas.get_cell_index(pos);
	canvas.raw_data[index] = new_val;
	canvas.texture_datas[index] = canvas.colors[new_val as usize].hex();
	
	let texture_index = canvas.get_texture_index_of_cell(pos);
	canvas.textures[texture_index].is_dirty = true;
	
}



pub fn update_canvas_dirty_textures(canvas: &mut Canvas, gfx: &mut Graphics) -> Result<()> {
	for y in 0..canvas.textures_size.1 {
		for x in 0..canvas.textures_size.0 {
			let texture_index = x + y * canvas.textures_size.0;
			if canvas.textures[texture_index].is_dirty {
				canvas.textures[texture_index].texture = Some(create_canvas_texture(canvas, (x, y), gfx)?);
				canvas.textures[texture_index].is_dirty = false;
			}
		}
	}
	Ok(())
}



pub fn create_new_canvas(size: (usize, usize), colors: Vec<Color>) -> Result<Canvas> {
	
	let texture_x_count = size.0.div_ceil(TEXTURE_SIZE);
	let texture_y_count = size.1.div_ceil(TEXTURE_SIZE);
	let pixels_count = size.0 * size.1;
	let default_color_u32 = colors[0].hex();
	
	let raw_data = vec![0; pixels_count];
	let texture_datas = vec![default_color_u32; pixels_count];
	
	let mut textures = Vec::with_capacity(texture_x_count * texture_y_count);
	for y in 0..texture_y_count {
		for x in 0..texture_x_count {
			textures.push(CanvasTexture {
				texture: None,
				is_dirty: true,
			});
		}
	}
	
	let mut output = Canvas {
		
		raw_data,
		texture_datas,
		textures,
		
		raw_data_size: size,
		textures_size: (texture_x_count, texture_y_count),
		
		colors,
		
	};
	
	Ok(output)
}



pub fn create_canvas_texture(canvas: &Canvas, pos: (usize, usize), gfx: &mut Graphics) -> Result<Texture> {
	let texture_width = (canvas.raw_data_size.0 - pos.0 * TEXTURE_SIZE).min(TEXTURE_SIZE);
	let texture_height = (canvas.raw_data_size.1 - pos.1 * TEXTURE_SIZE).min(TEXTURE_SIZE);
	
	let mut bytes = Vec::with_capacity(texture_width * texture_height * 4);
	
	let mut raw_data_index = pos.0 * TEXTURE_SIZE + pos.1 * TEXTURE_SIZE * canvas.raw_data_size.0;
	for _ in 0..texture_height {
		for _ in 0..texture_width {
			let cell_val = canvas.raw_data[raw_data_index];
			let color = canvas.colors[cell_val as usize];
			bytes.push((color.r * 255.) as u8);
			bytes.push((color.g * 255.) as u8);
			bytes.push((color.b * 255.) as u8);
			bytes.push((color.a * 255.) as u8);
			raw_data_index += 1;
		}
		raw_data_index -= texture_width;
		raw_data_index += canvas.raw_data_size.0;
	}
	
	gfx
		.create_texture()
		.from_bytes(&bytes, texture_width as u32, texture_height as u32)
		.build()
		.map_err(Error::msg)
}
