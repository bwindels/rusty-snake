type Coordinate = super::Coordinate;
use super::{Orientation, Direction, Point};
use std::iter::{Iterator, IntoIterator}
use std::cmp::{min, max};

#[derive(Copy, Clone)]
pub struct Segment {
	tail: Point,
	direction: Direction,
	length: Coordinate
}

impl Segment {

	pub fn shrink_tail(&mut self, amount: Coordinate) {
		self.tail = self.tail + self.direction.to_point() * amount;
		self.length -= amount;
	}

	pub fn shrink_head(&mut self, amount: Coordinate) {
		self.length -= amount;
	}

	pub fn grow_tail(&mut self, amount: Coordinate) {
		self.tail = self.tail + self.direction.to_point() * (-amount);
		self.length += amount;
	}

	pub fn grow_head(&mut self, amount: Coordinate) {
		self.length += amount;
	}

	pub fn length(&self) -> Coordinate {
		self.length
	}

	pub fn tail(&self) -> Point {
		self.tail
	}

	pub fn head(&self) -> Point {
		self.point_at(self.length - 1)
	}

	pub fn point_at(&self, index: Coordinate) -> Point {
		self.tail + self.direction.to_point() * index
	}

	pub fn contains(&self, p: Point) -> bool {
		let tail = self.tail;
		let orientation = self.direction.orientation();
		let is_on_line = match orientation {
			Vertical => p.x == tail.x,
			Horizontal => p.y == tail.y,
		}

		if !is_on_line {
			false
		}

		let head = self.head();
		let (min, max, value) = match orientation {
			Vertical => (min(tail.y, head.y), max(tail.y, head.y), p.y),
			Horizontal => (min(tail.x, head.x), max(tail.x, head.x), p.x),
		}

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

struct SegmentIterator {
	index: Coordinate,
	segment, Segment
}

impl SegmentIterator {
	fn new(segment: Segment) {
		SegmentIterator {index: segment.length + 1, segment: segment}
	}
}

impl Iterator for SegmentIterator {
	type Item = Point;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index == 0 {
			Option::None()
		}
		else {
			self.index -= 1;
			let point = self.segment.point_at(self.index);
			Option::Some(point)
		}
	}
}
