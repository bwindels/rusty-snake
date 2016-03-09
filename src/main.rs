mod input;
mod output;
mod geom;

use input::PollResult;
use std::time::Duration;
use output::ansiterm::AnsiTerm;
use geom::Point;

fn main() {
	let mut term = AnsiTerm::from_stdout().unwrap();
	term.clear();
	let message = format!("the terminal has {} rows and {} columns", term.rows(), term.columns());
	term.write(Point {x: 5, y: 5}, message.as_str());

	let mut keyboard = input::create_keyboard_poller().unwrap();
	match keyboard.poll(Duration::from_millis(1000)) {
		PollResult::KeyPressed(key) => println!("key pressed {}", key),
		PollResult::Timeout => println!("timed out"),
		PollResult::Err(msg) => println!("error: {:?}", msg)
	};
}