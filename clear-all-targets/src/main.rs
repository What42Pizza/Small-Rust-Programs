// created 24/04/02



#![feature(let_chains)]



use smart_read::prelude::*;
use jwalk::WalkDir;
use jwalk::rayon::prelude::*;
use anyhow::*;
use std::{process::Command, result::Result as StdResult, sync::{atomic::{AtomicUsize, Ordering}, mpsc}, time::Instant};



fn main() -> Result<()> {
	
	let top_folder = prompt!("Folder to search: "; ["C:/"]);
	let ignore_errors = prompt!("Ignore errors? "; [true] YesNoInput);
		
	let (cargo_folders_in, cargo_folders_out) = mpsc::channel();
	let i = AtomicUsize::new(0);
	let start_time = Instant::now();
	let _ =
		WalkDir::new(top_folder)
		.process_read_dir(move |_depth, path, _read_dir_state, children| {
			if !path.is_dir() {return;}
				
			let folders_searched = i.fetch_add(1, Ordering::Relaxed);
			if folders_searched % 1000 == 0 {println!("Folders searched: {folders_searched}");};
			
			let mut found_count = 0;
			for child in &*children {
				let StdResult::Ok(child) = child else {continue;};
				match &*child.file_name.to_string_lossy() {
					"src" => found_count += 1,
					"target" => found_count += 1,
					"Cargo.toml" => found_count += 1,
					_ => {}
				}
				if found_count == 3 {
					println!("Found rust project: {path:?}");
					let _ = cargo_folders_in.send(path.to_path_buf());
					children.clear();
					break;
				}
			}
			
		})
		.into_iter()
		.collect::<Vec<_>>();
	
	cargo_folders_out.into_iter().par_bridge().for_each(|cargo_folder| {
		println!("Cleaning project {cargo_folder:?}...");
		match
			Command::new("cargo")
				.args(["clean"])
				.current_dir(&cargo_folder)
				.output()
		{
			StdResult::Ok(output) => {
				if ignore_errors || output.status.success() {return;}
				println!("Error while running cargo clean on {cargo_folder:?}, std_err: {}", String::from_utf8_lossy(&output.stderr));
			}
			StdResult::Err(err) => {
				if !ignore_errors {
					println!("Error while cleaning target {cargo_folder:?}: {err}");
				}
			}
		};
	});
	
	println!("Finished in {:?}", start_time.elapsed());
	
	Ok(())
}
