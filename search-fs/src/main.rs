// created 24/04/02



#![feature(let_chains)]
#![allow(static_mut_refs)]



use smart_read::prelude::*;
use jwalk::WalkDir;
use anyhow::*;
use std::{result::Result as StdResult, sync::atomic::{AtomicUsize, Ordering}, time::Instant};



fn main() -> Result<()> {
	
	static mut COUNT: AtomicUsize = AtomicUsize::new(0);
	static mut PATTERN_LOWERCASE: String = String::new();
	
	#[cfg(target_os="windows")]
	let top_folder = prompt!("Starting folder: "; ["C:/"]);
	#[cfg(target_os="linux")]
	let top_folder = prompt!("Starting folder: "; ["/"]);
	let pattern = prompt!("Search string: ");
	let ignore_errors = prompt!("Ignore errors? "; [true] YesNoInput);
	
	unsafe {
		PATTERN_LOWERCASE = pattern;
	}
	
	let start_time = Instant::now();
	let walk_dir = WalkDir::new(top_folder)
		.process_read_dir(move |_depth, _path, _read_state, children| { unsafe {
			for child in children {
				
				let count = COUNT.fetch_add(1, Ordering::Relaxed);
				let child = match child {
					StdResult::Ok(v) => v,
					StdResult::Err(err) => {
						if !ignore_errors {println!("Error while searching: {err}");}
						return;
					}
				};
				let name_lowercase = child.file_name.to_string_lossy().to_lowercase();
				if name_lowercase.contains(&PATTERN_LOWERCASE) {
					println!("Found item: {:?}", child.path());
				}
				if count % 10000 == 0 {
					println!("Searched {count} items");
				}
				
			}
		}});
	for _ in walk_dir {}
	
	println!("Finished in {:?}", start_time.elapsed());
	
	Ok(())
}
