use std::time::Duration;

pub struct Instant {
	abs_time: u64,
}

pub trait Timer {
	fn now(&self) -> Instant;
	fn diff(&self, first: Instant, last: Instant) -> Duration;
}

#[cfg (target_os="macos")]
mod machtimer;
#[cfg (target_os="macos")]
pub fn create_timer() -> Box<Timer> {
	Box::new(self::machtimer::MachTimer::new()) as Box<Timer>
}
