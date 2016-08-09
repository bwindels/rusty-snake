extern crate libc;

use std::result::Result;
use std::ptr;
use std::time::Duration;
use std::io::{self, Read};
use std::mem;
use super::termios::{RestoreHandle, set_term_raw};
use super::PollResult;
use super::Key;

pub struct Keyboard {
  queue_fd: libc::c_int,
  descriptor: libc::kevent,
  stdin_restore_handle: RestoreHandle
}

fn map_code_to_key(code: u32) -> Key {
  match code {
    0x00445B1B => Key::Left,
    0x00435B1B => Key::Right,
    0x00415B1B => Key::Up,
    0x00425B1B => Key::Down,
    0x00000020 => Key::Space,
    0x0000001B => Key::Esc,
    _          => Key::Other
  }
}

impl Keyboard {
  pub fn new() -> Result<Keyboard, &'static str> {
    let event_queue_fd = unsafe { libc::kqueue() };
    if event_queue_fd == -1 {
      return Err("could not create event queue, kqueue returned -1");
    }

    let stdin = libc::STDIN_FILENO;
    let stdin_restore_handle = set_term_raw(stdin);

    let keyboard = Keyboard {
      descriptor: libc::kevent {
        ident:  stdin as usize,
        filter: libc::EVFILT_READ,
        flags:  libc::EV_ADD,
        fflags: 0,
        data:   0,
        udata:  ptr::null_mut(),
      },
      queue_fd: event_queue_fd,
      stdin_restore_handle: stdin_restore_handle
    };

    //and pass changes to kernel
    unsafe {
      libc::kevent(keyboard.queue_fd, &keyboard.descriptor, 1, ptr::null_mut(), 0, ptr::null_mut());
    }

    return Ok(keyboard);
  }
}

impl super::Keyboard for Keyboard {

  fn poll(&mut self, timeout: Option<Duration>) -> PollResult {
    //convert timeout format to struct that kevent call uses
    let timeout_spec = timeout.map(|d| libc::timespec {
      tv_sec: d.as_secs() as i64,
      tv_nsec: d.subsec_nanos() as i64
    });
    let timeout_spec_pointer = timeout_spec
      .map(|ts| &ts as * const libc::timespec)
      .unwrap_or(ptr::null() as * const libc::timespec);

    //wait until something becomes available on stdin, or timeout happens
    let change_count = unsafe {
      libc::kevent(self.queue_fd, ptr::null(), 0, &mut self.descriptor, 1, timeout_spec_pointer)
    };
    //the file descriptor didn't change, the kevent call timed out
    if change_count == 0 {
      return PollResult::Timeout;
    }

    //read up to 4 bytes from stdin
    let mut key = 0u32;
    let read_result = {
      let buffer: &mut [u8; 4] = unsafe { mem::transmute(&mut key) };
      io::stdin().read(buffer)
    };

    match read_result {
      Ok(_) => PollResult::KeyPressed(map_code_to_key(key)),
      Err(_) => PollResult::Err("read error from stdin")
    }
  }

}

impl Drop for Keyboard {

  fn drop(&mut self) {
    unsafe {
      libc::close(self.queue_fd);
    }
  }

}
