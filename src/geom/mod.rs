pub use self::point::Point;
pub use self::direction::{Direction, Orientation};
pub use self::segment::Segment;

mod point;
mod segment;
mod direction;

/**
The coordinate system for game logic is defined
as 0,0 in the top-left (north-west) corner and 
w-1,h-1 in the lower-right (south-east) corner of the screen.

This is expressed in Direction::to_point and assumed
in the rest of the geom and game code.
*/
pub type Coordinate = i16;
pub type UCoordinate = u16;

#[derive(Copy, Clone)]
pub struct Size {
  pub width: UCoordinate,
  pub height: UCoordinate
}

