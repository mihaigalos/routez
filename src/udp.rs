use std::net::UdpSocket;

pub fn route(from: &str, to: &str) -> std::io::Result<()> {
    println!("Routing UDP {from} ⏩ {to}");
    let socket = UdpSocket::bind(from)?;

    let mut buf = [0; 1*1024*1024];

    loop {
        let received_count = socket.recv(&mut buf)?;
        socket.send_to(&buf[..received_count], to)?;
    }
}
