pub const BUFFER_SIZE: usize = 4096;
pub const CONNECTION_TIMEOUT_MS: u64 = 500; // TODO: set this from the commandline to hold i.e. SSH open indefinetely (also when there is no traffic)
pub const STATS_TIMER_RESOLUTION_MS: u64 = 100;

pub const KILO: (f64, &str) = (1024., "KB");
pub const MEGA: (f64, &str) = (1024. * KILO.0, "MB");
pub const GIGA: (f64, &str) = (1024. * MEGA.0, "GB");
pub const TERRA: (f64, &str)= (1024. * GIGA.0, "TB");
pub const PETA: (f64, &str) = (1024. * TERRA.0,"PB");
pub const EXA: (f64, &str)  = (1024. * PETA.0,"EB");