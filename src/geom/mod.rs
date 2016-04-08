pub type Coordinate = u16;
pub type SignedCoordinate = i16;

#[derive(Copy, Clone)]
pub struct Size {
	pub width: Coordinate,
	pub height: Coordinate
}

mod point;
mod segment;
mod direction;

pub use self::point::Point;
pub use self::direction::{Direction, Orientation};
pub use self::segment::Segment;
