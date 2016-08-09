use std::time::Duration;
use input::{Keyboard, PollResult, Key};
use output::Screen;
use time::Timer;
use game::Game;

pub struct GameLoop<A, B, C, G> {
  pub keyboard: A,
  pub screen: B,
  pub timer: C,
  pub game: G
}

impl<A: Keyboard, B: Screen, C: Timer, G: Game> GameLoop<A, B, C, G> {

  pub fn run(&mut self) {
    self.screen.clear();
    self.game.full_draw(&mut self.screen);

    let mut last_update_time = self.timer.now();
    let mut should_exit = false;

    while !should_exit {
      let timeout = self.game.max_idle_time();

      let key = self.sleep_and_poll_input(timeout);
      should_exit = match key {
        Some(k) => match k {
          Key::Esc => true,
          _ => false
        },
        None => false
      };

      let start = last_update_time;
      last_update_time = self.timer.now();
      let passed_time = self.timer.diff(start, last_update_time);

      if self.game.update(key, passed_time) {
        self.game.incremental_draw(&mut self.screen);
      }
    }

    self.screen.clear();
  }

  fn sleep_and_poll_input(&mut self, timeout: Option<Duration>) -> Option<Key> {
    match self.keyboard.poll(timeout) {
      PollResult::KeyPressed(k) => Some(k),
      PollResult::Timeout => None,
      PollResult::Err(msg) => {
        panic!("error: {:?}", msg);
      }
    }
  }
}
