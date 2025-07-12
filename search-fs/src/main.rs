// created 24/04/02



#![feature(let_chains)]



use smart_read::prelude::*;
use jwalk::WalkDir;
use anyhow::*;
use std::{result::Result as StdResult, sync::atomic::{AtomicUsize, Ordering}, time::Instant};



fn main() -> Result<()> {
	
	let top_folder = prompt!("Starting folder: "; ["C:/"]);
	let pattern = prompt!("Search string: ");
	let ignore_errors = prompt!("Ignore errors? "; [true] YesNoInput);
	
	let start_time = Instant::now();
	let i = AtomicUsize::new(0);
	let _ = WalkDir::new(top_folder)
		.process_read_dir(move |_depth, _path, _read_state, children| {
			for child in children {
				
				let count = i.fetch_add(1, Ordering::Relaxed);
				let path = match child {
					StdResult::Ok(v) => v,
					StdResult::Err(err) => {
						if !ignore_errors {println!("Error while searching: {err}");}
						return;
					}
				}.path();
				let Some(name) = path.file_name() else {continue;};
				if name.to_string_lossy().find(&pattern).is_some() {
					println!("Found item: {path:?}");
				}
				if count % 10000 == 0 {
					println!("Searched {count} items");
				}
				
			}
		}).into_iter().collect::<Vec<_>>();
	
	println!("Finished in {:?}", start_time.elapsed());
	
	Ok(())
}
