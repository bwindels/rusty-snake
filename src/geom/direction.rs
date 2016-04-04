use super::Point;

#[derive(Copy, Clone)]
pub enum Orientation {
	Horizontal,
	Vertical
}

#[derive(Copy, Clone)]
pub enum Direction {
	North,
	East,
	South,
	West
}

impl Direction {

	fn turn_ccw(&self) -> Direction {
		match self {
			North => West,
			West => South,
			South => East,
			East => North,
		}
	}

	fn turn_cw(&self) -> Direction {
		match self {
			North => East,
			East => South,
			South => West,
			West => North,
		}
	}
	
	fn to_point(&self) -> Point {
		match self {
			North: Point {x:  0, y:  1},
			South: Point {x:  0, y: -1},
			East:  Point {x:  1, y:  0},
			West:  Point {x: -1, y:  0},
		}
	}

	fn orientation() -> Orientation {
		match self {
			North | South => Vertical,
			East | West => Horizontal,
		}
	}
}