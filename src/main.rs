use std::env;
use std::thread;

use routez::config::read_config;
use routez::tcp;
use routez::udp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut handles: Vec<_> = Vec::new();

    if args.len() == 2 {
        let lines = read_config(&args[1]);
        for line in lines {
            if let Some((from, to)) = line.expect("Invalid config").split_once(' '){
                let from_clone : String = from.to_string();
                let to_clone : String = to.to_string();
                handles.push(thread::spawn(move || tcp::route(&from_clone, &to_clone)));
            }
        }

    } else if args.len() == 4 {
        let (from, to, protocol) = (args[1].clone(), args[2].clone(), args[3].to_uppercase());
        match protocol.as_str() {
            "UDP" => handles.push(thread::spawn(move || udp::route(&from, &to).unwrap())),
            "TCP" => handles.push(thread::spawn(move || tcp::route(&from, &to))),
            _ => panic!("Please provide a 3rd parameter: {{tcp, udp}}")
        }
    }
    else {
        return println!("Example usage: {} 127.0.0.1:1234 127.0.0.1:4321 [TCP or UDP]", env!("CARGO_PKG_NAME"));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
