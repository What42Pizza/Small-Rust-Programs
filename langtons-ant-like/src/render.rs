use crate::{prelude::*, update::update_canvas_dirty_textures};
use notan::draw::{CreateDraw, DrawImages, DrawTransform};



pub fn render_wrapper(app: &mut App, gfx: &mut Graphics, program_data: &mut ProgramData) {
	render(app, gfx, program_data).unwrap_or_else(|err| {
		panic!("Error while rendering:\n{err}");
	})
}

pub fn render(app: &mut App, gfx: &mut Graphics, program_data: &mut ProgramData) -> Result<()> {
	
	
	
	// ======== TEXTURE UPDATES ========
	
	update_canvas_dirty_textures(&mut program_data.canvas, gfx)?;
	
	
	
	// ======== MISC ========
	
	program_data.last_screen_size = gfx.size();
	let program_data = &*program_data;
	
	let textures = &program_data.textures;
	
	let mut draw = gfx.create_draw();
	draw.clear(Color::from_rgba(0.2, 0.2, 0.2, 1.0));
	
	
	
	// ======== CANVAS ========
	
	let (camera_x, camera_y, camera_zoom) = (program_data.camera_pos.0, program_data.camera_pos.1, program_data.camera_zoom);
	let textures_size = program_data.canvas.textures_size;
	for y in 0..textures_size.1 {
		for x in 0..textures_size.0 {
			let texture_index = program_data.canvas.get_texture_index((x, y));
			let Some(texture) = &program_data.canvas.textures[texture_index].texture else {continue;};
			let pos = (x * TEXTURE_SIZE, y * TEXTURE_SIZE);
			let pos = (pos.0 as f32 - camera_x, pos.1 as f32 - camera_y);
			draw
				.image(texture)
				.position(pos.0, pos.1)
				.scale(camera_zoom, camera_zoom);
		}
	}
	
	
	
	// ======== GUI ========
	
	// render
	let mut render_data = GuiRenderingData {
		draw: &mut draw,
		textures,
		rendering_font: program_data.rendering_font,
		positioning_font: &program_data.positioning_font,
	};
	let mut errors = vec!();
	render_gui_element::<CustomGuiData, GuiRenderingData, GuiRenderFn>(&program_data.gui, &mut render_data, &mut errors);
	if !errors.is_empty() {
		println!("Errors ocurred while rendering:");
		for error in errors {
			println!("{error}");
		}
	}
	
	
	
	gfx.render(&draw);
	
	Ok(())
}
