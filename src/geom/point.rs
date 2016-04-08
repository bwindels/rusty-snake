use std::ops::{Add, Sub, Mul};
type Coordinate = super::Coordinate;
type SignedCoordinate = super::SignedCoordinate;

#[derive(Copy, Clone)]
pub struct Point {
	pub x: Coordinate,
	pub y: Coordinate
}

impl Mul<Coordinate> for Point {
    type Output = Point;

	fn Mul(&self, n: Coordinate) -> Point {
		Point {x: self.x * n, y: self.y * y}
	}
}

impl Mul<SignedCoordinate> for Point {
    type Output = Point;

	fn Mul(&self, n: SignedCoordinate) -> Point {
		Point {x: self.x * n, y: self.y * y}
	}
}

impl Add for Point {
	type Output = Point;

	fn Add(&self, b: Point) -> Point {
		Point {x: self.x + b.x, y: self.y + b.y}
	}
}

impl Sub for Point {
    type Output = Point;

	fn Sub(&self, b: Point) -> Point {
		Point {x: self.x - b.x, y: self.y - b.y}
	}
}
