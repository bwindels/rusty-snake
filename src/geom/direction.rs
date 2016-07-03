use super::Point;

#[derive(Copy, Clone)]
pub enum Orientation {
  Horizontal,
  Vertical
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
  North,
  East,
  South,
  West
}

impl Direction {

  pub fn turn_ccw(self) -> Direction {
    match self {
      Direction::North => Direction::West,
      Direction::West => Direction::South,
      Direction::South => Direction::East,
      Direction::East => Direction::North,
    }
  }

  pub fn turn_cw(self) -> Direction {
    match self {
      Direction::North => Direction::East,
      Direction::East => Direction::South,
      Direction::South => Direction::West,
      Direction::West => Direction::North,
    }
  }
  
  pub fn to_point(self) -> Point {
    match self {
      Direction::North => Point {x:  0, y:  1},
      Direction::South => Point {x:  0, y: -1},
      Direction::East  => Point {x:  1, y:  0},
      Direction::West  => Point {x: -1, y:  0},
    }
  }

  pub fn orientation(self) -> Orientation {
    match self {
      Direction::North | Direction::South => Orientation::Vertical,
      Direction::East | Direction::West => Orientation::Horizontal,
    }
  }
}

#[test]
fn turn_cw_4_times() {
  let d1 = Direction::East;
  let d2 = d1
    .turn_cw()
    .turn_cw()
    .turn_cw()
    .turn_cw();

  assert_eq!(d1, d2);
}

#[test]
fn turn_ccw_4_times() {
  let d1 = Direction::East;
  let d2 = d1
    .turn_ccw()
    .turn_ccw()
    .turn_ccw()
    .turn_ccw();

  assert_eq!(d1, d2);
}
