mod input;
mod output;
mod geom;
mod time;
mod gameloop;
mod game;
mod random;

use output::Terminal;

fn main() {

  let term = output::AnsiTerm::from_stdout().unwrap();
  let screen_size = term.size();

  let mut gameloop = gameloop::GameLoop {
    timer: time::create_timer(),
    screen: output::TermScreen::new(term),
    keyboard: input::create_keyboard().unwrap(),
    game: game::SnakeGame::new(screen_size, random::OSRandom {} )
  };

  gameloop.run();
}
