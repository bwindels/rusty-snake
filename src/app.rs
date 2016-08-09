use std::time::Duration;
use std::iter;
use input::{Keyboard, PollResult, Key};
use output::Screen;
use time::Timer;
use geom::Point;

pub struct SnakeApp<A, B, C> {
  pub keyboard: A,
  pub screen: B,
  pub timer: C,
  pub interval: Duration,
}

impl<A: Keyboard, B: Screen, C: Timer> SnakeApp<A, B, C> {

  pub fn run(&mut self) {
    self.screen.clear();

    let size = self.screen.size();
    let message = format!("the screen has height {} and width {}", size.height, size.width);
    self.screen.draw_text(Point {x: 5, y: 5}, message.as_str());

    let mut should_exit = false;
    let mut counter = 0;
    while !should_exit {
      let key = self.sleep_and_poll_keyboard();
      counter += 1;
      let message = format!("polled {} times", counter);

      self.screen.draw_text(Point {x: 5, y: 6}, message.as_str());

      should_exit = match key {
        Some(k) => match k {
          Key::Esc => true,
          _ => false
        },
        None => false
      };
    }
  }

  fn sleep_and_poll_keyboard(&mut self) -> Option<Key> {
    let start = self.timer.now();
    let mut key = None;
    let mut passed_time = Duration::from_millis(0);
    let mut return_immediately = false;

    while !return_immediately && passed_time < self.interval {

      let remaining_time = self.interval - passed_time;
      let result = self.keyboard.poll(remaining_time); 

      match result {
        PollResult::KeyPressed(k) => {
          key = Some(k);
          return_immediately = match k {
            Key::Esc => true,
            _ => false
          }
        },
        PollResult::Err(msg) => {
          panic!("error: {:?}", msg);
        }
        PollResult::Timeout => {}
      };

      if !return_immediately {
        let end = self.timer.now();
        passed_time = self.timer.diff(start, end);
      }
    }

    key
  }
}
