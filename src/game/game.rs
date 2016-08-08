use output::{Screen, Symbol};
use super::{StepResult, RelativeDirection};
use super::snake::Snake;
use super::field::Field;
use geom::{Point, Size};

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

  pub fn draw<S: Screen>(&mut self, screen: &mut S) {
    screen.clear();
    self.field.draw(screen);
    self.snake.draw(screen);
    screen.draw_point(self.apple_position, Symbol::Apple);
  }

  fn new_apple(&self) -> Point {
    return Point::new(1, 1);
  }

}

