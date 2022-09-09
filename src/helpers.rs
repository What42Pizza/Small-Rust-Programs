use std::error::Error;
use std::fmt;



pub fn pad_string (input: String, len: usize) -> String {
	let mut padding = String::from("");
	for _ in 0..(len - input.len()) {
		padding += " ";
	}
	padding + &input
}

pub fn dup_char (input: char, len: usize) -> String {
	let mut output = String::from("");
	for _ in 0..len {
		output += &input.to_string();
	}
	output
}



pub fn usize_len (input: usize) -> usize {
	((input as f64).log(10.0) + 1.0).floor() as usize
}



pub fn vec_to_string <T> (input: &Vec<T>) -> String where T: fmt::Display {
	if input.len() == 0 {return String::from("");}
	let mut output = String::from(format!("{}", input[0]));
	for i in 1..input.len() {
		output += &format!(", {}", input[i]);
	}
	output
}