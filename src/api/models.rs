use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Photo {
    pub id: String,
    pub color: String,
    pub width: i32,
    pub height: i32,
    pub user: User,
    pub urls: PhotoURLS,
    pub links: PhotoLinks,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct PhotoURLS {
    pub raw: String,
    pub full: String,
    pub regular: String,
    pub thumb: String,
}

#[derive(Debug, Deserialize)]
pub struct PhotoLinks {
    pub download_location: String,
    pub download: String,
}

#[derive(Deserialize, Debug)]
pub struct PhotoOfTheDay {
    pub id: String,
}

#[derive(Debug)]
pub enum Orientation {
    LANDSCAPE,
    PORTRAIT,
    SQUARISH,
    NONE,
}

impl Orientation {
    pub fn from(orientation: String) -> Orientation {
        Orientation::from_str(orientation.as_str())
    }

    pub fn from_str(orientation: &str) -> Orientation {
        match orientation {
            "landscape" => Orientation::LANDSCAPE,
            "portrait" => Orientation::PORTRAIT,
            "squarish" => Orientation::SQUARISH,
            _ => Orientation::NONE,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Orientation::LANDSCAPE => "landscape",
            Orientation::PORTRAIT => "portrait",
            Orientation::SQUARISH => "squarish",
            Orientation::NONE => "",
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I am A")
    }
}
