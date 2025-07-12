use crate::prelude::*;
use std::time::Instant;



pub fn update(program_data: &mut ProgramData) {
	
	match program_data.ant_controls.as_fast_as_possible {
		
		true => {
			let start_instant = Instant::now();
			while (start_instant.elapsed().as_millis() as usize) < AFAP_MILLIS_COUNT {
				for _ in 0..AFAP_STEP_COUNT {
					if !program_data.ant.is_active {return;}
					do_single_update(program_data);
				}
			}
		}
		
		false => {
			for _ in 0..program_data.ant_controls.steps_per_frame {
				if !program_data.ant.is_active {return;}
				do_single_update(program_data);
			}
		}
		
	}
}



pub fn do_single_update(program_data: &mut ProgramData) {
	let canvas = &mut program_data.canvas;
	let ant = &mut program_data.ant;
	
	let curr_val_index = canvas.get_cell_index(ant.pos);
	let curr_val = canvas.raw_data[curr_val_index];
	let curr_rule = &ant.rules[curr_val as usize];
	
	canvas.raw_data[curr_val_index] = curr_rule.next_val;
	let texture_index = canvas.get_texture_index_of_cell(ant.pos);
	canvas.textures[texture_index].is_dirty = true;
	
	ant.dir = (ant.dir + curr_rule.dir_change) % 4;
	let curr_move_vec = rotate_move_vec(curr_rule.move_vec, ant.dir);
	ant.pos.0 = ant.pos.0.wrapping_add_signed(curr_move_vec.0);
	ant.pos.1 = ant.pos.1.wrapping_add_signed(curr_move_vec.1);
	
	if
		ant.pos.0 >= canvas.raw_data_size.0
		|| ant.pos.1 >= canvas.raw_data_size.1
	{
		ant.is_active = false;
	}
	
}



pub fn rotate_move_vec(move_vec: (isize, isize), dir: u8) -> (isize, isize) {
	match dir {
		0 => {
			move_vec
		}
		1 => {
			(move_vec.1, -move_vec.0)
		}
		2 => {
			(-move_vec.0, -move_vec.1)
		}
		3 => {
			(-move_vec.1, move_vec.0)
		}
		_ => panic!("cannot rotate when dir is >4 (dir is {dir})"),
	}
}
