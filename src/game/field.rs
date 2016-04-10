pub struct Field {
	size: Size
}

impl<S: Screen> Field {

	fn draw(&self, screen: &mut S) {
		let sym = Symbol::Wall;
		screen.draw_segment(Segment::east(Point::new(0,0), self.size.width), sym);
		screen.draw_segment(Segment::south(Point::new(0,1), self.size.heigth - 2), sym);
		screen.draw_segment(Segment::south(Point::new(self.size.width - 1,1), self.size.heigth - 2), sym);
		screen.draw_segment(Segment::east(Point::new(0,self.size.heigth - 1), self.size.width), sym);
	}

	fn is_passable(&self, pos: Point) -> bool {
		if pos.x == 0 || pos.x == (self.width - 1) {
			return false;
		}
		if pos.y == 0 || pos.y == (self.heigth - 1) {
			return false;
		}
		true
	}

	fn initial_snake(&self) -> Snake {
		let length = 10;
		tail = Point {
			x: (self.size.width / 2) + (length / 2),
			y: self.size.heigth / 2
		};
		Snake::new(Segment::new(tail, Direction::West, length))
	}
}