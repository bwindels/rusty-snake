use std::time::Duration;

trait KeyboardPoller : Drop {
	fn poll(timeout: Duration) -> u32
}

#[cfg target_os="macos"]
