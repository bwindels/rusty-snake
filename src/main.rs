mod input;
mod output;
mod geom;
mod time;

use input::{PollResult, Key};
use std::time::Duration;
use output::ansiterm::AnsiTerm;
use geom::Point;
use time::Clock;

fn main() {
	let mut term = AnsiTerm::from_stdout().unwrap();
	term.clear();
	let message = format!("the terminal has {} rows and {} columns", term.rows(), term.columns());
	term.write(Point {x: 5, y: 5}, message.as_str());

	let keyboard_result_pos = Point {x: 5, y: 10};
	term.write(keyboard_result_pos, "Watching keyboard...");
	let pos2 = Point {y: keyboard_result_pos.y + 1, .. keyboard_result_pos};

	let mut keyboard = input::create_keyboard_poller().unwrap();

	let mut should_exit = false;
	let clock = Clock::new();

	while !should_exit {
		let start = clock.now();
		match keyboard.poll(Duration::from_millis(1000)) {
			PollResult::Timeout => {
				term.write(pos2, "timed out");
			},
			PollResult::Err(msg) => {
				panic!("error: {:?}", msg);
			}
			PollResult::KeyPressed(key) => {
				should_exit = match key {
					Key::Esc => true,
					_ => false,
				};
				term.write_repeated(pos2, " ", 40);
				term.write(pos2, format!("key pressed {}", key).as_str())
			},
		};
		let end = clock.now();
		let elapsed = clock.duration(start, end);
		term.write(Point {x: 40, y: 11}, format!("elapsed time: {:?}", elapsed).as_str());
	}

	term.write(Point {x:0, y: 15}, "");
}