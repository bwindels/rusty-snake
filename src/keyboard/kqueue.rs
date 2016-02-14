extern crate libc;

use std::result::Result;
use std::ptr;
use std::time::Duration;
use std::io;
use std::mem;
use keyboard;

struct KeyboardPoller {
  queue: libc::c_int,
  descriptor: libc::kevent
}

impl KeyboardPoller {
  fn new() -> Result<KeyboardPoller, &'static str> {
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
    let timeout_spec = libc::timespec {
      tv_sec: timeout.as_secs() as i64,
      tv_nsec: timeout.subsec_nanos() as i64
    };
    unsafe {
      libc::kevent(self.queue, ptr::null(), 0, &mut self.descriptor, 1, &timeout_spec);
    }
    let mut key = 0u32;
    {
      let buffer: &[u8; 4] = unsafe { mem::transmute(&key) };
      io::stdin().read(&buffer);
    }
    key
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