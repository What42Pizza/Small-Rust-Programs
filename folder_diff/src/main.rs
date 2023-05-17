// created 23/05/17



#![allow(unused)]

use std::path::{Path, PathBuf};



fn main() {
	
	let folder_1_path = Path::new("C:/Users/Jonathan/curseforge/minecraft/Instances/Eerie/mods");
	let folder_2_path = Path::new("C:/Users/Jonathan/curseforge/minecraft/Instances/Eerie Backup 13/mods");
	
	let mut folder_1_files = get_files_in_folder(folder_1_path);
	let mut folder_2_files = get_files_in_folder(folder_2_path);
	
	let mut i = folder_1_files.len() - 1;
	loop {
		let curr_file = &folder_1_files[i];
		let folder_2_i = folder_2_files.iter().position(|item| item.file_name() == curr_file.file_name());
		if let Some(folder_2_i) = folder_2_i {
			folder_1_files.remove(i);
			folder_2_files.remove(folder_2_i);
		}
		if i == 0 {break;}
		i -= 1;
	}
	
	println!("\n\n\nFolder 1 contents:");
	for file in folder_1_files {
		println!("{:?}", file.file_name());
	}
	println!("\n\n\nFolder 2 contents:");
	for file in folder_2_files {
		println!("{:?}", file.file_name());
	}
	
}



fn get_files_in_folder (folder: impl AsRef<Path>) -> Vec<PathBuf> {
	let folder = folder.as_ref();
	folder
		.read_dir()
		.expect("failed to read directory")
		.map(|entry| entry.expect("failed to get entry").path())
		.collect::<Vec<PathBuf>>()
}
