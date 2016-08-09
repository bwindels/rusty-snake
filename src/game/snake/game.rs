use output::{Screen, Symbol};
use super::snake::Snake;
use super::field::Field;
use game::Game;
use geom::{Point, Size};
use input::Key;
use std::time::Duration;

enum StepResult {
  Collision,
  CaughtApple,
  Moved
}

pub enum RelativeDirection {
  Straight,
  Left,
  Right
}

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
  field: Field,
  snake: Snake,
  apple_position: Point,
  grow_step_count: u16
}

impl SnakeGame {

  pub fn new(size: Size) -> SnakeGame {
    let field = Field::new(size);
    let snake = Snake::new(field.initial_snake_segment());

    SnakeGame {
      field: field,
      snake: snake,
      apple_position: Point::new(1, 1),
      grow_step_count: 0
    }
  }

  pub fn step(&mut self, dir: RelativeDirection) -> StepResult {

    if self.grow_step_count != 0 {
      self.grow_step_count -= 1;
    }
    else {
      self.snake.shrink_tail();
    }

    self.snake.grow_head(dir);

    let head = self.snake.head();

    if head == self.apple_position {
      self.apple_position = self.new_apple();
      return StepResult::CaughtApple;
    }

    if !self.field.is_passable(head) {
      return StepResult::Collision;
    }

    StepResult::Moved
  }

  fn new_apple(&self) -> Point {
    return Point::new(1, 1);
  }

}

impl Game for SnakeGame {

  fn update(&mut self, input: Option<Key>, passed_time: Duration) -> bool {
    let rel_dir = key_to_relative_direction(input);
    self.step(rel_dir);
    true
  }

  fn max_idle_time(&self) -> Duration {
    Duration::from_millis(200)
  }

  fn initial_draw<S: Screen>(&self, screen: &mut S) {
    screen.clear();
    self.field.draw(screen);
    self.snake.draw(screen);
    screen.draw_point(self.apple_position, Symbol::Apple);
  }

}
