mod input;
mod output;
mod geom;
mod time;
mod app;

use app::SnakeApp;
use input::Keyboard;
use output::Terminal;
use output::ansiterm::AnsiTerm;
use std::time::Duration;
use time::Timer;

fn main() {

	let ansiterm = AnsiTerm::from_stdout().unwrap();
	let term = Box::new(ansiterm) as Box<Terminal>;
	let keyboard = input::create_keyboard().unwrap();
	let timer = time::create_timer();

	let mut app = SnakeApp {
		timer: timer,
		term: term,
		keyboard: keyboard,
		interval: Duration::from_millis(1000)
	};

	app.run();
}
