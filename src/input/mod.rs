use std::fmt::{Formatter, Display, Error};
use std::time::Duration;

pub enum Key {
	Left,
	Right,
	Up,
	Down,
	Esc,
	Space,
	Other
}

impl Display for Key {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		match *self {
			Key::Left	=> fmt.write_str("Key::Left"),
			Key::Right	=> fmt.write_str("Key::Right"),
			Key::Up		=> fmt.write_str("Key::Up"),
			Key::Down	=> fmt.write_str("Key::Down"),
			Key::Esc	=> fmt.write_str("Key::Esc"),
			Key::Space	=> fmt.write_str("Key::Space"),
			Key::Other	=> fmt.write_str("Key::Other")
		}
	}
}

pub enum PollResult {
	KeyPressed(Key),
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