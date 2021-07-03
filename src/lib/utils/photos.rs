use std::io::Result;

use spinners::{Spinner, Spinners};

use crate::api::unsplash::{RandomPhotoParams, Unsplash};

pub fn random_photo(api: &Unsplash, params: RandomPhotoParams) -> Result<()> {
    let spinner = Spinner::new(Spinners::Arc, "Loading...".into());
    let photo = api.get_random_photo(params)?;

    wallpaper::set_from_url(&photo.urls.raw).expect("Wallpaper Error");

    spinner.stop();

    return Ok(());
}

pub fn photo_of_the_day(api: &Unsplash) -> Result<()> {
    let spinner = Spinner::new(Spinners::Arc, "Loading...".into());
    let photo = api.get_photo_of_the_day()?;

    wallpaper::set_from_url(&photo.urls.raw).expect("Wallpaper Error"); // TODO: Error handling done well...

    spinner.stop();

    return Ok(());
}
