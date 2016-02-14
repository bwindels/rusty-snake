
use std::time::Duration;

trait KeyboardPoller : Drop {
	fn poll(&mut self, timeout: Duration) -> Result<u32,&'static str>;
}

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