pub use self::snake::Snake;

mod snake;

pub enum StepResult {
  Collision,
  CaughtApple,
  Nothing
}

pub enum RelativeDirection {
  Straight,
  Left,
  Right
}