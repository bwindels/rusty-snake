use std::time::Duration;
use std::iter;
use input::{Keyboard, PollResult, Key};
use output::Screen;
use time::Timer;
use geom::Point;
use game::{SnakeGame, RelativeDirection};

fn key_to_relative_direction(key_option: Option<Key>) -> RelativeDirection {
  match key_option {
    Some(key) => match key {
      Key::Left => RelativeDirection::Left,
      Key::Right => RelativeDirection::Right,
      _ => RelativeDirection::Straight
    },
    None => RelativeDirection::Straight
  }
}


pub struct SnakeApp<A, B, C> {
  pub keyboard: A,
  pub screen: B,
  pub timer: C,
  pub interval: Duration,
  pub game: SnakeGame
}

impl<A: Keyboard, B: Screen, C: Timer> SnakeApp<A, B, C> {

  pub fn run(&mut self) {
    self.screen.clear();
    
    self.game.draw(&mut self.screen);

    let mut should_exit = false;
    while !should_exit {
      let key = self.sleep_and_poll_keyboard();
      should_exit = match key {
        Some(k) => match k {
          Key::Esc => true,
          _ => false
        },
        None => false
      };
      self.game.step(key_to_relative_direction(key));
      self.game.draw(&mut self.screen);
    }

    self.screen.clear();
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
