use isahc::prelude::*;
use url::Url;
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::{from_str};

pub enum Orientation {
	LANDSCAPE,
	PORTRAIT,
	SQUARISH,
}

pub struct Unsplash {
	api_key: String,
	url: Url
}

impl Unsplash {
	pub fn new(api_key: &str) -> Unsplash {
		Unsplash {
			api_key: String::from(api_key),
			url: Url::parse("https://api.unsplash.com").unwrap()
		}
	}

	pub fn get_photo(&mut self, photo_id: &str) -> Result<Photo, isahc::Error> {
		self.url.set_path(&format!("/photos/{}", photo_id));

		let mut response = isahc::get(self.url.as_str())?;

		let photo : Photo = from_str(&response.text()?)
			.expect("Error while decoding JSON");

		return Ok(photo);
	}

	pub fn get_random_photo(&mut self, options: HashMap<&str, &str>) -> Result<Photo, isahc::Error> {
		for key in options.keys() {
			self
				.url
				.query_pairs_mut()
				.append_pair(key, key);
		}

		let response_text = self.get("/photos/random")?;
		let photo : Photo = from_str(&response_text)
			.expect("Error while decoding json");


		return Ok(photo);
	}

	pub fn get_phot_of_the_day(&mut self) -> Result<Photo, isahc::Error> {
		let url = Url::parse("https://lambda.splash-cli.app")
			.unwrap();

		let mut response = isahc::get(url.as_str())?;
		let text = response.text()?;


		// Parse JSON
		let data: PhotoOfTheDay = from_str(&text)
			.expect("Error while parsing JSON");

		let photo = self.get_photo(&data.id)?;

		return Ok(photo);
	}


	// Private API
	fn get(&mut self, path: &str) -> Result<String, isahc::Error> {
		self.url.set_path(path);

		let req = isahc::Request::builder()
			.method("GET")
			.header("Authorization", &format!("Client-ID {}", self.api_key))
			.uri(self.url.as_str())
			.body(())?;

		let mut response = isahc::send(req)?;
		let text = response.text()?;

		self.reset_query();

		return Ok(text);
	}

	fn reset_query(&mut self) {
		self.url.set_query(Option::from(""));
	}
}


#[derive(Debug, Deserialize)]
pub struct Photo {
	pub id: String,
	pub color: String,
	pub width: i32,
	pub height: i32,
	pub user: User,
	pub urls: PhotoURLS
}

#[derive(Debug, Deserialize)]
pub struct User {
	pub id: String,
	pub username: String
}

#[derive(Debug, Deserialize)]
pub struct PhotoURLS {
	pub raw: String,
	pub full: String,
	pub regular: String,
	pub thumb: String,
	pub download_location: String,
	pub download: String
}

#[derive(Deserialize, Debug)]
pub struct PhotoOfTheDay {
	pub id: String
}
