use output::Screen;
use game::Game;
use geom::{Size, Point};
use input::Key;
use std::time::Duration;
use super::level::Level;
use super::field::Field;
use super::{RelativeDirection, StepResult};

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
  level: Level,
  game_over: bool
}

impl SnakeGame {

  pub fn new(size: Size) -> SnakeGame {
    let field = Field::new(size);
    let level = Level::new(field);

    SnakeGame {
      level: level,
      game_over: false
    }
  }

}

impl Game for SnakeGame {

  fn update(&mut self, input: Option<Key>, passed_time: Duration) -> bool {

    if self.game_over {
      return false;
    }

    let rel_dir = key_to_relative_direction(input);
    let step_result = self.level.step(rel_dir);

    match step_result {
      StepResult::Collision => {self.game_over = true;},
      _ => {}
    };

    return true;
  }

  fn max_idle_time(&self) -> Option<Duration> {
    if self.game_over {
      None
    }
    else {
      Some(Duration::from_millis(200))
    }
  }

  fn initial_draw<S: Screen>(&self, screen: &mut S) {
    self.level.full_draw(screen);

    if self.game_over {
      screen.draw_text(Point::new(8, 5), "GAME OVER! HIT ESC TO LEAVE THE GAME");
    }
  }

}
