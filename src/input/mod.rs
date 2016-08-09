use std::time::Duration;

#[derive(Copy, Clone, Debug)]
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
  KeyPressed(Key),
  Timeout,
  Err(&'static str)
}

pub trait Keyboard : Drop {
  fn poll(&mut self, timeout: Duration) -> PollResult;
}

mod termios;
#[cfg (target_os="macos")]
mod kqueue;
#[cfg (target_os="macos")]
pub fn create_keyboard() -> Result<kqueue::Keyboard, &'static str> {
  kqueue::Keyboard::new()
}
