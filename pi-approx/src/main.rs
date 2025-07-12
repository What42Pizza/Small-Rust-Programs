#![feature(duration_constants)]

use std::{thread, time::Duration};
use bigdecimal::BigDecimal;

fn main() {
	
	let mut v = BigDecimal::from(1).half().sqrt().unwrap();
	let mut mult = BigDecimal::from(4);
	let one_half = BigDecimal::from(1).half();
	let one_fourth = one_half.half();
	
	let mut i = 0;
	loop {
		let m2 = one_half.clone() - (one_fourth.clone() - v.clone() * v.clone() * one_fourth.clone()).sqrt().unwrap();
		v = (v.clone() * v.clone() * one_fourth.clone() + m2.clone() * m2.clone()).sqrt().unwrap();
		mult *= 2;
		i += 1;
		let approx = v.clone() * mult.clone();
		println!("Iteration {i} - approx: {}", approx);
		thread::sleep(Duration::SECOND / 10);
	}
	
}
