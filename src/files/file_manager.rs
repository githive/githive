
use std::io::prelude::*;
use std::fs::{File, OpenOptions, metadata, create_dir, remove_file, remove_dir_all};
use std::path::Path;

use errors::Error;

fn create_or_regenerate_empty_directory(directory: &str) {
	if !metadata(directory).is_ok() {
	}
	else if !metadata(directory).unwrap().is_dir() {
		remove_file(directory);
	}
	else {
		remove_dir_all(directory);
	}
	create_dir(directory);
}

pub struct SingleFileManager {
	file: File,
}

impl SingleFileManager {
	pub fn new(filename: &str) -> Result<SingleFileManager, Error> {
		create_or_regenerate_empty_directory(&String::from("data"));

		let mut dir_path = "".to_string();
		dir_path.push_str("data/");

		for dir in filename.split('/') {
			if dir != filename.split('/').last().unwrap() {
				dir_path.push('/');
				dir_path.push_str(dir);
				create_or_regenerate_empty_directory(&dir_path);	
			}
		}

		let mut path = Path::new("data").join(filename);
		let mut file = try!(OpenOptions::new().create(true).read(true).write(true).open(path));
		file.write_all(b"TESTING FILE IO");

		Ok(
			SingleFileManager {
				file: file,
			}
		)
	}
}
