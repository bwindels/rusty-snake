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
      let key = self.wait_for_keypress();
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

  fn wait_for_keypress(&mut self) -> Option<Key> {
    let start = self.timer.now();
    let mut key : Option<Key> = None;
    let mut passed_time = Duration::from_millis(0);
    let mut return_immediately = false;

    while !return_immediately && passed_time < self.interval {

      match self.keyboard.poll(self.interval - passed_time) {
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
