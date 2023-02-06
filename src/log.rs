use std::fs::File;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config;

pub fn config(config: &config::Config) {
	let mut file = File::create(format!("logs/config-{}.log", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())).unwrap();
	file.write_all("███████████████████████████████████████████████████████████▀█\n█▄─▀█▀─▄█▄─▄█▄─█─▄█─▄▄─███─▄▄▄─█─▄▄─█▄─▀█▄─▄█▄─▄▄─█▄─▄█─▄▄▄▄█\n██─█▄█─███─███─▄▀██─██─███─███▀█─██─██─█▄▀─███─▄████─██─██▄─█\n▀▄▄▄▀▄▄▄▀▄▄▄▀▄▄▀▄▄▀▄▄▄▄▀▀▀▄▄▄▄▄▀▄▄▄▄▀▄▄▄▀▀▄▄▀▄▄▄▀▀▀▄▄▄▀▄▄▄▄▄▀\n".as_bytes()).unwrap();
}