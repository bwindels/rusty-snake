use geom::{Segment, Point, Size, UCoordinate};

pub use self::termscreen::TermScreen;
pub use self::ansiterm::AnsiTerm;

mod termscreen;
mod ansiterm;
mod termsize;

pub trait Terminal {
  fn write(&mut self, pos: Point, string: &str);
  fn write_repeated(&mut self, pos: Point, string: &str, amount: UCoordinate);
  fn size(&self) -> Size;
  fn clear(&mut self);
  fn hide_cursor(&mut self);
  fn show_cursor(&mut self);
  fn flush(&mut self);
}

#[derive(Copy, Clone)]
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
