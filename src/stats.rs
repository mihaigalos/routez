use std::sync::mpsc::Receiver;
use std::io::Write;

use crate::constants::*;
use crate::timer::Timer;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>, from: &str, to: &str) -> std::io::Result<()> {
    let mut total_bytes = 0;
    let mut timer = Timer::new(STATS_TIMER_RESOLUTION_MS);

    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();

        total_bytes += num_bytes;

        if !silent && timer.ready {
            timer.ready = false;
            let timestamp = timer.start.elapsed().as_secs_f64();
            let rate_per_second = total_bytes as f64 / timestamp;
            //println!("bytes: {total_bytes}, timestamp: {}, rate: {:.7}", timestamp, rate_per_second.as_human_readable());
            output_progress(
                total_bytes,
                timestamp.to_string(),
                rate_per_second,
                from,
                to
            );
        }

        if num_bytes == 0 {
            break;
        }
    }

    if !silent {
        eprintln!();
    }

    Ok(())
}

fn output_progress(bytes: usize, elapsed: String, rate: f64, from: &str, to: &str) {
    let stats = format!("🚀 Bytes: {bytes}, Elapsed: {elapsed}, Rate: {} {from} -> {to}", rate.as_human_readable());
    eprint!("{stats}");
    eprint!("{}","\u{8}".repeat(stats.len()));
    let _ = std::io::stderr().flush();
}

pub trait BytesOutput {
    fn as_human_readable(&self) -> String;
}

impl BytesOutput for f64 {
    fn as_human_readable(&self) -> String {
        let (unit, description) = if *self >= KILO.0 && *self < MEGA.0 {
            KILO
        } else if *self >= MEGA.0 && *self < GIGA.0 {
            MEGA
        } else {
            GIGA
        };
        let result = *self / unit;

        format!("{result:.3} {description}/s")
    }
}

pub trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    /// Renders the u64 value into a time string
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);

        format!("{hours}:{minutes:02}:{seconds:02}")
    }
}
