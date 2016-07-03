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
      should_exit = self.wait_for_keypress();
    }

    self.screen.draw_text(Point {x:0, y: 15}, "");
  }

  fn wait_for_keypress(&mut self) -> bool {
    let mut should_exit = false;
    let start = self.timer.now();
    let pos2 = Point {x: 5, y: 11};

    match self.keyboard.poll(self.interval) {
      PollResult::Timeout => {
        self.screen.draw_text(pos2, "timed out");
      },
      PollResult::Err(msg) => {
        panic!("error: {:?}", msg);
      }
      PollResult::KeyPressed(key) => {
        should_exit = match key {
          Key::Esc => true,
          _ => false,
        };

        {
          let spaces = String::from_utf8(vec![b' '; 40]).unwrap();
          self.screen.draw_text(pos2, &spaces);
        }
        
        self.screen.draw_text(pos2, format!("key pressed {}", key).as_str())
      },
    };
    let end = self.timer.now();
    let elapsed = self.timer.diff(start, end);
    self.screen.draw_text(Point {x: 40, y: 11}, format!("elapsed time: {:?}", elapsed).as_str());

    should_exit
  }
}
