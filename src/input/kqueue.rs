extern crate libc;

use std::result::Result;
use std::ptr;
use std::time::Duration;
use std::io::{self, Read};
use std::mem;
use super::termios::{RestoreHandle, set_term_raw};
use super::PollResult;

pub struct KeyboardPoller {
  queue: libc::c_int,
  descriptor: libc::kevent,
  stdin_restore_handle: RestoreHandle
}

impl KeyboardPoller {
  pub fn new() -> Result<KeyboardPoller, &'static str> {
    let event_queue = unsafe { libc::kqueue() };
    if event_queue == -1 {
      return Err("could not create event queue, kqueue returned -1");
    }

    let stdin = libc::STDIN_FILENO;
    let stdin_restore_handle = set_term_raw(stdin);

    let poller = KeyboardPoller {
      descriptor: libc::kevent {
        ident:  stdin as usize,
        filter: libc::EVFILT_READ,
        flags:  libc::EV_ADD,
        fflags: 0,
        data:   0,
        udata:  ptr::null_mut(),
      },
      queue: event_queue,
      stdin_restore_handle: stdin_restore_handle
    };

    //and pass changes to kernel
    unsafe {
      libc::kevent(poller.queue, &poller.descriptor, 1, ptr::null_mut(), 0, ptr::null_mut());
    }

    return Ok(poller);
  }
}

impl super::KeyboardPoller for KeyboardPoller {

  fn poll(&mut self, timeout: Duration) -> PollResult {
    //convert timeout format to struct that kevent call uses
    let timeout_spec = libc::timespec {
      tv_sec: timeout.as_secs() as i64,
      tv_nsec: timeout.subsec_nanos() as i64
    };
    //wait until something becomes available on stdin, or timeout happens
    let change_count = unsafe {
      libc::kevent(self.queue, ptr::null(), 0, &mut self.descriptor, 1, &timeout_spec)
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
      Ok(_) => PollResult::KeyPressed(key),
      Err(_) => PollResult::Err("read error from stdin")
    }
  }

}

impl Drop for KeyboardPoller {

  fn drop(&mut self) {
    println!("cleaning up keyboard::kqueue::KeyboardPoller");
    unsafe {
      libc::close(self.queue);
    }
  }

}