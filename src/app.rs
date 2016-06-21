use std::time::Duration;
use input::{Keyboard, PollResult, Key};
use output::Terminal;
use time::Timer;
use geom::Point;

pub struct SnakeApp<A, B, C> {
  pub keyboard: A,
  pub term: B,
  pub timer: C,
  pub interval: Duration,
}

impl<A: Keyboard, B: Terminal, C: Timer> SnakeApp<A, B, C> {

  pub fn run(&mut self) {
    self.term.clear();

    let size = self.term.size();
    let message = format!("the screen has height {} and width {}", size.height, size.width);
    self.term.write(Point {x: 5, y: 5}, message.as_str());
    self.term.write(Point {x: 5, y: 10}, "Watching keyboard...");

    let mut should_exit = false;
    
    while !should_exit {
      should_exit = self.wait_for_keypress();
    }

    self.term.write(Point {x:0, y: 15}, "");
  }

  fn wait_for_keypress(&mut self) -> bool {
    let mut should_exit = false;
    let start = self.timer.now();
    let pos2 = Point {x: 5, y: 11};

    match self.keyboard.poll(self.interval) {
      PollResult::Timeout => {
        self.term.write(pos2, "timed out");
      },
      PollResult::Err(msg) => {
        panic!("error: {:?}", msg);
      }
      PollResult::KeyPressed(key) => {
        should_exit = match key {
          Key::Esc => true,
          _ => false,
        };
        self.term.write_repeated(pos2, " ", 40);
        self.term.write(pos2, format!("key pressed {}", key).as_str())
      },
    };
    let end = self.timer.now();
    let elapsed = self.timer.diff(start, end);
    self.term.write(Point {x: 40, y: 11}, format!("elapsed time: {:?}", elapsed).as_str());

    should_exit
  }
}
