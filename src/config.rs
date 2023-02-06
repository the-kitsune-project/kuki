use std::env;

pub struct Config {
	pub address: String,
	pub port: String
}

pub fn get() -> Config {
	Config {
		address: match env::var("ADDRESS") {
			Ok(val) => {val},
			Err(_e) => {String::from("0.0.0.0")}
		},
		port: match env::var("PORT") {
			Ok(val) => {val},
			Err(_e) => {String::from("8000")}
		}
	}
}