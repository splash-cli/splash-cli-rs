pub mod photos;

use reqwest;
use std::result::Result;
use std::io::{ Write };
use std::fs::{File, create_dir_all};

use dirs;

pub fn download_file(url: &str, filename: &str) -> Result<String, reqwest::Error> {
	let response = match reqwest::blocking::get(url) {
		Ok(response) => response.bytes()?,
		Err(e) => return Err(e),
	};

	let bytes = response.as_ref();

	let photos_dir = match dirs::picture_dir() {
		Some(dir) => {
			let mut directory = String::from(dir.to_str().unwrap());
			directory.push_str("/splash_RS_photos");

			directory
		},
		_ => panic!("Download Error!")
	};

	create_dir_all(&photos_dir).unwrap();

	let filepath = format!("{}/{}", photos_dir, filename);

	let mut file = File::create(&filepath)
		.expect("An error is occrred while creating the file.");

	file
		.write_all(&bytes)
		.expect("An erro is occurred while writing the file.");


	Ok(filepath)
}
