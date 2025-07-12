// created 23/12/07

// This program uses Lloyd's algorithm (https://en.wikipedia.org/wiki/Lloyd's_algorithm)



#![allow(unused)]
#![warn(unused_must_use)]





pub mod settings {
	
	pub const POINT_COUNT: usize = 5;
	pub const COMPUTE_RESOLUTION: usize = 512;
	pub const AVERAGE_MOVEMENT_EXIT_THRESHOLD: f32 = 0.0001;
	pub const MAX_ITERATIONS: usize = 100;
	
	pub const EVEN_LENGTHS_LERP: f32 = 0.2; // pulls the points towards lengths from center that are evenly spaced
	
	pub fn point_is_in_shape(x: f32, y: f32) -> bool {
		(x * x + y * y).sqrt() < 1.0
	}
	
	pub fn print_point_data(x: f32, y: f32) {
		const LEN_MULT: f32 = 1.0;
		let (x, y) = (x * LEN_MULT, y * LEN_MULT);
		let len = (x * x + y * y).sqrt();
		let gaussian = std::f32::consts::E.powf(len * len * -1. * 0.85);
		let gaussian_total = unsafe {
			static mut TOTAL: f32 = 0.;
			TOTAL += gaussian;
			TOTAL
		};
		println!("({x:>6.3}, {y:>6.3})   weight: {gaussian:.3}  total: {gaussian_total:.3}");
	}
	
}



pub mod prelude {
	pub use crate::settings::*;
}



use crate::prelude::*;





fn main() {
	
	let mut points = Vec::with_capacity(POINT_COUNT);
	for _ in 0..POINT_COUNT {
		let point = loop {
			let point_x: f32 = rand::random();
			let point_y: f32 = rand::random();
			if point_is_in_shape(point_x, point_y) {
				break (point_x, point_y);
			}
		};
		points.push(point);
	}
	
	let mut count = 0;
	loop {
		
		let prev_points = points.clone();
		iterate(&mut points);
		let average_movement = get_average_movement(&points, &prev_points);
		println!("Movement: {average_movement}");
		if average_movement < AVERAGE_MOVEMENT_EXIT_THRESHOLD {
			break;
		}
		
		count += 1;
		if count == MAX_ITERATIONS {
			println!("WARNING: Max iterations reached");
			break;
		}
		
	}
	
	println!();
	println!("Final Points:");
	for point in points {
		print_point_data(point.0, point.1);
	}
	
}



pub fn iterate(points: &mut [(f32, f32)]) {
	
	#[derive(Debug, Copy, Clone, Default)]
	struct AverageData {
		pub total: (f32, f32),
		pub count: usize,
	}
	let mut point_averages = vec![AverageData::default(); points.len()];
	
	const HALF_RESOLUTION: isize = COMPUTE_RESOLUTION as isize / 2;
	for y in -HALF_RESOLUTION ..= HALF_RESOLUTION {
		for x in -HALF_RESOLUTION ..= HALF_RESOLUTION {
			let (x, y) = (x as f32 / HALF_RESOLUTION as f32, y as f32 / HALF_RESOLUTION as f32);
			if !point_is_in_shape(x, y) {continue;}
			
			let mut closest_point_i = 0;
			let mut closest_dist = dist((x, y), points[0]);
			for (i, point) in points.iter().enumerate().skip(1) {
				let dist = dist((x, y), *point);
				if dist < closest_dist {
					closest_point_i = i;
					closest_dist = dist;
				}
			}
			
			let average_data = &mut point_averages[closest_point_i];
			average_data.total.0 += x;
			average_data.total.1 += y;
			average_data.count += 1;
			
		}
	}
	
	for (i, point) in points.iter_mut().enumerate() {
		let average_data = &point_averages[i];
		if average_data.count == 0 {
			println!("WARNING: point was found with no closest pixels (may need re-run and/or higher computer resolution");
			continue;
		}
		let new_x = average_data.total.0 / average_data.count as f32;
		let new_y = average_data.total.1 / average_data.count as f32;
		*point = (new_x, new_y);
	}
	
	points.sort_by(|a, b| {
		let a_len = (a.0 * a.0 + a.1 * a.1).sqrt();
		let b_len = (b.0 * b.0 + b.1 * b.1).sqrt();
		a_len.total_cmp(&b_len)
	});
	for (i, point) in points.iter_mut().enumerate() {
		let point_len = (point.0 * point.0 + point.1 * point.1).sqrt();
		let target_len = (i + 1) as f32 / POINT_COUNT as f32;
		let new_len = lerp(point_len, target_len, EVEN_LENGTHS_LERP);
		*point = (point.0 / point_len * target_len, point.1 / point_len * target_len);
	}
	
}



pub fn get_average_movement(points: &[(f32, f32)], prev_points: &[(f32, f32)]) -> f32 {
	let mut output = 0.;
	for (point, prev_point) in points.iter().zip(prev_points) {
		output += dist(*point, *prev_point);
	}
	output / POINT_COUNT as f32
}



pub fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
	let dx = a.0 - b.0;
	let dy = a.1 - b.1;
	(dx * dx + dy * dy).sqrt()
}

pub fn lerp(value: f32, low: f32, high: f32) -> f32 {
	low + value * (high - low)
}
