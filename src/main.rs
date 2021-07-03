use clap::{App, Arg};
use serde::Deserialize;
use spinners::{Spinner, Spinners};
use std::{collections::HashMap, io};
use ureq;
use wallpaper;

pub mod api;
use api::unsplash::{Photo, Unsplash};

#[derive(Deserialize)]
struct PhotoOfTheDay {
    id: String,
}

fn main() -> io::Result<()> {
    let mut api = Unsplash::new(&env!("UNSPLASH_CLIENT_ID"));

    let matches = App::new("splash")
        .about("Unsplash Photos")
        .arg(
            Arg::with_name("featured")
                .short("f")
                .long("featured")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("user")
                .short("u")
                .long("user")
                .takes_value(true)
                .conflicts_with("featured"),
        )
        .arg(
            Arg::with_name("orientation")
                .short("o")
                .long("orientation")
                .takes_value(true)
                .value_name("landscape|portrait|squarish"),
        )
        .subcommand(App::new("day").alias("d"))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if let Some(_) = matches.subcommand_matches("day") {
        let spinner = Spinner::new(Spinners::Arc, "Loading...".into());

        let response: PhotoOfTheDay = ureq::get("https://lambda.splash-cli.app/api")
            .call()
            .expect("An error is occurred.")
            .into_json()?;

        let photo: Photo = api.get_photo(&response.id)?;

        wallpaper::set_from_url(&photo.urls.raw).expect("Wallpaper Error");

        spinner.stop();

        return Ok(());
    }

    // RRANDOM
    let spinner = Spinner::new(Spinners::Arc, "Loading...".into());
    let mut options: HashMap<&str, &str> = HashMap::new();

    if let Some(orientation) = matches.value_of("orientation") {
        options.insert("orientation", orientation);
    }

    if let Some(_) = matches.value_of("featured") {
        options.insert("featured", "true");
    }

    if let Some(user) = matches.value_of("user") {
        options.insert("user", user);
    }

    spinner.message("Getting Url...".into());

    let photo: Photo = api.get_random_photo(options)?;

    wallpaper::set_from_url(&photo.urls.raw).expect("Wallpaper Error");

    spinner.stop();

    return Ok(());
}

// fn download_image(url: &Url) -> Result<String> {
//     let cache_dir = dirs::cache_dir().ok_or("no cache dir")?;
//     let segments = url.path_segments().ok_or("no path segments")?;
//     let mut file_name = segments.last().ok_or("no file name")?;
//     if file_name.is_empty() {
//         file_name = "wallpaper";
//     }
//     let file_path = cache_dir.join(file_name);
//     let mut file = File::create(&file_path)?;
//     reqwest::get(url.as_str())?.copy_to(&mut file)?;
//     Ok(file_path.to_str().to_owned().unwrap().into())
// }

// use dirs;

// fn download_image(url: &Url) -> Result<String, String> {
//     let images_dir = dirs::picture_dir().ok_or("no Picutres dir")?;
//     let segments = url.path_segments().ok_or("No path segments")?;
//     let mut filename = segments.last().ok_or("no filename")?;

//     let response = ureq::get(url.as_str()).call()?;
//     response.

//     Ok(());
// }
