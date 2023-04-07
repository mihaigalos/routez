use std::sync::mpsc::Receiver;

use crate::timer::Timer;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>, from: &str, to: &str) -> std::io::Result<()> {
    let mut total_bytes = 0;
    let start = std::time::Instant::now();
    let mut timer = Timer::new();

    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();

        total_bytes += num_bytes;

        if !silent && timer.ready {
            timer.ready = false;
            output_progress(
                total_bytes,
                start.elapsed().as_secs().as_time(),
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
    let stats = format!("{from} -> {to} Bytes: {bytes}, Elapsed: {elapsed}, Rate: {rate} b/s");
    eprint!("{stats}");
    eprint!("{}","\u{8}".repeat(stats.len()));
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
