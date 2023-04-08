pub const STATS_TIMER_RESOLUTION_MS: u64 = 1000;

pub const KILO: (f64, &str) = (1024., "KB");
pub const MEGA: (f64, &str) = (1024. * KILO.0, "MB");
pub const GIGA: (f64, &str) = (1024. * MEGA.0, "GB");
