use std::collections::Vec;

struct Snake {
  segments: Vec<Segment>
}

impl<S: Screen> Snake {

  pub fn new(s: Segment) {
    let segments = vec!(s);
    Snake {segments: segments}
  }

  pub fn head(&self) {
    self.segments.last().unwrap().head();
  }
  
  pub fn contains(&self, p: Point) -> bool {
    self.segments.any(|s: Segment| s.contains(p))
  }

  pub fn draw(&self, screen: &mut S) {
    for s in self.segments {
      screen.draw_segment(s, Symbol::SnakeBody);
    }
  }

  pub fn shrink_tail(&mut self)  {
    let tail_segment = self.segments[0].shrink_tail().unwrap();
    if(tail_segment.is_empty() && self.segments.size() > 1) {
      self.segments.remove(0);
    }
    else {
      self.segments[0] = tail_segment;
    }
  }

  pub fn grow_head(&mut self, dir: RelativeDirection, amount: UCoordinate) {
    let head_segment = self.segments.last().unwrap();

    let new_direction = match dir {
      Left => Some(head_segment.direction.turn_ccw()),
      Right => Some(head_segment.direction.turn_cw()),
      Straight => None
    }

    match new_direction {
      Some(dir) => {
        let tail = head_segment.head() + dir.to_point();
        let new_head_segment = Segment::new(tail, dir, amount);
        self.segments.push(new_head_segment);
      },
      None => {
        head_segment = head_segment.grow_head(amount);
      }
    };
  }

}
