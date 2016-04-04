pub mod ansiterm;
mod termsize;

use geom::{Point, Size};

pub trait Terminal {
	fn write(&mut self, pos: Point, string: &str);
	fn write_repeated(&mut self, pos: Point, string: &str, amount: u32);
	fn size(&self) -> Size;
	fn clear(&mut self);
	fn hide_cursor(&mut self);
	fn flush(&mut self);
}
