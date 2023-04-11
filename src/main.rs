use std::env;
use std::thread;

use routez::config::read_config;
use routez::tcp;
use routez::udp;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut handles: Vec<_> = Vec::new();

    if args.len() == 2 {
        let lines = read_config(&args[1]);
        for line in lines {
            let myline = line.expect("Invalid config");
            let split = myline.splitn(3, ' ').collect::<Vec<_>>();
            let (from, to, protocol) = (
                split[0].to_string(),
                split[1].to_string(),
                split[2].to_string(),
            );

            match protocol.as_str() {
                "UDP" => handles.push(thread::spawn(move || udp::route(&from, &to))),
                "TCP" => handles.push(thread::spawn(move || tcp::route(&from, &to))),
                _ => panic!("Please provide a 3rd parameter: {{tcp, udp}}"),
            }
        }
    } else if args.len() == 4 {
        let (from, to, protocol) = (args[1].clone(), args[2].clone(), args[3].to_uppercase());
        match protocol.as_str() {
            "UDP" => handles.push(thread::spawn(move || udp::route(&from, &to))),
            "TCP" => handles.push(thread::spawn(move || tcp::route(&from, &to))),
            _ => panic!("Please provide a 3rd parameter: {{tcp, udp}}"),
        }
    } else {
        panic!(
            "{}",
            format!(
                "Example usage: {} 127.0.0.1:1234 127.0.0.1:4321 [TCP or UDP]",
                env!("CARGO_PKG_NAME")
            )
        );
    }

    for handle in handles {
        let _ = handle.join().unwrap();
    }

    Ok(())
}
