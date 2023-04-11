use std::sync::mpsc::Receiver;

use crate::timer::Timer;
use crate::output::*;

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
        eprintln!();
    }

    Ok(())
}

