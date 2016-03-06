mod input;
mod output;

use input::PollResult;
use std::time::Duration;

fn main() {
	let term = output::ansiterm::AnsiTerm::from_stdout();
	term.clear();

	let mut keyboard = input::create_keyboard_poller().unwrap();
	match keyboard.poll(Duration::from_millis(1000)) {
		PollResult::KeyPressed(key) => println!("key pressed {}", key),
		PollResult::Timeout => println!("timed out"),
		PollResult::Err(msg) => println!("error: {:?}", msg)
	};
}