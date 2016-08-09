use output::Screen;
use game::Game;
use geom::Size;
use input::Key;
use std::time::Duration;
use super::level::Level;
use super::field::Field;
use super::RelativeDirection;

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

pub struct SnakeGame {
  level: Level
}

impl SnakeGame {

  pub fn new(size: Size) -> SnakeGame {
    let field = Field::new(size);
    let level = Level::new(field);

    SnakeGame {
      level: level
    }
  }

}

impl Game for SnakeGame {

  fn update(&mut self, input: Option<Key>, passed_time: Duration) -> bool {
    let rel_dir = key_to_relative_direction(input);
    self.level.step(rel_dir);
    true
  }

  fn max_idle_time(&self) -> Duration {
    Duration::from_millis(200)
  }

  fn initial_draw<S: Screen>(&self, screen: &mut S) {
    self.level.full_draw(screen);
  }

}
