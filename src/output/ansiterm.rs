extern crate libc;

use super::termsize::get_term_size;
use std::io::{Result};
use std::fmt::Write;
pub struct Point {
	x: u16,
	y: u16
}

fn init_buffer_with_pos(buffer: &mut String, pos: Point) {
	buffer.clear();
	buffer.push_str("\033[");
	write!(&mut buffer, "{}", pos.y).unwrap();
	buffer.push_str(";");
	write!(&mut buffer, "{}", pos.x).unwrap();
	buffer.push_str("H");
}

pub struct AnsiTerm {
	fd: libc::c_int,
	size: Point,
	compose_buffer: String
}

impl AnsiTerm {

	fn new(fd: libc::c_int) -> Result<AnsiTerm> {
		let size = get_term_size(fd);

		match size {
			Ok(p) => {
				let max_line_len = p.x + 10;	//screen width + 10 for ansi position prefix
				AnsiTerm {
					fd: fd,
					size: p,
					compose_buffer: String::with_capacity(max_line_len)
				}
			},
			Err(e) => Err(e)
		}
	}

	fn from_stdout() -> Result<AnsiTerm> {
		AnsiTerm::new(libc::STDOUT_FILENO);
	}

	fn flush(&mut self) {
		unsafe {
			libc::fflush(self.fd);
		}
	}

	fn hide_cursor(&mut self) {
		self.write_bytes("\033[?25l");
	}

	fn clear(&mut self) {
		self.write_bytes("\033[2J");
	} 

	fn write(&mut self, pos: Point, chr: &str) {
		init_buffer_with_pos(&mut self.compose_buffer, pos);
	    self.compose_buffer.push_str(chr);
		self.write_bytes(self.compose_buffer.as_str());
	}

	fn write_repeated(&mut self, pos: Point, chr: &str, amount: u32) {
		init_buffer_with_pos(&mut self.compose_buffer, pos);
		for i in 0..amount {
		    self.compose_buffer.push_str(chr);
		}
		self.write_bytes(self.compose_buffer.as_str());
	}

	fn rows(&self) -> u16 {
		self.size.y
	}

	fn columns(&self) -> u16 {
		self.size.x
	}
	
	fn write_bytes(&mut self, buffer: &str) {
		unsafe {
			libc::write(self.fd, buffer.as_ptr(), buffer.len());
		}
	}
}