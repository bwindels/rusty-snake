extern crate libc;

use std::io::{Result, Error, ErrorKind};

struct Point {
	x: u16,
	y: u16
}

#[repr(C)]
struct winsize {
	ws_row: 	libc::c_ushort, /* rows, in characters */
	ws_col: 	libc::c_ushort, /* columns, in characters */
	ws_xpixel: 	libc::c_ushort, /* horizontal size, pixels */
	ws_ypixel: 	libc::c_ushort /* vertical size, pixels */
}

const TIOCGWINSZ: libc::c_ulong = 0x40087468;

pub fn get_term_size(fd: libc::c_int) -> Result<Point> {
	let w = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
	let r = unsafe { libc::ioctl(fd, TIOCGWINSZ, &w) };

	match r {
		0 => Ok(Point{x: w.ws_col, y: w.ws_row}),
		errorCode => {
			let errorMessage = format!("ioctl returned error {:?}", errorCode);
			Err(Error::new(ErrorKind::Other, errorMessage))
		}
	}
}