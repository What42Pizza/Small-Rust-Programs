use std::error::Error;
use std::io::{self};

use crate::errors;
use crate::helpers;



pub fn get_string() -> Result<String, Box<dyn Error>> {
    
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)?;
    input_string = input_string[..input_string.len()-2].to_string();
    
    match input_string.as_str() {
        "panic!" => {panic!();},
        "force exit" => errors::ForcedExitError::new(),
        _ => Ok(input_string),
    }
    
}



pub fn get_string_bounded (allowed_strings: Vec<&str>) -> Result<String, Box<dyn Error>> {
	println!("Options: {}", helpers::vec_to_string(&allowed_strings));
	loop {
		
		let input_string = get_string()?;
		if !allowed_strings.contains(&&*input_string) {
			println!("Could not understant input. Please enter one of {}", helpers::vec_to_string(&allowed_strings));
			continue;
		}
		
		return Ok(input_string);
		
	}
}



pub fn get_int_bounded (lower_bound: usize, upper_bound: usize) -> Result<usize, Box<dyn Error>> {
    println!("Enter a number from {} to {}:", lower_bound, upper_bound);
    loop {
        
        let input_string = get_string()?;
        let input_result = input_string.parse();
        
        if let Err(error_cause) = input_result {
            println!("Could not understand input. (\"{}\", your message: \"{}\")", error_cause, input_string);
			println!("Only digits are allowed, do not enter any other characters.");
            continue;
        }
        
        let input = input_result.unwrap();
        
        if input < lower_bound || input > upper_bound {
             println!("Please enter a number from {} to {}.", lower_bound, upper_bound);
             continue;
        }
        
        return Ok(input);
        
    }
}



pub fn get_int() -> Result<usize, Box<dyn Error>> {
    loop {
        
        let input_string = get_string()?;
        let input_result = input_string.parse();
        
        if let Err(error_cause) = input_result {
            println!("Could not understand input. (\"{}\", your message: \"{}\")", error_cause, input_string);
			println!("Only digits are allowed, do not enter any other characters.");
            continue;
        }
		
        return Ok(input_result.unwrap());
        
    }
}



pub fn get_bool() -> Result<bool, Box<dyn Error>> {
    println!("Enter (y)es or (n)o:");
    loop {
        
        let input_string = get_string()?;
        
        match input_string.as_str() {
            "y" => {return Ok(true);}
            "n" => {return Ok(false);}
            _ => {
                println!("Please enter (y)es or (n)o:");
            }
        }
        
    }
}