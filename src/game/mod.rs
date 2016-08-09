use std::time::Duration;
use input::Key;
use output::Screen;

pub use self::snake::SnakeGame;

mod snake;

pub trait Game {
	fn max_idle_time(&self) -> Duration;
	fn update(&mut self, input: Option<Key>, passed_time: Duration) -> bool;
	fn initial_draw<S: Screen>(&self, screen: &mut S);
	fn incremental_draw<S: Screen>(&self, screen: &mut S) {
		self.initial_draw(screen);
	}
}