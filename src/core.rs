use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use std::time::SystemTime;

pub fn run(args: Vec<String>) {
    route_one_connection(&args[1], &args[2]);
}

fn route_one_connection(from: &str, to: &str) {
    let listener = TcpListener::bind(from).expect("Cannot bind from address");

    println!("Routing {from} -> {to}");

    for incoming_stream in listener.incoming() {
        let from_stream = if let Ok(val) = incoming_stream {
            val
        } else {
            continue
        };

        let (from_clone, to_clone) = (String::from(from), String::from(to));
        let connection_thread = TcpStream::connect(to)
            .map(|to_stream| thread::spawn(move || connection_handler(from_clone, to_clone, from_stream, to_stream)));

        let timestamp = get_timestamp();
        match connection_thread {
            Ok(_) => { println!("{timestamp} CONNECTED {from} -> {to}"); }
            Err(err) => { println!("Destination error: {err}"); }
        }
    }
}

fn get_timestamp() -> String {
    let now = SystemTime::now();
    let now_str = format!("{:?}",now);
    let now_str_digits_spaces: String = now_str.chars().filter(|c| c.is_ascii_digit() || *c == ',').collect();
    let now_splitted: Vec<&str> = now_str_digits_spaces.split(',').collect();
    let tv_sec:usize =  now_splitted[0].parse().unwrap();
    let tv_nsec:usize = now_splitted[1].parse().unwrap();

    tv_sec.to_string() + "." + &tv_nsec.to_string()
}

fn connection_handler(from: String, to: String, from_stream: TcpStream, to_stream: TcpStream) {
    let from_arc = Arc::new(from_stream);
    let to_arc = Arc::new(to_stream);

    let (mut from_tx, mut from_rx) = (from_arc.try_clone().unwrap(), from_arc.try_clone().unwrap());
    let (mut to_tx, mut to_rx) = (to_arc.try_clone().unwrap(), to_arc.try_clone().unwrap());

    let connections = vec![
        thread::spawn(move || io::copy(&mut from_tx, &mut to_rx).unwrap()),
        thread::spawn(move || io::copy(&mut to_tx, &mut from_rx).unwrap()),
    ];

    for t in connections {
        t.join().unwrap();
        let timestamp = get_timestamp();
        println!("{timestamp} CLOSED {from} -> {to}");
    }
}
