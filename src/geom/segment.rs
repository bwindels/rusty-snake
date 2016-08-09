use super::{UCoordinate, Coordinate, Orientation, Direction, Point};
use std::iter::{Iterator, IntoIterator};
use std::cmp::{min, max};

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
  
  pub fn shrink_tail(self) -> Option<Segment> {
    if self.length <= 1 {
      None
    }
    else {
      let s = Segment {
        tail: self.tail + self.direction.to_point(),
        length: self.length - 1,
        .. self
      };
      Some(s)
    }
  }

  pub fn is_empty(self) -> bool {
    self.length == 0
  }

  pub fn grow_head(self) -> Segment {
    Segment {length: self.length + 1, .. self}
  }

  pub fn length(self) -> UCoordinate {
    self.length
  }

  pub fn tail(self) -> Point {
    self.tail
  }

  pub fn head(self) -> Point {
    self.point_at(self.length - 1)
  }

  pub fn direction(self) -> Direction {
    self.direction
  }

  pub fn point_at(self, index: UCoordinate) -> Point {
    self.tail + (self.direction.to_point() * index as Coordinate)
  }

  pub fn contains(self, p: Point) -> bool {
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

  pub fn points(self) -> SegmentIterator {
    SegmentIterator::new(self)
  }
}

pub struct SegmentIterator {
  index: UCoordinate,
  segment: Segment
}

impl SegmentIterator {
  fn new(segment: Segment) -> SegmentIterator {
    SegmentIterator {index: segment.length, segment: segment}
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

#[test]
fn test_iterator() {
	let segment = Segment::new(Point::new(5,5), Direction::South, 3);
  let mut segment_iter = segment.points();

  assert_eq!(segment_iter.next().unwrap(), Point::new(5,7));
  assert_eq!(segment_iter.next().unwrap(), Point::new(5,6));
  assert_eq!(segment_iter.next().unwrap(), Point::new(5,5));
  assert_eq!(segment_iter.next(), None);
}

#[test]
fn test_contains() {
  let vertical = Segment::new(Point::new(5,5), Direction::South, 3);
  
  assert!(!vertical.contains(Point::new(4, 6)));
  assert!(!vertical.contains(Point::new(6, 6)));
  assert!(!vertical.contains(Point::new(5, 4)));
  assert!(!vertical.contains(Point::new(5, 8)));

  assert!(vertical.contains(Point::new(5, 5)));
  assert!(vertical.contains(Point::new(5, 6)));
  assert!(vertical.contains(Point::new(5, 7)));

  let horizontal = Segment::new(Point::new(5,5), Direction::East, 3);
  
  assert!(!horizontal.contains(Point::new(6, 4)));
  assert!(!horizontal.contains(Point::new(6, 6)));
  assert!(!horizontal.contains(Point::new(4, 5)));
  assert!(!horizontal.contains(Point::new(8, 5)));

  assert!(horizontal.contains(Point::new(5, 5)));
  assert!(horizontal.contains(Point::new(6, 5)));
  assert!(horizontal.contains(Point::new(7, 5)));
}

#[test]
fn test_shrink_tail() {
  let a = Segment::new(Point::new(5,5), Direction::South, 3);
  let b = a.shrink_tail().unwrap();
  assert_eq!(b.points().collect::<Vec<Point>>(), vec![Point::new(5,7), Point::new(5,6)]);
  let c = b.shrink_tail().unwrap();
  assert_eq!(c.points().collect::<Vec<Point>>(), vec![Point::new(5,7)]);
  let d = c.shrink_tail();
  assert!(d.is_none());
}

#[test]
fn test_grow_head() {
  let a = Segment::new(Point::new(5,5), Direction::South, 1);
  let b = a.grow_head();
  
  assert_eq!(
    b.points().collect::<Vec<Point>>(),
    vec![Point::new(5,6), Point::new(5,5)]
  );
}


