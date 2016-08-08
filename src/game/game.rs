pub struct SnakeGame {
  field: Field,
  snake: Snake,
  apple_position: Point,
  grow_step_count: u16
}

impl<S: Screen> SnakeGame {

  pub fn step(&mut self, dir: RelativeDirection) -> StepResult {

    if self.grow_step_count > 0 {
      self.grow_step_count--;
    }
    else {
      self.snake.shrink_tail();
    }

    self.snake.grow_head();

    let head = self.snake.head();

    if head == self.apple_position {
      self.apple_position = self.new_apple();
      return CaughtApple;
    }

    if !self.field.is_passable(head) {
      return Collision;
    }

    Nothing
  }

  pub fn draw(&mut self, screen: &mut S) {
    screen.clear();
    self.field.draw(screen);
    self.snake.draw(screen);
    screen.draw_point(self.apple_position, Symbol::Apple);
  }

}

