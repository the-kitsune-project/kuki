use std::net::TcpListener;
use std::env;
mod miko;

fn main() {
    match env::var("PORT") {
        Ok(port) => {
            let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

            for stream in listener.incoming() {
                let stream = stream.unwrap();

                miko::handle_connection(stream);
            }
        },
        Err(_e) => panic!("Enviorment variable `Address` not set. Set enviorment variable `Address` to `0.0.0.0` or `127.0.0.1")
    }
}