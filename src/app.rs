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
    self.screen.draw_text(Point {x: 5, y: 10}, "Watching keyboard...");

    let mut should_exit = false;
    
    while !should_exit {
      let key = self.wait_for_keypress();
      should_exit = match key {
        Some(k) => match k {
          Key::Esc => true,
          _ => false
        },
        None => false
      };
    }

    self.screen.draw_text(Point {x:0, y: 15}, "");
  }

  fn wait_for_keypress(&mut self) -> Option<Key> {
    let start = self.timer.now();
    let mut key : Option<Key> = None;
    let mut remaining_time = self.interval.clone();

    while remaining_time > Duration::from_millis(0) {
      match self.keyboard.poll(remaining_time) {
        PollResult::KeyPressed(k) => {
          key = Some(k);
          //immediately return Esc key
          match k {
            Key::Esc => {return key;},
            _ => {}
          }
        },
        PollResult::Err(msg) => {
          panic!("error: {:?}", msg);
        }
        PollResult::Timeout => {}
      };
      let end = self.timer.now();
      remaining_time = self.timer.diff(start, end);
    }

    return key;
  }
}
