use std::error::Error;
use std::io::{self};

use crate::errors;



pub fn get_string() -> Result<String, Box<dyn Error>> {
    
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)?;
    input_string = input_string[..input_string.len()-2].to_string();
    
    match input_string.as_str() {
        "panic!" => {panic!();},
        "exit" => errors::ForcedExitError::new("Exited by user".to_string()),
        _ => Ok(input_string),
    }
    
}



pub fn get_int (lower_bound: usize, upper_bound: usize) -> Result<usize, Box<dyn Error>> {
    println!("Enter a number from {} to {}:", lower_bound, upper_bound);
    loop {
        
        let input_string = get_string()?;
        let input_result = input_string.parse();
        
        if let Err(error_cause) = input_result {
            println!("Could not understand input. (\"{}\", your message: \"{}\")", error_cause, input_string);
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