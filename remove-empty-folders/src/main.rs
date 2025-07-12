// created 24/07/13



use smart_read::prelude::*;
use jwalk::WalkDir;
use jwalk::rayon::prelude::*;
use anyhow::*;
use std::{
	result::Result as StdResult,
	fs::{self, File},
	io::Write, path::PathBuf,
	sync::{atomic::{AtomicUsize, Ordering}, mpsc},
	time::Instant
};



#[cfg(target_os = "windows")]
const EXCLUDED_FOLDERS: &[&str] = &[
    "C:/Windows",
	"C:/ProgramData\\Microsoft\\Windows",
    "C:/System Volume Information",
    "C:/$Recycle.Bin",
];

// idk about msc or linux, so idk if these are good excluded folders
#[cfg(target_os = "macos")]
const EXCLUDED_FOLDERS: &[&str] = &[
    "/System",
    "/Library",
    "/Users/[Your Username]/Library",
    "/Applications",
    "/Volumes",
];

#[cfg(target_os = "linux")]
const EXCLUDED_FOLDERS: &[&str] = &[
    "/bin",
    "/boot",
    "/dev",
    "/etc",
    "/lib",
    "/lib64",
    "/proc",
    "/root",
    "/sbin",
    "/sys",
    "/usr",
    "/var",
];



fn main() -> Result<()> {
	
	let mut output_path = std::env::current_exe()?;
	output_path.pop();
	output_path.push("empty_folders.txt");
	
	// check for privileges
	if !privilege::user::privileged() {
		prompt!("Warning: this program was not opened as administrator, meaning it may not be able to delete all detected files. Either press enter to continue, or close this and reopen with administrator privileges.");
	}
	
	// check for previous output
	let use_previous_list = if output_path.exists() {
		prompt!("Output of previous run still exists, do you want to retry deleting these folders? "; [true] YesNoInput)
	} else {
		false
	};
	
	// get list of empty folders
	let empty_folders = if use_previous_list {
		let mut empty_folders = vec!();
		for folder_name in fs::read_to_string(&output_path)?.lines() {
			let folder_path = PathBuf::from(folder_name);
			if folder_path.exists() {
				empty_folders.push(folder_path);
			}
		}
		empty_folders
	} else {
		find_empty_folders()
	};
	
	// confirm deletions
	let mut output_file = File::create(&output_path)?;
	for folder in empty_folders {
		output_file.write_all(folder.to_string_lossy().as_bytes())?;
		output_file.write_all(&[b'\n'])?;
	}
	output_file.flush()?;
	drop(output_file);
	
	println!("Opening a list of empty folders. Please inspect this list, remove any folders that you don't want deleted, then close the editor to continue.");
	
	let result = edit::edit_file(&output_path);
	if let Err(err) = result {
		println!("Error: could not open system text editor. Error: {err}");
		let should_continue = prompt!("Would you like to continue anyways and delete the recommended folders? "; [true] YesNoInput);
		if !should_continue {
			prompt!("Affirmed, canceling deletion");
			return Ok(());
		}
	}
	
	// start deletions
	println!("Deleting folders...");
	fs::read_to_string(&output_path)?.lines().par_bridge().for_each(|folder_name| {
		let result = fs::remove_dir_all(folder_name);
		if let Err(err) = result {
			println!("Error while deleting {folder_name}: {err}");
		}
	});
	
	prompt!("Finished deleting empty folders, press enter to exit");
	fs::remove_file(output_path)?;
	
	Ok(())
}



fn find_empty_folders() -> Vec<PathBuf> {
	let top_folder = prompt!("Folder to search: "; ["C:/"]);
	let delete_empty_files = prompt!("Should folders of empty files be deleted? "; [true] YesNoInput);
	
	let (empty_folders_in, empty_folders_out) = mpsc::channel();
	let i = AtomicUsize::new(0);
	let start_time = Instant::now();
	let _ =
		WalkDir::new(top_folder)
		.skip_hidden(false)
		.process_read_dir(move |_depth, path, _read_dir_state, children| {
			
			let folders_searched = i.fetch_add(1, Ordering::Relaxed);
			if folders_searched % 1000 == 0 {println!("Folders searched: {folders_searched}");};
			
			if EXCLUDED_FOLDERS.contains(&&*path.to_string_lossy()) {
				children.clear();
				return;
			}
			
			let folder_is_empty = if delete_empty_files {
				children.iter()
					.find(|child| {
						let StdResult::Ok(child) = child else {return true;}; // default is assuming folder is important
						let path = child.path();
						if path.is_dir() {return true;} // folder is important if it contains other folders
						let StdResult::Ok(metadata) = fs::metadata(path) else {return true;};
						if metadata.len() > 0 {return true;} // folder is important if it has non-empty files
						false
					})
					.is_none() // folder is empty if it has no important contents
			} else {
				children.is_empty()
			};
			if folder_is_empty {
				let _ = empty_folders_in.send(path.to_path_buf());
			}
			
		})
		.into_iter()
		.collect::<Vec<_>>();
	
	let empty_folders = empty_folders_out.into_iter().collect::<Vec<_>>();
	println!("Finished searching in {:?}, found {} empty folders", start_time.elapsed(), empty_folders.len());
	empty_folders
}
