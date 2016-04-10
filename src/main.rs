mod input;
mod output;
mod geom;
mod time;
mod app;

use app::SnakeApp;
use output::ansiterm::AnsiTerm;
use std::time::Duration;

fn main() {

	let mut app = SnakeApp {
		timer: time::create_timer(),
		term: AnsiTerm::from_stdout().unwrap(),
		keyboard: input::create_keyboard().unwrap(),
		interval: Duration::from_millis(1000)
	};

	app.run();
}
