use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone)]
pub struct Point {
  pub x: super::Coordinate,
  pub y: super::Coordinate
}

impl Mul<super::Coordinate> for Point {
    type Output = Point;

  fn mul(self, n: super::Coordinate) -> Point {
    Point {x: self.x * n, y: self.y * n}
  }
}

impl Add for Point {
  type Output = Point;

  fn add(self, b: Point) -> Point {
    Point {x: self.x + b.x, y: self.y + b.y}
  }
}

impl Sub for Point {
    type Output = Point;

  fn sub(self, b: Point) -> Point {
    Point {x: self.x - b.x, y: self.y - b.y}
  }
}
