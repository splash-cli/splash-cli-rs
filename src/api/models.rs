use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

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
pub enum OrientationError {
    InvalidArgs { details: String },
}

impl std::string::ToString for OrientationError {
    fn to_string(&self) -> String {
        match self {
            OrientationError::InvalidArgs { details } => details.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Orientation {
    LANDSCAPE,
    PORTRAIT,
    SQUARISH,
    NONE,
}

impl Orientation {
    pub fn as_str(&self) -> &str {
        match self {
            Orientation::LANDSCAPE => "landscape",
            Orientation::PORTRAIT => "portrait",
            Orientation::SQUARISH => "squarish",
            Orientation::NONE => "landscape",
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Orientation::LANDSCAPE => String::from("landscape"),
            Orientation::PORTRAIT => String::from("portrait"),
            Orientation::SQUARISH => String::from("squarish"),
            Orientation::NONE => String::from("landscape"),
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Orientation {
    type Err = OrientationError;

    fn from_str(orientation: &str) -> Result<Self, Self::Err> {
        match orientation {
            "landscape" => Ok(Orientation::LANDSCAPE),
            "portrait" => Ok(Orientation::PORTRAIT),
            "squarish" => Ok(Orientation::SQUARISH),
            "" => Ok(Orientation::LANDSCAPE),
            _ => {
                let details =
                    "Choose between 'landscape', 'portrait' or 'squarish'. Default is 'landscape'";
                Err(OrientationError::InvalidArgs {
                    details: details.to_string(),
                })
            }
        }
    }
}
