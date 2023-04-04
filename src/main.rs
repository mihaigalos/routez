use std::env;
use std::thread;

use routez::config::read_config;
use routez::core::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let lines = read_config(&args[1]);
        let mut handles: Vec<_> = Vec::new();
        for line in lines {
            if let Some((from, to)) = line.expect("Invalid config").split_once(' '){
                let from_clone : String = from.to_string();
                let to_clone : String = to.to_string();
                handles.push(thread::spawn(move || route_one_connection(&from_clone, &to_clone)));
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }
    } else if args.len() == 3 {
        route_one_connection(&args[1], &args[2]);
    }
    else {
        return println!("Example usage: cargo run 127.0.0.1:1234 127.0.0.1:4321");
    }
}
