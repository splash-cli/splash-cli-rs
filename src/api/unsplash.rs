use isahc::prelude::*;
use serde_json::from_str;
use std::{collections::HashMap, str::FromStr};
use url::Url;

use crate::api::models::*;

pub type Params = HashMap<&'static str, &'static str>;

pub struct Unsplash {
    api_key: String,
    url: Url,
}

impl Unsplash {
    pub fn new(api_key: &str) -> Unsplash {
        Unsplash {
            api_key: String::from(api_key),
            url: Url::parse("https://api.unsplash.com").unwrap(),
        }
    }

    pub fn get_photo(&self, photo_id: &str) -> Result<Photo, isahc::Error> {
        let response_text = self.get(&format!("/photos/{}", photo_id), HashMap::new())?;

        let photo: Photo = from_str(&response_text).expect("Error while decoding JSON");

        Ok(photo)
    }

    pub fn get_random_photo(&self, params: RandomPhotoParams) -> Result<Photo, isahc::Error> {
        let response_text = self.get("/photos/random", params.into_hash_map())?;
        let photo: Photo = from_str(&response_text).expect("Error while decoding json");

        Ok(photo)
    }

    pub fn get_photo_of_the_day(&self) -> Result<Photo, isahc::Error> {
        let url = Url::parse("https://lambda.splash-cli.app/api").unwrap();

        let mut response = isahc::get(url.as_str())?;
        let text = response.text()?;

        // Parse JSON
        let data: PhotoOfTheDay = from_str(&text).expect("Error while parsing JSON");

        let photo = self.get_photo(&data.id)?;

        Ok(photo)
    }

    // Private API
    fn get(&self, path: &str, query_params: HashMap<&str, String>) -> Result<String, isahc::Error> {
        let mut url = self.url.clone();
        url.set_path(path);

        for key in query_params.keys() {
            if let Some(value) = query_params.get(key) {
                url.query_pairs_mut().append_pair(key, value.as_str());
            }
        }

        let req = isahc::Request::builder()
            .method("GET")
            .header("Authorization", &format!("Client-ID {}", self.api_key))
            .uri(url.as_str())
            .body(())?;

        let mut response = isahc::send(req)?;
        let text = response.text()?;

        Ok(text)
    }
}
pub struct RandomPhotoParams {
    pub orientation: Orientation,
    pub username: String,
    pub featured: bool,
    pub collections: Vec<String>,
    pub query: String,
}

impl RandomPhotoParams {
    pub fn from(hash_map: HashMap<&'static str, &'static str>) -> RandomPhotoParams {
        RandomPhotoParams {
            orientation: Orientation::from_str(hash_map["orientation"]).unwrap(),
            username: hash_map["username"].into(),
            query: hash_map["query"].into(),
            featured: hash_map["featured"] == "true",
            collections: hash_map["collections"]
                .split(',')
                .map(String::from)
                .collect::<Vec<String>>(),
        }
    }

    pub fn into_hash_map(self) -> HashMap<&'static str, String> {
        let mut hash_map = HashMap::<&'static str, String>::new();

        let mut collections: String = String::new();

        for collection in self.collections {
            collections.push_str(&format!("{},", collection));
        }

        hash_map.insert("collections", collections);
        hash_map.insert(
            "featured",
            String::from(match self.featured {
                true => "true",
                false => "false",
            }),
        );

        hash_map.insert("query", self.query);
        hash_map.insert("orientation", self.orientation.to_string());

        hash_map.insert("username", self.username);

        hash_map
    }
}
