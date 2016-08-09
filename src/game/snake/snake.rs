use std::vec::Vec;
use geom::{Segment, Point};
use output::{Symbol, Screen};
use super::RelativeDirection;

pub struct Snake {
  segments: Vec<Segment>
}

impl Snake {

  pub fn new(s: Segment) -> Snake {
    let segments = vec!(s);
    Snake {segments: segments}
  }

  pub fn head_collides_with_body(&self) -> bool {
    let head = self.head();
    let mut other_segments = self.segments.iter().rev().skip(1);
    other_segments.any(|s: &Segment| s.contains(head))
  }

  pub fn contains(&self, p: Point) -> bool {
    self.segments.iter().any(|s: &Segment| s.contains(p))
  }

  pub fn head(&self) -> Point {
    self.segments.last().unwrap().head()
  }

  pub fn draw<S: Screen>(&self, screen: &mut S) {
    for s in &self.segments {
      screen.draw_segment(*s, Symbol::SnakeBody);
    }
  }

  pub fn points(&self) -> Vec<Point> {
  	self.segments.iter().rev().flat_map(|s| s.points()).collect()
  }

  pub fn shrink_tail(&mut self)  {
    let tail_segment = self.segments[0];

    match tail_segment.shrink_tail() {
    	Some(s) => {
    		self.segments[0] = s;
    	},
    	None => {
    		self.segments.remove(0);
    	}
    };
  }

  pub fn grow_head(&mut self, dir: RelativeDirection) {
  	let segments = &mut self.segments;
    let head_segment = *segments.last().unwrap();

    let new_direction = match dir {
      RelativeDirection::Left => Some(head_segment.direction().turn_ccw()),
      RelativeDirection::Right => Some(head_segment.direction().turn_cw()),
      RelativeDirection::Straight => None
    };

    match new_direction {
      Some(dir) => {
        let tail = head_segment.head() + dir.to_point();
        let new_head_segment = Segment::new(tail, dir, 1);
        segments.push(new_head_segment);
      },
      None => {
        *segments.last_mut().unwrap() = head_segment.grow_head();
      }
    };
  }

}

#[test]
fn test_grow_head_left() {
	let mut snake = Snake::new(Segment::south(Point::new(0,0), 2));
	snake.grow_head(RelativeDirection::Left);
	assert_eq!(
		snake.points(),
		[Point::new(1, 1), Point::new(0, 1), Point::new(0, 0)]
	);
}

#[test]
fn test_grow_head_right() {
	let mut snake = Snake::new(Segment::east(Point::new(0,0), 2));
	snake.grow_head(RelativeDirection::Right);
	assert_eq!(
		snake.points(),
		[Point::new(1, 1), Point::new(1, 0), Point::new(0, 0)]
	);
}

#[test]
fn test_grow_head_straight() {
	let mut snake = Snake::new(Segment::north(Point::new(0,0), 2));
	snake.grow_head(RelativeDirection::Straight);
	assert_eq!(
		snake.points(),
		[Point::new(0, -2), Point::new(0, -1), Point::new(0, 0)]
	);
}

#[test]
fn test_shrink_tail_shorten_segment() {
	let mut snake = Snake::new(Segment::east(Point::new(0,0), 2));
	snake.shrink_tail();
	assert_eq!(
		snake.points(),
		[Point::new(1, 0)]
	);
}

#[test]
fn test_shrink_tail_remove_segment() {
	let mut snake = Snake::new(Segment::east(Point::new(0,0), 1));
	snake.grow_head(RelativeDirection::Left);
	snake.shrink_tail();
	assert_eq!(
		snake.points(),
		[Point::new(0, -1)]
	);
}
