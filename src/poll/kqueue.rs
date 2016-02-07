extern crate libc;

use std::result::Result;
use libc;

struct KQueueKeyboardPoller {
  queue: libc::c_int,
  descriptor: libc::kevent
}

impl KQueueKeyboardPoller {
  fn new() -> Result<KeyboardPoller, &'static str> {
    let event_queue = unsafe {libc::kqueue()};
    if(event_queue == -1) {
      Err("could not create event queue, kqueue returned -1");
    }

    let poller = KQueueKeyboardPoller {
      descriptor: libc::kevent {
        ident:  libc::STDIN_FILENO,
        filter: libc::EVFILT_READ,
        flags:  libc::EV_ADD,
        fflags: 0,
        data:   0,
        udata:  NULL,
      },
      queue: event_queue,
    };

    //and pass changes to kernel
    unsafe {
      libc::kevent(poller.queue, &poller.descriptor, 1, NULL, 0, NULL);
    }

    Ok(poller);
  }
}

impl KeyboardPoller for KQueueKeyboardPoller {

  fn poll(&self, timeout: Duration) -> u32 {
    let timeout_spec = libc::timespec {
      tv_sec: timeout.as_secs();
      tv_nsec: timeout.subsec_nanos();
    }
    unsafe {
      libc::kevent(self.queue, NULL, 0, self.descriptor, 1, &timeout_spec);
    }
  }

  fn Drop(&self) {
    unsafe {
      libc::close(self.queue);
    }
  }

}