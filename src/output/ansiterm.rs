extern crate libc;

use super::termsize::get_term_size;
use std::io::{Result};
use std::fmt::Write;
use geom::{Point, Size};

fn init_buffer_with_pos(buffer: &mut String, pos: Point) {
  buffer.clear();
  buffer.push_str("\x1B[");
  write!(buffer, "{}", (pos.y + 1)).unwrap();
  buffer.push_str(";");
  write!(buffer, "{}", (pos.x + 1)).unwrap();
  buffer.push_str("H");
}

pub struct AnsiTerm {
  fd: libc::c_int,
  size: Size,
  compose_buffer: String
}

impl AnsiTerm {

  pub fn new(fd: libc::c_int) -> Result<AnsiTerm> {
    let size = get_term_size(fd);
    let term = size.map(|s| {
      let max_line_len = s.width + 10;  //screen width + 10 for ansi position prefix
      AnsiTerm {
        fd: fd,
        size: s,
        compose_buffer: String::with_capacity(max_line_len as usize)
      }
    });
    term
  }

  pub fn from_stdout() -> Result<AnsiTerm> {
    AnsiTerm::new(libc::STDOUT_FILENO)
  }

  fn write_bytes(&self, buffer: &str) {
    unsafe {
      libc::write(self.fd, buffer.as_ptr() as *const libc::c_void, buffer.len());
    }
  }
}

impl super::Terminal for AnsiTerm {

  fn flush(&mut self) {
    unsafe {
      libc::fsync(self.fd);
    }
  }

  fn hide_cursor(&mut self) {
    self.write_bytes("\x1B[?25l");
  }

  fn clear(&mut self) {
    self.write_bytes("\x1B[2J");
  } 

  fn write(&mut self, pos: Point, string: &str) {
    init_buffer_with_pos(&mut self.compose_buffer, pos);
      self.compose_buffer.push_str(string);
    self.write_bytes(self.compose_buffer.as_str());
  }

  fn write_repeated(&mut self, pos: Point, string: &str, amount: u32) {
    init_buffer_with_pos(&mut self.compose_buffer, pos);
    for _ in 0..amount {
        self.compose_buffer.push_str(string);
    }
    self.write_bytes(self.compose_buffer.as_str());
  }

  fn size(&self) -> Size {
    self.size
  }
  
}
