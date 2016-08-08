mod input;
mod output;
mod geom;
mod time;
mod app;
mod game;

use std::time::Duration;

fn main() {

  let term = output::AnsiTerm::from_stdout().unwrap();

  let mut app = app::SnakeApp {
    timer: time::create_timer(),
    screen: output::TermScreen::new(term),
    keyboard: input::create_keyboard().unwrap(),
    interval: Duration::from_millis(1000)
  };

  app.run();
}
