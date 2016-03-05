extern crate libc;

use std::result::Result;
use std::ptr;
use std::time::Duration;
use std::io::{self, Read};
use std::mem;
use keyboard;

pub struct KeyboardPoller {
  queue: libc::c_int,
  descriptor: libc::kevent
}

impl KeyboardPoller {
  pub fn new() -> Result<KeyboardPoller, &'static str> {
    let event_queue = unsafe { libc::kqueue() };
    if event_queue == -1 {
      return Err("could not create event queue, kqueue returned -1");
    }

    let poller = KeyboardPoller {
      descriptor: libc::kevent {
        ident:  libc::STDIN_FILENO as usize,
        filter: libc::EVFILT_READ,
        flags:  libc::EV_ADD,
        fflags: 0,
        data:   0,
        udata:  ptr::null_mut(),
      },
      queue: event_queue,
    };

    //and pass changes to kernel
    unsafe {
      libc::kevent(poller.queue, &poller.descriptor, 1, ptr::null_mut(), 0, ptr::null_mut());
    }

    return Ok(poller);
  }
}

impl keyboard::KeyboardPoller for KeyboardPoller {

  fn poll(&mut self, timeout: Duration) -> Result<u32,&'static str> {
    //convert timeout format to struct that kevent call uses
    let timeout_spec = libc::timespec {
      tv_sec: timeout.as_secs() as i64,
      tv_nsec: timeout.subsec_nanos() as i64
    };
    //wait until something becomes available on stdin, or timeout happens
    unsafe {
      libc::kevent(self.queue, ptr::null(), 0, &mut self.descriptor, 1, &timeout_spec);
    }
    //read up to 4 bytes from stdin
    let mut key = 0u32;
    let read_result = {
      let buffer: &mut [u8; 4] = unsafe { mem::transmute(&mut key) };
      io::stdin().read(buffer)
    };

    match read_result {
      Ok(_) => Ok(key),
      Err(_) => Err("read error from stdin")
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