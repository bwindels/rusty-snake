mod snake;
mod field;

pub enum StepResult {
  Collision,
  CaughtApple,
  Moved
}

pub enum RelativeDirection {
  Straight,
  Left,
  Right
}