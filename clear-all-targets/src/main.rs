// created 24/04/02



use smart_read::prelude::*;
use walkdir::WalkDir;
use anyhow::*;
use std::{process::Command, result::Result as StdResult};



fn main() -> Result<()> {
	
	let top_folder = prompt!("Folder to search: ");
	let ignore_errors = prompt!("Ignore errors? "; [true] YesNoInput);
	
	let mut project_paths = vec!();
	let mut i = 0;
	for entry in WalkDir::new(top_folder) {
		
		let entry = match entry {
			StdResult::Ok(v) => v,
			StdResult::Err(err) => {
				if ignore_errors {continue;}
				return Err(err.into());
			}
		};
		let path = entry.into_path();
		if !path.is_dir() {continue;}
		
		if i % 1000 == 0 {
			println!("{i}");
		}
		i += 1;
		
		if !path.join("src").exists() {continue;}
		if !path.join("cargo.toml").exists() {continue;}
		if !path.join("target").exists() {continue;}
		
		println!("Found rust project: {path:?}");
		project_paths.push((true, path));
		
	}
	
	loop {
		println!();
		println!();
		println!();
		println!("All found projects:");
		for (i, (is_enabled, path)) in project_paths.iter().enumerate() {
			if *is_enabled {
				println!("{i}: {path:?}");
			} else {
				println!("{i}: {path:?} (disabled)");
			}
		}
		let option = prompt!("Enter \"cancel\", \"clean\", or index of path to toggle: ");
		match &*option {
			"cancel" => return Ok(()),
			"clean" => break,
			_ => {
				let StdResult::Ok(index) = option.parse::<usize>() else {
					println!("Could not parse input.");
					continue;
				};
				project_paths[index].0 = !project_paths[index].0;
			}
		}
	}
	
	println!();
	println!();
	println!();
	let ignore_errors = prompt!("About to start cleaning, ignore errors? "; [true] YesNoInput);
	
	for (is_enabled, path) in project_paths {
		if !is_enabled {continue;}
		println!("Cleaning {path:?}...");
		let result = Command::new("cargo")
			.args(["clean"])
			.current_dir(path)
			.output();
		println!("{result:?}");
		if let Err(err) = result {
			if !ignore_errors {return Err(err.into());}
		}
	}
	
	Ok(())
}
