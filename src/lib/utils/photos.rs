use std::io::Result;

use spinners::{Spinner, Spinners};

use wallpaper;
use crate::api::unsplash::{RandomPhotoParams, Unsplash};
use crate::lib::utils::download_file;
use crate::api::models;

pub fn random_photo(api: &Unsplash, params: RandomPhotoParams) -> Result<()> {
    let spinner = Spinner::new(Spinners::Arc, "Loading...".into());
    let photo = api.get_random_photo(params)?;

    match download_and_set(photo) {
        Ok(_) => {
            spinner.stop();
            println!("Wallpaper successfully set");
        },
        Err(e) => {
            spinner.stop();
            eprintln!("An error is occurred: {}", e);
        }
    }


    Ok(())
}

pub fn photo_of_the_day(api: &Unsplash) -> Result<()> {
    let spinner = Spinner::new(Spinners::Arc, "Loading...".into());
    let photo = api.get_photo_of_the_day()?;

    match download_and_set(photo) {
        Ok(_) => {
            spinner.stop();
            println!("Wallpaper successfully set");
        },
        Err(e) => {
            spinner.stop();
            eprintln!("An error is occurred: {}", e);
        }
    }


    Ok(())
}


fn download_and_set(photo: models::Photo) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let filepath = match download_file(&photo.urls.raw, &format!("{}.jpg", photo.id)) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("An error is occurred: {}", e);
            String::new()
        }
    };


    wallpaper::set_from_path(&filepath)
}
