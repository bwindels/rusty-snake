extern crate libc;

use std::time::Duration;

pub struct Clock {
	conv_info: libc::mach_timebase_info_data_t
}

pub struct Instant {
	abs_time: u64,
}

const NANO_SEC_IN_ONE_SEC : f64 = 1000000000f64;

impl Clock {

	pub fn new() -> Clock {
		let mut conv_info = libc::mach_timebase_info_data_t {numer: 0, denom: 0};
		unsafe {
			libc::mach_timebase_info(&mut conv_info);
		}
		let clock = Clock {conv_info: conv_info};
		clock
	}

	pub fn now(&self) -> Instant {
		let abs_time = unsafe {
			libc::mach_absolute_time()
		};
		Instant {abs_time: abs_time}
	}

	pub fn duration(&self, first: Instant, last: Instant) -> Duration {
		let abs_diff = last.abs_time - first.abs_time;
		let nano_diff = abs_diff as f64 * (self.conv_info.numer as f64 / self.conv_info.denom as f64);
		let sec_diff = (nano_diff / NANO_SEC_IN_ONE_SEC).floor();
		let subsec_nano_diff = nano_diff - (sec_diff * NANO_SEC_IN_ONE_SEC);
		Duration::new(sec_diff as u64, subsec_nano_diff as u32)
	}
}
