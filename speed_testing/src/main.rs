// created 11/14/22



use std::time::Instant;

use rand::prelude::*;



fn main() {
	LeetcodePalindromesTests{}.run_tests(10000, 10);
}





trait Test {

	type Input: 'static;
	type Output: 'static;

	fn get_test_fns() -> &'static [(&'static dyn Fn(Self::Input) -> Self::Output, &'static str)];
	fn get_input() -> Self::Input;

	fn run_tests (&self, batch_size: usize, batch_count: usize) {
		let test_fns = Self::get_test_fns();
		let mut test_fn_times = vec![0; test_fns.len()];
		for _ in 0..batch_count {
			for (i, current_fn) in test_fns.iter().enumerate() {
				let start_instant = Instant::now();
				for _ in 0..batch_size {
					let input = Self::get_input();
					current_fn.0(input);
				}
				test_fn_times[i] += start_instant.elapsed().as_nanos();
			}
		}
		println!("Times for each test:");
		for (i, time) in test_fn_times.iter().enumerate() {
			//println!("{}: {time} ns total, {} ns per call", test_fns[i].1, *time as f64 / ((count / batch_size) * batch_size) as f64);
			println!("{}: {} ns per call", test_fns[i].1, *time as f64 / (batch_size * batch_count) as f64);
		}
	}

}





/*
My results: (release mode)
get_digits_1:     298.29072 ns per call  #2
get_digits_2:     151.83426 ns per call  #1
get_digits_3:     349.41382 ns per call  #3
double_dabble_1: 2266.07851 ns per call  #6
double_dabble_2: 1509.38596 ns per call  #4
double_dabble_3: 1574.70155 ns per call  #5
Conclusion: a single, complicated instruction is MUCH faster than multiple easy instructions
*/

struct LeetcodePalindromesTests {}

impl Test for LeetcodePalindromesTests {
	type Input = i32;
	type Output = Vec<u8>;
	fn get_test_fns() -> &'static [(&'static dyn Fn(i32) -> Vec<u8>, &'static str)] {
		&[
			(&get_digits_1, "get_digits_1"),
			(&get_digits_2, "get_digits_2"),
			(&get_digits_3, "get_digits_3"),
			(&double_dabble_1, "double_dabble_1"),
			(&double_dabble_2, "double_dabble_2"),
			(&double_dabble_3, "double_dabble_3"),
		]
	}
	fn get_input() -> i32 {
		let mut rng = rand::thread_rng();
		rng.gen()
	}
}



fn double_dabble_3 (x: i32) -> Vec<u8> {
	let mut output = vec![0];
	let mut test_bit = 1_u64 << (32 - x.leading_zeros());
	let x = x as u64;
	while test_bit > 0 {
		let last_index = output.len() - 1;
		let mut add_bit = (x & test_bit) > 0;
		for i in 0..=last_index {
			let new_add_bit = output[i] >= 5;
			if new_add_bit {
				output[i] -= 5;
			}
			output[i] = (output[i] << 1) + add_bit as u8;
			add_bit = new_add_bit;
		}
		if add_bit {
			output.push(1);
		}
		test_bit >>= 1;
	}
	output
}



fn double_dabble_2 (x: i32) -> Vec<u8> {
	let mut output = vec![0];
	let mut test_bit = 1_u64 << (32 - x.leading_zeros());
	let x = x as u64;
	while test_bit > 0 {
		let last_index = output.len() - 1;
		let mut add_bit = (x & test_bit) > 0;
		for i in 0..=last_index {
			add_bit = if output[i] >= 5 {
				output[i] = ((output[i] - 5) << 1) + add_bit as u8;
				true
			} else {
				output[i] = (output[i] << 1) + add_bit as u8;
				false
			}
		}
		if add_bit {
			output.push(1);
		}
		test_bit >>= 1;
	}
	output
}



fn double_dabble_1 (x: i32) -> Vec<u8> {
	let mut x = x as u32;
	let mut output = vec![0];
	const THRESHOLD: u32 = u32::MAX >> 1;
	let leading_zeros = x.leading_zeros();
	x <<= leading_zeros;
	for _ in 0..32-leading_zeros {
		let last_index = output.len() - 1;
		for i2 in (0..=last_index).rev() {
			if output[i2] >= 5 {
				if i2 == last_index {output.push(0);}
				output[i2+1] += 1;
				output[i2] -= 5;
			}
			output[i2] <<= 1;
		}
		if x > THRESHOLD {
			output[0] += 1;
		}
		x <<= 1;
	}
	output
}



fn get_digits_3 (mut x: i32) -> Vec<u8> {
	let mut output = vec!();
	let starting_power = (x as f32).log10().floor() as u32;
	let mut current_mod = 10_i32.pow(starting_power);
	while current_mod > 1 {
		let mut count = 0;
		while x >= current_mod {
			x -= current_mod;
			count += 1;
		}
		current_mod /= 10;
		output.push(count);
	}
	output.push(x as u8);
	output
}



fn get_digits_2 (mut x: i32) -> Vec<u8> {
	let mut output = vec!();
	while x > 0 {
		output.push((x % 10) as u8);
		x /= 10;
	}
	output
}



fn get_digits_1 (mut x: i32) -> Vec<u8> {
	let mut output = vec!();
	let starting_power = (x as f32).log10().floor() as u32;
	let mut current_digit_mod = 10_i32.pow(starting_power);
	while current_digit_mod > 1 {
		let remaining = x % current_digit_mod;
		let cut_off = x - remaining;
		x = remaining;
		output.push((cut_off / current_digit_mod) as u8);
		current_digit_mod /= 10;
	}
	output.push(x as u8);
	output
}
