pub mod ansiterm;
mod termsize;

pub use termscreen;

use geom::{Point, Size};

pub trait Terminal {
  fn write(&mut self, pos: Point, string: &str);
  fn write_repeated(&mut self, pos: Point, string: &str, amount: u32);
  fn size(&self) -> Size;
  fn clear(&mut self);
  fn hide_cursor(&mut self);
  fn flush(&mut self);
}

pub enum Symbol {
  Apple,
  SnakeBody,
  Wall,
  Clear
}

pub trait Screen : Drop {
  fn draw_point(&mut self, pos: Point, symbol: Symbol);
  fn draw_segment(&mut self, s: Segment, symbol: Symbol);
  fn draw_text(&mut self, pos: Point, text: &str);
  fn clear(&mut self);
  fn size(&self) -> Size;
}
