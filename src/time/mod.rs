use std::time::Duration;

#[derive(Copy, Clone)]
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
pub fn create_timer() -> machtimer::Timer {
  self::machtimer::Timer::new()
}
