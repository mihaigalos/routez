use std::io::Write;
use std::time::SystemTime;

use crate::constants::*;

pub fn print_start(from: &str, to: &str) {
    println!("Routing TCP {from} ⏩ {to}");
}

pub fn print_connected(from: &str, to: &str) {
    let timestamp = get_timestamp();
    println!("{} {:>20}{:>14}{:>21} {} {:>21} {:>10} {:>11}","⚡", timestamp, "CONNECTED",from,"⏩",to,"-","-");
}


pub fn print_broken_pipe(from: &str, to: &str) {
    let timestamp = get_timestamp();
    println!("{} {:>20}{:>14}{:>21} {} {:>21} {:>10} {:>11}","💔", timestamp, "BROKEN_PIPE",from,"⏩",to,"-","-");
}

pub fn print_disconnected(from: &str, to: &str) {
    let timestamp = get_timestamp();
    println!("{} {:>20}{:>14}{:>21} {} {:>21} {:>10} {:>11}","🔌", timestamp, "DISCONNECTED",from,"⏩",to,"-","-");
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

pub fn output_progress(bytes: usize, elapsed: String, rate: f64, from: &str, to: &str) {
    let stats = format!("{} {:>20}{:>14}{:>21} {} {:>21} {:>10} {:>11}","🚀", elapsed, "TRANSFERRING",from,"⏩",to, bytes.as_human_readable(""), rate.as_human_readable("/s"));
    eprint!("{stats}");
    eprint!("{}","\u{8}".repeat(stats.len()));
    let _ = std::io::stderr().flush();
}


pub trait BytesOutput {
    fn as_human_readable(&self, suffix: &str) -> String;
}

impl BytesOutput for f64 {
    fn as_human_readable(&self, suffix: &str) -> String {
        let (unit, description) = if *self > EXA.0 {
            EXA
        } else if *self >= PETA.0 && *self < EXA.0 {
            PETA
        } else if *self >= TERRA.0 && *self < PETA.0 {
            TERRA
        } else if *self >= GIGA.0 && *self < TERRA.0 {
            GIGA
        } else if *self >= MEGA.0 && *self < GIGA.0 {
            MEGA
        } else if *self >= KILO.0 && *self < MEGA.0 {
            KILO
        } else {
            BYTE
        };

        let result = *self / unit;

        format!("{result:.3}{description}{suffix}")
    }
}

impl BytesOutput for usize {
    fn as_human_readable(&self, suffix: &str) -> String {
        (*self as f64).as_human_readable(suffix)
    }
}

pub trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);

        format!("{hours}:{minutes:02}:{seconds:02}")
    }
}
