use geom::{Point, Direction, Segment, Size};
use super::Symbol as Symbol;
use super::Terminal as Terminal;
use super::Screen as Screen;

fn map_symbol_to_char(s: Symbol) -> &'static str {
  match s {
    Symbol::Apple => "@",
    Symbol::SnakeBody => "*",
    Symbol::Wall => "#",
    Symbol::Clear => " "
  }
}

pub struct TermScreen<T> {
  term: T
}

impl<T: Terminal> TermScreen<T> {

  pub fn new(term: T) {
    term.hide_cursor();
    TermScreen {term: term}
  }
  
}

impl<T: Terminal> Drop for TermScreen<T> {

  fn drop(&mut self) {
    self.term.show_cursor();
  }

}

impl<T: Terminal> Screen for TermScreen<T> {

  fn draw_segment(&mut self, s: Segment, symbol: Symbol) {
    let chr = map_symbol_to_char(symbol);

    match s.direction {
      Direction::East => self.term.write_repeated(s.tail(), chr, s.length()),
      Direction::West => self.term.write_repeated(s.head(), chr, s.length()),
      Direction::South | Direction::North => 
        for p in s {
          self.term.write(p, chr);
        }
    };
  }

  fn draw_point(&mut self, pos: Point, symbol: Symbol) {
    let chr = map_symbol_to_char(symbol);
    self.draw_text(pos, chr);
  }

  fn draw_text(&mut self, pos: Point, text: &str) {
    self.term.write(pos, text);
  }

  fn clear(&mut self) {
    self.term.clear();
  }

  fn size(&self) -> Size {
    self.term.size()
  }

}
