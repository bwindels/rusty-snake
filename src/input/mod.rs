use std::fmt::{Formatter, Display, Error};
use std::time::Duration;

#[derive(Copy, Clone)]
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
      Key::Left => fmt.write_str("Key::Left"),
      Key::Right  => fmt.write_str("Key::Right"),
      Key::Up   => fmt.write_str("Key::Up"),
      Key::Down => fmt.write_str("Key::Down"),
      Key::Esc  => fmt.write_str("Key::Esc"),
      Key::Space  => fmt.write_str("Key::Space"),
      Key::Other  => fmt.write_str("Key::Other")
    }
  }
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
