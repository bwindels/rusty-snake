pub enum StepResult {
	Collision,
	CaughtApple,
	Nothing
}

pub enum RelativeDirection {
	Straight,
	Left,
	Right
}

pub struct SnakeGame {
	field: Field,
	snake: Snake,
	apple_position: Point
}

impl<S: Screen> SnakeGame {

	pub fn step_and_draw(&mut self, dir: RelativeDirection, screen: &mut S) -> StepResult {
		//if not discounting steps for last apple eaten, shrink tail
		//if turning, append segment of length 1
		//else, grow current head segment
		//see if new head equals apple_position
			//new random apple position while not colliding with snake and field walls
			//draw apple at new position
		//draw clear previous tail point
		//draw current head
	}

	pub fn draw(&mut self, screen: &mut S) {
		screen.clear();
		self.field.draw(screen);
		self.snake.draw(screen);
		screen.draw_point(self.apple_position, Symbol::Apple);
	}

}

