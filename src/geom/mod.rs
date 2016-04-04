pub type Coordinate = u16;
pub type SignedCoordinate = i16;

#[derive(Copy, Clone)]
pub struct Size {
	pub width: Coordinate,
	pub height: Coordinate
}

pub use point::Point;
pub use direction::{Direction, Orientation};
pub use segment::Segment;
