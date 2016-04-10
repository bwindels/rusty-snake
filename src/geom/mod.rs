pub type Coordinate = i16;
pub type UCoordinate = u16;

#[derive(Copy, Clone)]
pub struct Size {
	pub width: UnsignedCoordinate,
	pub height: UnsignedCoordinate
}

mod point;
mod segment;
mod direction;

pub use self::point::Point;
pub use self::direction::{Direction, Orientation};
pub use self::segment::Segment;
