use anyhow::Result;

use spinners::{Spinner, Spinners};

use crate::api::models::{self, Photo};
use crate::api::unsplash::{RandomPhotoParams, Unsplash};
use crate::lib::utils::download_file;
use wallpaper;

pub fn random_photo(api: &Unsplash, params: RandomPhotoParams) -> Result<()> {
    let spinner = Spinner::new(Spinners::Arc, "Loading...".into());
    let photo = api.get_random_photo(params)?;
    download_check(photo, spinner)
}

pub fn photo_of_the_day(api: &Unsplash) -> Result<()> {
    let spinner = Spinner::new(Spinners::Arc, "Loading...".into());
    let photo = api.get_photo_of_the_day()?;
    download_check(photo, spinner)
}

fn download_and_set(photo: models::Photo) -> Result<(), Box<dyn std::error::Error>> {
    let filepath = match download_file(&photo.urls.raw, &format!("{}.jpg", photo.id)) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("An error is occurred: {}", e);
            String::new()
        }
    };

    wallpaper::set_from_path(&filepath)
}

fn download_check(photo: Photo, spinner: Spinner) -> Result<()> {
    match download_and_set(photo) {
        Ok(_) => {
            spinner.stop();
            println!("Wallpaper successfully set");
        }
        Err(e) => {
            spinner.stop();
            eprintln!("An error is occurred: {}", e);
        }
    }

    Ok(())
}
