#![allow(unused)]
#![warn(unused_must_use)]

#![feature(duration_constants)]
#![feature(random)]



use std::{random::random, thread::sleep, time::Duration};
use smart_read::prelude::*;



fn main() {
	let mut money = prompt!("How much money do you want to bring? "; U16Input) as usize;
	let starting_money = money;
	let mut earned_money = 0usize;
	println!("Press ctrl+c to exit");
	loop {
		println!();
		match prompt!(format!("You have ${money}"); ["Work", "Gamble", "Exit"]).0 {
			
			0 => {
				for i in 1..=10 {
					println!("Working{}", ".".repeat(i));
					sleep(Duration::SECOND / 2);
				}
				money += 100;
				earned_money += 100;
				println!("Earned $100");
			}
			
			1 => {
				println!();
				println!("Enter \"stop\" to stop gambling");
				'gamble: loop {
					let gamble_amount = prompt!("How much do you want to gamble? "; TransformValidate (|mut input| {
						if &input == "stop" {return Ok(-1);}
						if input.starts_with("$") {input = input[1..].to_string();}
						let Ok(input) = input.parse::<isize>() else {return Err(String::from("Could not parse input, must be an integer. To exit gambling, enter \"stop\""));};
						if input < 10 {return Err(String::from("Input must be at least 10"));}
						if input > money as isize {return Err(String::from("Cannot bet more than you have"));}
						Ok(input)
					}));
					if gamble_amount == -1 {break 'gamble;}
					let gamble_amount = gamble_amount as usize;
					money -= gamble_amount;
					for i in 1..=3 {
						println!("The result is{}", ".".repeat(i));
						sleep(Duration::SECOND);
					}
					println!();
					if random::<u8>() < 128 {
						println!("Plus ${}!!!", gamble_amount * 2);
						money += gamble_amount * 2;
					} else {
						println!("No win!");
					}
					println!();
				}
			}
			
			2 => {
				println!();
				println!("You started with ${starting_money}, earned ${earned_money} from working, and ended with ${money}");
				match money as isize - starting_money as isize - earned_money as isize {
					diff if diff > 0 => println!("You ended up taking ${diff} from the house!"),
					0 => println!("You ended up right where you started."),
					diff => println!("You ended up giving ${} to the house", -diff),
				}
				break;
			}
			
			_ => unreachable!()
		}
	}
}
