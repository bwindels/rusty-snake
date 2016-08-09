pub use self::game::SnakeGame;

mod field;
mod snake;
mod game;
mod level;

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