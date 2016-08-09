use output::{Screen, Symbol};
use super::snake::Snake;
use super::field::Field;
use super::{RelativeDirection, StepResult};
use geom::{Point, Coordinate};
use random::Random;

pub struct Level {
  field: Field,
  snake: Snake,
  apple_position: Point,
  grow_step_count: u16
}

impl Level {

  pub fn new(field: Field) -> Level {
    let snake = Snake::new(field.initial_snake_segment());

    Level {
      field: field,
      snake: snake,
      apple_position: Point::new(1, 1),
      grow_step_count: 0
    }
  }

  pub fn step<R: Random>(&mut self, dir: RelativeDirection, random: &mut R) -> StepResult {

    if self.grow_step_count != 0 {
      self.grow_step_count -= 1;
    }
    else {
      self.snake.shrink_tail();
    }

    self.snake.grow_head(dir);

    let head = self.snake.head();

    if head == self.apple_position {
      self.apple_position = self.new_apple(random);
      return StepResult::CaughtApple;
    }

    if !self.field.is_passable(head) {
      return StepResult::Collision;
    }

    if self.snake.head_collides_with_body() {
      return StepResult::Collision;
    }

    StepResult::Moved
  }

  pub fn reset<R: Random>(&mut self, random: &mut R) {
    self.apple_position = self.new_apple(random);
  }

  fn new_apple<R: Random>(&self, random: &mut R) -> Point {
    let mut p = Point::new(0, 0);
    let mut intersects = true;
    let max = self.field.size();
    while intersects {
      p = random.new_point(max);
      intersects = !self.field.is_passable(p) || self.snake.contains(p);
    }
    return p;
  }

  pub fn full_draw<S: Screen>(&self, screen: &mut S) {
    screen.clear();
    self.field.draw(screen);
    self.snake.draw(screen);
    screen.draw_point(self.apple_position, Symbol::Apple);
  }

}
