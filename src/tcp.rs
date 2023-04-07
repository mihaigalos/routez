use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use std::time::SystemTime;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::io::Read;
use std::io::Write;



pub fn route(from: &str, to: &str) -> std::io::Result<()> {

    let listener = TcpListener::bind(from).expect("Cannot bind from address");

    println!("Routing TCP {from} ⏩ {to}");

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
            Ok(_) => { println!("⚡ {timestamp} CONNECTED {from} -> {to}"); }
            Err(err) => { println!("Destination error: {err}"); }
        }
    }

    Ok(())
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

    let (stats_tx, _stats_rx) = mpsc::channel();
    let connections = vec![
        thread::spawn(move || read_loop(from_tx, to_rx, stats_tx).unwrap()),
        thread::spawn(move || write_loop(to_tx, from_rx).unwrap()),
    ];


    let (from_clone, to_clone) = (from.clone(), to.clone());
    std::panic::set_hook(Box::new( move |_| {
        let timestamp = get_timestamp();
        println!("💔 {timestamp} BROKEN_PIPE {from_clone} -> {to_clone}");
    }));
    for t in connections {
        t.join().unwrap();
        let timestamp = get_timestamp();
        println!("🔌 {timestamp} DISCONNECTED {from} -> {to}");
    }
}

pub fn write_loop(
    mut to_stream: TcpStream,
    mut from_stream: TcpStream,
) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let num_read = match to_stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        if let Err(e) = from_stream.write_all(&buffer) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                // Stop processing
                return Ok(());
            }

            return Err(e);
            // eprintln!("Error: {}", e.to_string());
            // std::process::exit(1);
        }
    }

    Ok(())
}

pub fn read_loop(
    mut from_stream: TcpStream,
    mut to_stream: TcpStream,
    stats_tx: Sender<usize>,
) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let num_read = match from_stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        if let Err(e) = to_stream.write_all(&buffer) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                // Stop processing
                return Ok(());
            }

            return Err(e);
            // eprintln!("Error: {}", e.to_string());
            // std::process::exit(1);
        }
    }

    Ok(())
}
