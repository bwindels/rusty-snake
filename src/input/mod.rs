
use std::time::Duration;
#[allow(dead_code)]
pub enum Key {
	Left,
	Right,
	Up,
	Down,
	Esc,
	Space,
	Other
}

pub enum PollResult {
	KeyPressed(u32),
	Timeout,
	Err(&'static str)
}

pub trait KeyboardPoller : Drop {
	fn poll(&mut self, timeout: Duration) -> PollResult;
}

mod termios;
#[cfg (target_os="macos")]
mod kqueue;
#[cfg (target_os="macos")]
pub fn create_keyboard_poller() -> Result<Box<KeyboardPoller>, &'static str> {
	let poller = kqueue::KeyboardPoller::new();
	match poller {
		Ok(p) => Ok(Box::new(p) as Box<KeyboardPoller>),
		Err(err) => Err(err)
	}
}