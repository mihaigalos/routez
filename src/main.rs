use std::env;
use std::thread;

use routez::config::read_config;
use routez::tcp;
use routez::udp;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let lines = read_config(&args[1]);
        let mut handles: Vec<_> = Vec::new();
        for line in lines {
            if let Some((from, to)) = line.expect("Invalid config").split_once(' '){
                let from_clone : String = from.to_string();
                let to_clone : String = to.to_string();
                handles.push(thread::spawn(move || tcp::route(&from_clone, &to_clone)));
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }
    } else if args.len() == 3 {
        udp::route(&args[1], &args[2]).unwrap();
    }
    else {
        return println!("Example usage: {} 127.0.0.1:1234 127.0.0.1:4321", env!("CARGO_PKG_NAME"));
    }
}
