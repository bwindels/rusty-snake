use output::{Screen, Symbol};
use geom::{Size, Point, Segment, Coordinate, UCoordinate};

pub struct Field {
  size: Size
}

impl Field {

  pub fn new(size: Size) -> Field {
    Field {
      size: size
    }
  }

  pub fn draw<S: Screen>(&self, screen: &mut S) {
    let sym = Symbol::Wall;
    screen.draw_segment(Segment::east(Point::new(0,0), self.size.width), sym);
    screen.draw_segment(Segment::south(Point::new(0,1), self.size.height - 2), sym);
    screen.draw_segment(Segment::south(Point::new(self.size.width as Coordinate - 1,1), self.size.height - 2), sym);
    screen.draw_segment(Segment::east(Point::new(0,self.size.height as Coordinate - 1), self.size.width), sym);
  }

  pub fn is_passable(&self, pos: Point) -> bool {
    if pos.x == 0 || pos.x == (self.size.width as Coordinate - 1) {
      return false;
    }
    if pos.y == 0 || pos.y == (self.size.height as Coordinate - 1) {
      return false;
    }
    true
  }

  pub fn initial_snake_segment(&self) -> Segment {
    let length = 20 as UCoordinate;
    let tail = Point {
      x: (self.size.width as Coordinate / 2) + (length as Coordinate / 2),
      y: self.size.height as Coordinate / 2
    };
    Segment::west(tail, length)
  }

  pub fn size(&self) -> Size {
    self.size
  }
}
