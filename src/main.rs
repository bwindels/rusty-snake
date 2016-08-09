mod input;
mod output;
mod geom;
mod time;
mod app;
mod game;

use std::time::Duration;
use output::Terminal;

fn main() {

  let term = output::AnsiTerm::from_stdout().unwrap();
  let screen_size = term.size();

  let mut app = app::SnakeApp {
    timer: time::create_timer(),
    screen: output::TermScreen::new(term),
    keyboard: input::create_keyboard().unwrap(),
    interval: Duration::from_millis(500),
    game: game::SnakeGame::new(screen_size)
  };

  app.run();
}
