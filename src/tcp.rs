use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use std::time::SystemTime;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::io::Read;
use std::io::Write;

use crate::constants::*;
use crate::stats::stats_loop;

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
            Ok(_) => { println!("{} {:>20}{:>14}{:>21} {} {:>21} {:>10} {:>10}","⚡", timestamp, "CONNECTED",from,"⏩",to,"-","-"); }
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

     let (from_tx, from_rx) = (from_arc.try_clone().unwrap(), from_arc.try_clone().unwrap());
     let (to_tx, to_rx) = (to_arc.try_clone().unwrap(), to_arc.try_clone().unwrap());

    let (stats_input_blackhole, _) = mpsc::channel();
    let (stats_input, stats_output) = mpsc::channel();
    let (from_clone, to_clone) = (from.clone(), to.clone());
    let connections = vec![
        thread::spawn(move || thread_loop(from_tx, to_rx, stats_input_blackhole).unwrap()),
        thread::spawn(move || thread_loop(to_tx, from_rx, stats_input).unwrap()),
        thread::spawn(move || stats_loop(false, stats_output, &from_clone, &to_clone).unwrap()),
    ];



    let (from_clone, to_clone) = (from.clone(), to.clone());
    std::panic::set_hook(Box::new( move |_| {
        let timestamp = get_timestamp();
        println!("{} {:>20}{:>14}{:>21} {} {:>21} {:>10} {:>10}","💔", timestamp, "BROKEN_PIPE",from_clone,"⏩",to_clone,"-","-");
    }));

    for t in connections {
        t.join().unwrap();
    }

    let timestamp = get_timestamp();
    println!("{} {:>20}{:>14}{:>21} {} {:>21} {:>10} {:>10}","🔌", timestamp, "DISCONNECTED",from,"⏩",to,"-","-");
}

pub fn thread_loop(
    mut input: TcpStream,
    mut output: TcpStream,
    stats_input: Sender<usize>,
) -> std::io::Result<()> {
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let num_read = match input.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        let shadow = &buffer[..num_read];

        if let Err(e) = output.write_all(shadow) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return Ok(());
            }
            return Err(e);
        }
        let _ = stats_input.send(num_read);
    }
    let _ = stats_input.send(0);

    Ok(())
}
