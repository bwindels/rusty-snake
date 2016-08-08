pub use self::game::SnakeGame;

mod snake;
mod field;
mod game;

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