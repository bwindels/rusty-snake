use super::{UCoordinate, Orientation, Direction, Point};
use std::iter::{Iterator, IntoIterator};
use std::cmp::{min, max};

pub enum ShrinkResult {
  Empty {remaining_amount: UCoordinate},
  NonEmpty {new: Segment}
}

#[derive(Copy, Clone)]
pub struct Segment {
  tail: Point,
  direction: Direction,
  length: UCoordinate
}

impl Segment {

  pub fn new(tail: Point, dir: Direction, length: UCoordinate) -> Segment {
    Segment {
      tail: tail,
      direction: dir,
      length: length
    }
  }

  pub fn north(tail: Point, length: UCoordinate) -> Segment {
    Segment::new(tail, Direction::North, length)
  }

  pub fn east(tail: Point, length: UCoordinate) -> Segment {
    Segment::new(tail, Direction::East, length)
  }
  
  pub fn south(tail: Point, length: UCoordinate) -> Segment {
    Segment::new(tail, Direction::South, length)
  }
  
  pub fn west(tail: Point, length: UCoordinate) -> Segment {
    Segment::new(tail, Direction::West, length)
  }
  
  pub fn shrink_tail(&self, amount: UCoordinate) -> Option<Segment> {
    if self.is_empty() {
      None
    }
    else {
      let s = Segment {
        tail: self.tail + self.direction.to_point() * amount,
        length: self.length - 1,
        ..self
      };
      Some(s)
    }
  }

  pub fn is_empty(&self) -> bool {
    self.length == 0
  }

  pub fn grow_head(&self, amount: UCoordinate) {
    Segment {length: self.length + amount, ..self}
  }

  pub fn length(&self) -> UCoordinate {
    self.length
  }

  pub fn tail(&self) -> Point {
    self.tail
  }

  pub fn head(&self) -> Point {
    self.point_at(self.length - 1)
  }

  pub fn point_at(&self, index: UCoordinate) -> Point {
    self.tail + (self.direction.to_point() * index)
  }

  pub fn contains(&self, p: Point) -> bool {
    let tail = self.tail;
    let orientation = self.direction.orientation();
    let is_on_line = match orientation {
      Orientation::Vertical => p.x == tail.x,
      Orientation::Horizontal => p.y == tail.y,
    };

    if !is_on_line {
      return false;
    }

    let head = self.head();
    let (min, max, value) = match orientation {
      Orientation::Vertical => (min(tail.y, head.y), max(tail.y, head.y), p.y),
      Orientation::Horizontal => (min(tail.x, head.x), max(tail.x, head.x), p.x),
    };

    let is_on_segment = value <= max && value >= min;

    is_on_segment
  }
}

impl IntoIterator for Segment {
  type Item = Point;
    type IntoIter = SegmentIterator;

    fn into_iter(self) -> Self::IntoIter {
      SegmentIterator::new(self)
    }
}

pub struct SegmentIterator {
  index: UCoordinate,
  segment: Segment
}

impl SegmentIterator {
  fn new(segment: Segment) -> SegmentIterator {
    SegmentIterator {index: segment.length + 1, segment: segment}
  }
}

impl Iterator for SegmentIterator {
  type Item = Point;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index == 0 {
      Option::None
    }
    else {
      self.index -= 1;
      let point = self.segment.point_at(self.index);
      Option::Some(point)
    }
  }
}
