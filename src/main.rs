use std::io;

use clap::{App, Arg};

// API
pub mod api;
use crate::api::models::*;
use crate::api::unsplash::{RandomPhotoParams, Unsplash};

// Utils
pub mod lib;
use crate::lib::utils::photos;

fn main() -> io::Result<()> {
    let api = Unsplash::new(&env!("UNSPLASH_CLIENT_ID"));

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
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(false),
        )
        .subcommand(App::new("day").alias("d"))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // Photo of the day
    if let Some(_) = matches.subcommand_matches("day") {
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
        random_photo_params.orientation = Orientation::from(orientation.into());
    }

    if let Some(_) = matches.value_of("featured") {
        random_photo_params.featured = true;
    }

    if let Some(user) = matches.value_of("user") {
        random_photo_params.username = user.into();
    }

    return photos::random_photo(&api, random_photo_params);
}