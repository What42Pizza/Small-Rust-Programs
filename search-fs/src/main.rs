// created 24/04/02



#![feature(let_chains)]
#![allow(static_mut_refs)]



use smart_read::prelude::*;
use jwalk::WalkDir;
use anyhow::*;
use std::{result::Result as StdResult, sync::atomic::{AtomicUsize, Ordering}, time::Instant};



fn main() -> Result<()> {
	
	static mut COUNT: AtomicUsize = AtomicUsize::new(0);
	static mut PATTERN: String = String::new();
	static mut PATTERN_BYTES: &[u8] = &[];
	
	#[cfg(target_os="windows")]
	let top_folder = prompt!("Starting folder: "; ["C:/"]);
	#[cfg(target_os="linux")]
	let top_folder = prompt!("Starting folder: "; ["/"]);
	let pattern = prompt!("Search string: ");
	let ignore_errors = prompt!("Ignore errors? "; [true] YesNoInput);
	
	unsafe {
		PATTERN = pattern;
		PATTERN_BYTES = PATTERN.as_bytes();
	}
	let pattern_len = unsafe { PATTERN_BYTES.len() };
	
	let start_time = Instant::now();
	let _ = WalkDir::new(top_folder)
		.process_read_dir(move |_depth, _path, _read_state, children| { unsafe {
			for child in children {
				
				let count = COUNT.fetch_add(1, Ordering::Relaxed);
				let path = match child {
					StdResult::Ok(v) => v,
					StdResult::Err(err) => {
						if !ignore_errors {println!("Error while searching: {err}");}
						return;
					}
				}.path();
				let Some(name) = path.file_name() else {continue;};
				let name_bytes = name.as_encoded_bytes();
				if name_bytes.windows(pattern_len).any(|window| window == PATTERN_BYTES) {
					println!("Found item: {path:?}");
				}
				if count % 10000 == 0 {
					println!("Searched {count} items");
				}
				
			}
		}}).into_iter().collect::<Vec<_>>();
	
	println!("Finished in {:?}", start_time.elapsed());
	
	Ok(())
}
