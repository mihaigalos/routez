use std::net::UdpSocket;
use std::sync::Arc;
use std::thread;

use crate::constants::*;
use crate::output::print_start;

pub fn route(from: &str, to: &str) -> std::io::Result<()> {
    print_start(from, to, "UDP");
    let socket = UdpSocket::bind(from)?;

    loop {
        let mut buf = [0; BUFFER_SIZE];
        let socket = Arc::new(socket.try_clone().expect("Failed to clone socket"));
        let received_count = socket.recv(&mut buf)?;

        let to = to.to_string();
        thread::spawn(move || socket.send_to(&buf[..received_count], to));
    }
}
