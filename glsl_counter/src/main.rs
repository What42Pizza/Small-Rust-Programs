// created 23/08/06

use std::{path::{PathBuf, Path}, fs};
use anyhow::Result;

const SHADER_FOLDER: &str = "C:/Users/Jonathan/curseforge/minecraft/Instances/[MAIN] 1.19.4 Iris/shaderpacks/Custom";

const INCLUDE_FSH: bool = false;
const INCLUDE_VSH: bool = false;
const INCLUDE_GLSL: bool = true;
const INCLUDE_PROPERTIES: bool = true;



pub fn main() -> Result<()> {
	
	let mut folders_to_check = vec!(PathBuf::from(SHADER_FOLDER));
	let mut total = 0;
	loop {
		
		let Some(curr_folder) = folders_to_check.pop() else {
			break;
		};
		let children = fs::read_dir(curr_folder)?;
		
		for child in children {
			let child = child?.path();
			if child.is_file() {
				total += get_file_total(child)?;
			} else {
				folders_to_check.push(child);
			}
		}
		
	}
	
	println!("Total: {total}");
	
	Ok(())
}



pub fn get_file_total(path: impl AsRef<Path>) -> Result<usize> {
	let path = path.as_ref();
	
	let Some(extension) = path.extension() else {return Ok(0);};
	let extension = extension.to_string_lossy().to_string();
	match &*extension {
		"fsh" if INCLUDE_FSH => {},
		"vsh" if INCLUDE_VSH => {},
		"glsl" if INCLUDE_GLSL => {},
		"properties" if INCLUDE_PROPERTIES => {},
		_ => return Ok(0),
	}
	
	let total = fs::read_to_string(path)?
		.lines()
		.filter(|line| {
			let mut line = *line;
			let mut comment_start = line.find("//");
			if comment_start.is_none() && extension == "properties" {
				comment_start = line.find("#");
			}
			if let Some(comment_start) = comment_start {
				line = &line[0..comment_start];
			}
			!line.trim().is_empty()
		})
		.count();
		
	Ok(total)
}
