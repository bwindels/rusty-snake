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

	let keyboard_result_pos = Point {x: 5, y: 10};
	term.write(keyboard_result_pos, "Watching keyboard...");
	let pos2 = Point {y: keyboard_result_pos.y + 1, .. keyboard_result_pos};

	let mut keyboard = input::create_keyboard_poller().unwrap();
	match keyboard.poll(Duration::from_millis(5000)) {
		PollResult::KeyPressed(key) => term.write(pos2, format!("key pressed {}", key).as_str()),
		PollResult::Timeout => term.write(pos2, "timed out"),
		PollResult::Err(msg) => term.write(pos2, format!("error: {:?}", msg).as_str())
	};

	term.write(Point {x:0, y: 15}, "");
}