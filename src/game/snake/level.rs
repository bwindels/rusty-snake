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
  grow_step_count: u16,
  old_tail: Option<Point>
}

impl Level {

  pub fn new(field: Field) -> Level {
    let snake = Snake::new(field.initial_snake_segment());

    Level {
      field: field,
      snake: snake,
      apple_position: Point::new(1, 1),
      grow_step_count: 0,
      old_tail: None
    }
  }

  pub fn step<R: Random>(&mut self, dir: RelativeDirection, random: &mut R) -> StepResult {

    self.snake.grow_head(dir);
    let head = self.snake.head();

    if self.grow_step_count != 0 {
      self.grow_step_count -= 1;
      self.old_tail = None;
    }
    else {
      //store previous tail to clear it when drawing incrementally
      let old_tail = self.snake.tail();
      //but only if the head is not in the location of the old tail (biting it's own tail)
      if old_tail != head {
        self.old_tail = Some(old_tail);
      }

      self.snake.shrink_tail();
    }

    if head == self.apple_position {
      self.apple_position = self.new_apple(random);
      self.grow_step_count = 5;
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

  pub fn incremental_draw<S: Screen>(&self, screen: &mut S) {
    let new_head = self.snake.head();
    screen.draw_point(new_head, Symbol::SnakeBody);

    if let Some(old_tail) = self.old_tail {
      screen.draw_point(old_tail, Symbol::Clear);
    }

    screen.draw_point(self.apple_position, Symbol::Apple);
  }

}
