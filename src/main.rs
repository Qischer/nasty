use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let peer = stream.local_addr().unwrap();
    println!("New connection from {peer}");

    let mut buf: [u8; 4] = [0; 4];
    while stream.read(&mut buf).unwrap() > 0 {
        dbg!(buf);
    }

    drop(stream);
}

fn listen_and_serve(listener: TcpListener) {
    for stream in listener.incoming() {
        if let Ok(s) = stream {
            thread::spawn(move || handle_client(s));
        }
    }
}

fn main() {
    let listener = TcpListener::bind("localhost:8080");
    match listener {
        Ok(l) => listen_and_serve(l),
        Err(err) => println!("Something went wrong. {err}"),
    }
}
