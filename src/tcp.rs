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

    let from_tx = from_arc.try_clone().unwrap();
    let to_tx = to_arc.try_clone().unwrap();

    let (stats_tx, _) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();
    let connections = vec![
        thread::spawn(move || read_loop(from_tx, stats_tx, write_tx).unwrap()),
        thread::spawn(move || write_loop(to_tx, write_rx).unwrap()),
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

pub fn write_loop(mut to_stream: TcpStream, write_rx: Receiver<Vec<u8>>) -> std::io::Result<()> {
    loop {
        // TODO receive bytes
        let buffer = write_rx.recv().unwrap();

        if buffer.is_empty() {
            break;
        }

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

pub fn read_loop(
    mut from_stream: TcpStream,
    stats_tx: Sender<usize>,
    write_tx: Sender<Vec<u8>>,
) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let num_read = match from_stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        let _ = stats_tx.send(num_read);

        if write_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        }
    }

    let _ = stats_tx.send(0);
    let _ = write_tx.send(Vec::new());

    Ok(())
}
