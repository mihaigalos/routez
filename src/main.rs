use std::env;

use routez::core::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    if is_correct_args(&args) {
        return println!("Example usage: cargo run 127.0.0.1:1234 127.0.0.1:4321");
    }

    run(args);
}

fn is_correct_args(args: &Vec<String>) -> bool {
    //TODO: Implement config file
    args.len() != 3
}
