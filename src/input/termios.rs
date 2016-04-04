extern crate libc;

pub struct RestoreHandle {
	original_attr: libc::termios,
	fd: libc::c_int
}

impl Drop for RestoreHandle {
	fn drop(&mut self) {
		unsafe {
    		libc::tcsetattr(self.fd, 0, &self.original_attr);
    	}
	}
}

pub fn set_term_raw(fd: libc::c_int) -> RestoreHandle {

	let mut original_attr = libc::termios {
		c_iflag:	0,
		c_oflag:	0,
		c_cflag:	0,
		c_lflag:	0,
		c_cc:		[0; libc::NCCS],
		c_ispeed:	0,
		c_ospeed:	0
	};

	unsafe {
		libc::tcgetattr(fd, &mut original_attr);
	}

	let mut raw_attr = libc::termios {..original_attr};
	raw_attr.c_iflag &= !(libc::IGNBRK | libc::BRKINT | libc::PARMRK | libc::ISTRIP
                | libc::INLCR | libc::IGNCR | libc::ICRNL | libc::IXON);
	raw_attr.c_oflag &= !libc::OPOST;
	raw_attr.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::ISIG | libc::IEXTEN);
	raw_attr.c_cflag &= !(libc::CSIZE | libc::PARENB);
	raw_attr.c_cflag |= libc::CS8;

	unsafe {
		libc::tcsetattr(fd, 0, &raw_attr);
	}

	RestoreHandle {
		original_attr: original_attr,
		fd: fd
	}
}
