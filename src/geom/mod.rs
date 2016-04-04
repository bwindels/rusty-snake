pub type Coordinate = u16;

#[derive(Copy, Clone)]
pub struct Point {
	pub x: Coordinate,
	pub y: Coordinate
}

#[derive(Copy, Clone)]
pub struct Size {
	pub width: Coordinate,
	pub height: Coordinate
}
