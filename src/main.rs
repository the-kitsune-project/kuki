use std::net::*;
use std::io::prelude::*;

mod config;
mod base64;

fn main() {
	// println!("{}", base64::decode("bmV2ZXIgZ29ubmEgZ2l2ZSB5b3UgdXAKbmV2ZXIgZ29ubmEgbGV0IHlvdSBkb3duCm5ldmVyIGdvbm5hIHJ1biBhcm91bmQKYW5kIGRlc2VydCB5b3U="));
	println!("\n
████████████████████████	
█▄─█─▄█▄─██─▄█▄─█─▄█▄─▄█
██─▄▀███─██─███─▄▀███─██
▀▄▄▀▄▄▀▀▄▄▄▄▀▀▄▄▀▄▄▀▄▄▄▀\n");
	
    let config = config::get();
// 	// log::config(&config);
	// let pool = ThreadPool::new(4);

	let listener = TcpListener::bind(format!("{}:{}", config.address, config.port)).unwrap();
	println!("Started listening on:\n{}:{}\n", config.address, config.port);
	
	for stream in listener.incoming() {
		let stream = stream.unwrap();

		handle_connection(stream);
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buf = [0; 2048];
	while 0 != stream.read(&mut buf).unwrap() {
		match std::str::from_utf8(&buf) {
	        Ok(request) => println!("{request}"),
	        Err(e) => panic!("Request does not contain valid UTF-8"),
	    }
		
	}
}