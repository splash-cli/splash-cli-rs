pub mod photos;

use reqwest;
use self_update::cargo_crate_version;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::result::Result;

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
        }
        _ => panic!("Download Error!"),
    };

    create_dir_all(&photos_dir).unwrap();

    let filepath = format!("{}/{}", photos_dir, filename);

    let mut file = File::create(&filepath).expect("An error is occrred while creating the file.");

    file.write_all(&bytes)
        .expect("An erro is occurred while writing the file.");

    Ok(filepath)
}

#[allow(dead_code)]
fn update() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("jaemk")
        .repo_name("self_update")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
