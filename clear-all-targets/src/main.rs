// created 24/04/02



use smart_read::prelude::*;
use walkdir::WalkDir;
use anyhow::*;
use std::{process::Command, result::Result as StdResult};



fn main() -> Result<()> {
	
	let top_folder = prompt!("Folder to search: ");
	let ignore_errors = prompt!("Ignore errors? "; [true] YesNoInput);
	
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
			println!("Folders searched: {i}");
		}
		i += 1;
		
		if !path.join("src").exists() {continue;}
		if !path.join("cargo.toml").exists() {continue;}
		if !path.join("target").exists() {continue;}
		
		println!("Found rust project: {path:?}");
		let result = Command::new("cargo")
			.args(["clean"])
			.current_dir(path)
			.output();
		if let Err(err) = result {
			if !ignore_errors {return Err(err.into());}
		}
		
	}
	
	Ok(())
}
