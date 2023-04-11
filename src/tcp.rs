use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::io::Read;
use std::io::Write;

use crate::constants::*;
use crate::stats::stats_loop;
use crate::output::*;

pub fn route(from: &str, to: &str) -> std::io::Result<()> {

    let listener = TcpListener::bind(from).expect("Cannot bind from address");

    print_start(from, to, "TCP");
    for incoming_stream in listener.incoming() {
        let from_stream = if let Ok(val) = incoming_stream {
            val
        } else {
            continue
        };

        let (from_clone, to_clone) = (String::from(from), String::from(to));
        let connection_thread = TcpStream::connect(to)
            .map(|to_stream| thread::spawn(move || connection_handler(from_clone, to_clone, from_stream, to_stream)));

        match connection_thread {
            Ok(_) => print_connected(from, to),
            Err(err) => { println!("Destination error: {err}"); }
        }
    }

    Ok(())
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
    std::panic::set_hook(Box::new( move |_| { print_broken_pipe(&from_clone, &to_clone) }));

    for t in connections {
        t.join().unwrap();
    }

    print_disconnected(&from, &to);
}

pub fn thread_loop(
    mut input: TcpStream,
    mut output: TcpStream,
    stats_input: Sender<usize>,
) -> std::io::Result<()> {
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let num_read = match input.read(&mut buffer) {
            Ok(0) => {
                output.shutdown(Shutdown::Both).expect("Cannot shutdown output stream");
                break
            },
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
