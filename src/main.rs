use std::env;
use std::io;
use std::str::FromStr;

use clap::{App, Arg};

// API
pub mod api;

// pub mod wallpaper;

use crate::api::models::*;
use crate::api::unsplash::{RandomPhotoParams, Unsplash};

// Utils
pub mod lib;
use crate::lib::utils::photos;

fn main() -> io::Result<()> {
    let unsplah_client_id = match env::var("UNSPLASH_CLIENT_ID") {
        Ok(val) => val,
        Err(_) => panic!("Unsplash Client ID not defined."),
    };
    let api = Unsplash::new(unsplah_client_id.as_str());

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
                .default_value("landscape")
                .takes_value(true)
                .value_name("landscape|portrait|squarish"),
        )
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(false),
        )
        .subcommand(
            App::new("update")
                .alias("upgrade")
                .about("update the cli to the latest version")
        )
        .subcommand(App::new("day").alias("d"))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // Photo of the day
    if matches.subcommand_matches("day").is_some() {
        return photos::photo_of_the_day(&api);
    }

    // Otherwise => Random Photo
    let mut random_photo_params = RandomPhotoParams {
        orientation: Orientation::NONE,
        username: String::new(),
        featured: false,
        collections: vec![],
        query: String::new(),
    };

    if let Some(orientation) = matches.value_of("orientation") {
        match Orientation::from_str(orientation) {
            Ok(orientation) => random_photo_params.orientation = orientation,
            Err(e) => {
                eprintln!("{}", e.to_string());
                random_photo_params.orientation = Orientation::LANDSCAPE
            }
        }
    }

    if matches.value_of("featured").is_some() {
        random_photo_params.featured = true;
    }

    if let Some(user) = matches.value_of("user") {
        random_photo_params.username = user.into();
    }

    photos::random_photo(&api, random_photo_params)
}
