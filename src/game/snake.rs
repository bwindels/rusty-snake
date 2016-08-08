use std::vec::Vec;
use geom::{Segment, Point, UCoordinate};
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

  pub fn head(&self) -> Point {
    self.segments.last().unwrap().head()
  }
  
  pub fn contains(&self, p: Point) -> bool {
    self.segments.iter().any(|s: &Segment| s.contains(p))
  }

  pub fn draw<S: Screen>(&self, screen: &mut S) {
    for s in &self.segments {
      screen.draw_segment(*s, Symbol::SnakeBody);
    }
  }

  pub fn shrink_tail(&mut self)  {
    let tail_segment = self.segments[0].shrink_tail().unwrap();
    if tail_segment.is_empty() && self.segments.len() > 1 {
      self.segments.remove(0);
    }
    else {
      self.segments[0] = tail_segment;
    }
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
