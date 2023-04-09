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
            let elapsed = timer.start.elapsed().as_secs().to_string()+"s";
            let rate_per_second = total_bytes as f64 / timestamp;
            output_progress(
                total_bytes,
                elapsed,
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
        println!();
    }

    Ok(())
}

fn output_progress(bytes: usize, elapsed: String, rate: f64, from: &str, to: &str) {
    let stats = format!("{} {:>20}{:>14}{:>21} {} {:>21} {:>10} {:>11}","🚀", elapsed, "TRANSFERRING",from,"⏩",to, bytes.as_human_readable(""), rate.as_human_readable("/s"));
    //let stats = format!("🚀 {elapsed:>19}s TRANSFERRING {from} -> {to} {} {} ", bytes.as_human_readable(""), rate.as_human_readable("/s"));
    print!("{stats}");
    print!("{}","\u{8}".repeat(stats.len()));
    let _ = std::io::stdout().flush();
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
    /// Renders the u64 value into a time string
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);

        format!("{hours}:{minutes:02}:{seconds:02}")
    }
}
